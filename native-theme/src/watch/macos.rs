//! macOS theme change watcher using NSDistributedNotificationCenter.
//!
//! Observes `AppleInterfaceThemeChangedNotification` via a block-based observer
//! registered with the default distributed notification center. Distributed
//! notifications are delivered through the process's run loop, usually the
//! main thread's, so the application must run one in a common mode, as every
//! GUI toolkit's event loop does (AppKit's `NSApplication::run`): Apple,
//! Notification Programming Topics, "Notification Centers"
//! (<https://developer.apple.com/library/archive/documentation/Cocoa/Conceptual/Notifications/Articles/NotificationCenters.html>).
//! The observer's block runs on a private serial `NSOperationQueue`, so the
//! callback never runs on the main thread and never twice at once. There is
//! no watcher thread: dropping the `ThemeSubscription` removes the observer.

// Objective-C FFI via objc2 -- no safe alternative for platform bindings.
// This follows the same pattern as src/macos.rs (the theme reader).
#![allow(unsafe_code)]

use core::ptr::NonNull;
use std::sync::Mutex;

use block2::RcBlock;
use objc2::rc::Retained;
use objc2::runtime::{AnyObject, ProtocolObject};
use objc2_foundation::{
    NSDistributedNotificationCenter, NSNotification, NSNotificationCenter, NSNotificationName,
    NSObjectProtocol, NSOperationQueue, NSString,
};

use super::ThemeChangeEvent;

/// A registered block observer. Dropping it removes the observer from its
/// center.
struct ObserverGuard {
    center: Retained<NSNotificationCenter>,
    token: Retained<ProtocolObject<dyn NSObjectProtocol>>,
    /// The serial queue the observer's block runs on. The guard owns it: it
    /// keeps the queue alive while the observer is registered, and its drop
    /// waits on it.
    queue: Retained<NSOperationQueue>,
}

// SAFETY: `center` and `queue` are `Send + Sync` in objc2-foundation 0.3.2
// (NSNotificationCenter and NSOperationQueue are thread-safe; objc2-core-
// foundation 0.3.2's thread_safety.rs says the same of the distributed
// center). `token` is the opaque observer object the center returned: it is
// only passed back to that thread-safe center's `removeObserver:` and
// released, and Objective-C retain and release are atomic.
unsafe impl Send for ObserverGuard {}

impl Drop for ObserverGuard {
    fn drop(&mut self) {
        // SAFETY: `token` is the object returned by
        // addObserverForName_object_queue_usingBlock on this center and
        // conforms to NSObjectProtocol. We cast to &AnyObject via AsRef.
        unsafe {
            let observer_any: &AnyObject = (*self.token).as_ref();
            self.center.removeObserver(observer_any);
        }
        // removeObserver does not wait for a block already queued, so wait
        // for the queue to drain: no callback runs after the drop returns.
        // A drop from inside the callback runs on the queue itself, where
        // the wait would deadlock, so it returns without waiting.
        let on_own_queue = NSOperationQueue::currentQueue()
            .is_some_and(|current| Retained::as_ptr(&current) == Retained::as_ptr(&self.queue));
        if !on_own_queue {
            self.queue.waitUntilAllOperationsAreFinished();
        }
    }
}

/// Register `callback` for the notification `name` on `center`, delivered
/// on a new serial operation queue.
fn observe(
    center: Retained<NSNotificationCenter>,
    name: &NSNotificationName,
    callback: impl Fn(ThemeChangeEvent) + Send + 'static,
) -> ObserverGuard {
    let queue = NSOperationQueue::new();
    queue.setMaxConcurrentOperationCount(1);

    // The block may run on any of the queue's threads, so the callback,
    // which is only `Send`, is shared through a mutex. `fire` contains a
    // panic inside the lock, so the lock is never poisoned.
    let callback = Mutex::new(callback);
    let block: RcBlock<dyn Fn(NonNull<NSNotification>)> =
        RcBlock::new(move |_notification: NonNull<NSNotification>| {
            if let Ok(callback) = callback.lock() {
                super::fire(&*callback);
            }
        });

    // SAFETY: The notification name is a valid NSString, no object filter
    // (None = observe all senders), `queue` is a valid operation queue, and
    // the block is sendable: it captures only a `Mutex` of a `Send` callback.
    // The returned observer token is retained.
    let token = unsafe {
        center.addObserverForName_object_queue_usingBlock(Some(name), None, Some(&queue), &block)
    };

    ObserverGuard {
        center,
        token,
        queue,
    }
}

/// Watch for macOS theme changes.
///
/// Registers an observer for `AppleInterfaceThemeChangedNotification` on the
/// default `NSDistributedNotificationCenter`. The callback fires on a private
/// serial queue whenever the user toggles Appearance in System Settings,
/// provided the application runs its main run loop.
pub(crate) fn watch_macos(
    callback: impl Fn(ThemeChangeEvent) + Send + 'static,
) -> crate::Result<super::ThemeSubscription> {
    let center = NSDistributedNotificationCenter::defaultCenter().into_super();
    let name: &NSNotificationName = &NSString::from_str("AppleInterfaceThemeChangedNotification");
    let guard = observe(center, name, callback);

    Ok(super::ThemeSubscription::from_platform_shutdown(Box::new(
        move || drop(guard),
    )))
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;
    use std::sync::atomic::{AtomicUsize, Ordering};

    use super::*;

    /// A distributed notification reaches the process only through its main
    /// run loop, which a test binary does not run, so the guard is checked
    /// on the default (in-process) center: it posts on the calling thread,
    /// and the block runs on the guard's queue either way.
    #[test]
    fn dropping_the_guard_removes_the_observer() {
        let center = NSNotificationCenter::defaultCenter();
        let name: &NSNotificationName = &NSString::from_str("native-theme.watch.macos.test");
        let count = Arc::new(AtomicUsize::new(0));
        let counter = Arc::clone(&count);
        let guard = observe(center.clone(), name, move |_| {
            counter.fetch_add(1, Ordering::SeqCst);
        });
        let queue = guard.queue.clone();

        // SAFETY: `name` is a valid NSString and no object is attached.
        unsafe { center.postNotificationName_object(name, None) };
        queue.waitUntilAllOperationsAreFinished();
        assert_eq!(count.load(Ordering::SeqCst), 1);

        drop(guard);
        // SAFETY: as above.
        unsafe { center.postNotificationName_object(name, None) };
        queue.waitUntilAllOperationsAreFinished();
        assert_eq!(count.load(Ordering::SeqCst), 1);
    }

    /// A callback that panics does not end the observer: the next
    /// notification still runs it (M4-33).
    #[test]
    fn a_panicking_callback_does_not_stop_the_observer() {
        let center = NSNotificationCenter::defaultCenter();
        let name: &NSNotificationName = &NSString::from_str("native-theme.watch.macos.panic");
        let count = Arc::new(AtomicUsize::new(0));
        let counter = Arc::clone(&count);
        let guard = observe(center.clone(), name, move |_| {
            let call = counter.fetch_add(1, Ordering::SeqCst);
            assert!(call > 0, "the first callback panics");
        });
        let queue = guard.queue.clone();

        for _ in 0..2 {
            // SAFETY: `name` is a valid NSString and no object is attached.
            unsafe { center.postNotificationName_object(name, None) };
            queue.waitUntilAllOperationsAreFinished();
        }
        assert_eq!(count.load(Ordering::SeqCst), 2);
        drop(guard);
    }
}
