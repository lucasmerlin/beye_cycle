use crate::bike::{Bicycle, Player};
use crate::game_state::{GameState, RaceConfig, RaceState, MAPS};
use crate::ranking::{Progress, Rank};
use bevy::asset::AssetServer;
use bevy::log::tracing_subscriber::fmt::format;
use bevy::prelude::*;
use bevy_egui::egui::load::SizedTexture;
use bevy_egui::egui::{Align, Area, Frame, Id, Layout, Visuals};
use bevy_egui::{egui, EguiContexts};

pub fn finish_ui(
    assets: Res<AssetServer>,
    mut image: Local<Option<egui::TextureId>>,
    mut egui: EguiContexts,
    mut next_game_state: ResMut<NextState<GameState>>,
    query: Query<(&Bicycle, &Rank, &Name, &Progress), With<Player>>,
    mut race_config: ResMut<RaceConfig>,
) {
    let image = image.get_or_insert_with(|| egui.add_image(assets.load("things/Banner.png")));

    Area::new(Id::new("Finish"))
        .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
        .show(egui.ctx_mut(), |ui| {
            Frame::window(ui.style()).show(ui, |ui| {
                ui.image(SizedTexture {
                    id: *image,
                    size: egui::Vec2::new(500.0, 250.0),
                });

                ui.with_layout(Layout::top_down_justified(Align::Center), |ui| {
                    if let Some((_, rank, ..)) = query.iter().next() {
                        let text = match rank.0 {
                            1 => "You won!".to_string(),
                            2 => "You came in second!".to_string(),
                            3 => "You came in third!".to_string(),
                            _ => format!("You finished in position {}", rank.0),
                        };

                        ui.heading(text);
                    }

                    let map_idx = MAPS.iter().position(|map| map == &race_config.map).unwrap();
                    if let Some(next_map) = MAPS.get(map_idx + 1) {
                        if ui.button("Next Map").clicked() {
                            next_game_state.set(GameState::Restart);
                            race_config.map = next_map.to_string();
                        }
                    } else {
                        if race_config.is_cup {
                            ui.heading(
                                "Yay! You finished the cup and unlocked the level selector!",
                            );
                        }
                    }
                    if ui.button("Restart").clicked() {
                        next_game_state.set(GameState::Restart);
                    }

                    if ui.button("Main Menu").clicked() {
                        next_game_state.set(GameState::MainMenu);
                    }
                });
            });
        });
}

pub fn lap_ui(
    mut egui: EguiContexts,
    player_progress_query: Query<(&Player, &Progress, &Rank)>,
    race_config: Res<RaceConfig>,
) {
    if let Some((player, progress, rank)) = player_progress_query.iter().next() {
        egui::Area::new(Id::new("Laps"))
            .anchor(egui::Align2::RIGHT_TOP, [0.0, 0.0])
            .show(egui.ctx_mut(), |ui| {
                ui.style_mut().visuals = Visuals::dark();
                Frame::window(ui.style()).show(ui, |ui| {
                    ui.style_mut().visuals.override_text_color = Some(egui::Color32::WHITE);
                    ui.label("Pos:");
                    ui.heading(format!("{}", rank.0));
                    ui.label("Lap:");
                    ui.heading(format!("{}/{}", progress.round, race_config.laps));
                    ui.label("Checkpoint:");
                    ui.heading(format!("{}", progress.checkpoint_idx));
                });
            });
    }
}
