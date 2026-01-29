mod common;
mod components;
mod constants;
mod scenes;
mod state;

use bevy::prelude::*;

use common::CommonPlugin;
use scenes::{SceneListPlugin, SquirrelPlugin, TestScenePlugin};
use state::AppState;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Bevy Playground".into(),
                        resolution: (constants::WINDOW_WIDTH as u32, constants::WINDOW_HEIGHT as u32).into(),
                        ..default()
                    }),
                    ..default()
                }),
        )
        .init_state::<AppState>()
        .add_plugins((CommonPlugin, SceneListPlugin, SquirrelPlugin, TestScenePlugin))
        .run();
}
