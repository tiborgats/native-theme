// Feature-gated bundled spinner animation construction via include_bytes!
//
// Each function embeds pre-generated SVG frames at compile time and
// returns an AnimatedIcon ready for loading_indicator() dispatch.

use crate::model::animated::{AnimatedIcon, Repeat, TransformAnimation};
use crate::model::icons::IconData;

/// Material Design circular arc spinner (12 frames, 83ms per frame).
#[cfg(feature = "material-icons")]
pub(crate) fn material_spinner() -> AnimatedIcon {
    let frames = vec![
        IconData::Svg(include_bytes!("../animations/material/frame_00.svg").to_vec()),
        IconData::Svg(include_bytes!("../animations/material/frame_01.svg").to_vec()),
        IconData::Svg(include_bytes!("../animations/material/frame_02.svg").to_vec()),
        IconData::Svg(include_bytes!("../animations/material/frame_03.svg").to_vec()),
        IconData::Svg(include_bytes!("../animations/material/frame_04.svg").to_vec()),
        IconData::Svg(include_bytes!("../animations/material/frame_05.svg").to_vec()),
        IconData::Svg(include_bytes!("../animations/material/frame_06.svg").to_vec()),
        IconData::Svg(include_bytes!("../animations/material/frame_07.svg").to_vec()),
        IconData::Svg(include_bytes!("../animations/material/frame_08.svg").to_vec()),
        IconData::Svg(include_bytes!("../animations/material/frame_09.svg").to_vec()),
        IconData::Svg(include_bytes!("../animations/material/frame_10.svg").to_vec()),
        IconData::Svg(include_bytes!("../animations/material/frame_11.svg").to_vec()),
    ];
    AnimatedIcon::Frames {
        frames,
        frame_duration_ms: 83,
        repeat: Repeat::Infinite,
    }
}

/// Lucide loader icon with continuous spin transform.
#[cfg(feature = "lucide-icons")]
pub(crate) fn lucide_spinner() -> AnimatedIcon {
    AnimatedIcon::Transform {
        icon: IconData::Svg(include_bytes!("../icons/lucide/loader.svg").to_vec()),
        animation: TransformAnimation::Spin { duration_ms: 1000 },
    }
}

/// macOS-style radial spoke spinner (12 frames, 83ms per frame).
#[cfg(feature = "system-icons")]
pub(crate) fn macos_spinner() -> AnimatedIcon {
    let frames = vec![
        IconData::Svg(include_bytes!("../animations/macos/frame_00.svg").to_vec()),
        IconData::Svg(include_bytes!("../animations/macos/frame_01.svg").to_vec()),
        IconData::Svg(include_bytes!("../animations/macos/frame_02.svg").to_vec()),
        IconData::Svg(include_bytes!("../animations/macos/frame_03.svg").to_vec()),
        IconData::Svg(include_bytes!("../animations/macos/frame_04.svg").to_vec()),
        IconData::Svg(include_bytes!("../animations/macos/frame_05.svg").to_vec()),
        IconData::Svg(include_bytes!("../animations/macos/frame_06.svg").to_vec()),
        IconData::Svg(include_bytes!("../animations/macos/frame_07.svg").to_vec()),
        IconData::Svg(include_bytes!("../animations/macos/frame_08.svg").to_vec()),
        IconData::Svg(include_bytes!("../animations/macos/frame_09.svg").to_vec()),
        IconData::Svg(include_bytes!("../animations/macos/frame_10.svg").to_vec()),
        IconData::Svg(include_bytes!("../animations/macos/frame_11.svg").to_vec()),
    ];
    AnimatedIcon::Frames {
        frames,
        frame_duration_ms: 83,
        repeat: Repeat::Infinite,
    }
}

/// Windows-style arc expansion/contraction spinner (60 frames, 33ms per frame).
#[cfg(feature = "system-icons")]
pub(crate) fn windows_spinner() -> AnimatedIcon {
    let frames = vec![
        IconData::Svg(include_bytes!("../animations/windows/frame_00.svg").to_vec()),
        IconData::Svg(include_bytes!("../animations/windows/frame_01.svg").to_vec()),
        IconData::Svg(include_bytes!("../animations/windows/frame_02.svg").to_vec()),
        IconData::Svg(include_bytes!("../animations/windows/frame_03.svg").to_vec()),
        IconData::Svg(include_bytes!("../animations/windows/frame_04.svg").to_vec()),
        IconData::Svg(include_bytes!("../animations/windows/frame_05.svg").to_vec()),
        IconData::Svg(include_bytes!("../animations/windows/frame_06.svg").to_vec()),
        IconData::Svg(include_bytes!("../animations/windows/frame_07.svg").to_vec()),
        IconData::Svg(include_bytes!("../animations/windows/frame_08.svg").to_vec()),
        IconData::Svg(include_bytes!("../animations/windows/frame_09.svg").to_vec()),
        IconData::Svg(include_bytes!("../animations/windows/frame_10.svg").to_vec()),
        IconData::Svg(include_bytes!("../animations/windows/frame_11.svg").to_vec()),
        IconData::Svg(include_bytes!("../animations/windows/frame_12.svg").to_vec()),
        IconData::Svg(include_bytes!("../animations/windows/frame_13.svg").to_vec()),
        IconData::Svg(include_bytes!("../animations/windows/frame_14.svg").to_vec()),
        IconData::Svg(include_bytes!("../animations/windows/frame_15.svg").to_vec()),
        IconData::Svg(include_bytes!("../animations/windows/frame_16.svg").to_vec()),
        IconData::Svg(include_bytes!("../animations/windows/frame_17.svg").to_vec()),
        IconData::Svg(include_bytes!("../animations/windows/frame_18.svg").to_vec()),
        IconData::Svg(include_bytes!("../animations/windows/frame_19.svg").to_vec()),
        IconData::Svg(include_bytes!("../animations/windows/frame_20.svg").to_vec()),
        IconData::Svg(include_bytes!("../animations/windows/frame_21.svg").to_vec()),
        IconData::Svg(include_bytes!("../animations/windows/frame_22.svg").to_vec()),
        IconData::Svg(include_bytes!("../animations/windows/frame_23.svg").to_vec()),
        IconData::Svg(include_bytes!("../animations/windows/frame_24.svg").to_vec()),
        IconData::Svg(include_bytes!("../animations/windows/frame_25.svg").to_vec()),
        IconData::Svg(include_bytes!("../animations/windows/frame_26.svg").to_vec()),
        IconData::Svg(include_bytes!("../animations/windows/frame_27.svg").to_vec()),
        IconData::Svg(include_bytes!("../animations/windows/frame_28.svg").to_vec()),
        IconData::Svg(include_bytes!("../animations/windows/frame_29.svg").to_vec()),
        IconData::Svg(include_bytes!("../animations/windows/frame_30.svg").to_vec()),
        IconData::Svg(include_bytes!("../animations/windows/frame_31.svg").to_vec()),
        IconData::Svg(include_bytes!("../animations/windows/frame_32.svg").to_vec()),
        IconData::Svg(include_bytes!("../animations/windows/frame_33.svg").to_vec()),
        IconData::Svg(include_bytes!("../animations/windows/frame_34.svg").to_vec()),
        IconData::Svg(include_bytes!("../animations/windows/frame_35.svg").to_vec()),
        IconData::Svg(include_bytes!("../animations/windows/frame_36.svg").to_vec()),
        IconData::Svg(include_bytes!("../animations/windows/frame_37.svg").to_vec()),
        IconData::Svg(include_bytes!("../animations/windows/frame_38.svg").to_vec()),
        IconData::Svg(include_bytes!("../animations/windows/frame_39.svg").to_vec()),
        IconData::Svg(include_bytes!("../animations/windows/frame_40.svg").to_vec()),
        IconData::Svg(include_bytes!("../animations/windows/frame_41.svg").to_vec()),
        IconData::Svg(include_bytes!("../animations/windows/frame_42.svg").to_vec()),
        IconData::Svg(include_bytes!("../animations/windows/frame_43.svg").to_vec()),
        IconData::Svg(include_bytes!("../animations/windows/frame_44.svg").to_vec()),
        IconData::Svg(include_bytes!("../animations/windows/frame_45.svg").to_vec()),
        IconData::Svg(include_bytes!("../animations/windows/frame_46.svg").to_vec()),
        IconData::Svg(include_bytes!("../animations/windows/frame_47.svg").to_vec()),
        IconData::Svg(include_bytes!("../animations/windows/frame_48.svg").to_vec()),
        IconData::Svg(include_bytes!("../animations/windows/frame_49.svg").to_vec()),
        IconData::Svg(include_bytes!("../animations/windows/frame_50.svg").to_vec()),
        IconData::Svg(include_bytes!("../animations/windows/frame_51.svg").to_vec()),
        IconData::Svg(include_bytes!("../animations/windows/frame_52.svg").to_vec()),
        IconData::Svg(include_bytes!("../animations/windows/frame_53.svg").to_vec()),
        IconData::Svg(include_bytes!("../animations/windows/frame_54.svg").to_vec()),
        IconData::Svg(include_bytes!("../animations/windows/frame_55.svg").to_vec()),
        IconData::Svg(include_bytes!("../animations/windows/frame_56.svg").to_vec()),
        IconData::Svg(include_bytes!("../animations/windows/frame_57.svg").to_vec()),
        IconData::Svg(include_bytes!("../animations/windows/frame_58.svg").to_vec()),
        IconData::Svg(include_bytes!("../animations/windows/frame_59.svg").to_vec()),
    ];
    AnimatedIcon::Frames {
        frames,
        frame_duration_ms: 33,
        repeat: Repeat::Infinite,
    }
}

/// GNOME Adwaita-style overlapping arc spinner (20 frames, 60ms per frame).
#[cfg(feature = "system-icons")]
pub(crate) fn adwaita_spinner() -> AnimatedIcon {
    let frames = vec![
        IconData::Svg(include_bytes!("../animations/adwaita/frame_00.svg").to_vec()),
        IconData::Svg(include_bytes!("../animations/adwaita/frame_01.svg").to_vec()),
        IconData::Svg(include_bytes!("../animations/adwaita/frame_02.svg").to_vec()),
        IconData::Svg(include_bytes!("../animations/adwaita/frame_03.svg").to_vec()),
        IconData::Svg(include_bytes!("../animations/adwaita/frame_04.svg").to_vec()),
        IconData::Svg(include_bytes!("../animations/adwaita/frame_05.svg").to_vec()),
        IconData::Svg(include_bytes!("../animations/adwaita/frame_06.svg").to_vec()),
        IconData::Svg(include_bytes!("../animations/adwaita/frame_07.svg").to_vec()),
        IconData::Svg(include_bytes!("../animations/adwaita/frame_08.svg").to_vec()),
        IconData::Svg(include_bytes!("../animations/adwaita/frame_09.svg").to_vec()),
        IconData::Svg(include_bytes!("../animations/adwaita/frame_10.svg").to_vec()),
        IconData::Svg(include_bytes!("../animations/adwaita/frame_11.svg").to_vec()),
        IconData::Svg(include_bytes!("../animations/adwaita/frame_12.svg").to_vec()),
        IconData::Svg(include_bytes!("../animations/adwaita/frame_13.svg").to_vec()),
        IconData::Svg(include_bytes!("../animations/adwaita/frame_14.svg").to_vec()),
        IconData::Svg(include_bytes!("../animations/adwaita/frame_15.svg").to_vec()),
        IconData::Svg(include_bytes!("../animations/adwaita/frame_16.svg").to_vec()),
        IconData::Svg(include_bytes!("../animations/adwaita/frame_17.svg").to_vec()),
        IconData::Svg(include_bytes!("../animations/adwaita/frame_18.svg").to_vec()),
        IconData::Svg(include_bytes!("../animations/adwaita/frame_19.svg").to_vec()),
    ];
    AnimatedIcon::Frames {
        frames,
        frame_duration_ms: 60,
        repeat: Repeat::Infinite,
    }
}
