use bevy::prelude::*;

use crate::{
    common::{cleanup, spawn_back_hint},
    components::{ColorPulse, TestSceneEntity},
    state::AppState,
};

pub struct TestScenePlugin;

impl Plugin for TestScenePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::TestScene), setup_test_scene)
            .add_systems(OnExit(AppState::TestScene), cleanup::<TestSceneEntity>)
            .add_systems(
                Update,
                test_scene_update.run_if(in_state(AppState::TestScene)),
            );
    }
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
