use crate::bike::{Bicycle, Player};
use crate::game_state::{GameState, RaceConfig, RaceState};
use crate::waypoint::Waypoint;
use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};

pub struct RankingPlugin;

impl Plugin for RankingPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (track_progress_system, rank_bicycles_system, check_finish).run_if(in_state(GameState::Race).and_then(in_state(RaceState::Playing))),
        );
    }
}

#[derive(Debug, Component, Reflect)]
pub struct Rank(pub usize);

#[derive(Debug, Component, Reflect, Clone)]
pub struct Progress {
    pub round: usize,
    pub checkpoint_idx: usize,
    pub distance_to_next_checkpoint: f32,
    pub next_checkpoint: Entity,
}

pub const NEXT_CHECKPOINT_DISTANCE: f32 = 5.0;

pub fn rank_bicycles_system(mut query: Query<(&Bicycle, &Progress, &mut Rank)>) {
    let mut all = query.iter_mut().collect::<Vec<_>>();

    all.sort_by(|(_, a, _), (_, b, _)| {
        a.round
            .cmp(&b.round)
            .then_with(|| a.checkpoint_idx.cmp(&b.checkpoint_idx))
            .then_with(|| {
                b.distance_to_next_checkpoint
                    .partial_cmp(&a.distance_to_next_checkpoint)
                    .unwrap()
            })
    });

    for (i, (_, _, mut rank)) in all.into_iter().rev().enumerate() {
        rank.0 = i + 1;
    }
}

pub fn track_progress_system(
    mut query: Query<(&Bicycle, &GlobalTransform, &mut Progress)>,
    mut checkpoint_query: Query<(&GlobalTransform, &Waypoint)>,
) {
    for (bicycle, transform, mut progress) in query.iter_mut() {
        let (target_checkpoint_transform, target_checkpoint) = checkpoint_query
            .get(progress.next_checkpoint)
            .expect("Next checkpoint not found");

        let distance = transform
            .translation()
            .distance(target_checkpoint_transform.translation());

        progress.distance_to_next_checkpoint = distance;

        if distance < NEXT_CHECKPOINT_DISTANCE {
            let (next_checkpoint_transform, next_checkpoint_data) = checkpoint_query
                .get(progress.next_checkpoint)
                .expect("Next checkpoint not found");

            if target_checkpoint.index == 0 {
                progress.round += 1;
            }
            progress.checkpoint_idx = next_checkpoint_data.index;

            progress.next_checkpoint = target_checkpoint.next.unwrap();

            progress.distance_to_next_checkpoint = transform
                .translation()
                .distance(next_checkpoint_transform.translation());
        }
    }
}

pub fn check_finish(
    race_config: Res<RaceConfig>,
    mut next_race_state: ResMut<NextState<RaceState>>,
    player_progress_query: Query<(&Player, &Progress)>,
) {
    if let Some((_, progress)) = player_progress_query.iter().next() {
        if progress.round > race_config.laps {
            next_race_state.set(RaceState::Finished);
        }
    }
}
