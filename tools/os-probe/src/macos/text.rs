//! A font's descriptor as AppKit and Core Text report it (each font's
//! `descriptor` table: symbolic traits, slant and width traits, variation
//! axes and settings at two sizes, feature settings, the CSS weight), and
//! each text style's line height (`[macos.line_height]`: NSLayoutManager's
//! default and laid-out line heights and NSTextField's fitting heights, with
//! the tight and loose leading traits). Raw values only.

use objc2::rc::Retained;
use objc2::runtime::{AnyObject, NSObjectProtocol};
use objc2::{AnyThread, MainThreadMarker};
use objc2_app_kit::{
    NSFont, NSFontDescriptorSymbolicTraits, NSLayoutManager, NSTextContainer, NSTextField,
    NSTextStorage,
};
use objc2_core_foundation::CFType;
use objc2_foundation::{NSArray, NSDictionary, NSNumber, NSObject, NSRange, NSSize, NSString};
use toml::{Table, Value};

use super::{CONTROL_SIZES, ffi, ns, size};
use crate::out;

/// The second size a font's variation is read at: this many times its own.
const SECOND_SIZE_FACTOR: f64 = 2.0;

/// How deep the conversion of a returned object into TOML goes.
const OBJECT_DEPTH: u32 = 6;

/// The text the line heights are laid out with: two lines.
const TWO_LINES: &str = "Ag\nAg";

/// The width of the text container the two lines are laid out in, and the
/// wrapping label's preferred width, in points: wide enough for one line.
const LAYOUT_WIDTH: f64 = 400.0;

/// The named symbolic traits, as NSFontDescriptor.h declares them, but the
/// class bits (`ClassMask`), which `symbolicTraits_class` gives.
const SYMBOLIC_TRAITS: [(&str, NSFontDescriptorSymbolicTraits); 9] = [
    ("TraitItalic", NSFontDescriptorSymbolicTraits::TraitItalic),
    ("TraitBold", NSFontDescriptorSymbolicTraits::TraitBold),
    (
        "TraitExpanded",
        NSFontDescriptorSymbolicTraits::TraitExpanded,
    ),
    (
        "TraitCondensed",
        NSFontDescriptorSymbolicTraits::TraitCondensed,
    ),
    (
        "TraitMonoSpace",
        NSFontDescriptorSymbolicTraits::TraitMonoSpace,
    ),
    (
        "TraitVertical",
        NSFontDescriptorSymbolicTraits::TraitVertical,
    ),
    (
        "TraitUIOptimized",
        NSFontDescriptorSymbolicTraits::TraitUIOptimized,
    ),
    (
        "TraitTightLeading",
        NSFontDescriptorSymbolicTraits::TraitTightLeading,
    ),
    (
        "TraitLooseLeading",
        NSFontDescriptorSymbolicTraits::TraitLooseLeading,
    ),
];

/// What the `descriptor` table of each font holds.
pub(super) const DESCRIPTOR_METHOD: &str = "per font: symbolicTraits, the fontDescriptor's symbolic traits as an integer, symbolicTraits_set the NSFontDescriptor.h trait names whose bits it has, symbolicTraits_class its ClassMask bits shifted down by 28; traits, the fontDescriptor's NSFontTraitsAttribute dictionary whole, and NSFontSlantTrait and NSFontWidthTrait from it; italicAngle; fontAttributes, the fontDescriptor's fontAttributes whole, and NSFontVariationAttribute and NSFontFeatureSettingsAttribute from it; the font as a CTFont (toll-free bridged): CTFontCopyVariation, CTFontCopyVariationAxes, CTFontCopyFeatureSettings, CTFontCopyAttribute of kCTFontOpticalSizeAttribute and of kCTFontCSSWeightAttribute (the key looked up by name in the com.apple.CoreText bundle with CFBundleGetDataPointerForName, as the SDK does not declare it); at_second_size, fontWithDescriptor:size: of the fontDescriptor at twice the font's pointSize, its pointSize and CTFontCopyVariation; a dictionary is a table keyed by its keys' text (a number's stringValue), an NSNumber its doubleValue, an array an array; a value a method returns nil or a dictionary lacks is `unavailable`";

/// `font`'s descriptor reads, the `descriptor` table of [`super::font_table`].
pub(super) fn descriptor_table(font: &NSFont) -> Table {
    let mut t = Table::new();
    let keys = ffi::descriptor_keys();
    let descriptor = font.fontDescriptor();
    let traits = descriptor.symbolicTraits();
    out::put(&mut t, "symbolicTraits", out::int(i64::from(traits.0)));
    let set = SYMBOLIC_TRAITS
        .iter()
        .filter(|(_, bit)| traits.contains(*bit))
        .map(|(name, _)| out::text(*name))
        .collect();
    out::put(&mut t, "symbolicTraits_set", out::array(set));
    out::put(
        &mut t,
        "symbolicTraits_class",
        out::int(i64::from(
            (traits.0 & NSFontDescriptorSymbolicTraits::ClassMask.0) >> 28,
        )),
    );
    match ffi::font_trait_keys() {
        Ok((traits_key, _)) => match descriptor.objectForKey(traits_key) {
            Some(dict) => {
                out::put(&mut t, "traits", object_value(&dict, OBJECT_DEPTH));
                for (name, key) in [
                    ("NSFontSlantTrait", keys.slant),
                    ("NSFontWidthTrait", keys.width),
                ] {
                    out::put(&mut t, name, entry(&dict, key, "NSFontTraitsAttribute"));
                }
            }
            None => {
                let none = out::unavailable("the fontDescriptor has no NSFontTraitsAttribute");
                for name in ["traits", "NSFontSlantTrait", "NSFontWidthTrait"] {
                    out::put(&mut t, name, none.clone());
                }
            }
        },
        Err(reason) => out::put(&mut t, "traits", out::unavailable(reason)),
    }
    out::put(&mut t, "italicAngle", out::float(font.italicAngle()));
    let attributes = descriptor.fontAttributes();
    let attributes_obj: &AnyObject = attributes.as_ref();
    out::put(
        &mut t,
        "fontAttributes",
        object_value(attributes_obj, OBJECT_DEPTH),
    );
    for (name, key) in [
        ("NSFontVariationAttribute", keys.variation),
        ("NSFontFeatureSettingsAttribute", keys.feature_settings),
    ] {
        out::put(&mut t, name, entry(attributes_obj, key, "fontAttributes"));
    }
    out::put(
        &mut t,
        "CTFontCopyVariation",
        cf_value(ffi::ct_variation(font).as_deref().map(|d| &**d)),
    );
    out::put(
        &mut t,
        "CTFontCopyVariationAxes",
        cf_value(ffi::ct_variation_axes(font).as_deref().map(|a| &**a)),
    );
    out::put(
        &mut t,
        "CTFontCopyFeatureSettings",
        cf_value(ffi::ct_feature_settings(font).as_deref().map(|a| &**a)),
    );
    out::put(
        &mut t,
        "kCTFontOpticalSizeAttribute",
        cf_value(ffi::ct_attribute(font, ffi::ct_optical_size_key()).as_deref()),
    );
    out::put(
        &mut t,
        ffi::CT_CSS_WEIGHT_SYMBOL,
        match ffi::ct_css_weight_key() {
            Ok(key) => cf_value(ffi::ct_attribute(font, key).as_deref()),
            Err(reason) => out::unavailable(reason),
        },
    );
    let second = font.pointSize() * SECOND_SIZE_FACTOR;
    let mut at_second = Table::new();
    match NSFont::fontWithDescriptor_size(&descriptor, second) {
        Some(other) => {
            out::put(&mut at_second, "pointSize", out::float(other.pointSize()));
            out::put(
                &mut at_second,
                "fontName",
                out::text(other.fontName().to_string()),
            );
            out::put(
                &mut at_second,
                "CTFontCopyVariation",
                cf_value(ffi::ct_variation(&other).as_deref().map(|d| &**d)),
            );
        }
        None => out::put(
            &mut at_second,
            "error",
            out::unavailable(format!("fontWithDescriptor:size: {second} is nil")),
        ),
    }
    out::put_table(&mut t, "at_second_size", at_second);
    t
}

/// `dict[key]` of a dictionary `dict` named `what`, converted.
fn entry(dict: &AnyObject, key: &NSString, what: &str) -> Value {
    let Some(dict) = dict.downcast_ref::<NSDictionary>() else {
        return out::unavailable(format!("{what} is not a dictionary"));
    };
    let key_obj: &AnyObject = key.as_ref();
    match dict.objectForKey(key_obj) {
        Some(value) => object_value(&value, OBJECT_DEPTH),
        None => out::unavailable(format!("no {key} in {what}")),
    }
}

/// A Core Foundation object a call returned, converted; `None` the call's
/// NULL.
fn cf_value(cf: Option<&CFType>) -> Value {
    match cf {
        Some(cf) => object_value(cf.as_ref(), OBJECT_DEPTH),
        None => out::unavailable("NULL"),
    }
}

/// An Objective-C (or toll-free bridged Core Foundation) object as TOML: a
/// string its text, a number its `doubleValue`, an array an array, a
/// dictionary a table keyed by its keys' text, anything else its class and
/// description; `depth` levels deep at most.
fn object_value(obj: &AnyObject, depth: u32) -> Value {
    if let Some(s) = obj.downcast_ref::<NSString>() {
        return out::text(s.to_string());
    }
    if let Some(n) = obj.downcast_ref::<NSNumber>() {
        return out::float(n.doubleValue());
    }
    let Some(deeper) = depth.checked_sub(1) else {
        return out::unavailable(format!(
            "nested deeper than {OBJECT_DEPTH}: {}",
            describe(obj)
        ));
    };
    if let Some(a) = obj.downcast_ref::<NSArray>() {
        return out::array(a.iter().map(|v| object_value(&v, deeper)).collect());
    }
    if let Some(d) = obj.downcast_ref::<NSDictionary>() {
        let mut t = Table::new();
        for key in d.allKeys().iter() {
            let value = d
                .objectForKey(&key)
                .map_or_else(|| out::unavailable("nil"), |v| object_value(&v, deeper));
            out::put(&mut t, &key_text(&key), value);
        }
        return Value::Table(t);
    }
    out::text(describe(obj))
}

/// A dictionary key as a TOML key: a string's text, a number's
/// `stringValue`, anything else its class and description.
fn key_text(key: &AnyObject) -> String {
    if let Some(s) = key.downcast_ref::<NSString>() {
        return s.to_string();
    }
    if let Some(n) = key.downcast_ref::<NSNumber>() {
        return n.stringValue().to_string();
    }
    describe(key)
}

/// `<class> description`.
fn describe(obj: &AnyObject) -> String {
    let class = obj.class().name().to_string_lossy().into_owned();
    let description = obj
        .downcast_ref::<NSObject>()
        .map(NSObjectProtocol::description)
        .and_then(|d| d.downcast_ref::<NSString>().map(|s| s.to_string()));
    match description {
        Some(d) => format!("<{class}> {d}"),
        None => format!("<{class}>"),
    }
}

// ── Line height ─────────────────────────────────────────────────────────────

/// `[macos.line_height]`: per text style and named font the facts key, and
/// the system font at each control size's font size, the line heights as
/// laid out, plain and with the tight and loose leading traits added.
pub(super) fn line_height_table(mtm: MainThreadMarker) -> Table {
    let mut t = Table::new();
    out::put(
        &mut t,
        "method",
        out::text(format!(
            "per font, plain and with NSFontDescriptorTraitTightLeading and NSFontDescriptorTraitLooseLeading added to its fontDescriptor's symbolicTraits (fontDescriptorWithSymbolicTraits:, fontWithDescriptor:size: 0; symbolicTraits_set says which traits the font then has): fontName, pointSize, ascender, descender, leading; defaultLineHeightForFont: and defaultBaselineOffsetForFont: of a new NSLayoutManager; laid_out: an NSTextStorage of {two:?} with setFont: the font, an NSLayoutManager and an NSTextContainer {LAYOUT_WIDTH} pt wide (lineFragmentPadding 0), ensureLayoutForTextContainer:, then lineFragmentRectForGlyphAtIndex:effectiveRange: of the first line's first glyph and the second line's (glyphIndexForCharacterAtIndex: 3), their heights and the second's origin y less the first's (line_advance), and usedRectForTextContainer:'s height; NSTextField: labelWithString: of \"Ag\" and of {two:?}, and wrappingLabelWithString: of {two:?} with preferredMaxLayoutWidth {LAYOUT_WIDTH}, each with setFont: the font, their fittingSize; the fonts: preferredFontForTextStyle:options: per text style and its emphasized font (as [macos.fonts]), labelFontOfSize:, messageFontOfSize:, paletteFontOfSize:, menuBarFontOfSize:, controlContentFontOfSize: at size 0, systemFontOfSize: of smallSystemFontSize and of systemFontSizeForControlSize: per control size",
            two = TWO_LINES,
        )),
    );
    let mut fonts: Vec<(String, Result<Retained<NSFont>, String>)> = Vec::new();
    match ffi::text_styles() {
        Ok(styles) => {
            for (name, style) in styles {
                let font = ffi::preferred_font(style);
                let emphasized = font.as_ref().map_err(Clone::clone).and_then(|f| {
                    super::emphasized_font(f)
                        .ok_or_else(|| "fontWithDescriptor:size: is nil".to_owned())
                });
                fonts.push((format!("preferredFontForTextStyle.{name}"), font));
                fonts.push((
                    format!("preferredFontForTextStyle_emphasized.{name}"),
                    emphasized,
                ));
            }
        }
        Err(reason) => fonts.push(("preferredFontForTextStyle".to_owned(), Err(reason))),
    }
    let named: [(&str, super::FontOfSize); 5] = [
        ("labelFontOfSize", NSFont::labelFontOfSize),
        ("messageFontOfSize", NSFont::messageFontOfSize),
        ("paletteFontOfSize", NSFont::paletteFontOfSize),
        ("menuBarFontOfSize", NSFont::menuBarFontOfSize),
        ("controlContentFontOfSize", NSFont::controlContentFontOfSize),
    ];
    for (name, make) in named {
        fonts.push((format!("named.{name}"), Ok(make(0.0))));
    }
    fonts.push((
        "systemFontOfSize_smallSystemFontSize".to_owned(),
        Ok(NSFont::systemFontOfSize(NSFont::smallSystemFontSize())),
    ));
    for (name, control_size) in CONTROL_SIZES {
        fonts.push((
            format!("systemFontOfSize_systemFontSizeForControlSize.{name}"),
            Ok(NSFont::systemFontOfSize(
                NSFont::systemFontSizeForControlSize(control_size),
            )),
        ));
    }

    let mut all = Table::new();
    for (path, font) in fonts {
        let value = match font {
            Ok(font) => Value::Table(leadings_table(mtm, &font)),
            Err(reason) => out::unavailable(reason),
        };
        put_path(&mut all, &path, value);
    }
    out::put_table(&mut t, "fonts", all);
    t
}

/// Insert `value` at the dotted `path` in `t`, making the tables between.
fn put_path(t: &mut Table, path: &str, value: Value) {
    match path.split_once('.') {
        Some((head, rest)) => {
            let sub = t
                .entry(head.to_owned())
                .or_insert_with(|| Value::Table(Table::new()));
            match sub {
                Value::Table(sub) => put_path(sub, rest, value),
                other => *other = value,
            }
        }
        None => out::put(t, path, value),
    }
}

/// `font`'s line heights plain (`plain`) and with each leading trait added.
fn leadings_table(mtm: MainThreadMarker, font: &NSFont) -> Table {
    let mut t = Table::new();
    out::put_table(&mut t, "plain", line_table(mtm, font));
    let descriptor = font.fontDescriptor();
    for (name, bit) in [
        (
            "tightLeading",
            NSFontDescriptorSymbolicTraits::TraitTightLeading,
        ),
        (
            "looseLeading",
            NSFontDescriptorSymbolicTraits::TraitLooseLeading,
        ),
    ] {
        let with = descriptor.fontDescriptorWithSymbolicTraits(descriptor.symbolicTraits() | bit);
        match NSFont::fontWithDescriptor_size(&with, 0.0) {
            Some(f) => out::put_table(&mut t, name, line_table(mtm, &f)),
            None => out::put(
                &mut t,
                name,
                out::unavailable("fontWithDescriptor:size: is nil"),
            ),
        }
    }
    t
}

/// One font's metrics and line heights.
fn line_table(mtm: MainThreadMarker, font: &NSFont) -> Table {
    let mut t = Table::new();
    out::put(&mut t, "fontName", out::text(font.fontName().to_string()));
    out::put(&mut t, "pointSize", out::float(font.pointSize()));
    let traits = font.fontDescriptor().symbolicTraits();
    let set = SYMBOLIC_TRAITS
        .iter()
        .filter(|(_, bit)| traits.contains(*bit))
        .map(|(name, _)| out::text(*name))
        .collect();
    out::put(&mut t, "symbolicTraits_set", out::array(set));
    out::put(&mut t, "ascender", out::float(font.ascender()));
    out::put(&mut t, "descender", out::float(font.descender()));
    out::put(&mut t, "leading", out::float(font.leading()));
    let layout = NSLayoutManager::new();
    out::put(
        &mut t,
        "defaultLineHeightForFont",
        out::float(layout.defaultLineHeightForFont(font)),
    );
    out::put(
        &mut t,
        "defaultBaselineOffsetForFont",
        out::float(layout.defaultBaselineOffsetForFont(font)),
    );
    out::put_table(&mut t, "laid_out", laid_out(font));

    let mut field = Table::new();
    for (name, label) in [
        (
            "labelWithString_one_line",
            NSTextField::labelWithString(&ns("Ag"), mtm),
        ),
        (
            "labelWithString_two_lines",
            NSTextField::labelWithString(&ns(TWO_LINES), mtm),
        ),
        ("wrappingLabelWithString_two_lines", {
            let l = NSTextField::wrappingLabelWithString(&ns(TWO_LINES), mtm);
            l.setPreferredMaxLayoutWidth(LAYOUT_WIDTH);
            l
        }),
    ] {
        label.setFont(Some(font));
        out::put(
            &mut field,
            &format!("{name}_fittingSize"),
            size(label.fittingSize()),
        );
    }
    out::put_table(&mut t, "NSTextField", field);
    t
}

/// The two lines laid out in `font` by an NSLayoutManager.
fn laid_out(font: &NSFont) -> Table {
    let mut t = Table::new();
    let storage = NSTextStorage::new();
    storage.replaceCharactersInRange_withString(NSRange::new(0, 0), &ns(TWO_LINES));
    storage.setFont(Some(font));
    let layout = NSLayoutManager::new();
    let container = NSTextContainer::initWithSize(
        NSTextContainer::alloc(),
        NSSize::new(LAYOUT_WIDTH, f64::MAX),
    );
    container.setLineFragmentPadding(0.0);
    layout.addTextContainer(&container);
    storage.addLayoutManager(&layout);
    layout.ensureLayoutForTextContainer(&container);
    out::put(
        &mut t,
        "numberOfGlyphs",
        out::int(i64::try_from(layout.numberOfGlyphs()).unwrap_or(i64::MAX)),
    );
    let first = ffi::line_fragment_rect(&layout, 0);
    let second_glyph = layout.glyphIndexForCharacterAtIndex(3);
    let second = ffi::line_fragment_rect(&layout, second_glyph);
    out::put(&mut t, "first_line_height", out::float(first.size.height));
    out::put(&mut t, "second_line_height", out::float(second.size.height));
    out::put(
        &mut t,
        "line_advance",
        out::float(second.origin.y - first.origin.y),
    );
    out::put(
        &mut t,
        "usedRect_height",
        out::float(layout.usedRectForTextContainer(&container).size.height),
    );
    t
}
