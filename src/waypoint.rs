use avian2d::prelude::LinearVelocity;
use crate::bike::{BicycleControl, Player};
use bevy::prelude::*;

#[derive(Component, Debug)]
pub struct Waypoint {
    pub next: Option<Entity>,
}

#[derive(Component, Debug)]
pub struct WaypointAi {
    pub current_target: Entity,
}

pub fn follow_waypoint(
    waypoint_query: Query<(&Transform, &Waypoint), Without<WaypointAi>>,
    mut bike_query: Query<(&mut Transform, &LinearVelocity, &mut BicycleControl, &mut WaypointAi), Without<Player>>,
) {
    for (transform, velocity, mut control, mut ai) in bike_query.iter_mut() {

        let (target_transform, target_waypoint) = waypoint_query.get(ai.current_target).unwrap();
        let (next_transform, next_waypoint) = waypoint_query.get(target_waypoint.next.unwrap()).unwrap();

        let target_position = target_transform.translation;

        let direction = target_position - transform.translation;

        let distance = direction.length();

        if distance < 5.0 {
            if let Some(next) = waypoint_query.get(ai.current_target).unwrap().1.next {
                ai.current_target = next;
            } else {
                control.acceleration = 0.0;
            }
        } else {
            let direction = direction.normalize().xy();
            let forward = transform.up().xy();
            let angle_deg = -direction.angle_between(forward).to_degrees();

            let direction_next_waypoint = next_transform.translation - target_position;
            let distance_next_waypoint = direction_next_waypoint.length();
            let angle_next_waypoint_deg = -direction.angle_between(direction_next_waypoint.normalize().xy()).to_degrees();


            const CLOSING_IN_DIST: f32 = 10.0;
            // closing_in is 0 if distance > 10 and 1 if distance < 3
            let closing_in = 1.0 - (distance - 5.0).max(0.0).min(CLOSING_IN_DIST) / CLOSING_IN_DIST;

            let turn = (angle_deg / 45.0).clamp(-1.0, 1.0);
            let turn_next = (angle_next_waypoint_deg / 45.0).clamp(-1.0, 1.0);

            let current_speed = velocity.length();


            let target_speed_at_waypoint = distance_next_waypoint * 2.0;

            let accel_next = (target_speed_at_waypoint - current_speed).clamp(-1.0, 1.0);


            let brake = turn_next * 0.3 * current_speed * 0.1;


            // let angle_diff = angle - transform.rotation.to_axis_angle().1;
            control.turn = turn;

            let current_accel = 1.05 - (turn.abs() / 1.0);
            let selected_accel_next = f32::min(1.05 - (turn_next.abs() / 1.0), accel_next);

            let mut accel = (selected_accel_next * closing_in) + (current_accel * (1.0 - closing_in));

            accel = f32::min(accel, current_accel);

            if current_speed < 2.0 {
                accel = 1.0;
            }

            control.acceleration = accel;

        }

    }
}
