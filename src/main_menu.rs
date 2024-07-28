use std::f32::consts::PI;
use crate::bike::{Bicycle, BicycleParams, Player};
use crate::bike_config::PlayerConfig;
use crate::character_editor::character_editor;
use crate::game_state::{GameState, RaceConfig, MAPS, DespawnMe};
use bevy::prelude::*;
use bevy::render::camera::ScalingMode;
use bevy_egui::egui::ComboBox;
use bevy_egui::{egui, EguiContexts};

pub fn main_menu_ui(
    mut contexts: EguiContexts,
    mut race_config: ResMut<RaceConfig>,
    mut player_config: ResMut<PlayerConfig>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    let ctx = contexts.ctx_mut();

    egui::Window::new("Main Menu").show(ctx, |ui| {
        ui.heading("Main Menu");

        character_editor(ui, &mut player_config);

        ComboBox::new("Map", "Select Map")
            .selected_text(&race_config.map)
            .show_ui(ui, |ui| {
                for map in MAPS {
                    if ui.selectable_label(race_config.map == *map, map).clicked() {
                        race_config.map = map.to_string();
                    }
                }
            });

        if ui.button("Start Race").clicked() {
            next_state.set(GameState::Race);
        }
    });
}

pub fn setup_main_menu(mut commands: Commands, mut player_config: ResMut<PlayerConfig>) {
    let mut player = commands.spawn((
        Player,
        Bicycle,
        BicycleParams::default(),
        TransformBundle {
            local: Transform::from_rotation(Quat::from_rotation_z(-PI / 2.0)),
            ..Default::default()
        },
        VisibilityBundle::default(),
        DespawnMe,
    ));

    player.with_children(|commands| {
        commands.spawn((
            TransformBundle::default()
        ));
    });

    player_config.set_changed();

    let mut camera = Camera2dBundle {
        transform: Transform {
            translation: Vec3::new(0.0, 0.5, 10.0),
            ..Default::default()
        },
        ..Default::default()
    };

    camera.projection.scaling_mode = ScalingMode::FixedHorizontal(6.0);

    commands.spawn((camera, DespawnMe));
}
