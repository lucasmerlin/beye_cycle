use crate::waypoint::{Waypoint, WaypointAi};
use avian2d::prelude::*;
use bevy::prelude::*;
use std::f32::consts::PI;

#[derive(Component, Debug)]
pub struct Bicycle;

#[derive(Component, Debug)]
pub struct Player;

pub fn spawn_player(
    mut commands: Commands,
    mut asset_server: ResMut<AssetServer>,
    waypoint: Query<(Entity, &Waypoint, &Transform)>,
) {
    let (first_waypoint_entity, first_waypoint, start_post) = waypoint.iter().next().unwrap();

    let (next_waypoint_entity, next_waypoint, next_waypoint_transfrom) =
        waypoint.get(first_waypoint.next.unwrap()).unwrap();

    let mut spawn = |player: bool, offset: Vec2| {
        let mut entity = commands.spawn((
            Bicycle,
            BicycleControl {
                turn: 0.0,
                acceleration: 1.0,
            },
            BicycleParams {
                max_speed: 20.0,
                acceleration: 15.0,
                turn: 0.02,
                drift: 0.95,
            },
            RigidBody::Dynamic,
            Collider::rectangle(0.2, 2.0),
            ExternalForce::default(),
            TransformBundle {
                local: start_post.clone()
                    * Transform::from_translation(Vec3::new(offset.x, offset.y, 0.0)),
                ..Default::default()
            },
            LinearDamping::default(),
            AngularDamping(10.0),
        ));

        if player {
            entity.insert(Player);
        } else {
            entity.insert(WaypointAi {
                current_target: first_waypoint_entity,
            });
        }

        entity.with_children(|commands| {
            let mut transform = Transform::from_scale(Vec3::splat(1.0));
            transform.rotation = Quat::from_rotation_z(PI / 2.0);
            commands.spawn(
                (SpriteBundle {
                    texture: asset_server.load("bike.png"),
                    transform,
                    sprite: Sprite {
                        custom_size: Some(Vec2::new(2.0, 1.5)),
                        ..Default::default()
                    },
                    ..Default::default()
                }),
            );
        });
    };

    spawn(true, Vec2::new(0.0, 0.0));

    let direction = -(next_waypoint_transfrom.translation - start_post.translation).xy();

    // Places enemies in a F1 like  grid
    for i in 0..5 {
        let offset = direction.normalize() * (i as f32 * 2.0);
        spawn(false, offset);
    }
}

#[derive(Component, Debug)]
pub struct BicycleControl {
    pub(crate) acceleration: f32,
    pub(crate) turn: f32,
}

#[derive(Component, Debug)]
pub struct BicycleParams {
    acceleration: f32,
    max_speed: f32,
    turn: f32,
    drift: f32,
}

pub fn car_controller_system(
    mut query: Query<(
        &BicycleControl,
        &BicycleParams,
        &LinearVelocity,
        &mut Transform,
        &mut ExternalForce,
        &mut LinearDamping,
    )>,
) {
    for (control, params, velocity, mut transform, mut ext_force, mut damping) in query.iter_mut() {
        ext_force.clear();

        let bike_forward = (transform.rotation * Vec3::Y).xy();

        let forward_velocity = velocity.dot(bike_forward);

        if forward_velocity < params.max_speed && forward_velocity > -params.max_speed * 0.3 {
            let acceleration_clamped = control.acceleration.clamp(-1.0, 1.0);
            let acceleration = acceleration_clamped * params.acceleration;
            let current_rotation = transform.rotation * Vec3::Y;
            ext_force.apply_force(Vec2::new(acceleration, 0.0).rotate(current_rotation.xy()));
        }

        let slow_turn_factor = (forward_velocity / 8.0).clamp(-1.0, 1.0);
        let turn_clamped = control.turn.clamp(-1.0, 1.0);
        let turn = turn_clamped * params.turn * slow_turn_factor;
        transform.rotate_z(turn);

        if control.acceleration == 0.0 {
            **damping = FloatExt::lerp(**damping, 3.0, 0.01);
        } else {
            damping.0 = 0.0;
        }
    }
}

/// Basically kills the orthogonal velocity of the bike, as explained here: https://youtu.be/DVHcOS1E5OQ?si=UgpKyHxYqsRehCeZ&t=559
pub fn drift_factor_system(mut query: Query<(&mut LinearVelocity, &Transform, &BicycleParams)>) {
    for (mut lin_vel, transform, params) in query.iter_mut() {
        let drift = params.drift;

        let bike_forward = (transform.rotation * Vec3::Y).xy();
        let bike_right = (transform.rotation * Vec3::X).xy();

        let forward = bike_forward * lin_vel.dot(bike_forward.xy());
        let right = bike_right * lin_vel.dot(bike_right.xy());

        **lin_vel = forward + right * drift;
    }
}

pub fn control_player(
    mut commands: Commands,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&mut BicycleControl), With<Player>>,
) {
    for (mut control) in query.iter_mut() {
        control.acceleration = 0.0;
        if keyboard_input.pressed(KeyCode::KeyW) || keyboard_input.pressed(KeyCode::ArrowUp) {
            control.acceleration += 1.0;
        }
        if keyboard_input.pressed(KeyCode::KeyS) || keyboard_input.pressed(KeyCode::ArrowDown) {
            control.acceleration -= 1.0;
        }

        control.turn = 0.0;
        if keyboard_input.pressed(KeyCode::KeyA) || keyboard_input.pressed(KeyCode::ArrowLeft) {
            control.turn += 1.0;
        }
        if keyboard_input.pressed(KeyCode::KeyD) || keyboard_input.pressed(KeyCode::ArrowRight) {
            control.turn -= 1.0;
        }
    }
}
