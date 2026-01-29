use bevy::prelude::*;

/// App state for scene navigation
#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum AppState {
    #[default]
    SceneList,
    SquirrelWalking,
    TestScene,
}
