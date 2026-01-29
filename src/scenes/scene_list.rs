use bevy::prelude::*;

use crate::{
    common::cleanup,
    components::{SceneButton, SceneListEntity},
    constants::{HOVERED_BUTTON, NORMAL_BUTTON, PRESSED_BUTTON},
    state::AppState,
};

pub struct SceneListPlugin;

impl Plugin for SceneListPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::SceneList), setup_scene_list)
            .add_systems(OnExit(AppState::SceneList), cleanup::<SceneListEntity>)
            .add_systems(
                Update,
                scene_button_interaction.run_if(in_state(AppState::SceneList)),
            );
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
