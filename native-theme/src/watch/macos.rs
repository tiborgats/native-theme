//! macOS theme change watcher using NSDistributedNotificationCenter.
//!
//! Observes `AppleInterfaceThemeChangedNotification` via a block-based observer
//! registered with the default distributed notification center. A CFRunLoop on
//! the background thread processes notification delivery. Shutdown is handled
//! by calling `CFRunLoop::stop()` from the `ThemeWatcher` Drop handler.

// Objective-C FFI via objc2 -- no safe alternative for platform bindings.
// This follows the same pattern as src/macos.rs (the theme reader).
#![allow(unsafe_code)]

use core::ptr::NonNull;
use std::sync::mpsc;

use block2::RcBlock;
use objc2::runtime::ProtocolObject;
use objc2_core_foundation::{CFRetained, CFRunLoop};
use objc2_foundation::{
    NSDistributedNotificationCenter, NSNotification, NSNotificationName, NSObjectProtocol, NSString,
};

use super::ThemeChangeEvent;

/// A thin wrapper around a raw `CFRunLoop` pointer that is `Send`.
///
/// Apple documents `CFRunLoopStop` as thread-safe: it can be called from any
/// thread to stop a run loop running on another thread. The objc2 bindings do
/// not mark `CFRunLoop` as `Send` because most run loop operations are
/// thread-local, but `stop()` is the documented exception.
struct SendableCFRunLoop(NonNull<CFRunLoop>);

// SAFETY: We only use this to call `CFRunLoop::stop()`, which Apple documents
// as safe to call from any thread.
unsafe impl Send for SendableCFRunLoop {}

impl SendableCFRunLoop {
    /// Create from a `CFRetained<CFRunLoop>` reference.
    ///
    /// The caller must ensure the `CFRetained` outlives this wrapper (i.e. the
    /// run loop is kept alive by the background thread).
    fn from_retained(retained: &CFRetained<CFRunLoop>) -> Self {
        Self(NonNull::from(&**retained))
    }

    /// Stop the run loop.
    ///
    /// # Safety
    ///
    /// The underlying `CFRunLoop` must still be alive.
    unsafe fn stop(&self) {
        // SAFETY: Caller guarantees the run loop is alive. CFRunLoopStop is
        // documented as thread-safe.
        unsafe { self.0.as_ref().stop() };
    }
}

/// Spawn a background thread that watches for macOS theme changes.
///
/// Registers an observer for `AppleInterfaceThemeChangedNotification` on the
/// default `NSDistributedNotificationCenter`, then runs a `CFRunLoop` to
/// receive notifications. The callback fires on the background thread
/// whenever the user toggles Appearance in System Settings.
pub(crate) fn watch_macos(
    callback: impl Fn(ThemeChangeEvent) + Send + 'static,
) -> crate::Result<super::ThemeWatcher> {
    let (shutdown_tx, _shutdown_rx) = mpsc::channel::<()>();

    // Channel for the background thread to send its CFRunLoop handle back.
    let (loop_tx, loop_rx) = mpsc::channel::<SendableCFRunLoop>();

    let thread = std::thread::spawn(move || {
        // Get this thread's CFRunLoop (must be called on the watcher thread).
        let Some(run_loop) = CFRunLoop::current() else {
            return;
        };

        // Send the run loop handle back to the constructor so the
        // platform_shutdown closure can call stop() on it.
        let sendable = SendableCFRunLoop::from_retained(&run_loop);
        if loop_tx.send(sendable).is_err() {
            return;
        }

        // Get the default distributed notification center.
        let center = NSDistributedNotificationCenter::defaultCenter();

        // Create the notification name.
        let name: &NSNotificationName =
            &NSString::from_str("AppleInterfaceThemeChangedNotification");

        // Create the observer block. The block captures `callback` and fires
        // it with ColorSchemeChanged on each notification.
        let block: RcBlock<dyn Fn(NonNull<NSNotification>)> =
            RcBlock::new(move |_notification: NonNull<NSNotification>| {
                callback(ThemeChangeEvent::ColorSchemeChanged);
            });

        // Register the observer.
        // SAFETY: The notification name is a valid NSString, no object filter
        // (None = observe all senders), no operation queue (None = deliver on
        // the posting thread / run loop thread), and the block is a valid
        // Objective-C block. The returned observer token is retained.
        let observer: objc2::rc::Retained<ProtocolObject<dyn NSObjectProtocol>> = unsafe {
            center.addObserverForName_object_queue_usingBlock(
                Some(name),
                None, // any sender
                None, // deliver on run loop thread
                &block,
            )
        };

        // Run the CFRunLoop. This blocks until CFRunLoop::stop() is called
        // from the platform_shutdown closure in ThemeWatcher::drop().
        CFRunLoop::run();

        // After the run loop stops, unregister the observer.
        // SAFETY: `observer` is the object returned by
        // addObserverForName_object_queue_usingBlock and conforms to
        // NSObjectProtocol. We cast to &AnyObject via AsRef.
        unsafe {
            center.removeObserver(observer.as_ref().as_ref());
        }
    });

    // Receive the CFRunLoop handle from the background thread.
    let sendable_loop = loop_rx
        .recv()
        .map_err(|_| crate::Error::Unavailable("macOS watcher thread failed to start".into()))?;

    // Build the platform shutdown closure: calls CFRunLoop::stop() to
    // unblock CFRunLoop::run() on the watcher thread.
    let platform_shutdown = Box::new(move || {
        // SAFETY: The run loop is alive as long as the thread is running,
        // and we call stop() before joining the thread.
        unsafe { sendable_loop.stop() };
    });

    Ok(super::ThemeWatcher::with_platform_shutdown(
        shutdown_tx,
        thread,
        platform_shutdown,
    ))
}
