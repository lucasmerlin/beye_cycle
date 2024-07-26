use bevy::prelude::*;
use crate::bike::{Bicycle, Player};

pub fn update_camera(
    mut query: Query<(&mut Transform, &mut Camera), Without<Player>>,
    bike_query: Query<(&Transform), With<Player>>,
) {
    for (mut transform, mut camera) in query.iter_mut() {
        if let Some(bike_transform) = bike_query.iter().next() {
            transform.translation = bike_transform.translation;
        }
    }
}