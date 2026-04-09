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
| Linux (KDE) | Same portal signal (Plasma 5.27+ implements XDG portal); file watch on `~/.config/kdeglobals` as fallback for older KDE | file changed |
| macOS | `NSAppearance` KVO, `NSSystemColorsDidChangeNotification` | new appearance/colors |
| Windows | `UISettings::ColorValuesChanged` event | (none — re-query) |

Without change watching, native-theme's value for long-running desktop apps
is limited: the app starts with the right theme but can't follow system
changes at runtime.

---

## Scope

**Minimum:** Dark mode toggle detection. The user toggles light/dark mode
in system settings; the app receives a notification.

**Extended (future, same infrastructure):** Accent color, font, icon theme,
high contrast changes. Once the watcher exists, watching additional keys
is incremental.

---

## API design options

### Option A: Callback-based (recommended)

```rust
/// Handle returned by `on_theme_change`. Drop it to stop watching.
pub struct ThemeWatcher { /* platform handle */ }

/// Watch for system theme changes.
///
/// Calls `callback` on a background thread whenever the OS reports a
/// theme-related change (dark mode toggle, accent color, etc.).
/// Drop the returned `ThemeWatcher` to stop watching.
pub fn on_theme_change(
    callback: impl Fn(ThemeChangeEvent) + Send + 'static,
) -> Result<ThemeWatcher>;

/// What changed in the system theme.
#[non_exhaustive]
pub enum ThemeChangeEvent {
    /// Dark/light mode changed. New value is `is_dark`.
    DarkModeChanged { is_dark: bool },
    /// Accent color changed.
    AccentColorChanged,
    /// Something else changed (font, icon theme, etc.).
    Other,
}
```

**Pros:**
- No async runtime dependency — the watcher spawns its own `std::thread`.
- Works with any GUI framework (callback posts to the framework's event loop).
- RAII cleanup via `Drop` on `ThemeWatcher`.
- Lowest common denominator — every consumer can use it regardless of
  async runtime choice or framework.

**Cons:**
- Callback runs on background thread — consumer must marshal to UI thread.
  This is standard for OS event APIs, but adds boilerplate.
- No backpressure. If the consumer is slow, events queue up in the callback.
- Consumers using async code must bridge manually (channel from callback
  to async task).

### Option B: Async Stream

```rust
/// Returns an async Stream of theme change events.
pub fn theme_changes() -> impl Stream<Item = ThemeChangeEvent>;
```

**Pros:**
- Natural for async consumers (tokio select!, iced Subscription, etc.).
- Backpressure built in (Stream semantics).
- Can be combined with other async streams in the framework's event loop.

**Cons:**
- Forces an async runtime dependency on the consumer. native-theme already
  has this tension with portal-tokio vs portal-async-io — adding another
  runtime-dependent API doubles the problem.
- Not usable by synchronous GUI frameworks (egui, immediate-mode UIs)
  without wrapping in a thread + channel — negating the benefit.
- The watcher's actual I/O is trivial (one D-Bus signal subscription or
  file watch). An async Stream is overkill for the underlying work.

### Option C: Polling helper

```rust
/// Start a polling thread that calls `detect_is_dark()` every `interval`
/// and invokes `callback` when the value changes.
pub fn poll_dark_mode(
    interval: Duration,
    callback: impl Fn(bool) + Send + 'static,
) -> ThemeWatcher;
```

**Pros:**
- Simplest implementation. No platform-specific signal subscription.
- Works on every platform without OS API integration.
- No new dependencies.

**Cons:**
- Wasteful: wakes up every N seconds even when nothing changed.
  Unacceptable for laptop battery life with short intervals.
- Latency equals the poll interval. 1-second polling means up to 1 second
  delay on theme change. Longer intervals increase delay.
- Cannot detect accent color or font changes without re-reading the full
  theme (expensive) every poll.

### Why Option A

Option B adds async runtime coupling for trivial I/O — wrong trade-off for
a toolkit-agnostic library. Option C wastes resources and has inherent
latency. Option A is the minimal, universal primitive. Async consumers can
trivially wrap it:

```rust
let (tx, rx) = tokio::sync::mpsc::unbounded_channel();
let _watcher = on_theme_change(move |event| { let _ = tx.send(event); });
// rx is now an async stream of ThemeChangeEvent
```

Option C remains useful as an internal fallback for platforms where native
signals are unavailable.

---

## Platform implementation

### Linux (GNOME and modern KDE via portal)

**Signal:** `org.freedesktop.portal.Settings::SettingChanged` D-Bus signal
on the session bus. Available on GNOME and KDE Plasma 5.27+ (both implement
the XDG Desktop Portal settings interface).

**Watched keys:**
- `org.freedesktop.appearance` / `color-scheme` — 1=dark, 2=light
- `org.freedesktop.appearance` / `accent-color` — (future scope)

**Implementation:** `zbus` is already a transitive dependency via `ashpd`.
Use `zbus::blocking::Connection` on a dedicated thread:

```rust
fn watch_portal(callback: impl Fn(ThemeChangeEvent) + Send + 'static) -> Result<ThemeWatcher> {
    let (stop_tx, stop_rx) = std::sync::mpsc::channel();
    let handle = std::thread::spawn(move || {
        let conn = zbus::blocking::Connection::session()?;
        let proxy = conn.object_proxy(
            "org.freedesktop.portal.Desktop",
            "/org/freedesktop/portal/desktop",
        )?;
        // Subscribe to SettingChanged, filter for appearance namespace
        // Call callback(ThemeChangeEvent::DarkModeChanged { is_dark }) on match
        // Check stop_rx between iterations
    });
    Ok(ThemeWatcher { stop: stop_tx, thread: handle })
}
```

**Note:** `zbus::blocking` runs its own internal async reactor — no tokio
or async-io dependency needed at the consumer level. Verify this works
without the tokio feature on ashpd. If not, fall back to raw D-Bus via
`dbus` crate (pure blocking, no async).

### Linux (KDE fallback for older Plasma)

KDE Plasma < 5.27 doesn't implement the XDG portal settings interface.
Fallback: watch `~/.config/kdeglobals` for file modifications.

**Implementation:** Use `notify` crate (inotify backend on Linux):

```rust
fn watch_kdeglobals(callback: impl Fn(ThemeChangeEvent) + Send + 'static) -> Result<ThemeWatcher> {
    let path = kde::kdeglobals_path();
    let handle = std::thread::spawn(move || {
        let (tx, rx) = std::sync::mpsc::channel();
        let mut watcher = notify::recommended_watcher(move |res| {
            if let Ok(event) = res { let _ = tx.send(event); }
        })?;
        watcher.watch(&path, RecursiveMode::NonRecursive)?;
        // Debounce (200ms) — KDE writes multiple times per change
        // Call callback with new is_dark value
    });
    Ok(ThemeWatcher { ... })
}
```

**Dispatch logic:** Try portal first. If the portal connection fails or
`SettingChanged` never fires (no portal backend), fall back to file
watching if `kde` feature is enabled.

### macOS

**Signal:** `AppleInterfaceThemeChangedNotification` via
`NSDistributedNotificationCenter`, or KVO on
`NSApplication.effectiveAppearance`.

**Implementation:** Spawn a thread, create a `CFRunLoop`, register for
the distributed notification. The `objc2` bindings (already a dependency
with the `macos` feature) provide the necessary APIs.

### Windows

**Signal:** `UISettings::ColorValuesChanged` event.

**Implementation:** Spawn a thread, create `UISettings`, subscribe to
`ColorValuesChanged` with a `TypedEventHandler`. Run a message pump
(`GetMessage`/`DispatchMessage` loop) to receive WinRT events.

**Threading note:** `UISettings` requires STA (single-threaded apartment).
The watcher thread must call `CoInitializeEx(COINIT_APARTMENTTHREADED)`
before creating UISettings.

---

## New dependencies

| Crate | Platform | Purpose | Already in tree? |
|-------|----------|---------|-----------------|
| `zbus` (blocking) | Linux (portal) | D-Bus signal subscription | Yes, via ashpd |
| `notify` ~7.0 | Linux (KDE fallback) | inotify file watching | No — new dependency |

`notify` is the only truly new dependency. It's pure Rust, no C deps.
On macOS and Windows, the existing `objc2` and `windows` bindings provide
everything needed.

---

## Feature flag options

### Option 1: Automatic with existing features

`on_theme_change()` uses whatever platform features are already enabled.
`kde` → file watcher available. `portal` → D-Bus watcher available.
`macos` → KVO available. `windows` → UISettings event available.

**Pros:**
- No new features to document or configure.
- Users who enable `native` get watching for free.

**Cons:**
- `notify` crate (~5 transitive deps) is pulled in whenever `kde` is
  enabled, even if the consumer never calls `on_theme_change()`.
- Users who only want `from_kde()` pay the compile cost of `notify`.

### Option 2: Separate `watch` feature (recommended)

```toml
[features]
watch = []  # enables on_theme_change(); pulls in notify on Linux/KDE
```

`on_theme_change()` is gated behind `#[cfg(feature = "watch")]`.

**Pros:**
- Opt-in: users who don't need runtime watching don't pay for `notify`.
- Clear signal of intent — `watch` in Cargo.toml means the app wants
  live change tracking.
- Can be combined with any platform feature: `features = ["kde", "watch"]`.

**Cons:**
- One more feature flag to document (15 → 16).
- Users might forget to enable it and wonder why `on_theme_change` doesn't
  exist. Mitigated by a clear compile error message.

### Why Option 2

The `notify` crate is small but not free. Since most users of `from_kde()`
are reading the theme at startup and don't need runtime watching, making
it opt-in respects the zero-cost principle. The portal and macOS/Windows
watchers don't add new deps (zbus/objc2/windows are already present), but
gating them behind the same `watch` feature keeps the API surface consistent.

---

## Edge cases

1. **Debouncing:** KDE writes kdeglobals multiple times per theme change.
   GNOME portal may fire multiple `SettingChanged` signals. Debounce with
   a 200ms window — collapse rapid events into one callback invocation.

2. **Watcher lifetime:** `ThemeWatcher` stops on Drop. If the consumer
   doesn't hold the handle, the watcher dies immediately. Document this.

3. **Thread safety:** Callback runs on the watcher thread, not the UI thread.
   Consumers must marshal to their event loop.

4. **Startup race:** A toggle between app startup and watcher registration
   can be missed. Document: read theme first, then register watcher.

5. **KDE + portal overlap:** On modern KDE (Plasma 5.27+), both portal
   signals and kdeglobals file changes fire. Prefer portal when available;
   skip file watcher to avoid double-firing.

---

## Risk

Medium. Platform-specific event subscription involves:
- macOS: CFRunLoop management on a background thread
- Windows: COM apartment threading (STA required for UISettings)
- D-Bus: signal matching and connection lifecycle

Mitigation: implement Linux portal first (simplest, zbus handles connection
lifecycle). Each platform is independent — ship one at a time.

## Verification

- Unit tests: verify callback fires on simulated event dispatch
- Integration tests: manual on each platform (toggle dark mode, observe callback)
- Stress test: rapid toggle 10 times in 2 seconds, verify debounce and no lost events
