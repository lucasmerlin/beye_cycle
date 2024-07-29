use crate::bike::{Bicycle, BicycleParams, Player};
use crate::bike_config::PlayerConfig;
use crate::character_editor::character_editor;
use crate::game_state::{DespawnMe, GameConfig, GameState, RaceConfig, RaceState, MAPS};
use bevy::prelude::*;
use bevy::render::camera::ScalingMode;
use bevy_egui::egui::{ComboBox, Id, ScrollArea, Visuals, Widget};
use bevy_egui::{egui, EguiContexts};
use std::f32::consts::PI;

pub fn main_menu_ui(
    mut contexts: EguiContexts,
    mut race_config: ResMut<RaceConfig>,
    mut player_config: ResMut<PlayerConfig>,
    mut next_state: ResMut<NextState<GameState>>,
    mut next_race_state: ResMut<NextState<RaceState>>,
    game_config: Res<GameConfig>,
) {
    let ctx = contexts.ctx_mut();
    ctx.set_visuals(Visuals::light());

    egui::SidePanel::left(Id::new("Main Menu")).show(ctx, |ui| {
        ScrollArea::vertical().show(ui, |ui| {
            let width = ui.group(|ui| {
                character_editor(ui, &mut player_config);
            }).response.rect.width();

            ui.group(|ui| {
                ui.set_width(width - ui.style().spacing.window_margin.left * 2.0);
                ui.heading("Race Setup");
                ui.horizontal(|ui| {
                    ui.add_enabled_ui(game_config.level_selector_unlocked, |ui| {
                        ui.selectable_value(&mut race_config.is_cup, false, "Single")
                            .on_disabled_hover_text("Complete a cup to unlock the level selector");
                    });
                    ui.selectable_value(&mut race_config.is_cup, true, "Cup");
                });

                if !race_config.is_cup {
                    ComboBox::new("Map", "Select Map")
                        .selected_text(&race_config.map)
                        .show_ui(ui, |ui| {
                            for map in MAPS {
                                if ui.selectable_label(race_config.map == *map, map).clicked() {
                                    race_config.map = map.to_string();
                                }
                            }
                        });
                } else {
                    race_config.map = MAPS[0].to_string();
                }

                ui.add_enabled_ui(game_config.level_selector_unlocked, |ui| {
                    ui.label("AI Count:");
                    egui::widgets::DragValue::new(&mut race_config.ai_count)
                        .range(0..=20)
                        .ui(ui)
                        .on_disabled_hover_text("Complete a cup to unlock the level selector");

                    ui.label("Laps:");
                    egui::widgets::DragValue::new(&mut race_config.laps)
                        .range(1..=10)
                        .ui(ui)
                        .on_disabled_hover_text("Complete a cup to unlock the level selector");
                });

                if ui.button("Start Race").clicked() {
                    next_state.set(GameState::Race);
                    next_race_state.set(RaceState::Countdown);
                }
            });
        });
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
        commands.spawn((TransformBundle::default()));
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
