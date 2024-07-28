use crate::bike::{control_player, Bicycle, BicycleControl, Player, GAME_BICYCLE_LENGTH};
use crate::bike_config::ForBicycle;
use crate::waypoint::{follow_waypoint, WaypointAi};
use avian2d::prelude::{Collider, Collision, LinearVelocity, RigidBody};
use bevy::prelude::*;

pub struct GiraffePlugin;

impl Plugin for GiraffePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                giraffe_player_control_system,
                giraffe_ai_control_system,
                poo_collision.after(control_player).after(follow_waypoint),
                poo_collision_update,
            ),
        );
    }
}

#[derive(Debug, Component)]
pub struct Giraffe {
    ai_timer: Timer,
    poo_left: usize,
}

impl Default for Giraffe {
    fn default() -> Self {
        Self {
            ai_timer: Timer::from_seconds(3.0, TimerMode::Repeating),
            poo_left: 3,
        }
    }
}

#[derive(Debug, Component)]
pub struct Poo;

pub fn giraffe_player_control_system(
    mut commands: Commands,
    mut query: Query<
        (Entity, &mut Giraffe, &GlobalTransform, &ForBicycle, &Parent),
        Without<Player>,
    >,
    player_query: Query<(&Player, &GlobalTransform)>,
    input: Res<ButtonInput<KeyCode>>,
    assets: Res<AssetServer>,
) {
    for (entity, mut giraffe, transform, for_bicycle, parent) in query.iter_mut() {
        if let Ok((player, parent_transform)) = player_query.get(for_bicycle.0) {
            if input.just_pressed(KeyCode::Space) {
                if giraffe.poo_left > 0 {
                    giraffe.poo_left -= 1;
                    spawn_poo(&mut commands, parent_transform, &assets);
                }
                if giraffe.poo_left == 0 {
                    commands.entity(parent.get()).remove_children(&[entity]);
                    commands.entity(entity).despawn_recursive();
                }
            }
        }
    }
}

pub fn giraffe_ai_control_system(
    mut commands: Commands,
    mut query: Query<
        (Entity, &mut Giraffe, &GlobalTransform, &ForBicycle, &Parent),
        Without<WaypointAi>,
    >,
    player_query: Query<(&WaypointAi, &GlobalTransform)>,
    assets: Res<AssetServer>,
    time: Res<Time>,
) {
    for (entity, mut giraffe, transform, for_bicycle, parent) in query.iter_mut() {
        giraffe.ai_timer.tick(time.delta());

        if giraffe.ai_timer.finished() {
            if let Ok((player, parent_transform)) = player_query.get(for_bicycle.0) {
                if giraffe.poo_left > 0 {
                    giraffe.poo_left -= 1;
                    spawn_poo(&mut commands, parent_transform, &assets);
                }
                if giraffe.poo_left == 0 {
                    commands.entity(parent.get()).remove_children(&[entity]);
                    commands.entity(entity).despawn_recursive();
                }
            }
        }
    }
}

pub fn spawn_poo(
    commands: &mut Commands,
    bike_transform: &GlobalTransform,
    assets: &Res<AssetServer>,
) {
    let bike_dir = bike_transform.up().normalize();

    let offset = -bike_dir * (GAME_BICYCLE_LENGTH / 2.0 + 0.3);

    commands.spawn((
        SpriteBundle {
            transform: Transform::from_translation(bike_transform.translation() + offset),
            texture: assets.load("things/Scheyesse.png"),
            sprite: Sprite {
                custom_size: Some(Vec2::new(1.0, 1.0)),
                ..Default::default()
            },
            ..Default::default()
        },
        RigidBody::Static,
        Collider::circle(0.2),
        Poo,
    ));
}

#[derive(Debug, Component)]
pub struct PooCollision {
    pub timer: Timer,
}

pub fn poo_collision(
    mut commands: Commands,
    mut events: EventReader<Collision>,
    poo_query: Query<(Entity, &Poo)>,
    mut bicycle_query: Query<(Entity, &Bicycle, &mut BicycleControl, &mut LinearVelocity)>,
) {
    for Collision(contacts) in events.read() {
        if let Some((poo, ..)) = poo_query
            .iter_many([contacts.entity1, contacts.entity2])
            .next()
        {
            if let Some((bicycle, _, mut control, mut velocity)) = bicycle_query
                .iter_many_mut([contacts.entity1, contacts.entity2])
                .fetch_next()
            {
                commands.entity(bicycle).insert(PooCollision {
                    timer: Timer::from_seconds(0.5, TimerMode::Once),
                });
                velocity.0 = Vec2::ZERO;
                control.turn = 0.0;
                control.acceleration = 0.0;

                commands.entity(poo).despawn();
            }
        }
    }
}

pub fn poo_collision_update(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut PooCollision, &Children)>,
    mut transform_query: Query<&mut Transform>,
) {
    for (entity, mut collision, children) in query.iter_mut() {
        collision.timer.tick(time.delta());

        let fract = collision.timer.fraction();

        for child in children.iter() {
            if let Ok(mut transform) = transform_query.get_mut(*child) {
                transform.rotation = Quat::from_rotation_z(fract * std::f32::consts::PI * 2.0);
            }
        }

        if collision.timer.finished() {
            commands.entity(entity).remove::<PooCollision>();
        }
    }
}
