use bevy::prelude::*;

use crate::state::AppState;

// Marker components for scene cleanup
#[derive(Component, Clone)]
pub struct SceneListEntity;

#[derive(Component, Clone)]
pub struct SquirrelSceneEntity;

#[derive(Component, Clone)]
pub struct TestSceneEntity;

// Animation components
#[derive(Component)]
pub struct AnimationIndices {
    pub first: usize,
    pub last: usize,
}

#[derive(Component, Deref, DerefMut)]
pub struct AnimationTimer(pub Timer);

// Player component
#[derive(Component)]
pub struct Player {
    pub is_moving: bool,
    pub last_direction: Vec2,
}

// UI components
#[derive(Component)]
pub struct SceneButton {
    pub target_state: AppState,
}

#[derive(Component)]
pub struct ColorPulse {
    pub timer: f32,
}
