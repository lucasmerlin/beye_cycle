use crate::slow::Slow;
use crate::waypoint::{Waypoint, WaypointAi};
use avian2d::math::Vector;
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


    let direction = -(next_waypoint_transfrom.translation - start_post.translation).xy();

    let bicycle_length = 6.0;

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
            Collider::rectangle(bicycle_length / 10.0, bicycle_length),
            ExternalForce::default(),
            TransformBundle {
                local: start_post.clone()
                    * Transform::from_translation(Vec3::new(offset.x, offset.y, 0.0))
                    * Transform::from_rotation(Quat::from_rotation_z(direction.angle_between(Vec2::Y))),
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
                        custom_size: Some(Vec2::new(bicycle_length, bicycle_length * 0.75)),
                        ..Default::default()
                    },
                    ..Default::default()
                }),
            );
        });
    };

    spawn(true, Vec2::new(0.0, 0.0));

    let direction_right = direction.normalize().rotate(Vec2::from_angle(PI / 2.0));

    // Places enemies in a F1 like  grid
    for i in 0..4 {
        let offset = direction.normalize() * (i as f32 * 2.0) + direction_right * (i as f32 % 2.0);
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
    spatial_query: SpatialQuery,
    mut slow_query: Query<(&Slow)>,
) {
    for (control, params, velocity, mut transform, mut ext_force, mut damping) in query.iter_mut() {
        ext_force.clear();

        let intersections = spatial_query.point_intersections(
            Vector::new(transform.translation.x, transform.translation.y),
            SpatialQueryFilter::default(),
        );
        let slow = intersections
            .iter()
            .any(|entity| slow_query.get(*entity).is_ok());

        let max_speed = if slow {
            params.max_speed * 0.5
        } else {
            params.max_speed
        };
        let acceleration = if slow {
            params.acceleration * 0.5
        } else {
            params.acceleration
        };

        let bike_forward = (transform.rotation * Vec3::Y).xy();

        let forward_velocity = velocity.dot(bike_forward);

        let slower_than_max_speed =
            forward_velocity < max_speed && forward_velocity > -max_speed * 0.5;

        let mut control_acceleration_clamped = control.acceleration.clamp(-1.0, 1.0);

        if !slower_than_max_speed && slow {
            control_acceleration_clamped = -0.8;
        } else if !slower_than_max_speed {
            control_acceleration_clamped = 0.0;
        }

        let acceleration = control_acceleration_clamped * acceleration;
        let current_rotation = transform.rotation * Vec3::Y;
        ext_force.apply_force(Vec2::new(acceleration, 0.0).rotate(current_rotation.xy()));

        let slow_turn_factor = (forward_velocity / 8.0).clamp(-1.0, 1.0);
        let turn_clamped = control.turn.clamp(-1.0, 1.0);
        let turn = turn_clamped * params.turn * slow_turn_factor;
        transform.rotate_z(turn);

        if control.acceleration == 0.0 {
            **damping = FloatExt::lerp(**damping, 3.0, 0.01);
        } else {
            damping.0 = 0.0;
        }

        if slow {
            damping.0 *= 1.2;
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
