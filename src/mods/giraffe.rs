use crate::bike::Player;
use bevy::input::keyboard::KeyboardInput;
use bevy::prelude::*;
use bevy::ui::prepare_uinodes;

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
    query: Query<(&Giraffe, &GlobalTransform, &Parent), Without<Player>>,
    player_query: Query<(&Player, &GlobalTransform)>,
    input: Res<ButtonInput<KeyCode>>,
    assets: Res<AssetServer>,
) {
    for (_, transform, parent) in query.iter() {
        if let Ok((parent, parent_transform)) = player_query.get(parent.get()) {
            if input.just_pressed(KeyCode::Space) {
                let bike_dir = parent_transform.up().normalize();

                let offset = -bike_dir;

                println!("Spawning poo at");

                commands.spawn((SpriteBundle {
                    transform: Transform::from_translation(transform.translation() + offset),
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
