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

// UI colors
const NORMAL_BUTTON: Color = Color::srgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::srgb(0.25, 0.25, 0.25);
const PRESSED_BUTTON: Color = Color::srgb(0.35, 0.75, 0.35);

// App state for scene navigation
#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
enum AppState {
    #[default]
    SceneList,
    SquirrelWalking,
    TestScene,
}

// Marker components for scene cleanup
#[derive(Component, Clone)]
struct SceneListEntity;

#[derive(Component, Clone)]
struct SquirrelSceneEntity;

#[derive(Component, Clone)]
struct TestSceneEntity;

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

#[derive(Component)]
struct SceneButton {
    target_state: AppState,
}

#[derive(Component)]
struct ColorPulse {
    timer: f32,
}

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_nearest()) // Pixel art clarity
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Bevy Playground".into(),
                        resolution: (WINDOW_WIDTH as u32, WINDOW_HEIGHT as u32).into(),
                        ..default()
                    }),
                    ..default()
                }),
        )
        .init_state::<AppState>()
        .add_systems(Startup, setup_camera)
        // Scene List
        .add_systems(OnEnter(AppState::SceneList), setup_scene_list)
        .add_systems(OnExit(AppState::SceneList), cleanup::<SceneListEntity>)
        .add_systems(
            Update,
            scene_button_interaction.run_if(in_state(AppState::SceneList)),
        )
        // Squirrel Walking Scene
        .add_systems(OnEnter(AppState::SquirrelWalking), setup_squirrel_scene)
        .add_systems(OnExit(AppState::SquirrelWalking), cleanup::<SquirrelSceneEntity>)
        .add_systems(
            Update,
            (move_player, animate_sprite)
                .chain()
                .run_if(in_state(AppState::SquirrelWalking)),
        )
        // Test Scene
        .add_systems(OnEnter(AppState::TestScene), setup_test_scene)
        .add_systems(OnExit(AppState::TestScene), cleanup::<TestSceneEntity>)
        .add_systems(
            Update,
            test_scene_update.run_if(in_state(AppState::TestScene)),
        )
        // Common
        .add_systems(
            Update,
            back_to_menu.run_if(not(in_state(AppState::SceneList))),
        )
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}

fn cleanup<T: Component>(mut commands: Commands, query: Query<Entity, With<T>>) {
    for entity in &query {
        commands.entity(entity).despawn();
    }
}

fn setup_scene_list(mut commands: Commands) {
    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                row_gap: Val::Px(20.0),
                ..default()
            },
            SceneListEntity,
        ))
        .with_children(|parent| {
            // Title
            parent.spawn((
                Text::new("Scene Selection"),
                TextFont {
                    font_size: 48.0,
                    ..default()
                },
                TextColor(Color::WHITE),
                Node {
                    margin: UiRect::bottom(Val::Px(40.0)),
                    ..default()
                },
            ));

            // Squirrel Walking button
            parent
                .spawn((
                    Button,
                    Node {
                        width: Val::Px(250.0),
                        height: Val::Px(65.0),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    BackgroundColor(NORMAL_BUTTON),
                    SceneButton {
                        target_state: AppState::SquirrelWalking,
                    },
                ))
                .with_children(|parent| {
                    parent.spawn((
                        Text::new("Squirrel Walking"),
                        TextFont {
                            font_size: 24.0,
                            ..default()
                        },
                        TextColor(Color::srgb(0.9, 0.9, 0.9)),
                    ));
                });

            // Test Scene button
            parent
                .spawn((
                    Button,
                    Node {
                        width: Val::Px(250.0),
                        height: Val::Px(65.0),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    BackgroundColor(NORMAL_BUTTON),
                    SceneButton {
                        target_state: AppState::TestScene,
                    },
                ))
                .with_children(|parent| {
                    parent.spawn((
                        Text::new("Test Scene"),
                        TextFont {
                            font_size: 24.0,
                            ..default()
                        },
                        TextColor(Color::srgb(0.9, 0.9, 0.9)),
                    ));
                });
        });
}

fn scene_button_interaction(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, &SceneButton),
        (Changed<Interaction>, With<Button>),
    >,
    mut next_state: ResMut<NextState<AppState>>,
) {
    for (interaction, mut color, button) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                *color = PRESSED_BUTTON.into();
                next_state.set(button.target_state);
            }
            Interaction::Hovered => {
                *color = HOVERED_BUTTON.into();
            }
            Interaction::None => {
                *color = NORMAL_BUTTON.into();
            }
        }
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

fn setup_test_scene(mut commands: Commands) {
    // Spawn color pulse circle
    commands.spawn((
        Sprite {
            color: Color::srgb(1.0, 0.5, 0.0),
            custom_size: Some(Vec2::new(100.0, 100.0)),
            ..default()
        },
        Transform::from_translation(Vec3::ZERO),
        ColorPulse { timer: 0.0 },
        TestSceneEntity,
    ));

    // Scene description text
    commands.spawn((
        Text::new("Test Scene: Color Pulse Circle"),
        TextFont {
            font_size: 24.0,
            ..default()
        },
        TextColor(Color::WHITE),
        Node {
            position_type: PositionType::Absolute,
            bottom: Val::Px(30.0),
            left: Val::Px(0.0),
            right: Val::Px(0.0),
            justify_content: JustifyContent::Center,
            ..default()
        },
        TestSceneEntity,
    ));

    // Spawn back hint
    spawn_back_hint(&mut commands, TestSceneEntity);
}

fn test_scene_update(time: Res<Time>, mut query: Query<(&mut Sprite, &mut ColorPulse)>) {
    for (mut sprite, mut pulse) in &mut query {
        pulse.timer += time.delta_secs();

        // Oscillate between orange and blue
        let t = (pulse.timer.sin() + 1.0) / 2.0;

        // Lerp between orange (1.0, 0.5, 0.0) and blue (0.0, 0.5, 1.0)
        let r = 1.0 * (1.0 - t) + 0.0 * t;
        let g = 0.5;
        let b = 0.0 * (1.0 - t) + 1.0 * t;

        sprite.color = Color::srgb(r, g, b);
    }
}

fn spawn_back_hint<T: Component + Clone>(commands: &mut Commands, marker: T) {
    commands.spawn((
        Text::new("Press ESC to return"),
        TextFont {
            font_size: 18.0,
            ..default()
        },
        TextColor(Color::srgba(1.0, 1.0, 1.0, 0.7)),
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(20.0),
            left: Val::Px(20.0),
            ..default()
        },
        marker,
    ));
}

fn back_to_menu(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        next_state.set(AppState::SceneList);
    }
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
