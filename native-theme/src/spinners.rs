// Feature-gated bundled spinner construction.
//
// Uses genuine icon files from their respective open-source icon sets,
// the same SVGs used for static icons. No fabricated approximations.

use crate::model::animated::{AnimatedIcon, TransformAnimation};
use crate::model::icons::IconData;

/// Material Design progress_activity icon with continuous spin transform.
#[cfg(feature = "material-icons")]
pub(crate) fn material_spinner() -> AnimatedIcon {
    AnimatedIcon::Transform {
        icon: IconData::Svg(include_bytes!("../icons/material/progress_activity.svg").to_vec()),
        animation: TransformAnimation::Spin { duration_ms: 1000 },
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

/// Phosphor spinner icon with continuous spin transform.
#[cfg(feature = "phosphor-icons")]
pub(crate) fn phosphor_spinner() -> AnimatedIcon {
    AnimatedIcon::Transform {
        icon: IconData::Svg(include_bytes!("../icons/phosphor/spinner.svg").to_vec()),
        animation: TransformAnimation::Spin { duration_ms: 1000 },
    }
}
