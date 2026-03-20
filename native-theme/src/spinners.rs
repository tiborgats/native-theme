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
