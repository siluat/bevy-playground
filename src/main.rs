use bevy::prelude::*;

// Configuration constants
const ANIMATE_ONLY_WHEN_MOVING: bool = true; // true: animate only when moving, false: always animate
const FLIP_SPRITE_ON_DIRECTION: bool = true; // true: flip sprite based on direction, false: keep original

// Sprite settings
const SPRITE_COLUMNS: usize = 4;
const SPRITE_ROWS: usize = 1;
const FRAME_WIDTH: f32 = 108.0;
const FRAME_HEIGHT: f32 = 96.0;

// Gameplay settings
const ANIMATION_SPEED: f32 = 0.1; // Frame interval (seconds)
const MOVE_SPEED: f32 = 150.0; // Movement speed (pixels/second)

// Window settings
const WINDOW_WIDTH: f32 = 800.0;
const WINDOW_HEIGHT: f32 = 600.0;
const SPRITE_SCALE: f32 = 1.0;

#[derive(Component)]
struct AnimationIndices {
    first: usize,
    last: usize,
}

#[derive(Component, Deref, DerefMut)]
struct AnimationTimer(Timer);

#[derive(Component)]
struct Player {
    is_moving: bool,
    last_direction: Vec2,
}

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_nearest()) // Pixel art clarity
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Squirrel Walking Animation".into(),
                        resolution: (WINDOW_WIDTH as u32, WINDOW_HEIGHT as u32).into(),
                        ..default()
                    }),
                    ..default()
                }),
        )
        .add_systems(Startup, setup)
        .add_systems(Update, (move_player, animate_sprite).chain())
        .run();
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    // Camera setup
    commands.spawn(Camera2d);

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
            last_direction: Vec2::new(1.0, -1.0), // Initial direction (bottom-right)
        },
    ));
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
