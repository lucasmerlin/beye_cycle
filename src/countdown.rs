use crate::game_state::RaceState;
use bevy::prelude::*;
use bevy_egui::egui::load::SizedTexture;
use bevy_egui::egui::{Align2, Id};
use bevy_egui::{egui, EguiContexts};
use bevy_inspector_egui::egui::Area;

#[derive(Resource)]
pub struct RaceCountdown {
    timer: Timer,
}

impl Default for RaceCountdown {
    fn default() -> Self {
        Self {
            timer: Timer::from_seconds(5.0, TimerMode::Once),
        }
    }
}

pub fn countdown_ui(
    assets: Res<AssetServer>,
    mut egui: EguiContexts,
    race_state: Res<State<RaceState>>,
    mut next_race_state: ResMut<NextState<RaceState>>,
    mut images: Local<Vec<egui::TextureId>>,
    mut race_countdown: ResMut<RaceCountdown>,
    time: Res<Time>,
) {
    if images.is_empty() {
        for i in 0..4 {
            images.push(egui.add_image(assets.load(format!("countdown/{}.png", i))));
        }
    }

    if **race_state == RaceState::Countdown {
        race_countdown.timer.tick(time.delta());
        let second = race_countdown.timer.remaining_secs().floor() as u32;

        Area::new(Id::new("Countdown"))
            .anchor(Align2::CENTER_CENTER, [0.0, 0.0])
            .show(egui.ctx_mut(), |ui| {
                let image = images.get(second as usize);
                if let Some(image) = image {
                    ui.image(SizedTexture::new(*image, egui::Vec2::new(500.0, 250.0)));
                }
            });

        if race_countdown.timer.finished() {
            next_race_state.set(RaceState::Playing);
        }
    }
}
