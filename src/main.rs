#![allow(unused)]

use bevy::prelude::*;

// region: --- Asset Constants

const PLAYER_SPRITE: &str = "player_a_01.png";
const PLAYER_SIZE: (f32, f32) = (144., 75.);

// endregion: --- Asset Constants

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.04, 0.04, 0.04)))
        .insert_resource(WindowDescriptor {
            title: "Rust Invaders!".to_string(),
            width: 598.0,
            height: 676.0,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup_system)
        .run()
}

fn setup_system(mut commands: Commands, asset_server: Res<AssetServer>) {
    // camera
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    // add rectangle
    commands.spawn_bundle(SpriteBundle {
        texture: asset_server.load(PLAYER_SPRITE),
        ..Default::default()
    });
}
