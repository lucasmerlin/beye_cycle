// Bevy code commonly triggers these lints and they may be important signals
// about code quality. They are sometimes hard to avoid though, and the CI
// workflow treats them as errors, so this allows them throughout the project.
// Feel free to delete this line.
#![allow(clippy::too_many_arguments, clippy::type_complexity)]

mod bike;
mod map;
mod waypoint;
mod slow;
mod camera;
mod character_editor;
mod bike_config;

use crate::bike::{BicycleParams, spawn_player};
use crate::map::spawn_map_system;
use avian2d::prelude::{Gravity, PhysicsDebugPlugin, PhysicsSet};
use avian2d::PhysicsPlugins;
use bevy::asset::AssetMetaCheck;
use bevy::prelude::*;
use bevy::render::camera::ScalingMode;
use bevy_egui::EguiPlugin;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use crate::bike_config::{PlayerConfig, PlayerConfigChangedEvent};

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(AssetPlugin {
                // Wasm builds will check for meta files (that don't exist) if this isn't set.
                // This causes errors and even panics in web builds on itch.
                // See https://github.com/bevyengine/bevy_github_ci_template/issues/48.
                meta_check: AssetMetaCheck::Never,
                ..default()
            }),
            PhysicsPlugins::default().with_length_unit(1.0),
           //PhysicsDebugPlugin::default(),
            EguiPlugin,
            WorldInspectorPlugin::new()
        ))
        .insert_resource(Gravity(Vec2::new(0.0, 0.0)))

        .register_type::<BicycleParams>()

        .add_systems(
            Startup,
            (
                setup,
                spawn_player.after(spawn_map_system),
                spawn_map_system,
            ),
        )
        .add_systems(
            Update,
            (
                bike::control_player,
                bike::drift_factor_system,
                bike::bike_controller_system,
                bike::apply_config_to_player.run_if(resource_changed::<PlayerConfig>),
                waypoint::follow_waypoint,
                character_editor::character_editor,
            ),
        )
        .add_systems(
            PostUpdate,
            camera::update_camera
                .after(PhysicsSet::Sync)
                .before(TransformSystem::TransformPropagate),
        )

        .insert_resource(PlayerConfig::default())
        .add_event::<PlayerConfigChangedEvent>()

        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // let mut camera = Camera2dBundle::default();
    //
    // // camera.transform.scale = Vec3::splat(1.0 / 20.0);
    // camera.projection.scaling_mode = ScalingMode::AutoMin {
    //     min_height: 80.0,
    //     min_width: 80.0,
    // };
    //
    // commands.spawn(camera);
    // commands.spawn(SpriteBundle {
    //     texture: asset_server.load("ducky.png"),
    //     ..Default::default()
    // });
}
