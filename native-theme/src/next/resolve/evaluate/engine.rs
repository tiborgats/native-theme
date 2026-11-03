//! The transition engine's resolution, written by hand (S6.1): a
//! `WinUiAnimatedIcon` engine resolves the visual its `visual` names from
//! `token.icons.animated_visuals`, so the engine reads no token (S2.7); a
//! Breeze test holds no `Value` and resolves to itself.

use super::{Cx, Resolve, ResolveWhole};
use crate::next::blocks::motion::{
    AnimatedVisual, BreezeTest, ResolvedAnimatedVisual, ResolvedEngine, TransitionEngine,
};
use crate::next::resolve::Pos;
use crate::next::resolve::compose::{self, Fold, Target};
use crate::next::units::{Fraction, Ms};
use crate::next::value::Value;

impl ResolveWhole for TransitionEngine {
    fn resolve_whole<'t: 'f, 'f>(&'f self, pos: Pos, cx: &mut Cx<'t>) -> Option<ResolvedEngine> {
        Some(match self {
            TransitionEngine::GtkCss => ResolvedEngine::GtkCss,
            TransitionEngine::AdwAnimation { replays } => {
                ResolvedEngine::AdwAnimation { replays: *replays }
            }
            TransitionEngine::QtQuick => ResolvedEngine::QtQuick,
            TransitionEngine::Breeze(test) => ResolvedEngine::Breeze(test.resolve_whole(pos, cx)?),
            TransitionEngine::BreezeTabBar(test) => {
                ResolvedEngine::BreezeTabBar(test.resolve_whole(pos, cx)?)
            }
            TransitionEngine::ComposeAnimatable => ResolvedEngine::ComposeAnimatable,
            TransitionEngine::ComposeTransition => ResolvedEngine::ComposeTransition,
            TransitionEngine::ComposeHeld => ResolvedEngine::ComposeHeld,
            TransitionEngine::WinUi => ResolvedEngine::WinUi,
            TransitionEngine::WinUiAnimatedIcon { visual } => {
                let path = vec![
                    "icons".to_owned(),
                    "animated_visuals".to_owned(),
                    visual.as_str().to_owned(),
                ];
                let visual = cx.follow(Target::Tokens, path, false, |cx, folds| {
                    AnimatedVisual::resolve(folds, cx)
                })?;
                ResolvedEngine::WinUiAnimatedIcon(visual)
            }
            TransitionEngine::GtkSwitch => ResolvedEngine::GtkSwitch,
        })
    }
}

impl ResolveWhole for BreezeTest {
    fn resolve_whole<'t: 'f, 'f>(&'f self, _: Pos, _: &mut Cx<'t>) -> Option<BreezeTest> {
        Some(self.clone())
    }
}

/// An animated visual, each field from its steps; `None` where its duration
/// or a marker has no value, which validation rules out (S2.2).
impl Resolve for AnimatedVisual {
    fn resolve<'t: 'f, 'f>(folds: &[Fold<'f>], cx: &mut Cx<'t>) -> Option<ResolvedAnimatedVisual> {
        let duration = cx.at("duration", |cx| {
            cx.value::<Ms>(&compose::children(folds, "duration"), false)
        })?;
        let markers = cx.at("markers", |cx| {
            cx.map::<String, Value<Fraction>, Value<Fraction>, _>(
                &compose::children(folds, "markers"),
                compose::leaf,
                compose::leaf,
                |cx, e| Some(cx.value::<Fraction>(e, false)),
            )
        })?;
        let markers = markers
            .into_iter()
            .map(|(name, at)| at.map(|at| (name, at.0)))
            .collect::<Option<Vec<_>>>()?;
        Some(ResolvedAnimatedVisual { duration, markers })
    }
}

impl ResolveWhole for AnimatedVisual {
    fn resolve_whole<'t: 'f, 'f>(
        &'f self,
        pos: Pos,
        cx: &mut Cx<'t>,
    ) -> Option<ResolvedAnimatedVisual> {
        let fold = Fold {
            pos,
            target: cx.target(),
            found: compose::Found::Node(self),
        };
        <Self as Resolve>::resolve(&[fold], cx)
    }
}
