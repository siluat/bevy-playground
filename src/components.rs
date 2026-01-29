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

// Card flip scene components
#[derive(Component, Clone)]
pub struct CardFlipSceneEntity;

#[derive(Component)]
pub struct Card {
    pub is_front: bool,
    pub is_flipping: bool,
    pub flip_progress: f32,   // 0.0 to 1.0
    pub image_swapped: bool,  // Track if image was swapped during this flip
}
