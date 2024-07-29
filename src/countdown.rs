use crate::game_state::{DespawnMe, RaceConfig, RaceState};
use bevy::audio::Volume;
use bevy::prelude::*;
use bevy_egui::egui::load::SizedTexture;
use bevy_egui::egui::{Align2, Frame, Id};
use bevy_egui::{egui, EguiContexts};
use bevy_inspector_egui::egui::Area;

#[derive(Resource)]
pub struct RaceCountdown {
    timer: Timer,
    count: i32,
}

impl Default for RaceCountdown {
    fn default() -> Self {
        Self {
            timer: Timer::from_seconds(1.0, TimerMode::Repeating),
            count: 5,
        }
    }
}

pub fn race_setup(mut countdown: ResMut<RaceCountdown>) {
    countdown.timer.reset();
}

pub fn countdown_ui(
    mut commands: Commands,
    assets: Res<AssetServer>,
    mut egui: EguiContexts,
    race_state: Res<State<RaceState>>,
    mut next_race_state: ResMut<NextState<RaceState>>,
    mut images: Local<Vec<egui::TextureId>>,
    mut race_countdown: ResMut<RaceCountdown>,
    race_config: Res<RaceConfig>,
    time: Res<Time>,
) {
    if images.is_empty() {
        for i in 0..4 {
            images.push(egui.add_image(assets.load(format!("countdown/{}.webp", i))));
        }
    }

    if **race_state == RaceState::Countdown {
        race_countdown.timer.tick(time.delta());
        if race_countdown.timer.just_finished() {
            race_countdown.count -= 1;

            if race_countdown.count == 1 {
                commands.spawn((AudioBundle {
                    source: assets.load("sounds/go.mp3"),
                    settings: PlaybackSettings::default(),
                },));
            } else if race_countdown.count == 0 {
                next_race_state.set(RaceState::Playing);
                commands.spawn((
                    AudioBundle {
                        source: assets.load("sounds/bike.mp3"),
                        settings: PlaybackSettings::LOOP.with_volume(Volume::new(1.5)),
                    },
                    DespawnMe,
                ));
            } else {
                commands.spawn((AudioBundle {
                    source: assets.load("sounds/countdown.mp3"),
                    settings: PlaybackSettings::default(),
                },));
            }
        }
        let second = race_countdown.count - 1;

        Area::new(Id::new("Countdown"))
            .anchor(Align2::CENTER_CENTER, [0.0, 0.0])
            .show(egui.ctx_mut(), |ui| {
                let image = images.get(second as usize);
                if let Some(image) = image {
                    Frame::window(ui.style()).show(ui, |ui| {
                        ui.label("Track:");
                        ui.heading(&race_config.map);
                    });
                    ui.image(SizedTexture::new(*image, egui::Vec2::new(500.0, 250.0)));
                }
            });
    }
}
