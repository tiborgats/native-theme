//! The showcases' menus in the macOS system menu bar: their titles, in the
//! order the three showcases install them, and the check that the menu bar
//! AppKit holds has them.
//!
//! The list is Apple's HIG, *The menu bar*: the app menu first, under the
//! application's name, then File, View, the showcases' own Theme menu, and
//! Window ("Provide a Window menu even if your app has only one window");
//! no Help menu, which the HIG describes as access to help documentation,
//! and the showcases have none. Each showcase builds the menus' items
//! itself, in one wording: the app menu's About <name>, Settings…,
//! Services, Hide <name>, Hide Others, Show All and Quit <name>; File's
//! Close; Window's Minimize, Zoom and Bring All to Front.

/// The macOS menu bar's titles, in order: `app`, the app menu's title as the
/// showcase gives it, then File, View, Theme and Window.
#[must_use]
pub fn macos_menu_titles(app: &str) -> [String; 5] {
    [app, "File", "View", "Theme", "Window"].map(str::to_string)
}

/// Whether the menu bar read back, `read` (`None` where none could be read:
/// off the main thread, or no main menu installed), holds exactly the
/// titles [`macos_menu_titles`] gives for `app`, in order; the error says
/// what it holds instead.
pub fn check_main_menu(app: &str, read: Option<Vec<String>>) -> Result<(), String> {
    let built = macos_menu_titles(app);
    match read {
        Some(read) if read == built => Ok(()),
        Some(read) => Err(format!(
            "the macOS main menu holds {read:?}, not the showcase's {built:?}"
        )),
        None => Err("no macOS main menu to read back".to_string()),
    }
}

/// The titles of `NSApplication.mainMenu`'s items, in order, as AppKit
/// holds them: `None` off the main thread, or where no main menu is
/// installed.
#[cfg(target_os = "macos")]
#[must_use]
pub fn main_menu_titles() -> Option<Vec<String>> {
    let mtm = objc2::MainThreadMarker::new()?;
    let menu = objc2_app_kit::NSApplication::sharedApplication(mtm).mainMenu()?;
    Some(
        (0..menu.numberOfItems())
            .filter_map(|index| menu.itemAtIndex(index))
            .map(|item| item.title().to_string())
            .collect(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The app menu comes first, under the name the showcase gives it, and
    /// the Window menu last, with no Help menu.
    #[test]
    fn the_menu_bar_is_the_hig_list() {
        assert_eq!(
            macos_menu_titles("showcase"),
            ["showcase", "File", "View", "Theme", "Window"]
        );
    }

    /// The check passes on the titles in order only, and says what the
    /// menu bar holds otherwise.
    #[test]
    fn the_check_compares_the_titles_in_order() {
        let titles = |list: &[&str]| Some(list.iter().map(|t| t.to_string()).collect());
        assert_eq!(
            check_main_menu("app", titles(&["app", "File", "View", "Theme", "Window"])),
            Ok(())
        );
        let swapped = check_main_menu("app", titles(&["app", "View", "File", "Theme", "Window"]));
        assert!(
            swapped
                .as_ref()
                .is_err_and(|error| error.contains("\"View\", \"File\"")),
            "{swapped:?}"
        );
        assert!(check_main_menu("app", titles(&["app", "File", "View", "Theme"])).is_err());
        assert_eq!(
            check_main_menu("app", None),
            Err("no macOS main menu to read back".to_string())
        );
    }
}
