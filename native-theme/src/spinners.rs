// Feature-gated bundled spinner construction.
//
// Only genuinely sourced assets: real icon files from open-source icon sets,
// or runtime-loaded platform sprite sheets. No fabricated approximations.

use crate::model::animated::{AnimatedIcon, TransformAnimation};
use crate::model::icons::IconData;

/// Lucide loader icon with continuous spin transform.
#[cfg(feature = "lucide-icons")]
pub(crate) fn lucide_spinner() -> AnimatedIcon {
    AnimatedIcon::Transform {
        icon: IconData::Svg(include_bytes!("../icons/lucide/loader.svg").to_vec()),
        animation: TransformAnimation::Spin { duration_ms: 1000 },
    }
}
