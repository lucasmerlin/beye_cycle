use avian2d::prelude::*;
use bevy::prelude::*;

#[derive(Component, Debug)]
pub struct Bicycle;

#[derive(Component, Debug)]
pub struct Player;

pub fn spawn_player(mut commands: Commands) {
    commands.spawn((
        Player,
        Bicycle,
        BicycleControl {
            turn: 0.0,
            acceleration: 0.0,
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
        TransformBundle::default(),
        LinearDamping::default(),
        AngularDamping(10.0),
    ));
    commands.spawn((
        RigidBody::Dynamic,
        Collider::rectangle(0.2, 2.0),
        ExternalForce::default(),
        TransformBundle::default(),
        LinearDamping(10.0),
        AngularDamping(10.0),
    ));
}

#[derive(Component, Debug)]
pub struct BicycleControl {
    acceleration: f32,
    turn: f32,
}

#[derive(Component, Debug)]
pub struct BicycleParams {
    acceleration: f32,
    max_speed: f32,
    turn: f32,
    drift: f32,
}


pub fn car_controller_system(
    mut query: Query<(&BicycleControl, &BicycleParams, &LinearVelocity, &mut Transform, &mut ExternalForce, &mut LinearDamping)>,
) {
    for (control, params, velocity, mut transform, mut ext_force, mut damping) in query.iter_mut() {
        ext_force.clear();

        let bike_forward = (transform.rotation * Vec3::Y).xy();

        let forward_velocity = velocity.dot(bike_forward);

        if forward_velocity < params.max_speed && forward_velocity > -params.max_speed * 0.3 {
            let acceleration = control.acceleration * params.acceleration;
            let current_rotation = transform.rotation * Vec3::Y;
            ext_force.apply_force(Vec2::new(acceleration, 0.0).rotate(current_rotation.xy()));
        }


        let slow_turn_factor = (forward_velocity / 8.0).clamp(-1.0, 1.0);
        dbg!(slow_turn_factor, forward_velocity);
        let turn = control.turn * params.turn * slow_turn_factor;
        transform.rotate_z(turn);


        if control.acceleration == 0.0 {
            **damping = FloatExt::lerp(**damping, 3.0, 0.01);
        } else {
            damping.0 = 0.0;
        }
    }
}

/// Basically kills the orthogonal velocity of the bike, as explained here: https://youtu.be/DVHcOS1E5OQ?si=UgpKyHxYqsRehCeZ&t=559
pub fn drift_factor_system(
    mut query: Query<(&mut LinearVelocity, &Transform, &BicycleParams)>,
) {
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