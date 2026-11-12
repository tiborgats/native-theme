//! The transition engine's resolution, written by hand (S6.1): a Breeze
//! test holds no `Value` and resolves to itself. A `WinUiAnimatedIcon`
//! engine resolves the visual its `visual` names from WinUI's animated
//! visuals, a token staged with the motion unit (S6.1, ADD-A147): until it
//! enters, the engine has no value. No part lists `transitions` until that
//! unit (D6 outcome 3), and S12 refuses a layer stating one on a part that
//! does not list it, so a theme that validates holds no engine.

use super::{Cx, ResolveWhole};
use crate::blocks::motion::{BreezeTest, ResolvedEngine, TransitionEngine};
use crate::resolve::Pos;

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
            TransitionEngine::WinUiAnimatedIcon { .. } => return None,
            TransitionEngine::GtkSwitch => ResolvedEngine::GtkSwitch,
            TransitionEngine::CssAnimation => ResolvedEngine::CssAnimation,
        })
    }
}

impl ResolveWhole for BreezeTest {
    fn resolve_whole<'t: 'f, 'f>(&'f self, _: Pos, _: &mut Cx<'t>) -> Option<BreezeTest> {
        Some(self.clone())
    }
}
