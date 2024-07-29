use bevy::prelude::*;

#[derive(Debug, Component)]
pub struct DespawnMe;

#[derive(Debug, States, Clone, Eq, PartialEq, Hash)]
pub enum GameState {
    MainMenu,
    Race,
    // Utility to run race setup again
    Restart,
}

#[derive(Debug, States, Clone, Eq, PartialEq, Hash, Default)]
pub enum RaceState {
    #[default]
    Countdown,
    Pause,
    Playing,
    Finished,
}

#[derive(Debug, Resource)]
pub struct RaceConfig {
    pub ai_count: usize,
    pub map: String,
    pub laps: usize,
    pub is_cup: bool,
}

#[derive(Debug, Resource)]
pub struct GameConfig {
    pub level_selector_unlocked: bool,
}

impl Default for GameConfig {
    fn default() -> Self {
        Self {
            level_selector_unlocked: false,
        }
    }
}

pub const MAPS: [&str; 3] = ["Pool", "Uphill Both Ways", "Milky Way"];
pub const MAP_DATA: [&str; 3] = [
    include_str!("../assets/maps/Pool.svg"),
    include_str!("../assets/maps/Uphill Both Ways.svg"),
    include_str!("../assets/maps/Milky Way.svg"),
];

impl Default for RaceConfig {
    fn default() -> Self {
        Self {
            ai_count: 4,
            map: MAPS[0].to_string(),
            laps: 3,
            is_cup: true,
        }
    }
}

pub fn despawn_all(mut commands: Commands, query: Query<(Entity), With<DespawnMe>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

pub fn restart_system(
    mut next_state: ResMut<NextState<GameState>>,
    mut next_race_state: ResMut<NextState<RaceState>>,
) {
    next_state.set(GameState::Race);
    next_race_state.set(RaceState::Countdown);
}
