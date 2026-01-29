use bevy::prelude::*;

use crate::{
    common::{cleanup, spawn_back_hint},
    components::{AnimationIndices, AnimationTimer, Player, SquirrelSceneEntity},
    constants::{
        ANIMATE_ONLY_WHEN_MOVING, ANIMATION_SPEED, FLIP_SPRITE_ON_DIRECTION, FRAME_HEIGHT,
        FRAME_WIDTH, MOVE_SPEED, SPRITE_COLUMNS, SPRITE_ROWS, SPRITE_SCALE, WINDOW_HEIGHT,
        WINDOW_WIDTH,
    },
    state::AppState,
};

pub struct SquirrelPlugin;

impl Plugin for SquirrelPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::SquirrelWalking), setup_squirrel_scene)
            .add_systems(
                OnExit(AppState::SquirrelWalking),
                cleanup::<SquirrelSceneEntity>,
            )
            .add_systems(
                Update,
                (move_player, animate_sprite)
                    .chain()
                    .run_if(in_state(AppState::SquirrelWalking)),
            );
    }
}

fn setup_squirrel_scene(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    // Create texture atlas layout
    let texture = asset_server.load("DRG_Walking.png");
    let layout = TextureAtlasLayout::from_grid(
        UVec2::new(FRAME_WIDTH as u32, FRAME_HEIGHT as u32),
        SPRITE_COLUMNS as u32,
        SPRITE_ROWS as u32,
        None,
        None,
    );
    let texture_atlas_layout = texture_atlas_layouts.add(layout);

    // Animation indices
    let animation_indices = AnimationIndices {
        first: 0,
        last: SPRITE_COLUMNS - 1,
    };

    // Spawn squirrel sprite
    commands.spawn((
        Sprite {
            image: texture,
            texture_atlas: Some(TextureAtlas {
                layout: texture_atlas_layout,
                index: animation_indices.first,
            }),
            ..default()
        },
        Transform::from_scale(Vec3::splat(SPRITE_SCALE)),
        animation_indices,
        AnimationTimer(Timer::from_seconds(ANIMATION_SPEED, TimerMode::Repeating)),
        Player {
            is_moving: false,
            last_direction: Vec2::new(1.0, -1.0),
        },
        SquirrelSceneEntity,
    ));

    // Spawn back hint
    spawn_back_hint(&mut commands, SquirrelSceneEntity);
}

fn move_player(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut query: Query<(&mut Transform, &mut Sprite, &mut Player)>,
) {
    for (mut transform, mut sprite, mut player) in &mut query {
        let mut direction = Vec2::ZERO;

        // Handle WASD and arrow key input
        if keyboard_input.pressed(KeyCode::KeyW) || keyboard_input.pressed(KeyCode::ArrowUp) {
            direction.y += 1.0;
        }
        if keyboard_input.pressed(KeyCode::KeyS) || keyboard_input.pressed(KeyCode::ArrowDown) {
            direction.y -= 1.0;
        }
        if keyboard_input.pressed(KeyCode::KeyA) || keyboard_input.pressed(KeyCode::ArrowLeft) {
            direction.x -= 1.0;
        }
        if keyboard_input.pressed(KeyCode::KeyD) || keyboard_input.pressed(KeyCode::ArrowRight) {
            direction.x += 1.0;
        }

        // Update movement state
        player.is_moving = direction != Vec2::ZERO;

        if player.is_moving {
            // Normalize direction and move
            let normalized = direction.normalize();
            transform.translation.x += normalized.x * MOVE_SPEED * time.delta_secs();
            transform.translation.y += normalized.y * MOVE_SPEED * time.delta_secs();

            // Clamp to screen bounds
            let half_sprite_width = FRAME_WIDTH * SPRITE_SCALE / 2.0;
            let half_sprite_height = FRAME_HEIGHT * SPRITE_SCALE / 2.0;
            let x_bound = WINDOW_WIDTH / 2.0 - half_sprite_width;
            let y_bound = WINDOW_HEIGHT / 2.0 - half_sprite_height;

            transform.translation.x = transform.translation.x.clamp(-x_bound, x_bound);
            transform.translation.y = transform.translation.y.clamp(-y_bound, y_bound);

            // Store last movement direction
            player.last_direction = normalized;

            // Handle sprite horizontal flip
            if FLIP_SPRITE_ON_DIRECTION {
                if direction.x < 0.0 {
                    sprite.flip_x = false;
                } else if direction.x > 0.0 {
                    sprite.flip_x = true;
                }
            }
        }
    }
}

fn animate_sprite(
    time: Res<Time>,
    mut query: Query<(&AnimationIndices, &mut AnimationTimer, &mut Sprite, &Player)>,
) {
    for (indices, mut timer, mut sprite, player) in &mut query {
        // Skip animation if ANIMATE_ONLY_WHEN_MOVING is true and not moving
        if ANIMATE_ONLY_WHEN_MOVING && !player.is_moving {
            continue;
        }

        timer.tick(time.delta());
        if timer.just_finished() {
            if let Some(atlas) = &mut sprite.texture_atlas {
                atlas.index = if atlas.index == indices.last {
                    indices.first
                } else {
                    atlas.index + 1
                };
            }
        }
    }
}
