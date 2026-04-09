# Runtime dark mode / theme change watching

Status: Not started
Date: 2026-04-09

---

## Problem

`native-theme` is currently a "read once at startup" library. If the user
toggles dark mode, changes the accent color, or modifies their system font
while the app is running, the app has no way to know through native-theme.

The existing API provides `invalidate_caches()` + `detect_is_dark()` for
polling, and `SystemTheme::from_system()` for full re-read. But the app
has no signal to trigger these calls. Every major desktop toolkit provides
change notifications:

| Platform | Signal | Payload |
|----------|--------|---------|
| Linux (GNOME/GTK) | `org.freedesktop.portal.Settings::SettingChanged` D-Bus signal | namespace, key, value |
| Linux (KDE) | File watch on `~/.config/kdeglobals` | file changed |
| macOS | `NSAppearance` KVO, `NSSystemColorsDidChangeNotification` | new appearance/colors |
| Windows | `UISettings::ColorValuesChanged` event | (none — re-query) |

Without change watching, native-theme's value for long-running desktop apps
is limited: the app starts with the right theme but can't follow system
changes at runtime.

---

## Scope

This document covers **dark mode change detection** as the minimum viable
feature. Full theme change watching (accent color, font changes, icon theme
changes) uses the same infrastructure but has broader implications.

### Minimum scope: dark mode toggle

The user toggles light/dark mode in system settings. The app receives a
callback and can re-read the theme.

### Extended scope (future)

- Accent color changes (GNOME portal `accent-color` key, KDE kdeglobals
  `[Colors:View] DecorationFocus`, Windows `UISettings` accent)
- Font changes
- Icon theme changes
- High contrast toggle

The extended scope uses the same watcher infrastructure. Once the watcher
exists, adding more watched keys/signals is incremental.

---

## API design

### Option A: Callback-based (recommended)

```rust
/// Handle returned by `on_theme_change`. Drop it to stop watching.
pub struct ThemeWatcher { /* platform handle */ }

/// Watch for system theme changes.
///
/// Calls `callback` on a background thread whenever the OS reports a
/// theme-related change (dark mode toggle, accent color, etc.).
///
/// The callback receives a `ThemeChangeEvent` describing what changed.
/// For a full re-read, call `SystemTheme::from_system()` inside the
/// callback.
///
/// Drop the returned `ThemeWatcher` to stop watching.
///
/// # Platform behavior
///
/// - **Linux (GNOME):** Subscribes to `SettingChanged` on the
///   `org.freedesktop.portal.Settings` D-Bus interface.
/// - **Linux (KDE):** Watches `~/.config/kdeglobals` for modifications.
/// - **macOS:** Observes `NSAppearance` changes via KVO.
/// - **Windows:** Subscribes to `UISettings::ColorValuesChanged`.
/// - **Other:** Returns `Err(Error::Unsupported)`.
pub fn on_theme_change(
    callback: impl Fn(ThemeChangeEvent) + Send + 'static,
) -> Result<ThemeWatcher>;

/// What changed in the system theme.
#[non_exhaustive]
pub enum ThemeChangeEvent {
    /// Dark/light mode changed. New value is `is_dark`.
    DarkModeChanged { is_dark: bool },
    /// Accent color changed. Call `SystemTheme::from_system()` to get
    /// the new color.
    AccentColorChanged,
    /// Something else changed (font, icon theme, etc.).
    /// Call `SystemTheme::from_system()` for a full re-read.
    Other,
}
```

**Why callback-based:**
- No async runtime dependency — the watcher spawns its own `std::thread`
  (or uses OS event APIs directly)
- Works with any GUI framework's event loop (just post to the UI thread
  from the callback)
- RAII cleanup via `Drop` on `ThemeWatcher`

**Why not async Stream:**
- Forces an async runtime choice on the consumer
- The GNOME portal watcher already uses ashpd's async API internally,
  but the consumer-facing API should not require tokio/async-io
- A `Stream` adapter can be built on top of the callback API trivially:
  ```rust
  let (tx, rx) = tokio::sync::mpsc::unbounded_channel();
  let _watcher = on_theme_change(move |event| { let _ = tx.send(event); });
  // rx is now an async stream
  ```

### Option B: Polling helper

```rust
/// Start a polling thread that calls `detect_is_dark()` every `interval`
/// and invokes `callback` when the value changes.
pub fn poll_dark_mode(
    interval: Duration,
    callback: impl Fn(bool) + Send + 'static,
) -> ThemeWatcher;
```

Simpler but wasteful (CPU, battery) and has latency equal to the poll
interval. Acceptable as a fallback for platforms without native signals.

### Recommendation

Option A with Option B as internal fallback for unsupported platforms.

---

## Platform implementation

### Linux (GNOME / portal-based DEs)

**Signal:** `org.freedesktop.portal.Settings` interface, `SettingChanged`
signal on session bus.

**Watched keys:**
- `org.freedesktop.appearance` / `color-scheme` — dark mode (1 = dark, 2 = light)
- `org.freedesktop.appearance` / `accent-color` — accent (future scope)

**Implementation:**

Use `zbus` (already a transitive dependency via `ashpd`) to subscribe to
the signal. Spawn a dedicated thread running a `zbus::blocking::Connection`
event loop:

```rust
fn watch_portal(callback: impl Fn(ThemeChangeEvent) + Send + 'static) -> Result<ThemeWatcher> {
    let (stop_tx, stop_rx) = std::sync::mpsc::channel();
    let handle = std::thread::spawn(move || {
        let conn = zbus::blocking::Connection::session()?;
        // Subscribe to SettingChanged signal
        let proxy = conn.object_proxy(
            "org.freedesktop.portal.Desktop",
            "/org/freedesktop/portal/desktop",
        )?;
        // ... filter for appearance namespace, call callback on change ...
    });
    Ok(ThemeWatcher { stop: stop_tx, thread: handle })
}
```

No async runtime needed — `zbus::blocking` works in a plain thread.

### Linux (KDE)

**Signal:** No D-Bus signal for kdeglobals changes. KDE modifies
`~/.config/kdeglobals` when the user changes theme settings.

**Implementation:** Use `notify` crate (inotify on Linux) to watch the
file for `CLOSE_WRITE` events:

```rust
fn watch_kdeglobals(callback: impl Fn(ThemeChangeEvent) + Send + 'static) -> Result<ThemeWatcher> {
    let path = kde::kdeglobals_path();
    let (stop_tx, stop_rx) = std::sync::mpsc::channel();
    let handle = std::thread::spawn(move || {
        let (tx, rx) = std::sync::mpsc::channel();
        let mut watcher = notify::recommended_watcher(move |res| {
            if let Ok(event) = res { let _ = tx.send(event); }
        })?;
        watcher.watch(&path, RecursiveMode::NonRecursive)?;
        loop {
            match rx.recv_timeout(Duration::from_secs(1)) {
                Ok(_) => {
                    // Debounce: KDE writes multiple times in quick succession
                    std::thread::sleep(Duration::from_millis(200));
                    // Drain any pending events
                    while rx.try_recv().is_ok() {}
                    // Re-read and determine what changed
                    let is_dark = detect_is_dark();
                    callback(ThemeChangeEvent::DarkModeChanged { is_dark });
                }
                Err(RecvTimeoutError::Timeout) => {
                    if stop_rx.try_recv().is_ok() { break; }
                }
                Err(_) => break,
            }
        }
    });
    Ok(ThemeWatcher { stop: stop_tx, thread: handle })
}
```

### macOS

**Signal:** `NSAppearance` change via Objective-C KVO (Key-Value Observing)
on `NSApplication.effectiveAppearance`, or register for
`AppleInterfaceThemeChangedNotification` distributed notification.

**Implementation:**

```rust
fn watch_macos(callback: impl Fn(ThemeChangeEvent) + Send + 'static) -> Result<ThemeWatcher> {
    let handle = std::thread::spawn(move || {
        // Use distributed notification center for AppleInterfaceThemeChangedNotification
        // This fires when the user toggles dark mode in System Settings
        // objc2: register observer block, run CFRunLoop on this thread
    });
    Ok(ThemeWatcher { stop: ..., thread: handle })
}
```

The macOS implementation requires running a `CFRunLoop` on the watcher
thread to receive notifications.

### Windows

**Signal:** `UISettings::ColorValuesChanged` event.

**Implementation:**

```rust
fn watch_windows(callback: impl Fn(ThemeChangeEvent) + Send + 'static) -> Result<ThemeWatcher> {
    let handle = std::thread::spawn(move || {
        let settings = UISettings::new()?;
        let token = settings.ColorValuesChanged(&TypedEventHandler::new(
            move |_settings, _args| {
                let is_dark = detect_is_dark();
                callback(ThemeChangeEvent::DarkModeChanged { is_dark });
                Ok(())
            },
        ))?;
        // Run a message pump to keep receiving events
        // ...
        // On stop: settings.RemoveColorValuesChanged(token)
    });
    Ok(ThemeWatcher { stop: ..., thread: handle })
}
```

---

## New dependencies

| Crate | Platform | Purpose | Feature-gated |
|-------|----------|---------|---------------|
| `notify` ~7.0 | Linux (KDE) | inotify file watching for kdeglobals | `kde` feature |
| `zbus` (already transitive via ashpd) | Linux (GNOME) | Blocking D-Bus signal subscription | `portal` feature |

`notify` is the only truly new dependency. On macOS and Windows, the
existing `objc2` and `windows` crate bindings provide everything needed.

---

## Feature flag design

Two options:

### Option 1: Automatic with existing features

The `on_theme_change()` function uses whatever platform features are
already enabled. If `kde` is enabled, it watches kdeglobals. If `portal`
is enabled, it subscribes to D-Bus. No new feature flag.

### Option 2: Separate `watch` feature

```toml
[features]
watch = []  # enables on_theme_change()
watch-kde = ["kde", "dep:notify"]
watch-portal = ["portal"]  # zbus already available
```

**Recommendation:** Option 1 for simplicity. The `notify` dependency is
small (pure Rust, no C deps) and only pulled in when `kde` is already
enabled.

---

## Edge cases

1. **Multiple rapid toggles:** Debounce events (200ms window). KDE writes
   kdeglobals multiple times per theme change. GNOME portal may fire
   multiple `SettingChanged` signals.

2. **Watcher thread lifetime:** `ThemeWatcher` stops on Drop. If the user
   forgets to hold the handle, the watcher stops immediately. Document
   this clearly.

3. **Thread safety:** Callback is `Send + 'static`. The callback runs on
   the watcher thread, not the UI thread. Consumers must marshal to their
   UI thread (e.g., `iced::Subscription`, `gpui::cx.emit()`).

4. **Startup race:** If the user toggles dark mode between app startup
   and `on_theme_change()` registration, the app might miss the change.
   Document that apps should read the initial theme BEFORE registering
   the watcher.

5. **Nested environments:** Wayland on KDE fires both portal signals AND
   writes kdeglobals. If both `kde` and `portal` features are enabled,
   avoid double-firing by preferring the portal watcher (more reliable)
   and skipping the file watcher.

---

## Risk

Medium. Platform-specific event subscription is inherently tricky:
- macOS CFRunLoop management on a background thread
- Windows COM threading model (STA vs MTA for UISettings events)
- D-Bus signal matching and connection lifecycle

Mitigation: start with the simplest platform (Linux portal via zbus
blocking) and expand. Each platform implementation is independent.

## Verification

- Unit tests: mock `ThemeChangeEvent` dispatch and verify callback fires
- Integration tests: manual verification on each platform (toggle dark mode,
  observe callback)
- Stress test: rapid toggle 10 times in 2 seconds, verify debounce works
  and no events are lost
