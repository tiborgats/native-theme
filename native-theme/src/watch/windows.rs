//! Windows theme change watcher using UISettings::ColorValuesChanged.
//!
//! Creates a background thread with COM STA initialization and a
//! `GetMessageW`/`DispatchMessageW` message pump. The `UISettings`
//! `ColorValuesChanged` event fires whenever the user changes the system
//! color scheme. Shutdown is handled by posting `WM_QUIT` to the watcher
//! thread via `PostThreadMessageW` from the `ThemeWatcher` Drop handler.

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
    DispatchMessageW, GetMessageW, MSG, PostThreadMessageW, WM_QUIT,
};

use super::ThemeChangeEvent;

/// Spawn a background thread that watches for Windows theme changes.
///
/// Initializes COM as STA, subscribes to `UISettings::ColorValuesChanged`,
/// and runs a `GetMessageW`/`DispatchMessageW` message pump. The callback
/// fires on the background thread whenever the system color values change
/// (e.g. the user switches between light and dark mode).
pub(crate) fn watch_windows(
    callback: impl Fn(ThemeChangeEvent) + Send + 'static,
) -> crate::Result<super::ThemeWatcher> {
    let (shutdown_tx, _shutdown_rx) = mpsc::channel::<()>();

    // Channel for the background thread to send back its OS thread ID.
    let (tid_tx, tid_rx) = mpsc::channel::<u32>();

    let thread = std::thread::spawn(move || {
        // Get this thread's OS thread ID for PostThreadMessageW from shutdown.
        // SAFETY: GetCurrentThreadId is always safe to call and returns the
        // calling thread's ID.
        let thread_id = unsafe { GetCurrentThreadId() };

        // Send the thread ID back to the constructor.
        if tid_tx.send(thread_id).is_err() {
            return;
        }

        // Initialize COM as Single-Threaded Apartment (required for WinRT
        // UISettings on a background thread).
        // SAFETY: CoInitializeEx is safe to call once per thread. We pass
        // COINIT_APARTMENTTHREADED for STA mode.
        let com_result = unsafe { CoInitializeEx(None, COINIT_APARTMENTTHREADED) };
        if com_result.is_err() {
            return;
        }

        // Create UISettings. If this fails, clean up COM and return.
        let settings = match UISettings::new() {
            Ok(s) => s,
            Err(_) => {
                // SAFETY: Balances the successful CoInitializeEx above.
                unsafe { CoUninitialize() };
                return;
            }
        };

        // Create the event handler that fires the callback on color changes.
        let handler = TypedEventHandler::new(
            move |_sender, _args| {
                callback(ThemeChangeEvent::ColorSchemeChanged);
                Ok(())
            },
        );

        // Subscribe to ColorValuesChanged. On failure, clean up and return.
        let token = match settings.ColorValuesChanged(&handler) {
            Ok(t) => t,
            Err(_) => {
                // SAFETY: Balances the successful CoInitializeEx above.
                unsafe { CoUninitialize() };
                return;
            }
        };

        // Run the message pump. GetMessageW blocks until a message arrives.
        // When WM_QUIT is posted (via PostThreadMessageW from the shutdown
        // closure), GetMessageW returns FALSE (0), breaking the loop.
        let mut msg = MSG::default();
        // SAFETY: GetMessageW and DispatchMessageW are standard Win32 message
        // pump functions. We pass None for HWND (thread messages), 0 for
        // filter range (all messages).
        unsafe {
            while GetMessageW(&mut msg, None, 0, 0).as_bool() {
                DispatchMessageW(&msg);
            }
        }

        // Cleanup: unregister the event handler (ignore errors -- the
        // UISettings may already be torn down).
        let _ = settings.RemoveColorValuesChanged(token);

        // SAFETY: Balances the successful CoInitializeEx above. Called on
        // the same thread that initialized COM.
        unsafe { CoUninitialize() };
    });

    // Receive the thread ID from the background thread.
    let thread_id = tid_rx.recv().map_err(|_| {
        crate::Error::Unavailable("failed to start Windows theme watcher thread".into())
    })?;

    // Build the platform shutdown closure: posts WM_QUIT to the watcher
    // thread's message queue, causing GetMessageW to return FALSE.
    let platform_shutdown = Box::new(move || {
        // SAFETY: PostThreadMessageW is safe to call from any thread. The
        // thread_id is valid as long as the thread is running, and we call
        // this before joining the thread.
        unsafe {
            let _ = PostThreadMessageW(thread_id, WM_QUIT, WPARAM(0), LPARAM(0));
        }
    });

    Ok(super::ThemeWatcher::with_platform_shutdown(
        shutdown_tx,
        thread,
        platform_shutdown,
    ))
}
