//! Windows theme change watcher using UISettings::ColorValuesChanged.
//!
//! Creates a background thread with COM STA initialization and a
//! `GetMessageW`/`DispatchMessageW` message pump. The `UISettings`
//! `ColorValuesChanged` event fires whenever the user changes the system
//! color scheme. Shutdown is handled by posting `WM_QUIT` to the watcher
//! thread via `PostThreadMessageW` from the `ThemeSubscription` Drop handler.

// Windows FFI via the `windows` crate -- no safe alternative for COM/WinRT bindings.
// This follows the same pattern as src/windows.rs (the theme reader).
#![allow(unsafe_code)]

use std::sync::mpsc;

use ::windows::Foundation::TypedEventHandler;
use ::windows::UI::ViewManagement::UISettings;
use ::windows::Win32::Foundation::{LPARAM, WPARAM};
use ::windows::Win32::System::Com::{COINIT_APARTMENTTHREADED, CoInitializeEx, CoUninitialize};
use ::windows::Win32::System::Threading::GetCurrentThreadId;
use ::windows::Win32::UI::WindowsAndMessaging::{
    DispatchMessageW, GetMessageW, MSG, PM_NOREMOVE, PeekMessageW, PostThreadMessageW, WM_QUIT,
    WM_USER,
};

use super::ThemeChangeEvent;

/// Balances a successful `CoInitializeEx` when dropped. Declared before the
/// WinRT objects of the watcher thread, so it drops after them: every COM
/// object is released before `CoUninitialize` unloads the thread's DLLs.
struct ComGuard;

impl Drop for ComGuard {
    fn drop(&mut self) {
        // SAFETY: A `ComGuard` exists only after `CoInitializeEx` succeeded
        // on this thread, and it is never sent to another thread.
        unsafe { CoUninitialize() };
    }
}

/// Spawn a background thread that watches for Windows theme changes.
///
/// Initializes COM as STA, subscribes to `UISettings::ColorValuesChanged`,
/// and runs a `GetMessageW`/`DispatchMessageW` message pump. The callback
/// fires on the background thread whenever the system color values change
/// (e.g. the user switches between light and dark mode).
///
/// Returns once the thread has its message queue and the subscription is in
/// place, or with `Error::ReaderFailed` when a setup step failed.
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
        let _com = ComGuard;

        let settings = match UISettings::new() {
            Ok(s) => s,
            Err(err) => {
                let _ = ready_tx.send(Err(setup_failed("UISettings::new", err)));
                return;
            }
        };

        // Create the event handler that fires the callback on color changes.
        let handler = TypedEventHandler::new(move |_sender, _args| {
            callback(ThemeChangeEvent::Changed);
            Ok(())
        });

        let token = match settings.ColorValuesChanged(&handler) {
            Ok(t) => t,
            Err(err) => {
                let _ = ready_tx.send(Err(setup_failed("ColorValuesChanged", err)));
                return;
            }
        };

        // Create this thread's message queue before announcing the thread
        // ID: PostThreadMessageW fails on a thread without one, and the
        // WM_QUIT of a drop would be lost.
        let mut msg = MSG::default();
        // SAFETY: `msg` is a valid, writable MSG. PM_NOREMOVE leaves any
        // message in the queue; the call only makes sure the queue exists.
        unsafe {
            let _ = PeekMessageW(&mut msg, None, WM_USER, WM_USER, PM_NOREMOVE);
        }

        // SAFETY: GetCurrentThreadId is always safe to call and returns the
        // calling thread's ID.
        let thread_id = unsafe { GetCurrentThreadId() };

        if ready_tx.send(Ok(thread_id)).is_ok() {
            // Run the message pump. GetMessageW blocks until a message
            // arrives. When WM_QUIT is posted (via PostThreadMessageW from
            // the shutdown closure), GetMessageW returns FALSE (0), breaking
            // the loop.
            // SAFETY: GetMessageW and DispatchMessageW are standard Win32
            // message pump functions. We pass None for HWND (thread
            // messages), 0 for filter range (all messages).
            unsafe {
                while GetMessageW(&mut msg, None, 0, 0).as_bool() {
                    DispatchMessageW(&msg);
                }
            }
        }

        // Cleanup: unregister the event handler (ignore errors -- the
        // UISettings may already be torn down). `handler` and `settings` drop
        // here, before `_com` uninitializes COM.
        let _ = settings.RemoveColorValuesChanged(token);
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
