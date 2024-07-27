use crate::bike_config::{BicycleModTrait, PlayerConfig, Selectable};
use bevy::prelude::{DetectChangesMut, ResMut};
use bevy_egui::{egui, EguiContexts};
use egui_extras::{Size, StripBuilder};
use rand::{random, thread_rng, Rng};

pub fn character_editor(mut contexts: EguiContexts, mut player: ResMut<PlayerConfig>) {
    let ctx = contexts.ctx_mut();

    egui::Window::new("Character Editor").show(ctx, |ui| {
        let config = player.bypass_change_detection();
        let mut changed = false;

        ui.heading("Character Editor");
        if ui.button("Randomize All").clicked() {
            *config = random();
            changed = true;
        };

        ui.label("Skin:");
        changed |= select(ui, &mut config.0.skin);

        ui.label("Hat:");
        changed |= select(ui, &mut config.0.hat);

        ui.label("Frame:");
        changed |= select(ui, &mut config.0.bike.frame);

        ui.label("Back Wheel:");
        changed |= select(ui, &mut config.0.bike.rear_wheel);

        ui.label("Addon:");
        changed |= select(ui, &mut config.0.bike.addon);

        if changed {
            player.set_changed();
        }
    });
}

pub fn select(ui: &mut egui::Ui, item: &mut (impl Selectable + BicycleModTrait)) -> bool {
    let mut changed = false;
    ui.horizontal_top(|ui| {
        ui.set_height(20.0);
        StripBuilder::new(ui)
            .size(Size::exact(30.0))
            .size(Size::exact(100.0))
            .size(Size::exact(30.0))
            .size(Size::exact(30.0))
            .horizontal(|mut strip| {
                strip.cell(|ui| {
                    if ui.button("⬅").clicked() {
                        item.prev();
                        changed = true;
                    }
                });
                strip.cell(|ui| {
                    ui.label(item.name());
                });
                strip.cell(|ui| {
                    if ui.button("➡").clicked() {
                        item.next();
                        changed = true;
                    }
                });
                strip.cell(|ui| {
                    if ui.button("R").clicked() {
                        item.rand();
                        changed = true;
                    }
                });
            });
    });
    changed
}
