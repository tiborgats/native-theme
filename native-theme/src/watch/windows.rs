//! Windows theme change watcher using the `UISettings` change events.
//!
//! Creates a background thread with COM STA initialization and a
//! `GetMessageW`/`DispatchMessageW` message pump. The WinRT events
//! `UISettings::ColorValuesChanged`, `TextScaleFactorChanged`,
//! `AnimationsEnabledChanged` and `AdvancedEffectsEnabledChanged` and
//! `AccessibilitySettings::HighContrastChanged` fire whenever the user changes
//! the system colours, the text scale, the animations, the transparency
//! effects or high contrast; their handlers post a thread message to the
//! pump, which fires the callback on the watcher thread. Shutdown is handled
//! by posting
//! `WM_QUIT` to the watcher thread via `PostThreadMessageW` from the
//! `ThemeSubscription` Drop handler.

// Windows FFI via the `windows` crate -- no safe alternative for COM/WinRT bindings.
// This follows the same pattern as src/windows.rs (the theme reader).
#![allow(unsafe_code)]

use std::marker::PhantomData;
use std::sync::mpsc;

use ::windows::Foundation::TypedEventHandler;
use ::windows::UI::ViewManagement::{
    AccessibilitySettings, UISettings, UISettingsAnimationsEnabledChangedEventArgs,
};
use ::windows::Win32::Foundation::{LPARAM, WPARAM};
use ::windows::Win32::System::Com::{COINIT_APARTMENTTHREADED, CoInitializeEx, CoUninitialize};
use ::windows::Win32::System::Threading::GetCurrentThreadId;
use ::windows::Win32::UI::WindowsAndMessaging::{
    DispatchMessageW, GetMessageW, KillTimer, MSG, PM_NOREMOVE, PeekMessageW, PostThreadMessageW,
    SetTimer, WM_APP, WM_QUIT, WM_TIMER, WM_USER,
};
use ::windows::core::{IInspectable, Ref};

use super::ThemeChangeEvent;

/// The WinRT events the watcher subscribes to: the Windows reader's sources
/// (`windows.rs`, "Sources").
#[cfg(test)]
pub(super) const SUBSCRIBED_EVENTS: [&str; 5] = [
    COLOR_VALUES_CHANGED,
    TEXT_SCALE_FACTOR_CHANGED,
    HIGH_CONTRAST_CHANGED,
    ANIMATIONS_ENABLED_CHANGED,
    ADVANCED_EFFECTS_ENABLED_CHANGED,
];

/// `UISettings::ColorValuesChanged`, for `UISettings.GetColorValue`.
const COLOR_VALUES_CHANGED: &str = "ColorValuesChanged";

/// `UISettings::TextScaleFactorChanged`, for `UISettings.TextScaleFactor`.
const TEXT_SCALE_FACTOR_CHANGED: &str = "TextScaleFactorChanged";

/// `AccessibilitySettings::HighContrastChanged`, for `SPI_GETHIGHCONTRAST`.
const HIGH_CONTRAST_CHANGED: &str = "HighContrastChanged";

/// `UISettings::AnimationsEnabledChanged`, for `SPI_GETCLIENTAREAANIMATION`.
#[cfg(test)]
const ANIMATIONS_ENABLED_CHANGED: &str = "AnimationsEnabledChanged";

/// `UISettings::AdvancedEffectsEnabledChanged`, for
/// `UISettings.AdvancedEffectsEnabled`.
#[cfg(test)]
const ADVANCED_EFFECTS_ENABLED_CHANGED: &str = "AdvancedEffectsEnabledChanged";

/// The thread message an event handler posts to the watcher thread.
const WM_THEME_CHANGED: u32 = WM_APP;

/// Post [`WM_THEME_CHANGED`] to the watcher thread `thread_id`, from an event
/// handler on whatever thread WinRT raises the event.
fn post_theme_changed(thread_id: u32) {
    // SAFETY: PostThreadMessageW is safe to call from any thread; the
    // watcher thread's message queue exists before any handler is
    // registered. A failure (the thread has ended) loses only this message.
    unsafe {
        let _ = PostThreadMessageW(thread_id, WM_THEME_CHANGED, WPARAM(0), LPARAM(0));
    }
}

/// Balances a successful `CoInitializeEx` when dropped. Declared before the
/// WinRT objects of the watcher thread, so it drops after them: every COM
/// object is released before `CoUninitialize` unloads the thread's DLLs.
/// The raw-pointer marker makes it `!Send`, so it stays on the thread that
/// initialized COM.
struct ComGuard(PhantomData<*const ()>);

impl Drop for ComGuard {
    fn drop(&mut self) {
        // SAFETY: A `ComGuard` exists only after `CoInitializeEx` succeeded
        // on this thread, and it is `!Send`, so it drops on that thread.
        unsafe { CoUninitialize() };
    }
}

/// Spawn a background thread that watches for Windows theme changes.
///
/// Initializes COM as STA, subscribes to `UISettings::ColorValuesChanged`,
/// `TextScaleFactorChanged`, `AnimationsEnabledChanged` and
/// `AdvancedEffectsEnabledChanged` and to
/// `AccessibilitySettings::HighContrastChanged`, and runs a
/// `GetMessageW`/`DispatchMessageW` message pump. The callback fires on the
/// background thread, once per burst of events, 300ms after the last of them
/// (a thread timer the pump receives), whenever the system colours change
/// (e.g. the user switches between light and dark mode or picks an accent
/// colour), or the text scale, the animations setting, the transparency
/// effects setting or high contrast does.
///
/// Returns once the thread has its message queue and the subscriptions are
/// in place, or with `Error::ReaderFailed` when a setup step failed.
pub(crate) fn watch_windows(
    callback: impl Fn(ThemeChangeEvent) + Send + 'static,
) -> crate::Result<super::ThemeSubscription> {
    let (shutdown_tx, _shutdown_rx) = mpsc::channel::<()>();

    // Channel for the background thread to send back its OS thread ID, or
    // the setup step that failed.
    let (ready_tx, ready_rx) = mpsc::channel::<crate::Result<u32>>();

    let thread = std::thread::spawn(move || {
        let setup_failed = |step: &str, err: ::windows::core::Error| crate::Error::ReaderFailed {
            reader: "windows-watcher",
            source: format!("{step} failed: {err}").into(),
        };

        // Initialize COM as Single-Threaded Apartment (required for WinRT
        // UISettings on a background thread).
        // SAFETY: CoInitializeEx is safe to call once per thread. We pass
        // COINIT_APARTMENTTHREADED for STA mode.
        let com_result = unsafe { CoInitializeEx(None, COINIT_APARTMENTTHREADED) };
        if let Err(err) = com_result.ok() {
            let _ = ready_tx.send(Err(setup_failed("CoInitializeEx", err)));
            return;
        }
        let _com = ComGuard(PhantomData);

        // Create this thread's message queue before subscribing and before
        // announcing the thread ID: PostThreadMessageW fails on a thread
        // without one, and the message of an event, or the WM_QUIT of a
        // drop, would be lost.
        let mut msg = MSG::default();
        // SAFETY: `msg` is a valid, writable MSG. PM_NOREMOVE leaves any
        // message in the queue; the call only makes sure the queue exists.
        unsafe {
            let _ = PeekMessageW(&mut msg, None, WM_USER, WM_USER, PM_NOREMOVE);
        }

        // SAFETY: GetCurrentThreadId is always safe to call and returns the
        // calling thread's ID.
        let thread_id = unsafe { GetCurrentThreadId() };

        let settings = match UISettings::new() {
            Ok(s) => s,
            Err(err) => {
                let _ = ready_tx.send(Err(setup_failed("UISettings::new", err)));
                return;
            }
        };

        let accessibility = match AccessibilitySettings::new() {
            Ok(a) => a,
            Err(err) => {
                let _ = ready_tx.send(Err(setup_failed("AccessibilitySettings::new", err)));
                return;
            }
        };

        // The handlers only post a message to this thread, so the callback
        // runs on it alone, never twice at once, whichever thread raises the
        // event.
        let settings_changed = move |_: Ref<'_, UISettings>, _: Ref<'_, IInspectable>| {
            post_theme_changed(thread_id);
            Ok(())
        };
        let color_handler = TypedEventHandler::new(settings_changed);
        let text_scale_handler = TypedEventHandler::new(settings_changed);
        let advanced_effects_handler = TypedEventHandler::new(settings_changed);
        let high_contrast_handler = TypedEventHandler::new(
            move |_: Ref<'_, AccessibilitySettings>, _: Ref<'_, IInspectable>| {
                post_theme_changed(thread_id);
                Ok(())
            },
        );
        let animations_handler = TypedEventHandler::new(
            move |_: Ref<'_, UISettings>,
                  _: Ref<'_, UISettingsAnimationsEnabledChangedEventArgs>| {
                post_theme_changed(thread_id);
                Ok(())
            },
        );

        let color_token = match settings.ColorValuesChanged(&color_handler) {
            Ok(t) => t,
            Err(err) => {
                let _ = ready_tx.send(Err(setup_failed(COLOR_VALUES_CHANGED, err)));
                return;
            }
        };
        let text_scale_token = match settings.TextScaleFactorChanged(&text_scale_handler) {
            Ok(t) => t,
            Err(err) => {
                let _ = settings.RemoveColorValuesChanged(color_token);
                let _ = ready_tx.send(Err(setup_failed(TEXT_SCALE_FACTOR_CHANGED, err)));
                return;
            }
        };
        let high_contrast_token = match accessibility.HighContrastChanged(&high_contrast_handler) {
            Ok(t) => t,
            Err(err) => {
                let _ = settings.RemoveColorValuesChanged(color_token);
                let _ = settings.RemoveTextScaleFactorChanged(text_scale_token);
                let _ = ready_tx.send(Err(setup_failed(HIGH_CONTRAST_CHANGED, err)));
                return;
            }
        };
        // For `SPI_GETCLIENTAREAANIMATION`: no Microsoft page says
        // `UISettings.AnimationsEnabled` is the same setting, but the two read
        // the same on the three Windows probe images (`docs/platform-facts.md`,
        // `reduce_motion`; probe.toml:682, :745 of the 2026-10-11 run), an
        // agreement, not a documented identity. Windows before 10 2004 lacks
        // the event (`IUISettings6`); the watcher then does without it, as the
        // other sources are still watched.
        let animations_token = settings.AnimationsEnabledChanged(&animations_handler).ok();
        // Windows before 10 1703 lacks the event (`IUISettings4`), and the
        // property with it.
        let advanced_effects_token = settings
            .AdvancedEffectsEnabledChanged(&advanced_effects_handler)
            .ok();

        if ready_tx.send(Ok(thread_id)).is_ok() {
            // The thread timer of the pending burst, 0 for none.
            let mut timer: usize = 0;
            // Run the message pump. GetMessageW blocks until a message
            // arrives. When WM_QUIT is posted (via PostThreadMessageW from
            // the shutdown closure), GetMessageW returns FALSE (0), breaking
            // the loop; it returns -1 on an error, which ends the loop too
            // instead of spinning on it.
            // SAFETY: GetMessageW and DispatchMessageW are standard Win32
            // message pump functions. We pass None for HWND (thread
            // messages), 0 for filter range (all messages). SetTimer and
            // KillTimer with no window create and destroy a thread timer of
            // this thread, whose WM_TIMER comes with no window and its ID in
            // `wParam` (https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-settimer).
            unsafe {
                while GetMessageW(&mut msg, None, 0, 0).0 > 0 {
                    let thread_message = msg.hwnd.is_invalid();
                    if thread_message && msg.message == WM_THEME_CHANGED {
                        // Restart the window: the callback fires once, after
                        // the last event of a burst. The window is T20's KDE
                        // one reused to keep one user action one event, not
                        // a Windows value.
                        if timer != 0 {
                            let _ = KillTimer(None, timer);
                        }
                        timer = SetTimer(None, 0, super::DEBOUNCE_MS, None);
                        if timer == 0 {
                            super::fire(&callback);
                        }
                    } else if thread_message
                        && msg.message == WM_TIMER
                        && timer != 0
                        && msg.wParam.0 == timer
                    {
                        let _ = KillTimer(None, timer);
                        timer = 0;
                        super::fire(&callback);
                    } else {
                        DispatchMessageW(&msg);
                    }
                }
                if timer != 0 {
                    let _ = KillTimer(None, timer);
                }
            }
        }

        // Cleanup: unregister the event handlers (ignore errors -- the
        // UISettings may already be torn down). The handlers, `settings` and
        // `accessibility` drop here, before `_com` uninitializes COM.
        let _ = settings.RemoveColorValuesChanged(color_token);
        let _ = settings.RemoveTextScaleFactorChanged(text_scale_token);
        let _ = accessibility.RemoveHighContrastChanged(high_contrast_token);
        if let Some(token) = animations_token {
            let _ = settings.RemoveAnimationsEnabledChanged(token);
        }
        if let Some(token) = advanced_effects_token {
            let _ = settings.RemoveAdvancedEffectsEnabledChanged(token);
        }
    });

    // Receive the thread ID, or the setup error, from the background thread.
    let thread_id = match ready_rx.recv() {
        Ok(Ok(thread_id)) => thread_id,
        Ok(Err(err)) => {
            let _ = thread.join();
            return Err(err);
        }
        Err(_) => {
            let _ = thread.join();
            return Err(crate::Error::ReaderFailed {
                reader: "windows-watcher",
                source: "failed to start Windows theme watcher thread".into(),
            });
        }
    };

    // Build the platform shutdown closure: posts WM_QUIT to the watcher
    // thread's message queue, causing GetMessageW to return FALSE.
    let platform_shutdown = Box::new(move || {
        // SAFETY: PostThreadMessageW is safe to call from any thread. The
        // thread has its message queue and runs the pump until this WM_QUIT
        // arrives, and we call this before joining the thread.
        unsafe {
            let _ = PostThreadMessageW(thread_id, WM_QUIT, WPARAM(0), LPARAM(0));
        }
    });

    Ok(super::ThemeSubscription::new(
        shutdown_tx,
        thread,
        Some(platform_shutdown),
    ))
}
