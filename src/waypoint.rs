use bevy::prelude::*;

#[derive(Component, Debug)]
pub struct Waypoint {
    pub next: Option<Entity>,
}