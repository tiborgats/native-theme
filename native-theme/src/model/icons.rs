// Icon type definitions: IconRole, IconData, IconSet
//
// These are the core icon types for the native-theme icon system.
// Implementation will follow TDD -- tests first.

#[cfg(test)]
mod tests {
    use super::*;

    // === IconRole tests ===

    #[test]
    fn icon_role_all_has_42_variants() {
        assert_eq!(IconRole::ALL.len(), 42);
    }

    #[test]
    fn icon_role_all_contains_every_variant() {
        // Verify specific variants from each category are present
        let all = &IconRole::ALL;

        // Dialog (6)
        assert!(all.contains(&IconRole::DialogWarning));
        assert!(all.contains(&IconRole::DialogError));
        assert!(all.contains(&IconRole::DialogInfo));
        assert!(all.contains(&IconRole::DialogQuestion));
        assert!(all.contains(&IconRole::DialogSuccess));
        assert!(all.contains(&IconRole::Shield));

        // Window (4)
        assert!(all.contains(&IconRole::WindowClose));
        assert!(all.contains(&IconRole::WindowMinimize));
        assert!(all.contains(&IconRole::WindowMaximize));
        assert!(all.contains(&IconRole::WindowRestore));

        // Action (14)
        assert!(all.contains(&IconRole::ActionSave));
        assert!(all.contains(&IconRole::ActionDelete));
        assert!(all.contains(&IconRole::ActionCopy));
        assert!(all.contains(&IconRole::ActionPaste));
        assert!(all.contains(&IconRole::ActionCut));
        assert!(all.contains(&IconRole::ActionUndo));
        assert!(all.contains(&IconRole::ActionRedo));
        assert!(all.contains(&IconRole::ActionSearch));
        assert!(all.contains(&IconRole::ActionSettings));
        assert!(all.contains(&IconRole::ActionEdit));
        assert!(all.contains(&IconRole::ActionAdd));
        assert!(all.contains(&IconRole::ActionRemove));
        assert!(all.contains(&IconRole::ActionRefresh));
        assert!(all.contains(&IconRole::ActionPrint));

        // Navigation (6)
        assert!(all.contains(&IconRole::NavBack));
        assert!(all.contains(&IconRole::NavForward));
        assert!(all.contains(&IconRole::NavUp));
        assert!(all.contains(&IconRole::NavDown));
        assert!(all.contains(&IconRole::NavHome));
        assert!(all.contains(&IconRole::NavMenu));

        // Files (5)
        assert!(all.contains(&IconRole::FileGeneric));
        assert!(all.contains(&IconRole::FolderClosed));
        assert!(all.contains(&IconRole::FolderOpen));
        assert!(all.contains(&IconRole::TrashEmpty));
        assert!(all.contains(&IconRole::TrashFull));

        // Status (3)
        assert!(all.contains(&IconRole::StatusLoading));
        assert!(all.contains(&IconRole::StatusCheck));
        assert!(all.contains(&IconRole::StatusError));

        // System (4)
        assert!(all.contains(&IconRole::UserAccount));
        assert!(all.contains(&IconRole::Notification));
        assert!(all.contains(&IconRole::Help));
        assert!(all.contains(&IconRole::Lock));
    }

    #[test]
    fn icon_role_all_no_duplicates() {
        let all = &IconRole::ALL;
        for (i, role) in all.iter().enumerate() {
            for (j, other) in all.iter().enumerate() {
                if i != j {
                    assert_ne!(role, other, "Duplicate at index {i} and {j}");
                }
            }
        }
    }

    #[test]
    fn icon_role_derives_copy_clone() {
        let role = IconRole::ActionCopy;
        let cloned = role.clone();
        let copied = role;
        assert_eq!(role, cloned);
        assert_eq!(role, copied);
    }

    #[test]
    fn icon_role_derives_debug() {
        let s = format!("{:?}", IconRole::DialogWarning);
        assert!(s.contains("DialogWarning"));
    }

    #[test]
    fn icon_role_derives_hash() {
        use std::collections::HashSet;
        let mut set = HashSet::new();
        set.insert(IconRole::ActionSave);
        set.insert(IconRole::ActionDelete);
        assert_eq!(set.len(), 2);
        assert!(set.contains(&IconRole::ActionSave));
    }

    // === IconData tests ===

    #[test]
    fn icon_data_svg_construct_and_match() {
        let svg_bytes = b"<svg></svg>".to_vec();
        let data = IconData::Svg(svg_bytes.clone());
        match data {
            IconData::Svg(bytes) => assert_eq!(bytes, svg_bytes),
            _ => panic!("Expected Svg variant"),
        }
    }

    #[test]
    fn icon_data_rgba_construct_and_match() {
        let pixels = vec![255, 0, 0, 255]; // 1 red pixel
        let data = IconData::Rgba {
            width: 1,
            height: 1,
            data: pixels.clone(),
        };
        match data {
            IconData::Rgba {
                width,
                height,
                data,
            } => {
                assert_eq!(width, 1);
                assert_eq!(height, 1);
                assert_eq!(data, pixels);
            }
            _ => panic!("Expected Rgba variant"),
        }
    }

    #[test]
    fn icon_data_derives_debug() {
        let data = IconData::Svg(vec![]);
        let s = format!("{:?}", data);
        assert!(s.contains("Svg"));
    }

    #[test]
    fn icon_data_derives_clone() {
        let data = IconData::Rgba {
            width: 16,
            height: 16,
            data: vec![0; 16 * 16 * 4],
        };
        let cloned = data.clone();
        assert_eq!(data, cloned);
    }

    #[test]
    fn icon_data_derives_eq() {
        let a = IconData::Svg(b"<svg/>".to_vec());
        let b = IconData::Svg(b"<svg/>".to_vec());
        assert_eq!(a, b);

        let c = IconData::Svg(b"<other/>".to_vec());
        assert_ne!(a, c);
    }

    // === IconSet tests ===

    #[test]
    fn icon_set_from_name_sf_symbols() {
        assert_eq!(IconSet::from_name("sf-symbols"), Some(IconSet::SfSymbols));
    }

    #[test]
    fn icon_set_from_name_segoe_fluent() {
        assert_eq!(
            IconSet::from_name("segoe-fluent"),
            Some(IconSet::SegoeIcons)
        );
    }

    #[test]
    fn icon_set_from_name_freedesktop() {
        assert_eq!(
            IconSet::from_name("freedesktop"),
            Some(IconSet::Freedesktop)
        );
    }

    #[test]
    fn icon_set_from_name_material() {
        assert_eq!(IconSet::from_name("material"), Some(IconSet::Material));
    }

    #[test]
    fn icon_set_from_name_lucide() {
        assert_eq!(IconSet::from_name("lucide"), Some(IconSet::Lucide));
    }

    #[test]
    fn icon_set_from_name_unknown() {
        assert_eq!(IconSet::from_name("unknown"), None);
    }

    #[test]
    fn icon_set_name_sf_symbols() {
        assert_eq!(IconSet::SfSymbols.name(), "sf-symbols");
    }

    #[test]
    fn icon_set_name_segoe_fluent() {
        assert_eq!(IconSet::SegoeIcons.name(), "segoe-fluent");
    }

    #[test]
    fn icon_set_name_freedesktop() {
        assert_eq!(IconSet::Freedesktop.name(), "freedesktop");
    }

    #[test]
    fn icon_set_name_material() {
        assert_eq!(IconSet::Material.name(), "material");
    }

    #[test]
    fn icon_set_name_lucide() {
        assert_eq!(IconSet::Lucide.name(), "lucide");
    }

    #[test]
    fn icon_set_from_name_name_round_trip() {
        let sets = [
            IconSet::SfSymbols,
            IconSet::SegoeIcons,
            IconSet::Freedesktop,
            IconSet::Material,
            IconSet::Lucide,
        ];
        for set in &sets {
            let name = set.name();
            let parsed = IconSet::from_name(name);
            assert_eq!(parsed, Some(*set), "Round-trip failed for {:?}", set);
        }
    }

    #[test]
    fn icon_set_derives_copy_clone() {
        let set = IconSet::Material;
        let cloned = set.clone();
        let copied = set;
        assert_eq!(set, cloned);
        assert_eq!(set, copied);
    }

    #[test]
    fn icon_set_derives_hash() {
        use std::collections::HashSet;
        let mut map = HashSet::new();
        map.insert(IconSet::SfSymbols);
        map.insert(IconSet::Lucide);
        assert_eq!(map.len(), 2);
    }

    #[test]
    fn icon_set_derives_debug() {
        let s = format!("{:?}", IconSet::Freedesktop);
        assert!(s.contains("Freedesktop"));
    }

    #[test]
    fn icon_set_serde_round_trip() {
        let set = IconSet::SfSymbols;
        let json = serde_json::to_string(&set).unwrap();
        let deserialized: IconSet = serde_json::from_str(&json).unwrap();
        assert_eq!(set, deserialized);
    }
}
