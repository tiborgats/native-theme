//! Font matching, and the platform's own typeface for a family.
//!
//! [`select_face`](crate::fonts::select_face) is the one matcher — CSS Fonts Module Level 4 §5.1
//! "Localized name matching" and §5.2 "Matching font styles" over a list of
//! face descriptions — and needs no font database, so it is compiled
//! unconditionally. Behind the `system-fonts` feature, `system_face` runs
//! it over the system font database, loaded once per process, and returns
//! the chosen face's bytes; on macOS the system UI font is found by its file
//! through Core Text, because no font database files it under the name the
//! platform documents. `system_face` substitutes no family; `substitute_family`,
//! behind the same feature, names the family the platform draws in place of
//! one it has no face of — on Linux and the other fontconfig platforms,
//! fontconfig's best match, which `system_face` then finds by name.

use crate::theme::FontStyle;

/// What [`select_face`] knows about one candidate face.
#[derive(Clone, Copy, Debug)]
pub struct FaceTraits<'a> {
    /// Every family name the face records.
    pub families: &'a [&'a str],
    /// OS/2 `usWidthClass`, `1..=9`; `5` is normal.
    pub width: u16,
    /// The face's style.
    pub style: FontStyle,
    /// OS/2 `usWeightClass`, the CSS weight.
    pub weight: u16,
}

/// The index in `faces` of the face CSS Fonts Module Level 4 selects for
/// `family`, `weight` and `style`, or `None` where no face records `family`.
///
/// Only faces one of whose family names equals `family` under Unicode
/// default caseless matching (§5.1 "Localized name matching"; `unicase`)
/// are candidates, and **no other family is ever substituted**: an empty
/// candidate set is `None`. Among the candidates the three steps of §5.2
/// "Matching font styles" narrow the set in order, with the model's
/// normal width — `font-width`: normal first, then narrower widths
/// descending, then wider ascending; `font-style`: `Italic` takes italic,
/// then oblique, then normal, `Oblique` oblique, italic, normal, and
/// `Normal` normal, oblique, italic; `font-weight`: for a desired weight
/// inclusively between 400 and 500, weights at or above it ascending up to
/// and including 500, then below it descending, then above 500; below 400,
/// weights at or below it descending, then above it ascending; above 500,
/// weights at or above it ascending, then below it descending — and the
/// first face in `faces` order among those left is returned.
///
/// fontdb's own `query` is deliberately not used: it compares family names
/// case-sensitively, and its weight step follows Level 3 with a `450`
/// cut-off, which picks `300` where Level 4 picks `450` for a desired `400`.
#[must_use]
pub fn select_face(
    faces: &[FaceTraits<'_>],
    family: &str,
    weight: u16,
    style: FontStyle,
) -> Option<usize> {
    let wanted = unicase::UniCase::new(family);
    let mut candidates: Vec<(usize, &FaceTraits<'_>)> = faces
        .iter()
        .enumerate()
        .filter(|(_, face)| {
            face.families
                .iter()
                .any(|name| unicase::UniCase::new(*name) == wanted)
        })
        .collect();
    if candidates.is_empty() {
        return None;
    }
    keep_best(&mut candidates, |face| width_rank(face.width));
    keep_best(&mut candidates, |face| style_rank(face.style, style));
    keep_best(&mut candidates, |face| weight_rank(face.weight, weight));
    candidates.first().map(|(index, _)| *index)
}

/// Keep the candidates of the lowest rank.
fn keep_best(
    candidates: &mut Vec<(usize, &FaceTraits<'_>)>,
    rank: impl Fn(&FaceTraits<'_>) -> (u8, u16),
) {
    if let Some(best) = candidates.iter().map(|(_, face)| rank(face)).min() {
        candidates.retain(|(_, face)| rank(face) == best);
    }
}

/// §5.2 `font-width` for the normal width: `(group, distance)`, lower first.
fn width_rank(width: u16) -> (u8, u16) {
    const NORMAL: u16 = 5;
    match width.cmp(&NORMAL) {
        std::cmp::Ordering::Equal => (0, 0),
        std::cmp::Ordering::Less => (1, NORMAL.saturating_sub(width)),
        std::cmp::Ordering::Greater => (2, width),
    }
}

/// §5.2 `font-style`: the position of `face` in the order `wanted` sets.
fn style_rank(face: FontStyle, wanted: FontStyle) -> (u8, u16) {
    let rank = match (wanted, face) {
        (w, f) if w == f => 0,
        (FontStyle::Italic, FontStyle::Oblique)
        | (FontStyle::Oblique, FontStyle::Italic)
        | (FontStyle::Normal, FontStyle::Oblique) => 1,
        _ => 2,
    };
    (rank, 0)
}

/// §5.2 `font-weight`: the three quoted rules as `(group, distance)`.
fn weight_rank(face: u16, wanted: u16) -> (u8, u16) {
    let below = wanted.saturating_sub(face);
    match wanted {
        400..=500 if face >= wanted && face <= 500 => (0, face),
        400..=500 if face < wanted => (1, below),
        400..=500 => (2, face),
        0..=399 if face <= wanted => (0, below),
        0..=399 => (1, face),
        _ if face >= wanted => (0, face),
        _ => (1, below),
    }
}

/// Whether `family` names the macOS system UI font: caselessly equal to
/// "SF Pro", the name the platform documents for it
/// (`docs/platform-facts.md:59-67`) and `macos-sonoma` states. A pure
/// predicate, on every platform, with no font database: `system_face`'s
/// macOS branch asks it beside its Core Text family comparison, and the
/// gpui connector asks it alone to map the family to gpui's own alias.
#[must_use]
pub fn is_macos_system_ui_family(family: &str) -> bool {
    unicase::UniCase::new(family) == unicase::UniCase::new("SF Pro")
}

/// A face [`system_face`] chose: its bytes, and what fontdb records for it.
#[cfg(feature = "system-fonts")]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SystemFace {
    /// The face's file, whole: for a face of a collection (`.ttc`), the
    /// collection, addressed by `index`.
    pub data: std::sync::Arc<[u8]>,
    /// The face's index in `data`.
    pub index: u32,
    /// The face's own family name as fontdb records it: the first of its
    /// `FaceInfo::families`, "always English US, unless it's missing from
    /// the font" (fontdb 0.23.0 `src/lib.rs` lines 826–827). A toolkit that draws
    /// by name draws this one.
    pub family: std::sync::Arc<str>,
    /// The face's own weight, OS/2 `usWeightClass`.
    pub weight: u16,
    /// The face's own style.
    pub style: FontStyle,
}

/// The system font database, loaded once per process and shared by every
/// [`system_face`] call. A font installed while the process runs is seen
/// after a restart, as with iced's own font system.
#[cfg(feature = "system-fonts")]
fn database() -> &'static fontdb::Database {
    static DB: std::sync::OnceLock<fontdb::Database> = std::sync::OnceLock::new();
    DB.get_or_init(|| {
        let mut db = fontdb::Database::new();
        db.load_system_fonts();
        db
    })
}

#[cfg(feature = "system-fonts")]
fn style_of(style: fontdb::Style) -> FontStyle {
    match style {
        fontdb::Style::Normal => FontStyle::Normal,
        fontdb::Style::Italic => FontStyle::Italic,
        fontdb::Style::Oblique => FontStyle::Oblique,
    }
}

/// [`select_face`] over `db`'s faces; the chosen face's bytes copied once.
#[cfg(feature = "system-fonts")]
fn face_in(
    db: &fontdb::Database,
    family: &str,
    weight: u16,
    style: FontStyle,
) -> Option<SystemFace> {
    let infos: Vec<&fontdb::FaceInfo> = db.faces().collect();
    let names: Vec<Vec<&str>> = infos
        .iter()
        .map(|info| {
            info.families
                .iter()
                .map(|(name, _)| name.as_str())
                .collect()
        })
        .collect();
    let traits: Vec<FaceTraits<'_>> = infos
        .iter()
        .zip(&names)
        .map(|(info, names)| FaceTraits {
            families: names,
            width: info.stretch.to_number(),
            style: style_of(info.style),
            weight: info.weight.0,
        })
        .collect();
    let chosen = select_face(&traits, family, weight, style)?;
    let info = infos.get(chosen)?;
    let (data, index) = db.with_face_data(info.id, |data, index| {
        (std::sync::Arc::<[u8]>::from(data), index)
    })?;
    let (recorded, _) = info.families.first()?;
    Some(SystemFace {
        data,
        index,
        family: std::sync::Arc::from(recorded.as_str()),
        weight: info.weight.0,
        style: style_of(info.style),
    })
}

/// The system's own face for `family`, `weight` and `style`, or `None`.
///
/// The first call in a process loads the system font database
/// (`fontdb::Database::load_system_fonts`), which every later call shares;
/// each call runs [`select_face`] over its faces and copies the chosen
/// face's file into the returned [`SystemFace`], so call it when a theme
/// changes, not per frame. A family the system has no face of is `None`:
/// nothing is substituted here; [`substitute_family`] names the family the
/// platform draws in its place, to ask this function for next. On macOS a
/// `family` that names the system UI
/// font — [`is_macos_system_ui_family`], or caselessly Core Text's own
/// family name for it — is found by its file instead: fontdb files that
/// font under `.SF NS`, not under the platform's name, so Core Text is
/// asked which file its system font is (upright, or its italic for an
/// `Italic` or `Oblique` style), that file alone is loaded, and
/// [`select_face`] picks among its faces by width, style and weight under
/// the family the file itself records; a `None` from Core Text, or a file
/// that yields no face, is `None` here too. Every other family, on macOS as
/// elsewhere, goes through the name lookup.
#[cfg(feature = "system-fonts")]
#[must_use]
pub fn system_face(family: &str, weight: u16, style: FontStyle) -> Option<SystemFace> {
    #[cfg(target_os = "macos")]
    {
        let italic = matches!(style, FontStyle::Italic | FontStyle::Oblique);
        let ui = crate::macos::system_ui_font(italic);
        let names_it = is_macos_system_ui_family(family)
            || ui.as_ref().is_some_and(|font| {
                unicase::UniCase::new(family) == unicase::UniCase::new(font.family.as_str())
            });
        if names_it {
            return ui.and_then(|font| face_in_file(&font.path, weight, style));
        }
    }
    face_in(database(), family, weight, style)
}

/// The face of one font file — the macOS system UI font's — selected by
/// width, style and weight under the family the file's first face records.
#[cfg(all(feature = "system-fonts", target_os = "macos"))]
fn face_in_file(path: &std::path::Path, weight: u16, style: FontStyle) -> Option<SystemFace> {
    let mut db = fontdb::Database::new();
    db.load_font_file(path).ok()?;
    let recorded = db.faces().next()?.families.first()?.0.clone();
    face_in(&db, &recorded, weight, style)
}

/// The family the platform draws in place of `family`, or `None` where the
/// platform has that family, gives no answer, or its fallback is not
/// implemented here.
///
/// On Linux and the other fontconfig platforms — Unix other than macOS, iOS
/// and Android — fontconfig is asked through its own `fc-match` tool, run as
/// `fc-match --format=%{family[0]} <pattern>`: `fc-match` parses the pattern
/// with `FcNameParse`, runs `FcConfigSubstitute` and `FcDefaultSubstitute`
/// on it and prints `FcFontMatch`'s best match (fontconfig 2.15.0
/// `fc-match/fc-match.c` lines 169, 188–189 and 218), which is the font
/// every native application of the system gets for that family, and
/// `%{family[0]}` is that match's first family name (`FcPatternFormat(3)`).
/// The pattern is `family` with the characters fontconfig's name syntax
/// reserves in a family — `\`, `-`, `:` and `,` — each preceded by a `\`
/// (fontconfig user's guide, "Font Names"). `None` when `fc-match` cannot be run or fails,
/// prints nothing, or prints a family caselessly equal to `family`: the
/// system has that family, and [`system_face`] finds it by name. Each call
/// starts a process, so call it when a theme changes, not per frame.
///
/// macOS, Windows and every other platform: `None`. Their own fallback for a
/// missing family is not implemented: no code path of this crate reads it.
#[cfg(feature = "system-fonts")]
#[must_use]
pub fn substitute_family(family: &str) -> Option<String> {
    platform_substitute(family)
}

#[cfg(all(
    feature = "system-fonts",
    unix,
    not(target_os = "macos"),
    not(target_os = "ios"),
    not(target_os = "android")
))]
fn platform_substitute(family: &str) -> Option<String> {
    let output = std::process::Command::new("fc-match")
        .arg("--format=%{family[0]}")
        .arg(fontconfig_family(family))
        .output()
        .ok()?;
    if !output.status.success() {
        return None;
    }
    let printed = String::from_utf8(output.stdout).ok()?;
    let matched = printed.trim();
    if matched.is_empty() || unicase::UniCase::new(matched) == unicase::UniCase::new(family) {
        return None;
    }
    Some(matched.to_owned())
}

#[cfg(all(
    feature = "system-fonts",
    not(all(
        unix,
        not(target_os = "macos"),
        not(target_os = "ios"),
        not(target_os = "android")
    ))
))]
fn platform_substitute(_family: &str) -> Option<String> {
    None
}

/// `family` as a fontconfig pattern's family: "The '\\', '-', ':' and ','
/// characters in family names must be preceded by a '\\' character to avoid
/// having them misinterpreted" (fontconfig user's guide, "Font Names";
/// `FcNameParse` reads the families up to an unescaped `-`, `,` or `:`, and
/// takes the character after a `\` as it is, fontconfig 2.15.0
/// `src/fcname.c` lines 415–443 and 467).
#[cfg(all(
    feature = "system-fonts",
    unix,
    not(target_os = "macos"),
    not(target_os = "ios"),
    not(target_os = "android")
))]
fn fontconfig_family(family: &str) -> String {
    let mut pattern = String::with_capacity(family.len());
    for c in family.chars() {
        if matches!(c, '\\' | '-' | ':' | ',') {
            pattern.push('\\');
        }
        pattern.push(c);
    }
    pattern
}

#[cfg(test)]
#[allow(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::indexing_slicing,
    reason = "a test fails by panicking"
)]
mod tests {
    use super::*;

    const N: FontStyle = FontStyle::Normal;
    const I: FontStyle = FontStyle::Italic;
    const O: FontStyle = FontStyle::Oblique;

    fn face(
        families: &'static [&'static str],
        width: u16,
        style: FontStyle,
        weight: u16,
    ) -> FaceTraits<'static> {
        FaceTraits {
            families,
            width,
            style,
            weight,
        }
    }

    /// CSS Fonts Level 4 §5.1: names match case-insensitively by Unicode
    /// default caseless matching; a family with no face is `None`, never
    /// another family.
    #[test]
    fn the_family_is_matched_caselessly_and_never_substituted() {
        let faces = [face(&["Inter"], 5, N, 400), face(&["Roboto"], 5, N, 400)];
        assert_eq!(select_face(&faces, "inter", 400, N), Some(0));
        assert_eq!(select_face(&faces, "INTER", 400, N), Some(0));
        assert_eq!(select_face(&faces, "Helvetica", 400, N), None);
        assert_eq!(select_face(&[], "Inter", 400, N), None);
    }

    /// Any of the names a face records matches, not only the first.
    #[test]
    fn any_recorded_family_name_matches() {
        let faces = [face(&["Segoe UI", "Segoe UI Regular"], 5, N, 400)];
        assert_eq!(select_face(&faces, "segoe ui regular", 400, N), Some(0));
    }

    /// §5.2 `font-width`, with the model's normal width `5`: normal first,
    /// then narrower widths descending, then wider ascending.
    #[test]
    fn width_prefers_normal_then_narrower_descending_then_wider_ascending() {
        let normal = [
            face(&["F"], 3, N, 400),
            face(&["F"], 7, N, 400),
            face(&["F"], 5, N, 400),
        ];
        assert_eq!(select_face(&normal, "F", 400, N), Some(2));
        let narrow = [
            face(&["F"], 2, N, 400),
            face(&["F"], 7, N, 400),
            face(&["F"], 3, N, 400),
        ];
        assert_eq!(select_face(&narrow, "F", 400, N), Some(2));
        let wide = [face(&["F"], 8, N, 400), face(&["F"], 6, N, 400)];
        assert_eq!(select_face(&wide, "F", 400, N), Some(1));
    }

    /// §5.2 `font-style`: italic → italic, oblique, normal; oblique →
    /// oblique, italic, normal; normal → normal, oblique, italic.
    #[test]
    fn style_falls_back_in_the_css_order() {
        let no_italic = [face(&["F"], 5, N, 400), face(&["F"], 5, O, 400)];
        assert_eq!(select_face(&no_italic, "F", 400, I), Some(1));
        let no_oblique = [face(&["F"], 5, N, 400), face(&["F"], 5, I, 400)];
        assert_eq!(select_face(&no_oblique, "F", 400, O), Some(1));
        let no_normal = [face(&["F"], 5, I, 400), face(&["F"], 5, O, 400)];
        assert_eq!(select_face(&no_normal, "F", 400, N), Some(1));
        let exact = [face(&["F"], 5, O, 400), face(&["F"], 5, I, 400)];
        assert_eq!(select_face(&exact, "F", 400, I), Some(1));
    }

    /// §5.2 `font-weight`, desired weight inclusively between 400 and 500:
    /// weights at or above the target ascending up to and including 500,
    /// then below descending, then above 500 ascending. Level 4 picks 450
    /// for 400 from {300, 450}, where fontdb's Level 3 rule picks 300
    /// (fontdb 0.23.0 `src/lib.rs` lines 1207–1208).
    #[test]
    fn weight_between_400_and_500_climbs_to_500_first() {
        let faces = [face(&["F"], 5, N, 300), face(&["F"], 5, N, 450)];
        assert_eq!(select_face(&faces, "F", 400, N), Some(1));
        let faces = [face(&["F"], 5, N, 600), face(&["F"], 5, N, 300)];
        assert_eq!(select_face(&faces, "F", 400, N), Some(1));
        let faces = [face(&["F"], 5, N, 400), face(&["F"], 5, N, 500)];
        assert_eq!(select_face(&faces, "F", 450, N), Some(1));
        let faces = [face(&["F"], 5, N, 700), face(&["F"], 5, N, 400)];
        assert_eq!(select_face(&faces, "F", 500, N), Some(1));
    }

    /// Desired weight below 400: at or below the target descending, then
    /// above ascending.
    #[test]
    fn weight_below_400_descends_first() {
        let faces = [face(&["F"], 5, N, 350), face(&["F"], 5, N, 200)];
        assert_eq!(select_face(&faces, "F", 300, N), Some(1));
        let faces = [face(&["F"], 5, N, 700), face(&["F"], 5, N, 350)];
        assert_eq!(select_face(&faces, "F", 300, N), Some(1));
    }

    /// Desired weight above 500: at or above the target ascending, then
    /// below descending.
    #[test]
    fn weight_above_500_ascends_first() {
        let faces = [face(&["F"], 5, N, 600), face(&["F"], 5, N, 800)];
        assert_eq!(select_face(&faces, "F", 700, N), Some(1));
        let faces = [face(&["F"], 5, N, 500), face(&["F"], 5, N, 600)];
        assert_eq!(select_face(&faces, "F", 700, N), Some(1));
    }

    /// The three steps narrow in order, and a tie keeps the first face.
    #[test]
    fn width_outranks_style_which_outranks_weight() {
        let faces = [
            face(&["F"], 3, N, 400),
            face(&["F"], 5, I, 700),
            face(&["F"], 5, I, 400),
        ];
        assert_eq!(select_face(&faces, "F", 400, N), Some(2));
        let tie = [face(&["F"], 5, N, 400), face(&["F"], 5, N, 400)];
        assert_eq!(select_face(&tie, "F", 400, N), Some(0));
    }

    /// The macOS system UI font's documented name, caselessly, and nothing
    /// else (`docs/platform-facts.md:59-67`).
    #[test]
    fn the_macos_system_ui_family_is_sf_pro_caselessly() {
        assert!(is_macos_system_ui_family("SF Pro"));
        assert!(is_macos_system_ui_family("sf pro"));
        assert!(!is_macos_system_ui_family("SF Mono"));
        assert!(!is_macos_system_ui_family("SF Pro Text"));
        assert!(!is_macos_system_ui_family("Inter"));
    }

    /// fontconfig's name-syntax specials in a family — `\`, `-`, `:` and `,` —
    /// are each preceded by a `\`; every other character, a space included,
    /// is kept as it is (fontconfig user's guide, "Font Names").
    #[cfg(all(
        feature = "system-fonts",
        unix,
        not(target_os = "macos"),
        not(target_os = "ios"),
        not(target_os = "android")
    ))]
    #[test]
    fn a_fontconfig_family_escapes_the_name_syntax_specials() {
        assert_eq!(fontconfig_family("JetBrains Mono"), "JetBrains Mono");
        assert_eq!(fontconfig_family("sans-serif"), r"sans\-serif");
        assert_eq!(fontconfig_family(r"a\b:c,d-e"), r"a\\b\:c\,d\-e");
        assert_eq!(fontconfig_family(r"\\"), r"\\\\");
        assert_eq!(fontconfig_family(""), "");
        assert_eq!(
            fontconfig_family("Noto Sans CJK 日本"),
            "Noto Sans CJK 日本"
        );
    }

    #[cfg(feature = "system-fonts")]
    mod system {
        use super::super::*;

        /// A family no system has is drawn in fontconfig's best match, a
        /// family the system has, so its substitute is itself `None`. On a
        /// fontconfig platform where `fc-match` cannot be run, and on every
        /// other platform, there is no substitute.
        #[test]
        fn a_missing_family_has_a_substitute_the_system_has() {
            let missing = format!("native-theme-no-such-family-{}", std::process::id());
            let substitute = substitute_family(&missing);
            let fontconfig_platform = cfg!(all(
                unix,
                not(target_os = "macos"),
                not(target_os = "ios"),
                not(target_os = "android")
            ));
            let fc_match_runs = std::process::Command::new("fc-match")
                .arg("--version")
                .output()
                .is_ok_and(|output| output.status.success());
            if !(fontconfig_platform && fc_match_runs) {
                println!(
                    "a_missing_family_has_a_substitute_the_system_has: fc-match not run here, \
                     substitute {substitute:?}"
                );
                assert_eq!(substitute, None);
                return;
            }
            let substitute = substitute.expect("fc-match names a substitute");
            assert!(
                system_face(&substitute, 400, FontStyle::Normal).is_some(),
                "the substitute {substitute:?} has a system face"
            );
            assert_eq!(substitute_family(&substitute), None);
        }

        /// Never substituted: a family no system has is `None`.
        #[test]
        fn a_family_no_system_has_is_none() {
            assert_eq!(
                system_face("native-theme-no-such-family-7f3c1a", 400, FontStyle::Normal),
                None
            );
        }

        /// A name that differs only in case matches, and the face reports
        /// the family as fontdb records it, not as asked. The family is
        /// taken from the loaded database itself, so the clause holds on
        /// every runner that has a font with a lower-case letter in its
        /// family name; a runner with none fails here, and says so.
        #[test]
        fn a_family_matches_caselessly_and_reports_its_recorded_name() {
            let db = database();
            let (recorded, weight, style) = db
                .faces()
                .find_map(|info| {
                    let (name, _) = info.families.first()?;
                    name.chars()
                        .any(char::is_lowercase)
                        .then(|| (name.clone(), info.weight.0, style_of(info.style)))
                })
                .expect("the font database holds a face whose family has a lower-case letter");
            let asked = recorded.to_uppercase();
            assert_ne!(asked, recorded);
            let face = system_face(&asked, weight, style)
                .expect("the upper-cased family name finds the face");
            assert_ne!(
                face.family.as_ref(),
                asked.as_str(),
                "not the spelling asked for"
            );
            let recorded_by_a_face_of_that_family = db.faces().any(|info| {
                info.families
                    .first()
                    .is_some_and(|(first, _)| **first == *face.family)
                    && info.families.iter().any(|(name, _)| {
                        unicase::UniCase::new(name.as_str())
                            == unicase::UniCase::new(recorded.as_str())
                    })
            });
            assert!(
                recorded_by_a_face_of_that_family,
                "the family fontdb records for the face"
            );
        }

        /// Two calls with the same arguments return the same face.
        #[test]
        fn the_same_query_gives_the_same_face() {
            let info = database()
                .faces()
                .next()
                .expect("the font database holds a face");
            let (name, _) = info.families.first().expect("a face records a family");
            let a = system_face(name, info.weight.0, style_of(info.style));
            let b = system_face(name, info.weight.0, style_of(info.style));
            assert!(a.is_some());
            assert_eq!(a, b);
        }

        /// The file route: "SF Pro" and Core Text's family name both give
        /// the face whose family is the first family the Core Text file's
        /// first face records. The repository holds no font file a unit test
        /// could load in the Core Text file's place, so this is the route's
        /// only check; the screenshot workflow (`.github/workflows/screenshots.yml`)
        /// runs the egui connector's `system_faces_resolve` on the macOS
        /// runner too.
        #[cfg(target_os = "macos")]
        #[test]
        fn the_macos_system_ui_font_is_found_by_its_file() {
            let ui = crate::macos::system_ui_font(false)
                .expect("Core Text names the system UI font's file");
            let expected = {
                let mut db = fontdb::Database::new();
                db.load_font_file(&ui.path)
                    .expect("the system UI font's file loads");
                db.faces()
                    .next()
                    .and_then(|face| face.families.first().map(|(name, _)| name.clone()))
                    .expect("the file records a family")
            };
            for asked in ["SF Pro", ui.family.as_str()] {
                let face = system_face(asked, 400, FontStyle::Normal)
                    .unwrap_or_else(|| panic!("{asked} resolves through Core Text"));
                assert_eq!(face.family.as_ref(), expected.as_str());
            }
        }
    }
}
