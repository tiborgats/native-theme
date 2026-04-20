//! Internal backend-reader contract.
//!
//! Per `docs/todo_v0.5.7_gaps.md` §G8: each platform backend (KDE, GNOME,
//! macOS, Windows, GNOME+portal-KDE composite) implements this trait with
//! a unit struct. Dispatch happens via [`pipeline::select_reader`].
//!
//! The trait is `pub(crate)` — consumers of native-theme call
//! [`SystemTheme::from_system`] or [`SystemTheme::from_system_async`] and
//! never touch readers directly. This keeps the dispatch machinery as an
//! implementation detail (per Phase 79-02 C6 audit).
//!
//! ## Why `#[async_trait::async_trait]` instead of native async-fn-in-trait
//!
//! This trait is consumed via `Box<dyn ThemeReader>` in
//! [`pipeline::select_reader`]. Native async-fn-in-trait (stable Rust 1.75+,
//! edition 2024) supports STATIC dispatch but is NOT object-safe for
//! `dyn Trait` as of Rust 1.95 (2026-04). The `async_trait` macro rewrites
//! `async fn read(&self) -> Result<...>` into
//! `fn read<'a>(&'a self) -> Pin<Box<dyn Future<Output=...> + Send + 'a>>`
//! — a concrete return type that vtables can hold, making `Box<dyn>` compile.
//!
//! See plan 94-03 objective for the full three-option decision rationale.
//!
//! [`SystemTheme::from_system`]: crate::SystemTheme::from_system
//! [`SystemTheme::from_system_async`]: crate::SystemTheme::from_system_async
//! [`pipeline::select_reader`]: crate::pipeline

/// Internal backend-reader contract used by [`crate::pipeline`] to drive
/// platform-specific theme detection through a single trait-object vtable.
///
/// Sync backends (KDE filesystem, macOS CoreGraphics, Windows registry)
/// wrap their synchronous work in the trait's async body with no
/// `.await` points — the resulting future resolves immediately.
///
/// Async backends (GNOME portal, GNOME+portal-KDE composite) use the
/// D-Bus portal API and contain genuine `.await` points.
///
/// The `: Send + Sync` supertrait bound is required so that the macro-
/// generated `Pin<Box<dyn Future + Send>>` return type satisfies `Send`.
/// All current impls (`KdeReader`, `GnomeReader`, `GnomePortalKdeReader`,
/// `MacosReader`, `WindowsReader`) are zero-size unit structs with no
/// interior mutability, so the bound adds no runtime cost.
#[async_trait::async_trait]
pub(crate) trait ThemeReader: Send + Sync {
    /// Read the current OS theme state and produce a structured result.
    async fn read(&self) -> crate::Result<crate::ReaderResult>;
}
