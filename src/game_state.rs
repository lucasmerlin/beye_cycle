use bevy::prelude::*;

#[derive(Debug, Component)]
pub struct DespawnMe;

#[derive(Debug, States, Clone, Eq, PartialEq, Hash)]
pub enum GameState {
    MainMenu,
    Race,
}

#[derive(Debug, Resource)]
pub struct RaceConfig {
    pub ai_count: usize,
    pub map: String,
    pub laps: usize,
}

pub const MAPS: [&str; 3] = ["Milky Way", "Uphill Both Ways", "Pool"];
pub const MAP_DATA: [&str; 3] = [
    include_str!("../assets/maps/Milky Way.svg"),
    include_str!("../assets/maps/Uphill Both Ways.svg"),
    include_str!("../assets/maps/Pool.svg"),
];

impl Default for RaceConfig {
    fn default() -> Self {
        Self {
            ai_count: 3,
            map: MAPS[0].to_string(),
            laps: 3,
        }
    }
}

pub fn despawn_all(mut commands: Commands, query: Query<(Entity), With<DespawnMe>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
