// Bevy code commonly triggers these lints and they may be important signals
// about code quality. They are sometimes hard to avoid though, and the CI
// workflow treats them as errors, so this allows them throughout the project.
// Feel free to delete this line.
#![allow(clippy::too_many_arguments, clippy::type_complexity)]

mod bike;

use avian2d::PhysicsPlugins;
use avian2d::prelude::{Gravity, PhysicsDebugPlugin};
use bevy::asset::AssetMetaCheck;
use bevy::prelude::*;
use crate::bike::{spawn_player};

fn main() {
    App::new()
        .add_plugins((DefaultPlugins.set(AssetPlugin {
            // Wasm builds will check for meta files (that don't exist) if this isn't set.
            // This causes errors and even panics in web builds on itch.
            // See https://github.com/bevyengine/bevy_github_ci_template/issues/48.
            meta_check: AssetMetaCheck::Never,
            ..default()
        }), PhysicsPlugins::default().with_length_unit(20.0), PhysicsDebugPlugin::default()))
        .insert_resource(Gravity(Vec2::new(0.0, 0.0)))
        .add_systems(Startup, (setup, spawn_player))
        .add_systems(Update, (bike::control_player, bike::drift_factor_system, bike::car_controller_system))
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let mut camera = Camera2dBundle::default();

    camera.transform.scale = Vec3::splat(1.0 / 20.0);

    commands.spawn(camera);
    // commands.spawn(SpriteBundle {
    //     texture: asset_server.load("ducky.png"),
    //     ..Default::default()
    // });
}
