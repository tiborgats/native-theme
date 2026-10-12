//! macOS theme change watcher using NSDistributedNotificationCenter.
//!
//! Observes `AppleInterfaceThemeChangedNotification` via a block-based observer
//! registered with the default distributed notification center. The background
//! thread runs its CFRunLoop in slices of [`RUN_SLICE_SECONDS`] to process
//! notification delivery, and checks the shutdown channel between slices. The
//! `ThemeSubscription` Drop handler signals the channel and calls
//! `CFRunLoop::stop()` to end a running slice early.

// Objective-C FFI via objc2 -- no safe alternative for platform bindings.
// This follows the same pattern as src/macos.rs (the theme reader).
#![allow(unsafe_code)]

use core::ptr::NonNull;
use std::sync::mpsc::{self, TryRecvError};

use block2::RcBlock;
use objc2::runtime::{AnyObject, ProtocolObject};
use objc2_core_foundation::{
    CFRetained, CFRunLoop, CFRunLoopRunResult, CFTimeInterval, kCFRunLoopDefaultMode,
};
use objc2_foundation::{
    NSDistributedNotificationCenter, NSNotification, NSNotificationName, NSObjectProtocol, NSString,
};

use super::ThemeChangeEvent;

/// Length of one `CFRunLoopRunInMode` slice of the watcher thread. It bounds
/// how long a stop that arrives before the loop runs waits: `CFRunLoopStop`
/// does nothing to a run loop that is not running, so such a stop is seen
/// only when the current slice times out.
const RUN_SLICE_SECONDS: CFTimeInterval = 0.25;

/// A retained `CFRunLoop` that is `Send`, so the shutdown closure can stop
/// the watcher thread's run loop from the thread that drops the subscription.
///
/// The objc2 bindings do not mark `CFRunLoop` as `Send` because most run loop
/// operations are thread-local; `CFRunLoopStop` is the documented exception.
/// Holding a retain keeps the run loop alive after the watcher thread exits,
/// so `stop()` on a finished thread's loop is a no-op.
struct SendableCFRunLoop(CFRetained<CFRunLoop>);

// SAFETY: The wrapper is only used to call `CFRunLoop::stop()`, which Apple
// documents as safe to call from any thread, and to release the retain when
// it drops; CoreFoundation's retain and release are thread-safe.
unsafe impl Send for SendableCFRunLoop {}

impl SendableCFRunLoop {
    /// Stop the run loop's current activation, if it is running.
    fn stop(&self) {
        self.0.stop();
    }
}

/// Spawn a background thread that watches for macOS theme changes.
///
/// Registers an observer for `AppleInterfaceThemeChangedNotification` on the
/// default `NSDistributedNotificationCenter`, then runs a `CFRunLoop` to
/// receive notifications. The callback fires on the background thread
/// whenever the user toggles Appearance in System Settings.
///
/// Returns once the observer is registered and the thread's run loop has a
/// source to wait on, or with `Error::ReaderFailed` when a setup step failed.
pub(crate) fn watch_macos(
    callback: impl Fn(ThemeChangeEvent) + Send + 'static,
) -> crate::Result<super::ThemeSubscription> {
    let (shutdown_tx, shutdown_rx) = mpsc::channel::<()>();

    // Channel for the background thread to send back its CFRunLoop handle,
    // or the setup step that failed.
    let (ready_tx, ready_rx) = mpsc::channel::<crate::Result<SendableCFRunLoop>>();

    let thread = std::thread::spawn(move || {
        let setup_failed = |reason: &str| crate::Error::ReaderFailed {
            reader: "macos-watcher",
            source: reason.into(),
        };

        // Get this thread's CFRunLoop (must be called on the watcher thread).
        let Some(run_loop) = CFRunLoop::current() else {
            let _ = ready_tx.send(Err(setup_failed(
                "CFRunLoopGetCurrent returned no run loop",
            )));
            return;
        };

        // Get the default distributed notification center.
        let center = NSDistributedNotificationCenter::defaultCenter();

        // Create the notification name.
        let name: &NSNotificationName =
            &NSString::from_str("AppleInterfaceThemeChangedNotification");

        // Create the observer block. The block captures `callback` and fires
        // it with Changed on each notification.
        let block: RcBlock<dyn Fn(NonNull<NSNotification>)> =
            RcBlock::new(move |_notification: NonNull<NSNotification>| {
                callback(ThemeChangeEvent::Changed);
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

        // SAFETY: `kCFRunLoopDefaultMode` is an immutable CFString constant
        // exported by CoreFoundation; reading the static is a plain load.
        let mode = unsafe { kCFRunLoopDefaultMode };

        // A run loop whose mode has no source returns Finished at once, without
        // waiting, so a zero-length pass shows whether notifications can
        // arrive on this thread before the subscription is reported started.
        let ready = if CFRunLoop::run_in_mode(mode, 0.0, false) == CFRunLoopRunResult::Finished {
            Err(setup_failed(
                "the run loop has no source in the default mode after registering the observer",
            ))
        } else {
            Ok(SendableCFRunLoop(run_loop))
        };
        let started = ready.is_ok();

        if ready_tx.send(ready).is_ok() && started {
            // Run the CFRunLoop slice by slice until the shutdown channel
            // receives a message or disconnects. CFRunLoop::stop() from the
            // platform_shutdown closure ends a running slice early.
            while let Err(TryRecvError::Empty) = shutdown_rx.try_recv() {
                if CFRunLoop::run_in_mode(mode, RUN_SLICE_SECONDS, false)
                    == CFRunLoopRunResult::Finished
                {
                    // The mode lost its sources: no notification can arrive,
                    // and running again would return at once.
                    break;
                }
            }
        }

        // After the run loop stops, unregister the observer.
        // SAFETY: `observer` is the object returned by
        // addObserverForName_object_queue_usingBlock and conforms to
        // NSObjectProtocol. We cast to &AnyObject via AsRef.
        unsafe {
            let observer_any: &AnyObject = (*observer).as_ref();
            center.removeObserver(observer_any);
        }
    });

    // Receive the CFRunLoop handle, or the setup error, from the background
    // thread.
    let run_loop = match ready_rx.recv() {
        Ok(Ok(run_loop)) => run_loop,
        Ok(Err(err)) => {
            let _ = thread.join();
            return Err(err);
        }
        Err(_) => {
            let _ = thread.join();
            return Err(crate::Error::ReaderFailed {
                reader: "macos-watcher",
                source: "macOS watcher thread failed to start".into(),
            });
        }
    };

    // Build the platform shutdown closure: signals the shutdown channel, then
    // calls CFRunLoop::stop() so a running slice returns at once and the
    // thread sees the signal.
    let stop_tx = shutdown_tx.clone();
    let platform_shutdown = Box::new(move || {
        let _ = stop_tx.send(());
        run_loop.stop();
    });

    Ok(super::ThemeSubscription::new(
        shutdown_tx,
        thread,
        Some(platform_shutdown),
    ))
}
