//! The handle that serves gpui-component's widgets their own icons in the
//! theme's icon set, [`WidgetIcons`], with its asset source and its report
//! (feature `svg-rasterize`).

use super::{GpuiIcon, IconUse, load_gpui_icon};
use gpui::{App, AssetSource, Global, SharedString};
use gpui_component::theme::Theme as GpuiTheme;
use gpui_component::{IconName, IconNamed};
use native_theme::icons::IconData;
use native_theme::theme::{ColorMode, IconSet};
use native_theme::{Resolved, SystemTheme};
use std::borrow::Cow;
use std::collections::HashMap;
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::{Arc, PoisonError, RwLock};

/// The glyphs nearly every application's windows, dialogs and forms draw: the
/// window controls, the close button of a dialog, a sheet and a notification,
/// the caret of a select and a combo box, the check mark of a check box, a
/// chosen row and a menu item, and a number input's minus and plus (the minus
/// is a mixed check box's dash too). An icon set that lacks one of them, in the
/// icons of either colour mode, is not served (see [`WidgetIcons`]); an
/// application replaces the list with [`WidgetIcons::set_essentials`].
pub const ESSENTIAL_ICONS: [IconName; 9] = [
    IconName::WindowClose,
    IconName::WindowMinimize,
    IconName::WindowMaximize,
    IconName::WindowRestore,
    IconName::Close,
    IconName::ChevronDown,
    IconName::Check,
    IconName::Minus,
    IconName::Plus,
];

/// Every gpui-component [`IconName`] this crate knows, the 106 of
/// gpui-component 0.10.0, in the order of the icon tests' list (a test holds
/// the two equal). The handle loads each of them; gpui-component has no list
/// of its own.
pub(crate) const GPUI_ICON_NAMES: &[IconName] = &[
    IconName::ALargeSmall,
    IconName::ArrowDown,
    IconName::ArrowLeft,
    IconName::ArrowRight,
    IconName::ArrowUp,
    IconName::Asterisk,
    IconName::Ban,
    IconName::Battery,
    IconName::BatteryCharging,
    IconName::BatteryFull,
    IconName::BatteryLow,
    IconName::BatteryMedium,
    IconName::BatteryWarning,
    IconName::Bell,
    IconName::BookOpen,
    IconName::Bot,
    IconName::Building2,
    IconName::Calendar,
    IconName::CaseSensitive,
    IconName::ChartPie,
    IconName::Check,
    IconName::ChevronDown,
    IconName::ChevronLeft,
    IconName::ChevronRight,
    IconName::ChevronsUpDown,
    IconName::ChevronUp,
    IconName::CircleAlert,
    IconName::CircleCheck,
    IconName::CircleUser,
    IconName::CircleX,
    IconName::Close,
    IconName::Copy,
    IconName::Cpu,
    IconName::Dash,
    IconName::Delete,
    IconName::Ellipsis,
    IconName::EllipsisVertical,
    IconName::ExternalLink,
    IconName::Eye,
    IconName::EyeOff,
    IconName::File,
    IconName::FileText,
    IconName::Folder,
    IconName::FolderClosed,
    IconName::FolderOpen,
    IconName::Frame,
    IconName::GalleryVerticalEnd,
    IconName::Github,
    IconName::Globe,
    IconName::HardDrive,
    IconName::Heart,
    IconName::HeartOff,
    IconName::Inbox,
    IconName::Info,
    IconName::Inspector,
    IconName::LayoutDashboard,
    IconName::Loader,
    IconName::LoaderCircle,
    IconName::Map,
    IconName::Maximize,
    IconName::MemoryStick,
    IconName::Menu,
    IconName::Mic,
    IconName::Minimize,
    IconName::Minus,
    IconName::Moon,
    IconName::Network,
    IconName::Palette,
    IconName::PanelBottom,
    IconName::PanelBottomOpen,
    IconName::PanelLeft,
    IconName::PanelLeftClose,
    IconName::PanelLeftOpen,
    IconName::PanelRight,
    IconName::PanelRightClose,
    IconName::PanelRightOpen,
    IconName::Pause,
    IconName::Play,
    IconName::Plus,
    IconName::Redo,
    IconName::Redo2,
    IconName::RefreshCw,
    IconName::Replace,
    IconName::ResizeCorner,
    IconName::RotateCw,
    IconName::Search,
    IconName::Settings,
    IconName::Settings2,
    IconName::SortAscending,
    IconName::SortDescending,
    IconName::Square,
    IconName::SquareTerminal,
    IconName::Star,
    IconName::StarFill,
    IconName::StarOff,
    IconName::Sun,
    IconName::ThumbsDown,
    IconName::ThumbsUp,
    IconName::TriangleAlert,
    IconName::Undo,
    IconName::Undo2,
    IconName::User,
    IconName::WindowClose,
    IconName::WindowMaximize,
    IconName::WindowMinimize,
    IconName::WindowRestore,
];

/// The size a freedesktop icon theme's icons are looked up at for the
/// widgets, the size [`mask_drawable`](super::mask_drawable) judges an icon
/// at.
const WIDGET_ICON_SIZE: u32 = 16;

/// The empty icon's path: the asset source answers it with nothing, so an
/// icon drawn by it paints nothing.
const EMPTY_PATH: &str = "native-theme-gpui/empty.svg";

/// An icon set and the freedesktop icon theme of each colour mode, as the
/// handle is told them. A `Resolved` and a `Loaded` carry one mode's icon
/// theme: told one mode, the handle keeps the other mode's while the set stays
/// the same, and takes the told one for both when the set changes, until it is
/// told the other. An icon theme of `None` is the system's, detected when the
/// handle makes a choice from it (it is told it while following, fixed to it,
/// or set following again). The icon themes matter for the freedesktop set
/// only.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ThemeIcons {
    set: IconSet,
    /// The light mode's icon theme: `None` where nothing was told for it,
    /// `Some(None)` for the system's.
    light: Option<Option<String>>,
    /// The dark mode's, as the light mode's.
    dark: Option<Option<String>>,
}

impl ThemeIcons {
    /// `set`, with the icon theme of each colour mode (`None`: the system's).
    #[must_use]
    pub fn new(set: IconSet, light: Option<&str>, dark: Option<&str>) -> Self {
        Self {
            set,
            light: Some(light.map(str::to_string)),
            dark: Some(dark.map(str::to_string)),
        }
    }

    /// `set`, with the icon theme of one colour mode (`None`: the system's);
    /// nothing is told for the other.
    #[must_use]
    pub fn for_mode(set: IconSet, mode: ColorMode, icon_theme: Option<&str>) -> Self {
        let told = Some(icon_theme.map(str::to_string));
        if mode.is_dark() {
            Self {
                set,
                light: None,
                dark: told,
            }
        } else {
            Self {
                set,
                light: told,
                dark: None,
            }
        }
    }

    /// These icons with a mode nothing was told for taking the other mode's.
    fn filled(&self) -> ThemeIcons {
        ThemeIcons {
            set: self.set,
            light: self.light.clone().or_else(|| self.dark.clone()),
            dark: self.dark.clone().or_else(|| self.light.clone()),
        }
    }

    /// These icons, as last told, after `told`: a mode `told` gives replaces
    /// this one's; a mode it does not give keeps this one's while the set
    /// stays the same, and takes the mode `told` gives where the set changes.
    fn merged(&self, told: &ThemeIcons) -> ThemeIcons {
        if told.set != self.set {
            return told.filled();
        }
        ThemeIcons {
            set: told.set,
            light: told.light.clone().or_else(|| self.light.clone()),
            dark: told.dark.clone().or_else(|| self.dark.clone()),
        }
        .filled()
    }
}

impl From<&Resolved> for ThemeIcons {
    fn from(resolved: &Resolved) -> Self {
        Self::for_mode(
            resolved.icon_set,
            resolved.mode,
            resolved.icon_theme.as_deref(),
        )
    }
}

impl From<&SystemTheme> for ThemeIcons {
    fn from(sys: &SystemTheme) -> Self {
        Self::new(
            sys.icon_set,
            sys.icon_theme_for(ColorMode::Light),
            sys.icon_theme_for(ColorMode::Dark),
        )
    }
}

impl From<&crate::Loaded> for ThemeIcons {
    fn from(loaded: &crate::Loaded) -> Self {
        Self::for_mode(loaded.icon_set, loaded.mode, loaded.icon_theme.as_deref())
    }
}

/// What [`WidgetIcons::lookup`] answers for an icon's asset path, the answer
/// the handle's icon resolver gives gpui-component.
#[derive(Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum IconAnswer {
    /// The served set's icon for this name: the path the handle's asset
    /// source serves it at.
    Draw(SharedString),
    /// The served set has no icon for this name: the empty icon's path, which
    /// paints nothing.
    Empty(SharedString),
    /// Not the handle's to answer: gpui-component draws its own glyph.
    NotMine,
}

/// What the handle serves, from [`WidgetIcons::report`], in the order it is
/// checked.
// `Loaded` holds the set's findings by value: a report is built when it is
// asked for, never kept in a collection, so its size costs nothing a box
// would save.
#[allow(clippy::large_enum_variant)]
#[derive(Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum WidgetIconsReport {
    /// [`WidgetIcons::install`] never ran for this handle, so no resolver asks
    /// it.
    NotInstalled,
    /// Switched off ([`WidgetIcons::switch_off`]).
    Off,
    /// Installed, but this handle's [`WidgetIcons::assets`] was never called,
    /// so no path it served could load; it serves nothing.
    NoAssetSource,
    /// Following the theme's icons, and told none yet.
    NotTold,
    /// A set this connector does not serve: SF Symbols, Segoe Fluent, or a set
    /// it does not know.
    NotServed {
        /// The set.
        set: IconSet,
        /// Whether the application fixed it ([`WidgetIcons::fix`]), else the
        /// theme's.
        fixed: bool,
    },
    /// The set's icons, loaded and judged, whether served or not
    /// ([`LoadedSet::served`]).
    Loaded(LoadedSet),
}

/// A set the handle loaded, and what it found in it.
#[derive(Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub struct LoadedSet {
    /// The set.
    pub set: IconSet,
    /// Whether the application fixed it ([`WidgetIcons::fix`]), else the
    /// theme's.
    pub fixed: bool,
    /// The light mode's icons.
    pub light: ModeIcons,
    /// The dark mode's icons.
    pub dark: ModeIcons,
    /// The asset paths of the essential icons outside the 106 names the
    /// tables know: an `IconName` newer than this crate, not judged.
    pub unknown_essentials: Vec<SharedString>,
}

impl LoadedSet {
    /// Whether the set is served: neither mode gave nothing nor lacks an
    /// essential icon.
    #[must_use]
    pub fn served(&self) -> bool {
        served(&self.light, &self.dark)
    }
}

/// What one colour mode's icons of a [`LoadedSet`] hold. Every name is the
/// `IconName`'s asset path (`icons/check.svg`).
#[derive(Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub struct ModeIcons {
    /// The freedesktop icon theme loaded, the detected one where none was
    /// told; `None` for the bundled sets, or where detection failed.
    pub icon_theme: Option<String>,
    /// Whether the mode gave no icon at all.
    pub gave_nothing: bool,
    /// The essential icons the mode lacks.
    pub missing_essentials: Vec<SharedString>,
    /// Every name the mode has no icon for: drawn empty while the set is
    /// served, and, where it is not, the names it lacks.
    pub missing: Vec<SharedString>,
    /// The names that keep gpui-component's glyph: `StarFill` under Lucide.
    pub kept: Vec<SharedString>,
}

/// Whether a set whose modes hold `light` and `dark` is served.
fn served(light: &ModeIcons, dark: &ModeIcons) -> bool {
    [light, dark]
        .iter()
        .all(|mode| !mode.gave_nothing && mode.missing_essentials.is_empty())
}

/// Serves gpui-component's widgets their own icons from the theme's icon set:
/// the check box's mark, a select's caret, the window controls, and every
/// other icon a widget draws by an [`IconName`], in place of gpui-component's
/// bundled Lucide glyphs (feature `svg-rasterize`).
///
/// A handle is three things: an asset source around the application's
/// ([`assets`](Self::assets)), which serves the icons it loaded; the icon
/// resolver gpui-component asks for the path of each icon it draws
/// ([`install`](Self::install)); and the theme's icons, as it is told them. It
/// is in one of three states: following the theme's icons (a new handle's
/// state, and [`follow_theme`](Self::follow_theme)'s), fixed to icons the
/// application chose ([`fix`](Self::fix)), or off
/// ([`switch_off`](Self::switch_off)).
///
/// An application wraps its asset source and installs the same handle:
///
/// ```no_run
/// use native_theme_gpui::icons::WidgetIcons;
/// use native_theme_gpui::{SystemTheme, apply_system_theme};
///
/// let icons = WidgetIcons::new();
/// gpui_kit::application()
///     .with_assets(icons.assets(gpui_kit::assets::Assets))
///     .run(move |cx| {
///         gpui_kit::init(cx);
///         icons.install(cx);
///         // Tells the installed handle the theme's icons.
///         if let Ok(sys) = SystemTheme::from_system() {
///             apply_system_theme(&sys, cx);
///         }
///         // Then open the windows with `gpui_kit::open_window`.
///     });
/// ```
///
/// The handle installed must be the one whose `assets()` went to
/// `Application::with_assets`: otherwise [`report`](Self::report) says
/// [`WidgetIconsReport::NoAssetSource`] and the widgets keep gpui-component's
/// glyphs. [`apply_system_theme`](crate::apply_system_theme) tells an
/// installed handle the theme's icons; an application on the
/// [`apply`](crate::apply) path tells it itself,
/// `set_theme_icons(&resolved)`, with the [`Resolved`] it built the theme
/// from. The handle follows the colour mode: it answers from the icons of the
/// mode gpui-component's theme is in.
///
/// It answers the 106 asset paths of gpui-component's [`IconName`], whoever
/// draws them, an application's own artwork under those paths included. A set
/// that lacks one of the [essential icons](ESSENTIAL_ICONS) in either mode, a
/// set that gives nothing, and SF Symbols and Segoe Fluent are not served: the
/// widgets keep gpui-component's glyphs. Under Lucide `StarFill` keeps
/// gpui-component's glyph, a filled star Lucide lacks. An icon a served set
/// lacks is drawn empty, and [`report`](Self::report) says which.
#[derive(Clone)]
pub struct WidgetIcons {
    shared: SharedState,
}

/// The state a handle, its clones and its asset source share.
type SharedState = Arc<Shared>;

/// What a handle shares: the state, replaced whole by each method, and beside
/// it whether the handle was installed and wrapped, and its count of loaded
/// tables.
struct Shared {
    state: RwLock<Arc<State>>,
    installed: AtomicBool,
    wrapped: AtomicBool,
    loads: AtomicUsize,
}

impl Shared {
    /// The state as it stands. A poisoned lock is read through: its one write
    /// is an assignment, which leaves no half-written value.
    fn current(&self) -> Arc<State> {
        Arc::clone(&self.state.read().unwrap_or_else(PoisonError::into_inner))
    }

    /// Replaces the state.
    fn replace(&self, state: State) {
        *self.state.write().unwrap_or_else(PoisonError::into_inner) = Arc::new(state);
    }
}

/// What the handle chooses its icons from.
#[derive(Clone, Debug)]
enum Choosing {
    /// The theme's icons, as last told.
    Following,
    /// The application's choice.
    Fixed(ThemeIcons),
    /// Nothing: switched off.
    Off,
}

/// The handle's state, immutable once built.
struct State {
    /// The theme's icons as last told, each mode told.
    told: Option<ThemeIcons>,
    choosing: Choosing,
    /// The essential icons' asset paths.
    floor: Vec<SharedString>,
    /// The key of the choice the state makes, resolved when the method made
    /// it; `None` while off, or following with nothing told.
    choice: Option<IconsKey>,
    cache: Cache,
    /// The last choice served.
    serving: Option<Serving>,
}

/// A choice of icons as the handle loads it: the set, and each mode's icon
/// theme, detected where none was told.
#[derive(Clone, Debug, PartialEq, Eq)]
struct IconsKey {
    set: IconSet,
    light: ModeKey,
    dark: ModeKey,
}

/// One mode's icon theme in an [`IconsKey`].
#[derive(Clone, Debug, PartialEq, Eq)]
enum ModeKey {
    /// A set other than freedesktop's, which has no icon themes.
    NoIconTheme,
    /// The freedesktop icon theme, told or detected.
    Theme(String),
    /// Told the system's freedesktop icon theme, which could not be detected.
    DetectionFailed,
}

/// The tables loaded since the set last changed.
#[derive(Clone, Default)]
struct Cache {
    /// The set the tables belong to.
    set: Option<IconSet>,
    /// The tables by icon theme: `None` for a bundled set's one table.
    tables: HashMap<Option<String>, Arc<Table>>,
}

/// One icon theme's (or bundled set's) answers for the 106 names, and the
/// bytes of the icons it draws.
#[derive(Default)]
struct Table {
    /// By the `IconName`'s asset path.
    answers: HashMap<SharedString, Answer>,
    /// By the served path.
    bytes: HashMap<SharedString, Cow<'static, [u8]>>,
}

/// A table's answer for a name.
enum Answer {
    /// The set's icon, at this served path.
    Draw(SharedString),
    /// The set has none: the empty icon.
    Empty,
    /// gpui-component's glyph is kept.
    NotMine,
}

/// A served choice: its key, and its tables with their findings, or `None`
/// for a set this connector does not serve.
#[derive(Clone)]
struct Serving {
    key: IconsKey,
    tables: Option<ServedTables>,
}

impl Serving {
    /// This serving with its tables judged again under `floor`, nothing
    /// loaded.
    fn judged(&self, floor: &[SharedString]) -> Serving {
        Serving {
            key: self.key.clone(),
            tables: self.tables.as_ref().map(|tables| {
                judge(
                    &self.key,
                    Arc::clone(&tables.light),
                    Arc::clone(&tables.dark),
                    floor,
                )
            }),
        }
    }

    /// The report of this serving, the set `fixed` or the theme's.
    fn report(&self, fixed: bool) -> WidgetIconsReport {
        match &self.tables {
            Some(tables) => WidgetIconsReport::Loaded(LoadedSet {
                set: self.key.set,
                fixed,
                light: tables.light_icons.clone(),
                dark: tables.dark_icons.clone(),
                unknown_essentials: tables.unknown_essentials.clone(),
            }),
            None => WidgetIconsReport::NotServed {
                set: self.key.set,
                fixed,
            },
        }
    }
}

/// The two tables of a served choice, and what [`judge`] found in them.
#[derive(Clone)]
struct ServedTables {
    light: Arc<Table>,
    dark: Arc<Table>,
    light_icons: ModeIcons,
    dark_icons: ModeIcons,
    unknown_essentials: Vec<SharedString>,
}

/// What [`serve`] gives: the new cache, the new serving, and the number of
/// tables it loaded.
struct Served {
    cache: Cache,
    serving: Serving,
    loaded: usize,
}

impl Table {
    /// The table of `set`, the freedesktop icon theme `icon_theme` under
    /// Freedesktop: each name's icon a mask carries, at
    /// [`WIDGET_ICON_SIZE`] under Freedesktop; `StarFill` under Lucide keeps
    /// gpui-component's glyph.
    fn load(set: IconSet, icon_theme: Option<&str>) -> Table {
        let size = (set == IconSet::Freedesktop).then_some(WIDGET_ICON_SIZE);
        let mut table = Table::default();
        for icon in GPUI_ICON_NAMES {
            let path = icon.clone().path();
            let answer = if set == IconSet::Lucide && matches!(icon, IconName::StarFill) {
                Answer::NotMine
            } else {
                match load_gpui_icon(icon.clone(), set, icon_theme, size, None, IconUse::Mask) {
                    Some(GpuiIcon {
                        data: IconData::Svg(bytes),
                        ..
                    }) => {
                        let served = served_path(set, icon_theme, &path);
                        table.bytes.insert(served.clone(), bytes);
                        Answer::Draw(served)
                    }
                    _ => Answer::Empty,
                }
            };
            table.answers.insert(path, answer);
        }
        table
    }

    /// A table that has no icon: every name `Empty`.
    fn empty() -> Table {
        Table {
            answers: GPUI_ICON_NAMES
                .iter()
                .map(|icon| (icon.clone().path(), Answer::Empty))
                .collect(),
            bytes: HashMap::new(),
        }
    }
}

/// The path the asset source serves the icon of `set` (and of the freedesktop
/// icon theme `icon_theme`) for the `IconName` at `path`: one path per set,
/// icon theme and name, all under `native-theme-gpui/`, which no upstream
/// path begins with.
fn served_path(set: IconSet, icon_theme: Option<&str>, path: &str) -> SharedString {
    let file = path.strip_prefix("icons/").unwrap_or(path);
    match icon_theme {
        Some(theme) => format!("native-theme-gpui/{}/{theme}/{file}", set.name()),
        None => format!("native-theme-gpui/{}/{file}", set.name()),
    }
    .into()
}

/// Whether `path` is the asset path of one of [`GPUI_ICON_NAMES`].
fn is_known(path: &SharedString) -> bool {
    GPUI_ICON_NAMES
        .iter()
        .any(|icon| icon.clone().path() == *path)
}

/// What `table` holds for the mode whose key is `mode`, under `floor`.
fn mode_icons(table: &Table, mode: &ModeKey, floor: &[SharedString]) -> ModeIcons {
    let mut icons = ModeIcons {
        icon_theme: match mode {
            ModeKey::Theme(theme) => Some(theme.clone()),
            ModeKey::NoIconTheme | ModeKey::DetectionFailed => None,
        },
        gave_nothing: true,
        missing_essentials: Vec::new(),
        missing: Vec::new(),
        kept: Vec::new(),
    };
    for icon in GPUI_ICON_NAMES {
        let path = icon.clone().path();
        match table.answers.get(&path) {
            Some(Answer::Draw(_)) => icons.gave_nothing = false,
            Some(Answer::Empty) => icons.missing.push(path),
            Some(Answer::NotMine) => icons.kept.push(path),
            None => {}
        }
    }
    icons.missing_essentials = floor
        .iter()
        .filter(|path| matches!(table.answers.get(*path), Some(Answer::Empty)))
        .cloned()
        .collect();
    icons
}

/// The tables `light` and `dark` of `key`, judged under `floor`.
fn judge(
    key: &IconsKey,
    light: Arc<Table>,
    dark: Arc<Table>,
    floor: &[SharedString],
) -> ServedTables {
    ServedTables {
        light_icons: mode_icons(&light, &key.light, floor),
        dark_icons: mode_icons(&dark, &key.dark, floor),
        unknown_essentials: floor
            .iter()
            .filter(|path| !is_known(path))
            .cloned()
            .collect(),
        light,
        dark,
    }
}

/// Loads and judges the tables of `key`, reusing those `cache` holds for the
/// same set: a set other than `cache`'s empties it, and a set this connector
/// does not serve loads nothing. A freedesktop mode whose icon theme could not
/// be detected gets a table with no icon, and counts no load.
fn serve(key: &IconsKey, floor: &[SharedString], cache: &Cache) -> Served {
    let mut tables = if cache.set == Some(key.set) {
        cache.tables.clone()
    } else {
        HashMap::new()
    };
    let bundled = matches!(key.set, IconSet::Lucide | IconSet::Material);
    if !bundled && key.set != IconSet::Freedesktop {
        return Served {
            cache: Cache {
                set: Some(key.set),
                tables,
            },
            serving: Serving {
                key: key.clone(),
                tables: None,
            },
            loaded: 0,
        };
    }
    let mut loaded: usize = 0;
    let mut table_of = |mode: &ModeKey| -> Arc<Table> {
        let icon_theme = match mode {
            _ if bundled => None,
            ModeKey::Theme(theme) => Some(theme.clone()),
            ModeKey::NoIconTheme | ModeKey::DetectionFailed => return Arc::new(Table::empty()),
        };
        if let Some(table) = tables.get(&icon_theme) {
            return Arc::clone(table);
        }
        let table = Arc::new(Table::load(key.set, icon_theme.as_deref()));
        loaded = loaded.saturating_add(1);
        tables.insert(icon_theme, Arc::clone(&table));
        table
    };
    let light = table_of(&key.light);
    let dark = table_of(&key.dark);
    Served {
        serving: Serving {
            key: key.clone(),
            tables: Some(judge(key, light, dark, floor)),
        },
        cache: Cache {
            set: Some(key.set),
            tables,
        },
        loaded,
    }
}

/// The tables `state` answers from: those of the choice, where serving holds
/// its key and the set is served; `None` while the handle answers nothing.
fn answering(state: &State) -> Option<&ServedTables> {
    if matches!(state.choosing, Choosing::Off) {
        return None;
    }
    let choice = state.choice.as_ref()?;
    let serving = state.serving.as_ref()?;
    if serving.key != *choice {
        return None;
    }
    serving
        .tables
        .as_ref()
        .filter(|tables| served(&tables.light_icons, &tables.dark_icons))
}

/// The answer of `state` for `path` in the dark mode or the light: the part
/// of [`WidgetIcons::lookup`] that reads the state.
fn answer(state: &State, path: &str, dark: bool) -> IconAnswer {
    let Some(tables) = answering(state) else {
        return IconAnswer::NotMine;
    };
    let table = if dark { &tables.dark } else { &tables.light };
    match table.answers.get(path) {
        Some(Answer::Draw(served)) => IconAnswer::Draw(served.clone()),
        Some(Answer::Empty) => IconAnswer::Empty(SharedString::new_static(EMPTY_PATH)),
        Some(Answer::NotMine) | None => IconAnswer::NotMine,
    }
}

/// Whether `before` and `after` answer every path alike: the same tables, or
/// none.
fn same_answers(before: &State, after: &State) -> bool {
    match (answering(before), answering(after)) {
        (None, None) => true,
        (Some(before), Some(after)) => {
            Arc::ptr_eq(&before.light, &after.light) && Arc::ptr_eq(&before.dark, &after.dark)
        }
        _ => false,
    }
}

/// The key of `icons`: a freedesktop mode told no icon theme takes the
/// system's, detected once here.
fn key_of(icons: &ThemeIcons) -> IconsKey {
    if icons.set != IconSet::Freedesktop {
        return IconsKey {
            set: icons.set,
            light: ModeKey::NoIconTheme,
            dark: ModeKey::NoIconTheme,
        };
    }
    let mut detected: Option<ModeKey> = None;
    let mut mode = |told: &Option<Option<String>>| match told {
        Some(Some(theme)) => ModeKey::Theme(theme.clone()),
        Some(None) | None => detected
            .get_or_insert_with(|| match native_theme::detect::system_icon_theme() {
                Ok(theme) => ModeKey::Theme(theme),
                Err(_) => ModeKey::DetectionFailed,
            })
            .clone(),
    };
    IconsKey {
        set: icons.set,
        light: mode(&icons.light),
        dark: mode(&icons.dark),
    }
}

/// Whether the colour mode the widgets are drawn in is dark: the connector's
/// mode where a `NativeTheme` is installed, else gpui-component's theme's;
/// `None` with neither.
fn is_dark(cx: &App) -> Option<bool> {
    match cx.try_global::<crate::NativeTheme>() {
        Some(native) => Some(native.is_dark(cx)),
        None => cx.try_global::<GpuiTheme>().map(GpuiTheme::is_dark),
    }
}

impl WidgetIcons {
    /// A handle following the theme's icons, told nothing yet, with
    /// [`ESSENTIAL_ICONS`] as its essentials; it loads nothing.
    #[must_use]
    pub fn new() -> Self {
        let state = State {
            told: None,
            choosing: Choosing::Following,
            floor: ESSENTIAL_ICONS.into_iter().map(IconNamed::path).collect(),
            choice: None,
            cache: Cache::default(),
            serving: None,
        };
        Self {
            shared: Arc::new(Shared {
                state: RwLock::new(Arc::new(state)),
                installed: AtomicBool::new(false),
                wrapped: AtomicBool::new(false),
                loads: AtomicUsize::new(0),
            }),
        }
    }

    /// The application's asset source `inner`, wrapped so that it serves the
    /// handle's icons too, for `Application::with_assets`. Every other path
    /// goes to `inner`.
    #[must_use]
    pub fn assets<A: AssetSource>(&self, inner: A) -> WidgetIconAssets<A> {
        self.shared.wrapped.store(true, Ordering::SeqCst);
        WidgetIconAssets {
            inner,
            shared: Arc::clone(&self.shared),
        }
    }

    /// Installs the handle: gpui-component's icon resolver asks it for the
    /// path of every icon drawn from now on. It replaces a resolver and a
    /// handle installed before, and refreshes the windows.
    pub fn install(&self, cx: &mut App) {
        self.shared.installed.store(true, Ordering::SeqCst);
        cx.set_global(InstalledWidgetIcons(self.clone()));
        let handle = self.clone();
        gpui_component::set_icon_resolver(cx, move |path, cx| match handle.lookup(path, cx) {
            IconAnswer::Draw(path) | IconAnswer::Empty(path) => Some(path),
            IconAnswer::NotMine => None,
        });
        cx.refresh_windows();
    }

    /// Tells the handle the theme's icons. While it follows them, it loads
    /// the icons of the choice where it has not yet.
    pub fn set_theme_icons(&self, icons: impl Into<ThemeIcons>, cx: &mut App) {
        let before = self.shared.current();
        let icons = icons.into();
        let told = match &before.told {
            Some(told) => told.merged(&icons),
            None => icons.filled(),
        };
        match &before.choosing {
            Choosing::Following => self.choose(&before, Some(told), Choosing::Following, cx),
            Choosing::Fixed(_) | Choosing::Off => self.commit(
                &before,
                State {
                    told: Some(told),
                    choosing: before.choosing.clone(),
                    floor: before.floor.clone(),
                    choice: before.choice.clone(),
                    cache: before.cache.clone(),
                    serving: before.serving.clone(),
                },
                cx,
            ),
        }
    }

    /// Follows the theme's icons, as last told.
    pub fn follow_theme(&self, cx: &mut App) {
        let before = self.shared.current();
        if !matches!(before.choosing, Choosing::Following) {
            self.choose(&before, before.told.clone(), Choosing::Following, cx);
        }
    }

    /// Fixes the icons to `icons`, whatever the theme's; a mode `icons` does
    /// not give takes the mode it gives.
    pub fn fix(&self, icons: impl Into<ThemeIcons>, cx: &mut App) {
        let before = self.shared.current();
        let fixed = Choosing::Fixed(icons.into().filled());
        self.choose(&before, before.told.clone(), fixed, cx);
    }

    /// Switches the handle off: every widget draws gpui-component's glyphs.
    /// The icons loaded are kept for a later choice.
    pub fn switch_off(&self, cx: &mut App) {
        let before = self.shared.current();
        if matches!(before.choosing, Choosing::Off) {
            return;
        }
        self.commit(
            &before,
            State {
                told: before.told.clone(),
                choosing: Choosing::Off,
                floor: before.floor.clone(),
                choice: None,
                cache: before.cache.clone(),
                serving: before.serving.clone(),
            },
            cx,
        );
    }

    /// Replaces the essential icons ([`ESSENTIAL_ICONS`]): a set that lacks
    /// one of `icons` is not served. Nothing is loaded again.
    pub fn set_essentials(&self, icons: impl IntoIterator<Item = IconName>, cx: &mut App) {
        let before = self.shared.current();
        let floor: Vec<SharedString> = icons.into_iter().map(IconNamed::path).collect();
        let serving = before
            .serving
            .as_ref()
            .map(|serving| serving.judged(&floor));
        self.commit(
            &before,
            State {
                told: before.told.clone(),
                choosing: before.choosing.clone(),
                floor,
                choice: before.choice.clone(),
                cache: before.cache.clone(),
                serving,
            },
            cx,
        );
    }

    /// What the handle answers for the icon at `path`: the answer its
    /// resolver gives gpui-component. `NotMine` unless the handle is
    /// installed, its [`assets`](Self::assets) were called, it is not off, its
    /// choice is loaded and served, `path` is one of gpui-component's
    /// [`IconName`]s, and the colour mode is known.
    #[must_use]
    pub fn lookup(&self, path: &str, cx: &App) -> IconAnswer {
        if !self.shared.installed.load(Ordering::SeqCst)
            || !self.shared.wrapped.load(Ordering::SeqCst)
        {
            return IconAnswer::NotMine;
        }
        let Some(dark) = is_dark(cx) else {
            return IconAnswer::NotMine;
        };
        answer(&self.shared.current(), path, dark)
    }

    /// What the handle serves, and why it serves nothing where it does not.
    #[must_use]
    pub fn report(&self) -> WidgetIconsReport {
        if !self.shared.installed.load(Ordering::SeqCst) {
            return WidgetIconsReport::NotInstalled;
        }
        let state = self.shared.current();
        if matches!(state.choosing, Choosing::Off) {
            return WidgetIconsReport::Off;
        }
        if !self.shared.wrapped.load(Ordering::SeqCst) {
            return WidgetIconsReport::NoAssetSource;
        }
        let fixed = matches!(state.choosing, Choosing::Fixed(_));
        match (&state.choice, &state.serving) {
            (Some(_), Some(serving)) => serving.report(fixed),
            _ => WidgetIconsReport::NotTold,
        }
    }

    /// Makes the choice `choosing` makes from `told`: loads and judges
    /// its icons unless serving holds its key already.
    fn choose(&self, before: &State, told: Option<ThemeIcons>, choosing: Choosing, cx: &mut App) {
        let icons = match &choosing {
            Choosing::Following => told.as_ref(),
            Choosing::Fixed(icons) => Some(icons),
            Choosing::Off => None,
        };
        let choice = icons.map(key_of);
        let (cache, serving) = match &choice {
            Some(key)
                if before
                    .serving
                    .as_ref()
                    .is_none_or(|serving| serving.key != *key) =>
            {
                let served = serve(key, &before.floor, &before.cache);
                self.shared.loads.fetch_add(served.loaded, Ordering::SeqCst);
                (served.cache, Some(served.serving))
            }
            _ => (before.cache.clone(), before.serving.clone()),
        };
        self.commit(
            before,
            State {
                told,
                choosing,
                floor: before.floor.clone(),
                choice,
                cache,
                serving,
            },
            cx,
        );
    }

    /// Replaces the state `before` with `after`, and refreshes the windows
    /// where what [`lookup`](Self::lookup) answers changed.
    fn commit(&self, before: &State, after: State, cx: &mut App) {
        let changed = !same_answers(before, &after);
        self.shared.replace(after);
        if changed {
            cx.refresh_windows();
        }
    }
}

impl Default for WidgetIcons {
    fn default() -> Self {
        Self::new()
    }
}

impl std::fmt::Debug for WidgetIcons {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("WidgetIcons").field(&self.report()).finish()
    }
}

/// The installed handle, kept per `App`.
struct InstalledWidgetIcons(WidgetIcons);

impl Global for InstalledWidgetIcons {}

/// Tells the installed handle, where one is installed, the icons of `sys`.
pub(crate) fn tell_installed(sys: &SystemTheme, cx: &mut App) {
    let Some(handle) = cx
        .try_global::<InstalledWidgetIcons>()
        .map(|installed| installed.0.clone())
    else {
        return;
    };
    handle.set_theme_icons(sys, cx);
}

/// An application's asset source that serves a [`WidgetIcons`] handle's icons
/// too, from [`WidgetIcons::assets`].
pub struct WidgetIconAssets<A> {
    inner: A,
    shared: SharedState,
}

impl<A: AssetSource> AssetSource for WidgetIconAssets<A> {
    /// The empty icon's path: nothing. A path of the current serving: its
    /// icon. Any other path: the application's asset source's answer.
    fn load(&self, path: &str) -> gpui::Result<Option<Cow<'static, [u8]>>> {
        if path == EMPTY_PATH {
            return Ok(None);
        }
        let state = self.shared.current();
        let served = state
            .serving
            .as_ref()
            .and_then(|serving| serving.tables.as_ref())
            .and_then(|tables| {
                tables
                    .light
                    .bytes
                    .get(path)
                    .or_else(|| tables.dark.bytes.get(path))
            });
        match served {
            Some(bytes) => Ok(Some(bytes.clone())),
            None => self.inner.load(path),
        }
    }

    fn list(&self, path: &str) -> gpui::Result<Vec<SharedString>> {
        self.inner.list(path)
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used)]
mod tests {
    use super::*;
    #[cfg(any(feature = "lucide-icons", feature = "material-icons"))]
    use gpui::TestAppContext;

    /// The asset paths of `names`.
    fn paths(names: &[IconName]) -> Vec<SharedString> {
        names.iter().cloned().map(IconNamed::path).collect()
    }

    /// The handle's loads so far.
    #[cfg(any(feature = "lucide-icons", feature = "material-icons"))]
    fn loads(icons: &WidgetIcons) -> usize {
        icons.shared.loads.load(Ordering::SeqCst)
    }

    /// gpui-component initialised, and a handle wrapped and installed.
    #[cfg(any(feature = "lucide-icons", feature = "material-icons"))]
    fn installed(cx: &mut App) -> (WidgetIcons, WidgetIconAssets<()>) {
        gpui_kit::init(cx);
        let icons = WidgetIcons::new();
        let assets = icons.assets(());
        icons.install(cx);
        (icons, assets)
    }

    /// What gpui-component draws for `path`.
    #[cfg(any(
        feature = "lucide-icons",
        feature = "material-icons",
        all(target_os = "linux", feature = "system-icons")
    ))]
    fn resolved(path: &SharedString, cx: &App) -> SharedString {
        gpui_component::resolve_icon_path(path.clone(), cx)
    }

    /// The set the report says is served, and whether it was fixed.
    #[cfg(any(feature = "lucide-icons", feature = "material-icons"))]
    fn serving(icons: &WidgetIcons) -> Option<(IconSet, bool)> {
        match icons.report() {
            WidgetIconsReport::Loaded(set) if set.served() => Some((set.set, set.fixed)),
            _ => None,
        }
    }

    /// Whether every one of the 106 paths is `NotMine` and resolves to
    /// itself.
    #[cfg(any(feature = "lucide-icons", feature = "material-icons"))]
    fn nothing_served(icons: &WidgetIcons, cx: &App) -> bool {
        paths(GPUI_ICON_NAMES).iter().all(|path| {
            icons.lookup(path, cx) == IconAnswer::NotMine && resolved(path, cx) == *path
        })
    }

    #[test]
    fn the_library_list_is_every_icon_name() {
        assert_eq!(
            paths(GPUI_ICON_NAMES),
            paths(super::super::tests::ALL_ICON_NAMES)
        );
    }

    #[cfg(feature = "lucide-icons")]
    #[gpui::test]
    fn a_path_outside_the_106_is_not_mine(cx: &mut TestAppContext) {
        cx.update(|cx| {
            let (icons, _assets) = installed(cx);
            icons.set_theme_icons(ThemeIcons::new(IconSet::Lucide, None, None), cx);
            assert_eq!(serving(&icons), Some((IconSet::Lucide, false)));
            let outside = SharedString::from("icons/no-such.svg");
            assert_eq!(resolved(&outside, cx), outside);
            assert_eq!(icons.lookup(&outside, cx), IconAnswer::NotMine);
        });
    }

    #[cfg(feature = "lucide-icons")]
    #[gpui::test]
    fn without_its_asset_source_the_handle_serves_nothing(cx: &mut TestAppContext) {
        cx.update(|cx| {
            gpui_kit::init(cx);
            let icons = WidgetIcons::new();
            icons.install(cx);
            icons.set_theme_icons(ThemeIcons::new(IconSet::Lucide, None, None), cx);
            assert!(nothing_served(&icons, cx));
            assert_eq!(icons.report(), WidgetIconsReport::NoAssetSource);
            let _assets = icons.assets(());
            assert_eq!(serving(&icons), Some((IconSet::Lucide, false)));
            assert_eq!(
                icons.lookup("icons/check.svg", cx),
                IconAnswer::Draw("native-theme-gpui/lucide/check.svg".into())
            );
        });
    }

    #[cfg(feature = "lucide-icons")]
    #[gpui::test]
    fn lucide_is_served_and_star_fill_kept(cx: &mut TestAppContext) {
        cx.update(|cx| {
            let (icons, assets) = installed(cx);
            icons.set_theme_icons(ThemeIcons::new(IconSet::Lucide, None, None), cx);
            let star_fill = IconName::StarFill.path();
            let mut drawn = 0;
            for icon in GPUI_ICON_NAMES {
                let path = icon.clone().path();
                if path == star_fill {
                    assert_eq!(resolved(&path, cx), path, "StarFill keeps upstream's");
                    continue;
                }
                let file = path.strip_prefix("icons/").unwrap();
                let served = SharedString::from(format!("native-theme-gpui/lucide/{file}"));
                assert_eq!(resolved(&path, cx), served, "{path}");
                let name = super::super::lucide_name_for_gpui_icon(icon.clone()).unwrap();
                let Some(IconData::Svg(bytes)) =
                    native_theme::icons::LucideLoader::new(name).load()
                else {
                    panic!("Lucide's {name} is an SVG");
                };
                assert_eq!(assets.load(&served).ok().flatten(), Some(bytes), "{path}");
                drawn += 1;
            }
            assert_eq!(drawn, 105);
            assert_eq!(assets.load(EMPTY_PATH).ok().flatten(), None);
            let WidgetIconsReport::Loaded(set) = icons.report() else {
                panic!("Lucide is loaded: {:?}", icons.report());
            };
            assert!(set.served());
            for mode in [&set.light, &set.dark] {
                assert_eq!(mode.kept, vec![star_fill.clone()]);
                assert!(mode.missing.is_empty(), "{:?}", mode.missing);
            }
        });
    }

    #[cfg(feature = "material-icons")]
    #[gpui::test]
    fn material_draws_star_off_empty(cx: &mut TestAppContext) {
        cx.update(|cx| {
            let (icons, _assets) = installed(cx);
            icons.set_theme_icons(ThemeIcons::new(IconSet::Material, None, None), cx);
            assert_eq!(
                icons.lookup("icons/star-off.svg", cx),
                IconAnswer::Empty(EMPTY_PATH.into())
            );
            let WidgetIconsReport::Loaded(set) = icons.report() else {
                panic!("Material is loaded: {:?}", icons.report());
            };
            assert!(set.served());
            for mode in [&set.light, &set.dark] {
                assert_eq!(mode.missing, vec![SharedString::from("icons/star-off.svg")]);
            }
        });
    }

    #[cfg(all(feature = "lucide-icons", feature = "material-icons"))]
    #[gpui::test]
    fn the_three_states(cx: &mut TestAppContext) {
        cx.update(|cx| {
            let (icons, _assets) = installed(cx);
            let lucide = ThemeIcons::new(IconSet::Lucide, None, None);
            let material = ThemeIcons::new(IconSet::Material, None, None);
            let start = loads(&icons);
            icons.set_theme_icons(lucide.clone(), cx);
            assert_eq!(serving(&icons), Some((IconSet::Lucide, false)));
            assert_eq!(loads(&icons), start + 1, "told Lucide");
            icons.fix(material, cx);
            assert_eq!(serving(&icons), Some((IconSet::Material, true)));
            assert_eq!(loads(&icons), start + 2, "fixed to Material");
            icons.set_theme_icons(lucide, cx);
            assert_eq!(serving(&icons), Some((IconSet::Material, true)));
            assert_eq!(loads(&icons), start + 2, "told Lucide while fixed");
            icons.follow_theme(cx);
            assert_eq!(serving(&icons), Some((IconSet::Lucide, false)));
            assert_eq!(
                loads(&icons),
                start + 3,
                "following again: Material's choice dropped Lucide's table"
            );
            icons.switch_off(cx);
            assert!(nothing_served(&icons, cx));
            assert_eq!(icons.report(), WidgetIconsReport::Off);
            assert_eq!(loads(&icons), start + 3, "switched off");
            icons.follow_theme(cx);
            assert_eq!(serving(&icons), Some((IconSet::Lucide, false)));
            assert_eq!(loads(&icons), start + 3, "serving still held Lucide's key");
        });
    }

    #[cfg(feature = "material-icons")]
    #[gpui::test]
    fn a_choice_with_nothing_told_serves_nothing(cx: &mut TestAppContext) {
        cx.update(|cx| {
            let (icons, _assets) = installed(cx);
            icons.fix(ThemeIcons::new(IconSet::Material, None, None), cx);
            assert_eq!(serving(&icons), Some((IconSet::Material, true)));
            icons.follow_theme(cx);
            assert!(nothing_served(&icons, cx));
            assert_eq!(icons.report(), WidgetIconsReport::NotTold);
        });
    }

    #[cfg(feature = "lucide-icons")]
    #[gpui::test]
    fn sf_symbols_and_segoe_fluent_are_not_served(cx: &mut TestAppContext) {
        cx.update(|cx| {
            let (icons, _assets) = installed(cx);
            let lucide = ThemeIcons::new(IconSet::Lucide, None, None);
            icons.set_theme_icons(lucide.clone(), cx);
            assert_eq!(serving(&icons), Some((IconSet::Lucide, false)));
            icons.set_theme_icons(ThemeIcons::new(IconSet::SfSymbols, None, None), cx);
            assert!(nothing_served(&icons, cx));
            assert_eq!(
                icons.report(),
                WidgetIconsReport::NotServed {
                    set: IconSet::SfSymbols,
                    fixed: false
                }
            );
            let before = loads(&icons);
            icons.set_theme_icons(lucide.clone(), cx);
            assert_eq!(serving(&icons), Some((IconSet::Lucide, false)));
            assert_eq!(loads(&icons), before + 1, "SF Symbols emptied the cache");
            icons.fix(ThemeIcons::new(IconSet::SegoeFluent, None, None), cx);
            assert!(nothing_served(&icons, cx));
            assert_eq!(
                icons.report(),
                WidgetIconsReport::NotServed {
                    set: IconSet::SegoeFluent,
                    fixed: true
                }
            );
            let before = loads(&icons);
            icons.fix(lucide, cx);
            assert_eq!(serving(&icons), Some((IconSet::Lucide, true)));
            assert_eq!(loads(&icons), before + 1, "Segoe Fluent emptied the cache");
        });
    }

    #[test]
    fn told_one_mode_keeps_the_other_while_the_set_stays() {
        let breeze = ThemeIcons::new(IconSet::Freedesktop, Some("breeze"), Some("breeze-dark"));
        let told = ThemeIcons::for_mode(IconSet::Freedesktop, ColorMode::Light, Some("a"));
        assert_eq!(
            breeze.merged(&told),
            ThemeIcons::new(IconSet::Freedesktop, Some("a"), Some("breeze-dark"))
        );
        let lucide = ThemeIcons::for_mode(IconSet::Lucide, ColorMode::Light, None);
        let told = ThemeIcons::for_mode(IconSet::Freedesktop, ColorMode::Dark, Some("b"));
        assert_eq!(
            breeze.merged(&lucide).merged(&told),
            ThemeIcons::new(IconSet::Freedesktop, Some("b"), Some("b"))
        );
    }

    #[cfg(feature = "lucide-icons")]
    #[gpui::test]
    fn the_same_set_loads_once(cx: &mut TestAppContext) {
        cx.update(|cx| {
            let (icons, _assets) = installed(cx);
            let start = loads(&icons);
            icons.set_theme_icons(ThemeIcons::new(IconSet::Lucide, None, None), cx);
            icons.set_theme_icons(ThemeIcons::new(IconSet::Lucide, None, None), cx);
            assert_eq!(loads(&icons), start + 1);
        });
    }

    /// The floor's names outside the 106 are listed in `unknown_essentials`,
    /// not judged. No `IconName` outside the 106 exists to give the handle
    /// (gpui-component 0.10.0), so the handle is given all 106 and lists none;
    /// a variant upstream adds, given as an essential, shows there. The same
    /// rule, given a path outside the 106, lists it.
    #[cfg(feature = "lucide-icons")]
    #[gpui::test]
    fn a_floor_name_outside_the_tables_is_listed(cx: &mut TestAppContext) {
        cx.update(|cx| {
            let (icons, _assets) = installed(cx);
            icons.set_theme_icons(ThemeIcons::new(IconSet::Lucide, None, None), cx);
            icons.set_essentials(GPUI_ICON_NAMES.iter().cloned(), cx);
            let WidgetIconsReport::Loaded(set) = icons.report() else {
                panic!("Lucide is loaded: {:?}", icons.report());
            };
            assert!(set.unknown_essentials.is_empty());
            assert!(set.served(), "every name judged, StarFill kept");
        });
        let key = IconsKey {
            set: IconSet::Lucide,
            light: ModeKey::NoIconTheme,
            dark: ModeKey::NoIconTheme,
        };
        let newer = SharedString::from("icons/newer.svg");
        let served = serve(&key, std::slice::from_ref(&newer), &Cache::default());
        let tables = served.serving.tables.expect("Lucide is loaded");
        assert_eq!(tables.unknown_essentials, vec![newer]);
    }

    #[cfg(feature = "lucide-icons")]
    #[gpui::test]
    fn a_handle_never_installed_says_so(cx: &mut TestAppContext) {
        cx.update(|cx| {
            gpui_kit::init(cx);
            let icons = WidgetIcons::new();
            let _assets = icons.assets(());
            icons.set_theme_icons(ThemeIcons::new(IconSet::Lucide, None, None), cx);
            assert_eq!(icons.report(), WidgetIconsReport::NotInstalled);
            assert_eq!(icons.lookup("icons/check.svg", cx), IconAnswer::NotMine);
        });
    }

    /// The tables and findings of a serving that loaded, under `served`.
    fn loaded(served: &Served) -> Result<&ServedTables, &'static str> {
        served
            .serving
            .tables
            .as_ref()
            .ok_or("the set was not loaded")
    }

    /// The essential icons' asset paths.
    fn essentials() -> Vec<SharedString> {
        paths(&ESSENTIAL_ICONS)
    }

    #[test]
    fn a_mode_whose_detection_failed_gives_nothing() -> Result<(), Box<dyn std::error::Error>> {
        let key = IconsKey {
            set: IconSet::Freedesktop,
            light: ModeKey::Theme("light-a".into()),
            dark: ModeKey::DetectionFailed,
        };
        let served = serve(&key, &essentials(), &Cache::default());
        assert_eq!(served.loaded, 1, "light-a's table, not the failed mode's");
        let tables = loaded(&served)?;
        let all = paths(GPUI_ICON_NAMES);
        assert!(tables.light_icons.gave_nothing);
        assert_eq!(tables.light_icons.icon_theme.as_deref(), Some("light-a"));
        assert!(tables.dark_icons.gave_nothing);
        assert_eq!(tables.dark_icons.icon_theme, None);
        assert_eq!(tables.dark_icons.missing, all);
        assert_eq!(tables.dark_icons.missing_essentials, essentials());
        assert!(!served_tables(tables));
        let key = IconsKey {
            set: IconSet::Freedesktop,
            light: ModeKey::DetectionFailed,
            dark: ModeKey::DetectionFailed,
        };
        let served = serve(&key, &essentials(), &Cache::default());
        assert_eq!(served.loaded, 0);
        let tables = loaded(&served)?;
        assert!(tables.light_icons.gave_nothing && tables.dark_icons.gave_nothing);
        assert_eq!(tables.dark_icons.missing, all);
        assert_eq!(tables.dark_icons.missing_essentials, essentials());
        assert!(!served_tables(tables));
        Ok(())
    }

    /// Whether `tables` are served.
    fn served_tables(tables: &ServedTables) -> bool {
        served(&tables.light_icons, &tables.dark_icons)
    }

    /// The freedesktop fixtures: icon themes in a child process
    /// ([`crate::icon_fixture`]).
    #[cfg(all(target_os = "linux", feature = "system-icons"))]
    mod freedesktop {
        use super::*;
        use crate::icon_fixture::Child;
        use gpui::TestAppContext;
        use gpui_component::theme::ThemeMode as GpuiThemeMode;

        /// The first freedesktop name of each essential icon (`Close` is
        /// `tab-close`, then `window-close`).
        const ESSENTIAL_FILES: [&str; 8] = [
            "window-close",
            "window-minimize",
            "window-maximize",
            "window-restore",
            "go-down",
            "dialog-ok",
            "list-remove",
            "list-add",
        ];

        /// A one-colour icon that names its theme and file.
        fn svg(theme: &str, file: &str) -> String {
            format!(
                r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 16 16"><title>{theme}/{file}</title><path d="M2 2h12v12H2z"/></svg>"#
            )
        }

        /// Writes the theme `name` with the essential icons, `except` those
        /// left out.
        fn write_essentials(
            icons: &std::path::Path,
            name: &str,
            except: &[&str],
        ) -> std::io::Result<()> {
            let svgs: Vec<(&str, String)> = ESSENTIAL_FILES
                .iter()
                .filter(|file| !except.contains(file))
                .map(|&file| (file, svg(name, file)))
                .collect();
            let files: Vec<(&str, &str)> = svgs
                .iter()
                .map(|(file, svg)| (*file, svg.as_str()))
                .collect();
            crate::icon_fixture::write_scalable_theme(icons, name, &files)
        }

        /// A state that follows `key`'s icons and serves `served`.
        fn serving_state(key: &IconsKey, served: Served, floor: Vec<SharedString>) -> State {
            State {
                told: None,
                choosing: Choosing::Following,
                floor,
                choice: Some(key.clone()),
                cache: served.cache,
                serving: Some(served.serving),
            }
        }

        /// The key of the freedesktop icon themes `light` and `dark`.
        fn key(light: &str, dark: &str) -> IconsKey {
            IconsKey {
                set: IconSet::Freedesktop,
                light: ModeKey::Theme(light.into()),
                dark: ModeKey::Theme(dark.into()),
            }
        }

        #[test]
        fn a_set_without_an_essential_is_not_served() -> Result<(), Box<dyn std::error::Error>> {
            const NAME: &str =
                "icons::widget::tests::freedesktop::a_set_without_an_essential_is_not_served";
            if crate::icon_fixture::in_child() {
                let key = key("no-plus", "no-plus");
                let served = serve(&key, &essentials(), &Cache::default());
                let plus = SharedString::from("icons/plus.svg");
                let tables = loaded(&served)?;
                for mode in [&tables.light_icons, &tables.dark_icons] {
                    assert_eq!(mode.missing_essentials, vec![plus.clone()]);
                    assert!(mode.missing.contains(&plus));
                }
                let state = serving_state(&key, served, essentials());
                for path in paths(GPUI_ICON_NAMES) {
                    for dark in [false, true] {
                        assert_eq!(answer(&state, &path, dark), IconAnswer::NotMine, "{path}");
                    }
                }
                return Ok(());
            }
            crate::icon_fixture::run(NAME, Child::kde("no-plus"), |icons| {
                write_essentials(icons, "no-plus", &["list-add"])
            })?;
            Ok(())
        }

        #[test]
        fn an_empty_floor_serves_what_the_set_gives() -> Result<(), Box<dyn std::error::Error>> {
            const NAME: &str =
                "icons::widget::tests::freedesktop::an_empty_floor_serves_what_the_set_gives";
            if crate::icon_fixture::in_child() {
                let key = key("no-plus", "no-plus");
                let mut served = serve(&key, &essentials(), &Cache::default());
                assert!(!served_tables(loaded(&served)?));
                served.serving = served.serving.judged(&[]);
                assert!(served_tables(loaded(&served)?));
                let state = serving_state(&key, served, Vec::new());
                assert_eq!(
                    answer(&state, "icons/plus.svg", false),
                    IconAnswer::Empty(EMPTY_PATH.into())
                );
                assert_eq!(
                    answer(&state, "icons/minus.svg", false),
                    IconAnswer::Draw("native-theme-gpui/freedesktop/no-plus/minus.svg".into())
                );
                return Ok(());
            }
            crate::icon_fixture::run(NAME, Child::kde("no-plus"), |icons| {
                write_essentials(icons, "no-plus", &["list-add"])
            })?;
            Ok(())
        }

        #[test]
        fn a_set_that_gives_nothing_is_not_served() -> Result<(), Box<dyn std::error::Error>> {
            const NAME: &str =
                "icons::widget::tests::freedesktop::a_set_that_gives_nothing_is_not_served";
            if crate::icon_fixture::in_child() {
                let served = serve(
                    &key("not-installed", "not-installed"),
                    &essentials(),
                    &Cache::default(),
                );
                let tables = loaded(&served)?;
                assert!(tables.light_icons.gave_nothing && tables.dark_icons.gave_nothing);
                assert!(!served_tables(tables));
                return Ok(());
            }
            crate::icon_fixture::run(NAME, Child::kde("installed"), |icons| {
                write_essentials(icons, "installed", &[])
            })?;
            Ok(())
        }

        #[gpui::test]
        fn the_resolver_follows_the_mode(cx: &mut TestAppContext) {
            const NAME: &str = "icons::widget::tests::freedesktop::the_resolver_follows_the_mode";
            let close = SharedString::from("icons/window-close.svg");
            let served = |theme: &str| {
                SharedString::from(format!(
                    "native-theme-gpui/freedesktop/{theme}/window-close.svg"
                ))
            };
            if crate::icon_fixture::in_child() {
                let (icons, assets) = cx.update(|cx| {
                    gpui_kit::init(cx);
                    let icons = WidgetIcons::new();
                    let assets = icons.assets(());
                    icons.install(cx);
                    icons.set_theme_icons(
                        ThemeIcons::new(IconSet::Freedesktop, Some("light-a"), Some("dark-a")),
                        cx,
                    );
                    (icons, assets)
                });
                cx.update(|cx| {
                    GpuiTheme::change(GpuiThemeMode::Light, None, cx);
                    assert_eq!(resolved(&close, cx), served("light-a"));
                });
                cx.update(|cx| GpuiTheme::change(GpuiThemeMode::Dark, None, cx));
                cx.update(|cx| assert_eq!(resolved(&close, cx), served("dark-a")));
                for theme in ["light-a", "dark-a"] {
                    assert_eq!(
                        assets.load(&served(theme)).ok().flatten().as_deref(),
                        Some(svg(theme, "window-close").as_bytes())
                    );
                }
                drop(icons);
                return;
            }
            crate::icon_fixture::run(NAME, Child::kde("light-a"), |icons| {
                write_essentials(icons, "light-a", &[])?;
                write_essentials(icons, "dark-a", &[])
            })
            .unwrap();
        }

        #[test]
        fn a_freedesktop_path_names_its_theme() -> Result<(), Box<dyn std::error::Error>> {
            const NAME: &str =
                "icons::widget::tests::freedesktop::a_freedesktop_path_names_its_theme";
            if crate::icon_fixture::in_child() {
                let key = key("named", "named");
                let served = serve(&key, &essentials(), &Cache::default());
                let state = serving_state(&key, served, essentials());
                assert_eq!(
                    answer(&state, "icons/window-close.svg", false),
                    IconAnswer::Draw("native-theme-gpui/freedesktop/named/window-close.svg".into())
                );
                return Ok(());
            }
            crate::icon_fixture::run(NAME, Child::kde("named"), |icons| {
                write_essentials(icons, "named", &[])
            })?;
            Ok(())
        }

        #[test]
        fn a_mode_switch_loads_only_the_new_icon_theme() -> Result<(), Box<dyn std::error::Error>> {
            const NAME: &str =
                "icons::widget::tests::freedesktop::a_mode_switch_loads_only_the_new_icon_theme";
            if crate::icon_fixture::in_child() {
                let first = serve(&key("light-a", "light-a"), &essentials(), &Cache::default());
                assert_eq!(first.loaded, 1);
                let second = serve(&key("light-a", "dark-a"), &essentials(), &first.cache);
                assert_eq!(second.loaded, 1);
                let third = serve(&key("light-a", "light-a"), &essentials(), &second.cache);
                assert_eq!(third.loaded, 0);
                assert!(served_tables(loaded(&third)?));
                return Ok(());
            }
            crate::icon_fixture::run(NAME, Child::kde("light-a"), |icons| {
                write_essentials(icons, "light-a", &[])?;
                write_essentials(icons, "dark-a", &[])
            })?;
            Ok(())
        }
    }
}
