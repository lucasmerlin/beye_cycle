use bevy::prelude::*;
use crate::bike::BicycleParams;
use crate::bike_config::ForBicycle;

#[derive(Debug, Component)]
pub struct RocketAddon {
    timer: Timer,
}

impl Default for RocketAddon {
    fn default() -> Self {
        Self {
            timer: Timer::from_seconds(5.0, TimerMode::Once),
        }
    }
}

pub fn despawn_rocket_system(
    mut commands: Commands,
    mut query: Query<(Entity, &mut RocketAddon, &Parent)>,
    time: Res<Time>,
) {
    for (entity, mut rocket_addon, parent) in query.iter_mut() {
        if rocket_addon.timer.tick(time.delta()).just_finished() {
            commands.entity(parent.get()).remove_children(&[entity]);
            commands.entity(entity).despawn_recursive();
        }
    }
}