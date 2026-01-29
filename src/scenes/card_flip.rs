use bevy::prelude::*;

use crate::{
    common::{cleanup, spawn_back_hint},
    components::{Card, CardFlipSceneEntity},
    state::AppState,
};

const CARD_SCALE: f32 = 1.0;
const FLIP_DURATION: f32 = 0.15;

#[derive(Resource)]
struct CardImages {
    front: Handle<Image>,
    back: Handle<Image>,
}

pub struct CardFlipPlugin;

impl Plugin for CardFlipPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::CardFlipScene), setup_card_flip_scene)
            .add_systems(OnExit(AppState::CardFlipScene), (cleanup::<CardFlipSceneEntity>, cleanup_card_images))
            .add_systems(
                Update,
                (card_click_system, card_flip_animation_system)
                    .chain()
                    .run_if(in_state(AppState::CardFlipScene)),
            );
    }
}

fn setup_card_flip_scene(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Preload both card images
    let card_images = CardImages {
        front: asset_server.load("card_front_chocolate.png"),
        back: asset_server.load("card_back.png"),
    };

    // Spawn card with back face initially
    commands.spawn((
        Sprite {
            image: card_images.back.clone(),
            ..default()
        },
        Transform::from_translation(Vec3::ZERO).with_scale(Vec3::splat(CARD_SCALE)),
        Card {
            is_front: false,
            is_flipping: false,
            flip_progress: 0.0,
            image_swapped: false,
        },
        CardFlipSceneEntity,
    ));

    commands.insert_resource(card_images);

    // Scene description text
    commands.spawn((
        Text::new("Click the card to flip"),
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
        CardFlipSceneEntity,
    ));

    // Spawn back hint
    spawn_back_hint(&mut commands, CardFlipSceneEntity);
}

fn cleanup_card_images(mut commands: Commands) {
    commands.remove_resource::<CardImages>();
}

fn card_click_system(
    mouse_button: Res<ButtonInput<MouseButton>>,
    windows: Query<&Window>,
    camera_query: Query<(&Camera, &GlobalTransform)>,
    mut card_query: Query<(&Sprite, &mut Card, &Transform)>,
) {
    if !mouse_button.just_pressed(MouseButton::Left) {
        return;
    }

    let Ok(window) = windows.single() else {
        return;
    };

    let Some(cursor_position) = window.cursor_position() else {
        return;
    };

    let Ok((camera, camera_transform)) = camera_query.single() else {
        return;
    };

    let Ok(world_position) = camera.viewport_to_world_2d(camera_transform, cursor_position) else {
        return;
    };

    for (sprite, mut card, transform) in &mut card_query {
        // Skip if already flipping
        if card.is_flipping {
            continue;
        }

        let base_size = sprite.custom_size.unwrap_or(Vec2::new(64.0, 64.0));
        let card_size = base_size * transform.scale.truncate();
        let half_size = card_size / 2.0;
        let card_pos = transform.translation.truncate();

        // Check if click is within card bounds
        if world_position.x >= card_pos.x - half_size.x
            && world_position.x <= card_pos.x + half_size.x
            && world_position.y >= card_pos.y - half_size.y
            && world_position.y <= card_pos.y + half_size.y
        {
            // Start flip animation
            card.is_flipping = true;
            card.flip_progress = 0.0;
            card.image_swapped = false;
        }
    }
}

fn card_flip_animation_system(
    time: Res<Time>,
    card_images: Res<CardImages>,
    mut card_query: Query<(&mut Sprite, &mut Card, &mut Transform)>,
) {
    for (mut sprite, mut card, mut transform) in &mut card_query {
        if !card.is_flipping {
            continue;
        }

        card.flip_progress += time.delta_secs() / FLIP_DURATION;

        if card.flip_progress >= 1.0 {
            // Animation complete
            card.is_flipping = false;
            card.flip_progress = 0.0;
            card.image_swapped = false;
            transform.scale.x = CARD_SCALE;
        } else if card.flip_progress >= 0.5 && !card.image_swapped {
            // At halfway point, swap the image (only once)
            card.is_front = !card.is_front;
            card.image_swapped = true;
            sprite.image = if card.is_front {
                card_images.front.clone()
            } else {
                card_images.back.clone()
            };
        }

        // Scale X based on progress (1 -> 0 -> 1)
        let scale_x = if card.flip_progress < 0.5 {
            // First half: 1.0 -> 0.0
            1.0 - (card.flip_progress * 2.0)
        } else {
            // Second half: 0.0 -> 1.0
            (card.flip_progress - 0.5) * 2.0
        };
        transform.scale.x = scale_x * CARD_SCALE;
    }
}
