//! macOS theme change watcher using notification-center observers.
//!
//! One observer object is registered for the notification of each of the
//! macOS reader's sources (`macos.rs`, "Sources"):
//!
//! - `AppleInterfaceThemeChangedNotification` on the default
//!   `NSDistributedNotificationCenter`, with the suspension behaviour
//!   `DeliverImmediately`: `NSApplication` suspends distributed delivery
//!   while the application is not active (Apple,
//!   <https://developer.apple.com/documentation/foundation/distributednotificationcenter/suspended>),
//!   and the change would otherwise wait until it is
//!   (<https://developer.apple.com/documentation/foundation/distributednotificationcenter/addobserver(_:selector:name:object:suspensionbehavior:)>).
//! - `NSSystemColorsDidChangeNotification` (the colours, the accent among
//!   them) and `NSPreferredScrollerStyleDidChangeNotification` on the
//!   process's default `NSNotificationCenter`. Apple's pages for them name
//!   no center, so the default one is an inference, the conventional center
//!   for AppKit's own notifications; no test sees a real AppKit post, so it
//!   stays unverified until checked on a Mac (`docs/todo.md`).
//! - `NSWorkspaceAccessibilityDisplayOptionsDidChangeNotification` on
//!   `NSWorkspace`'s `notificationCenter`, the one center it is posted on
//!   (<https://developer.apple.com/documentation/appkit/nsworkspace/accessibilitydisplayoptionsdidchangenotification>).
//!
//! Distributed notifications are delivered through the process's run loop,
//! usually the main thread's, so the application must run one in a common
//! mode, as every GUI toolkit's event loop does (AppKit's
//! `NSApplication::run`): Apple, Notification Programming Topics,
//! "Notification Centers"
//! (<https://developer.apple.com/library/archive/documentation/Cocoa/Conceptual/Notifications/Articles/NotificationCenters.html>).
//! The observer's method runs on the thread that posts the notification; it
//! only records it and queues the callback on a private serial
//! `NSOperationQueue`, so the callback never runs on the main thread and
//! never twice at once. The callback fires once per burst of notifications,
//! 300ms after the last of them, should one change post more than one.
//! There is no watcher thread: dropping the `ThemeSubscription` removes the
//! observer.

// Objective-C FFI via objc2 -- no safe alternative for platform bindings.
// This follows the same pattern as src/macos.rs (the theme reader).
#![allow(unsafe_code)]

use std::sync::{Arc, Condvar, Mutex};
use std::time::Instant;

use block2::RcBlock;
use objc2::rc::Retained;
use objc2::runtime::AnyObject;
use objc2::{AnyThread, DefinedClass, Message, define_class, msg_send, sel};
use objc2_app_kit::{
    NSPreferredScrollerStyleDidChangeNotification, NSSystemColorsDidChangeNotification,
    NSWorkspace, NSWorkspaceAccessibilityDisplayOptionsDidChangeNotification,
};
use objc2_foundation::{
    NSDistributedNotificationCenter, NSNotification, NSNotificationCenter, NSNotificationName,
    NSNotificationSuspensionBehavior, NSObject, NSObjectProtocol, NSOperationQueue, NSString,
};

use super::ThemeChangeEvent;

/// A notification center the observer registers with.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Center {
    /// The default `NSDistributedNotificationCenter`, delivering at once.
    Distributed,
    /// The process's default `NSNotificationCenter`; for AppKit's colour
    /// and scroller notifications an inference, as Apple names no center.
    Local,
    /// `NSWorkspace`'s `notificationCenter`.
    Workspace,
}

/// A notification the observer registers for.
pub(super) struct Registration {
    /// The notification's constant, as the source table (`watch/mod.rs`)
    /// names it.
    #[cfg_attr(not(test), allow(dead_code))]
    pub(super) name_id: &'static str,
    center: Center,
    name: Retained<NSNotificationName>,
}

/// The notifications of the macOS reader's sources.
pub(super) fn registrations() -> [Registration; 4] {
    // SAFETY: Each is an immutable NSString constant that AppKit defines
    // (objc2-app-kit 0.3.2 `NSColor.rs:1068`, `NSScroller.rs:324`,
    // `NSAccessibility.rs:190`), valid for the life of the process.
    let (system_colors, scroller_style, accessibility_display) = unsafe {
        (
            NSSystemColorsDidChangeNotification.retain(),
            NSPreferredScrollerStyleDidChangeNotification.retain(),
            NSWorkspaceAccessibilityDisplayOptionsDidChangeNotification.retain(),
        )
    };
    [
        Registration {
            name_id: "AppleInterfaceThemeChangedNotification",
            center: Center::Distributed,
            name: NSString::from_str("AppleInterfaceThemeChangedNotification"),
        },
        Registration {
            name_id: "NSSystemColorsDidChangeNotification",
            center: Center::Local,
            name: system_colors,
        },
        Registration {
            name_id: "NSPreferredScrollerStyleDidChangeNotification",
            center: Center::Local,
            name: scroller_style,
        },
        Registration {
            name_id: "NSWorkspaceAccessibilityDisplayOptionsDidChangeNotification",
            center: Center::Workspace,
            name: accessibility_display,
        },
    ]
}

/// Whether a callback is queued, and since when it waits.
#[derive(Default)]
struct Pending {
    /// When the last notification not yet fired arrived; `None` while no
    /// callback is queued.
    last: Option<Instant>,
    /// Set by the guard's drop: no callback is queued or fired after it.
    closed: bool,
}

/// What the observer's notifications share: the coalescing state, the
/// serial queue the callback runs on, and the callback.
struct Delivery {
    pending: Mutex<Pending>,
    /// Wakes a queued callback that waits out the debounce when the guard
    /// drops.
    wake: Condvar,
    queue: Retained<NSOperationQueue>,
    /// The callback is only `Send`, and the queue may run it on any of its
    /// threads, so it is shared through a mutex. `fire` contains a panic
    /// inside the lock, so the lock is never poisoned.
    callback: Mutex<Box<dyn Fn(ThemeChangeEvent) + Send>>,
}

impl Delivery {
    /// Record a notification, and queue the callback unless one is queued.
    fn notify(self: &Arc<Self>) {
        let Ok(mut pending) = self.pending.lock() else {
            return;
        };
        if pending.closed {
            return;
        }
        if pending.last.replace(Instant::now()).is_none() {
            let delivery = Arc::clone(self);
            let block = RcBlock::new(move || delivery.run());
            // SAFETY: The block is sendable: it captures only an `Arc` of a
            // `Delivery`, which is `Send + Sync`.
            unsafe { self.queue.addOperationWithBlock(&block) };
        }
    }

    /// Wait until [`super::DEBOUNCE`] has passed since the last
    /// notification, then fire the callback, unless the guard dropped
    /// meanwhile. Runs on the queue. The window is T20's KDE one reused to
    /// keep one user action one event, not a macOS value.
    fn run(&self) {
        {
            let Ok(mut pending) = self.pending.lock() else {
                return;
            };
            loop {
                if pending.closed {
                    return;
                }
                let Some(last) = pending.last else {
                    return;
                };
                let wait = super::DEBOUNCE.saturating_sub(last.elapsed());
                if wait.is_zero() {
                    pending.last = None;
                    break;
                }
                pending = match self.wake.wait_timeout(pending, wait) {
                    Ok((pending, _)) => pending,
                    Err(_) => return,
                };
            }
        }
        if let Ok(callback) = self.callback.lock() {
            super::fire(&*callback);
        }
    }

    /// Stop queueing and firing the callback, and wake a queued one.
    fn close(&self) {
        if let Ok(mut pending) = self.pending.lock() {
            pending.closed = true;
        }
        self.wake.notify_all();
    }
}

define_class!(
    // SAFETY:
    // - The superclass NSObject does not have any subclassing requirements.
    // - `Observer` does not implement `Drop`.
    #[unsafe(super(NSObject))]
    #[ivars = Arc<Delivery>]
    struct Observer;

    impl Observer {
        // SAFETY: The signature is a notification observer's: one
        // NSNotification argument, no return value.
        #[unsafe(method(themeSettingChanged:))]
        fn theme_setting_changed(&self, _notification: &NSNotification) {
            self.ivars().notify();
        }
    }

    // SAFETY: `Observer` subclasses NSObject (the `super` above), which conforms
    // to NSObjectProtocol; the protocol adds no requirement of its own.
    unsafe impl NSObjectProtocol for Observer {}
);

impl Observer {
    fn new(delivery: Arc<Delivery>) -> Retained<Self> {
        let this = Self::alloc().set_ivars(delivery);
        // SAFETY: NSObject's `init` takes no argument and returns the
        // initialized object.
        unsafe { msg_send![super(this), init] }
    }
}

/// A registered observer. Dropping it removes the observer from its
/// centers.
struct ObserverGuard {
    /// Kept alive while it is registered: a center does not retain it.
    observer: Retained<Observer>,
    centers: Vec<Retained<NSNotificationCenter>>,
}

impl Drop for ObserverGuard {
    fn drop(&mut self) {
        let observer: &AnyObject = &self.observer;
        for center in &self.centers {
            // SAFETY: `observer` is the object registered on `center`, alive
            // until this guard drops it.
            unsafe { center.removeObserver(observer) };
        }
        let delivery = self.observer.ivars();
        delivery.close();
        // removeObserver does not wait for a callback already queued, so
        // wait for the queue to drain: no callback runs after the drop
        // returns. A drop from inside the callback runs on the queue itself,
        // where the wait would deadlock, so it returns without waiting.
        let on_own_queue = NSOperationQueue::currentQueue()
            .is_some_and(|current| Retained::as_ptr(&current) == Retained::as_ptr(&delivery.queue));
        if !on_own_queue {
            delivery.queue.waitUntilAllOperationsAreFinished();
        }
    }
}

/// Register one observer for every notification of `registrations`, which
/// fires `callback` on a new serial operation queue.
fn observe(
    registrations: &[Registration],
    callback: impl Fn(ThemeChangeEvent) + Send + 'static,
) -> ObserverGuard {
    let queue = NSOperationQueue::new();
    queue.setMaxConcurrentOperationCount(1);
    let observer = Observer::new(Arc::new(Delivery {
        pending: Mutex::new(Pending::default()),
        wake: Condvar::new(),
        queue,
        callback: Mutex::new(Box::new(callback)),
    }));
    let observer_any: &AnyObject = &observer;

    let mut centers: Vec<Retained<NSNotificationCenter>> = Vec::new();
    for registration in registrations {
        let name: &NSNotificationName = &registration.name;
        let center = match registration.center {
            Center::Distributed => {
                let center = NSDistributedNotificationCenter::defaultCenter();
                // SAFETY: `observer_any` is an NSObject the guard keeps alive
                // until it removes it, the selector is `Observer`'s method
                // taking one NSNotification, and no object filter is given.
                unsafe {
                    center.addObserver_selector_name_object_suspensionBehavior(
                        observer_any,
                        sel!(themeSettingChanged:),
                        Some(name),
                        None,
                        NSNotificationSuspensionBehavior::DeliverImmediately,
                    );
                }
                center.into_super()
            }
            Center::Local | Center::Workspace => {
                let center = if registration.center == Center::Local {
                    NSNotificationCenter::defaultCenter()
                } else {
                    NSWorkspace::sharedWorkspace().notificationCenter()
                };
                // SAFETY: as above.
                unsafe {
                    center.addObserver_selector_name_object(
                        observer_any,
                        sel!(themeSettingChanged:),
                        Some(name),
                        None,
                    );
                }
                center
            }
        };
        if !centers
            .iter()
            .any(|known| Retained::as_ptr(known) == Retained::as_ptr(&center))
        {
            centers.push(center);
        }
    }

    ObserverGuard { observer, centers }
}

/// Watch for macOS theme changes.
///
/// Registers an observer for the notifications of [`registrations`]. The
/// callback fires on a private serial queue whenever the user changes the
/// appearance, the accent or highlight colour, the scroll bars or an
/// accessibility display option in System Settings, provided the
/// application runs its main run loop.
pub(crate) fn watch_macos(
    callback: impl Fn(ThemeChangeEvent) + Send + 'static,
) -> crate::Result<super::ThemeSubscription> {
    let guard = observe(&registrations(), callback);

    Ok(super::ThemeSubscription::from_platform_shutdown(Box::new(
        move || drop(guard),
    )))
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;
    use std::sync::atomic::{AtomicUsize, Ordering};

    use super::*;

    /// A registration for the test notification `name` on the default
    /// (in-process) center.
    fn local(name: &str) -> Registration {
        Registration {
            name_id: "test",
            center: Center::Local,
            name: NSString::from_str(name),
        }
    }

    /// Post `name` on the default center, as AppKit posts its notifications.
    fn post(name: &NSNotificationName) {
        // SAFETY: `name` is a valid NSString and no object is attached.
        unsafe { NSNotificationCenter::defaultCenter().postNotificationName_object(name, None) };
    }

    /// The queue the guard's callback runs on.
    fn queue(guard: &ObserverGuard) -> Retained<NSOperationQueue> {
        guard.observer.ivars().queue.clone()
    }

    /// A distributed notification reaches the process only through its main
    /// run loop, which a test binary does not run, so the guard is checked
    /// on the default (in-process) center: it posts on the calling thread,
    /// and the callback runs on the guard's queue either way.
    #[test]
    fn dropping_the_guard_removes_the_observer() {
        let registrations = [local("native-theme.watch.macos.test")];
        let count = Arc::new(AtomicUsize::new(0));
        let counter = Arc::clone(&count);
        let guard = observe(&registrations, move |_| {
            counter.fetch_add(1, Ordering::SeqCst);
        });
        let queue = queue(&guard);

        post(&registrations[0].name);
        queue.waitUntilAllOperationsAreFinished();
        assert_eq!(count.load(Ordering::SeqCst), 1);

        drop(guard);
        post(&registrations[0].name);
        queue.waitUntilAllOperationsAreFinished();
        assert_eq!(count.load(Ordering::SeqCst), 1);
    }

    /// A callback that panics does not end the observer: the next
    /// notification still runs it (M4-33).
    #[test]
    fn a_panicking_callback_does_not_stop_the_observer() {
        let registrations = [local("native-theme.watch.macos.panic")];
        let count = Arc::new(AtomicUsize::new(0));
        let counter = Arc::clone(&count);
        let guard = observe(&registrations, move |_| {
            let call = counter.fetch_add(1, Ordering::SeqCst);
            assert!(call > 0, "the first callback panics");
        });
        let queue = queue(&guard);

        for _ in 0..2 {
            post(&registrations[0].name);
            queue.waitUntilAllOperationsAreFinished();
        }
        assert_eq!(count.load(Ordering::SeqCst), 2);
        drop(guard);
    }

    /// Two notifications of one change, posted together, fire the callback
    /// once.
    #[test]
    fn notifications_posted_together_fire_once() {
        let registrations = [
            local("native-theme.watch.macos.burst-a"),
            local("native-theme.watch.macos.burst-b"),
        ];
        let count = Arc::new(AtomicUsize::new(0));
        let counter = Arc::clone(&count);
        let guard = observe(&registrations, move |_| {
            counter.fetch_add(1, Ordering::SeqCst);
        });
        let queue = queue(&guard);

        post(&registrations[0].name);
        post(&registrations[1].name);
        queue.waitUntilAllOperationsAreFinished();
        assert_eq!(count.load(Ordering::SeqCst), 1);
        drop(guard);
    }

    /// The watcher registers for every notification of the reader's
    /// sources, and its drop returns.
    #[test]
    fn the_watcher_registers_and_drops() {
        let guard = observe(&registrations(), |_| {});
        assert_eq!(guard.centers.len(), 3);
        drop(guard);
    }
}
