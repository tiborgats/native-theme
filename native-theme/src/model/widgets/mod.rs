// Per-widget struct pairs and macros

/// A resolved (non-optional) font specification produced after theme resolution.
///
/// Unlike [`crate::model::FontSpec`], all fields are required (non-optional)
/// because resolution has already filled in all defaults.
#[derive(Clone, Debug, PartialEq)]
pub struct ResolvedFontSpec {
    /// Font family name.
    pub family: String,
    /// Font size in logical pixels.
    pub size: f32,
    /// CSS font weight (100–900).
    pub weight: u16,
}

/// Generates a paired Option-based theme struct and a Resolved struct from a single definition.
///
/// # Usage
///
/// ```ignore
/// define_widget_pair! {
///     /// Doc comment
///     ButtonTheme / ResolvedButtonTheme {
///         option {
///             color: crate::Rgba,
///             size: f32,
///         }
///         optional_nested {
///             font: crate::model::FontSpec / ResolvedFontSpec,
///         }
///     }
/// }
/// ```
///
/// This generates:
/// - `ButtonTheme` with all `option` fields as `Option<T>` and all `optional_nested` fields
///   as `Option<FontSpec>` (the first type in the pair). Derives: Clone, Debug, Default,
///   PartialEq, Serialize, Deserialize. Attributes: skip_serializing_none, serde(default).
/// - `ResolvedButtonTheme` with all `option` fields as plain `T` and all `optional_nested`
///   fields as `ResolvedFontSpec` (the second type in the pair). Derives: Clone, Debug, PartialEq.
/// - `impl_merge!` invocation for `ButtonTheme` using the `optional_nested` clause for font fields.
macro_rules! define_widget_pair {
    (
        $(#[$attr:meta])*
        $opt_name:ident / $resolved_name:ident {
            $(option {
                $($opt_field:ident : $opt_type:ty),* $(,)?
            })?
            $(optional_nested {
                $($on_field:ident : [$on_opt_type:ty, $on_res_type:ty]),* $(,)?
            })?
        }
    ) => {
        $(#[$attr])*
        #[serde_with::skip_serializing_none]
        #[derive(Clone, Debug, Default, PartialEq, serde::Serialize, serde::Deserialize)]
        #[serde(default)]
        pub struct $opt_name {
            $($(pub $opt_field: Option<$opt_type>,)*)?
            $($(pub $on_field: Option<$on_opt_type>,)*)?
        }

        $(#[$attr])*
        #[derive(Clone, Debug, PartialEq)]
        pub struct $resolved_name {
            $($(pub $opt_field: $opt_type,)*)?
            $($(pub $on_field: $on_res_type,)*)?
        }

        $crate::impl_merge!($opt_name {
            $(option { $($opt_field),* })?
            $(optional_nested { $($on_field),* })?
        });
    };
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::FontSpec;

    // Define a test widget pair using the macro
    define_widget_pair! {
        /// Test widget for macro verification.
        TestWidget / ResolvedTestWidget {
            option {
                size: f32,
                label: String,
            }
            optional_nested {
                font: [FontSpec, ResolvedFontSpec],
            }
        }
    }

    // === ResolvedFontSpec tests ===

    #[test]
    fn resolved_font_spec_fields_are_concrete() {
        let rfs = ResolvedFontSpec {
            family: "Inter".into(),
            size: 14.0,
            weight: 400,
        };
        assert_eq!(rfs.family, "Inter");
        assert_eq!(rfs.size, 14.0);
        assert_eq!(rfs.weight, 400);
    }

    // === define_widget_pair! generated struct tests ===

    #[test]
    fn generated_option_struct_has_option_fields() {
        let w = TestWidget::default();
        assert!(w.size.is_none());
        assert!(w.label.is_none());
        assert!(w.font.is_none());
    }

    #[test]
    fn generated_option_struct_is_empty_by_default() {
        assert!(TestWidget::default().is_empty());
    }

    #[test]
    fn generated_option_struct_not_empty_when_size_set() {
        let w = TestWidget {
            size: Some(24.0),
            ..Default::default()
        };
        assert!(!w.is_empty());
    }

    #[test]
    fn generated_option_struct_not_empty_when_font_set() {
        let w = TestWidget {
            font: Some(FontSpec {
                size: Some(14.0),
                ..Default::default()
            }),
            ..Default::default()
        };
        assert!(!w.is_empty());
    }

    #[test]
    fn generated_resolved_struct_has_concrete_fields() {
        let resolved = ResolvedTestWidget {
            size: 24.0,
            label: "Click me".into(),
            font: ResolvedFontSpec {
                family: "Inter".into(),
                size: 14.0,
                weight: 400,
            },
        };
        assert_eq!(resolved.size, 24.0);
        assert_eq!(resolved.label, "Click me");
        assert_eq!(resolved.font.family, "Inter");
    }

    // === merge tests for generated structs ===

    #[test]
    fn generated_merge_option_field_overlay_wins() {
        let mut base = TestWidget {
            size: Some(20.0),
            ..Default::default()
        };
        let overlay = TestWidget {
            size: Some(24.0),
            ..Default::default()
        };
        base.merge(&overlay);
        assert_eq!(base.size, Some(24.0));
    }

    #[test]
    fn generated_merge_option_field_none_preserves_base() {
        let mut base = TestWidget {
            size: Some(20.0),
            ..Default::default()
        };
        let overlay = TestWidget::default();
        base.merge(&overlay);
        assert_eq!(base.size, Some(20.0));
    }

    #[test]
    fn generated_merge_optional_nested_both_some_merges_inner() {
        let mut base = TestWidget {
            font: Some(FontSpec {
                family: Some("Noto Sans".into()),
                size: Some(12.0),
                weight: None,
            }),
            ..Default::default()
        };
        let overlay = TestWidget {
            font: Some(FontSpec {
                family: None,
                size: None,
                weight: Some(700),
            }),
            ..Default::default()
        };
        base.merge(&overlay);
        let font = base.font.as_ref().unwrap();
        assert_eq!(font.family.as_deref(), Some("Noto Sans")); // preserved
        assert_eq!(font.size, Some(12.0));                     // preserved
        assert_eq!(font.weight, Some(700));                    // overlay sets
    }

    #[test]
    fn generated_merge_optional_nested_none_plus_some_clones() {
        let mut base = TestWidget::default();
        let overlay = TestWidget {
            font: Some(FontSpec {
                family: Some("Inter".into()),
                size: Some(14.0),
                weight: Some(400),
            }),
            ..Default::default()
        };
        base.merge(&overlay);
        let font = base.font.as_ref().unwrap();
        assert_eq!(font.family.as_deref(), Some("Inter"));
        assert_eq!(font.size, Some(14.0));
        assert_eq!(font.weight, Some(400));
    }

    #[test]
    fn generated_merge_optional_nested_some_plus_none_preserves_base() {
        let mut base = TestWidget {
            font: Some(FontSpec {
                family: Some("Inter".into()),
                size: Some(14.0),
                weight: Some(400),
            }),
            ..Default::default()
        };
        let overlay = TestWidget::default();
        base.merge(&overlay);
        let font = base.font.as_ref().unwrap();
        assert_eq!(font.family.as_deref(), Some("Inter"));
    }

    #[test]
    fn generated_merge_optional_nested_none_plus_none_stays_none() {
        let mut base = TestWidget::default();
        let overlay = TestWidget::default();
        base.merge(&overlay);
        assert!(base.font.is_none());
    }

    // === impl_merge! optional_nested clause direct tests ===

    // Verify the optional_nested clause directly on a FontSpec-containing struct
    #[derive(Clone, Debug, Default, PartialEq, serde::Serialize, serde::Deserialize)]
    struct WithFont {
        name: Option<String>,
        font: Option<FontSpec>,
    }

    impl_merge!(WithFont {
        option { name }
        optional_nested { font }
    });

    #[test]
    fn impl_merge_optional_nested_none_none_stays_none() {
        let mut base = WithFont::default();
        let overlay = WithFont::default();
        base.merge(&overlay);
        assert!(base.font.is_none());
    }

    #[test]
    fn impl_merge_optional_nested_some_none_preserves_base() {
        let mut base = WithFont {
            font: Some(FontSpec {
                size: Some(12.0),
                ..Default::default()
            }),
            ..Default::default()
        };
        let overlay = WithFont::default();
        base.merge(&overlay);
        assert_eq!(base.font.as_ref().unwrap().size, Some(12.0));
    }

    #[test]
    fn impl_merge_optional_nested_none_some_clones_overlay() {
        let mut base = WithFont::default();
        let overlay = WithFont {
            font: Some(FontSpec {
                family: Some("Inter".into()),
                ..Default::default()
            }),
            ..Default::default()
        };
        base.merge(&overlay);
        assert_eq!(
            base.font.as_ref().unwrap().family.as_deref(),
            Some("Inter")
        );
    }

    #[test]
    fn impl_merge_optional_nested_some_some_merges_inner() {
        let mut base = WithFont {
            font: Some(FontSpec {
                family: Some("Noto".into()),
                size: Some(11.0),
                weight: None,
            }),
            ..Default::default()
        };
        let overlay = WithFont {
            font: Some(FontSpec {
                family: None,
                size: Some(14.0),
                weight: Some(400),
            }),
            ..Default::default()
        };
        base.merge(&overlay);
        let f = base.font.as_ref().unwrap();
        assert_eq!(f.family.as_deref(), Some("Noto")); // preserved
        assert_eq!(f.size, Some(14.0));                 // overlay wins
        assert_eq!(f.weight, Some(400));                // overlay sets
    }

    #[test]
    fn impl_merge_optional_nested_is_empty_none() {
        let w = WithFont::default();
        assert!(w.is_empty());
    }

    #[test]
    fn impl_merge_optional_nested_is_empty_some() {
        let w = WithFont {
            font: Some(FontSpec::default()),
            ..Default::default()
        };
        assert!(!w.is_empty());
    }
}
