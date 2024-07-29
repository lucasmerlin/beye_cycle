// Bevy code commonly triggers these lints and they may be important signals
// about code quality. They are sometimes hard to avoid though, and the CI
// workflow treats them as errors, so this allows them throughout the project.
// Feel free to delete this line.
#![allow(clippy::too_many_arguments, clippy::type_complexity)]

mod addons;
mod bike;
mod bike_config;
mod camera;
mod character_editor;
mod game_state;
mod item_pickup;
mod main_menu;
mod map;
mod ranking;
mod slow;
mod waypoint;

use crate::addons::giraffe::GiraffePlugin;
use crate::addons::hook::HookPlugin;
use crate::addons::lasso::{FireLassoEvent, LassoPlugin};
use crate::addons::rocket;
use crate::bike::{spawn_bikes, BicycleParams};
use crate::bike_config::{PlayerConfig, PlayerConfigChangedEvent};
use crate::game_state::{despawn_all, GameState, RaceConfig};
use crate::item_pickup::ItemPickupPlugin;
use crate::map::spawn_map_system;
use crate::ranking::{Progress, Rank, RankingPlugin};
use crate::waypoint::Waypoint;
use avian2d::prelude::{Gravity, PhysicsDebugPlugin, PhysicsSet};
use avian2d::PhysicsPlugins;
use bevy::asset::AssetMetaCheck;
use bevy::prelude::*;
use bevy::render::camera::ScalingMode;
use bevy_egui::{EguiPlugin, EguiSettings};
use bevy_inspector_egui::quick::WorldInspectorPlugin;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins
                .set(AssetPlugin {
                    // Wasm builds will check for meta files (that don't exist) if this isn't set.
                    // This causes errors and even panics in web builds on itch.
                    // See https://github.com/bevyengine/bevy_github_ci_template/issues/48.
                    meta_check: AssetMetaCheck::Never,
                    ..default()
                })
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        fit_canvas_to_parent: true,
                        ..Default::default()
                    }),
                    ..default()
                }),
            PhysicsPlugins::default().with_length_unit(1.0),
            //PhysicsDebugPlugin::default(),
            EguiPlugin,
            GiraffePlugin,
            ItemPickupPlugin,
            RankingPlugin,
            LassoPlugin,
            HookPlugin,
        ))
        .insert_resource(EguiSettings {
            scale_factor: 1.5,
            default_open_url_target: Some("_blank".to_string()),
        })
        .insert_resource(Gravity(Vec2::new(0.0, 0.0)))
        .insert_state(GameState::MainMenu)
        .register_type::<BicycleParams>()
        .register_type::<Rank>()
        .register_type::<Progress>()
        .register_type::<Waypoint>()
        .add_systems(
            OnEnter(GameState::Race),
            (despawn_all, spawn_map_system, spawn_bikes).chain(),
        )
        .add_systems(
            OnEnter(GameState::MainMenu),
            (despawn_all, main_menu::setup_main_menu).chain(),
        )
        .add_systems(
            Update,
            (
                (
                    bike::control_player,
                    bike::drift_factor_system,
                    bike::bike_controller_system,
                    bike::mirror_bike_system,
                    waypoint::follow_waypoint,
                    rocket::despawn_rocket_system,
                )
                    .run_if(in_state(GameState::Race)),
                bike::apply_config_to_player.run_if(resource_changed::<PlayerConfig>),
                main_menu::main_menu_ui.run_if(in_state(GameState::MainMenu)),
            ),
        )
        .add_systems(
            PostUpdate,
            camera::update_camera
                .after(PhysicsSet::Sync)
                .before(TransformSystem::TransformPropagate)
                .run_if(in_state(GameState::Race)),
        )
        .insert_resource(PlayerConfig::default())
        .insert_resource(RaceConfig::default())
        .add_event::<PlayerConfigChangedEvent>()
        .run();
}
