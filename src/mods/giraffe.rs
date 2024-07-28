use crate::bike::{GAME_BICYCLE_LENGTH, Player};
use bevy::input::keyboard::KeyboardInput;
use bevy::prelude::*;
use bevy::ui::prepare_uinodes;
use crate::bike_config::ForBicycle;

pub struct GiraffePlugin;

impl Plugin for GiraffePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (giraffe_player_control_system));
    }
}

#[derive(Debug, Component)]
pub struct Giraffe;

#[derive(Debug, Component)]
pub struct Poo;

pub fn giraffe_player_control_system(
    mut commands: Commands,
    query: Query<(&Giraffe, &GlobalTransform, &ForBicycle), Without<Player>>,
    player_query: Query<(&Player, &GlobalTransform)>,
    input: Res<ButtonInput<KeyCode>>,
    assets: Res<AssetServer>,
) {
    for (_, transform, parent) in query.iter() {
        if let Ok((player, parent_transform)) = player_query.get(parent.0) {
            if input.just_pressed(KeyCode::Space) {
                let bike_dir = parent_transform.up().normalize();

                let offset = -bike_dir * (GAME_BICYCLE_LENGTH / 2.0 + 0.3);

                println!("Spawning poo at");

                commands.spawn((SpriteBundle {
                    transform: Transform::from_translation(parent_transform.translation() + offset),
                    texture: assets.load("things/Scheyesse.png"),
                    sprite: Sprite {
                        custom_size: Some(Vec2::new(1.0, 1.0)),
                        ..Default::default()
                    },
                    ..Default::default()
                },));
            }
        }
    }
}
