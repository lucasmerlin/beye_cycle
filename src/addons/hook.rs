use crate::addons::lasso::{MoveToTarget, MovedToTargetEvent};
use crate::bike::{Bicycle, Player};
use crate::bike_config::ForBicycle;
use crate::ranking::{Progress, Rank};
use bevy::prelude::*;
use rand::random;
use crate::game_state::DespawnMe;

pub struct HookPlugin;

impl Plugin for HookPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                player_hook_control_system,
                fire_hook_system,
                hook_hit_system,
                ai_hook_control_system,
            ),
        )
        .add_event::<FireHookEvent>();
    }
}

#[derive(Debug, Component)]
pub struct HookAddon {
    ai_timer: Timer,
}

impl Default for HookAddon {
    fn default() -> Self {
        Self {
            ai_timer: Timer::from_seconds(random::<f32>() * 5.0, TimerMode::Once),
        }
    }
}

pub const HOOK_SPEED: f32 = 0.1;

#[derive(Debug, Component)]
pub struct Hook {
    target: Entity,
    by: Entity,
}

#[derive(Debug, Event)]
pub struct FireHookEvent {
    by: Entity,
}

#[derive(Debug, Component)]
pub struct HookCaughtAndMovingBack;

pub fn player_hook_control_system(
    mut commands: Commands,
    mut query: Query<(Entity, &HookAddon, &GlobalTransform, &ForBicycle, &Parent)>,
    mut is_player_query: Query<&Player>,
    input: Res<ButtonInput<KeyCode>>,
    mut events: EventWriter<FireHookEvent>,
) {
    for (entity, _, transform, for_bicycle, parent) in query.iter_mut() {
        if let Ok(_) = is_player_query.get(for_bicycle.0) {
            if input.just_pressed(KeyCode::Space) {
                events.send(FireHookEvent { by: for_bicycle.0 });

                commands.entity(parent.get()).remove_children(&[entity]);
                commands.entity(entity).despawn_recursive();
            }
        }
    }
}

pub fn ai_hook_control_system(
    mut commands: Commands,
    mut query: Query<(
        Entity,
        &mut HookAddon,
        &GlobalTransform,
        &ForBicycle,
        &Parent,
    )>,
    mut is_not_player_query: Query<&Bicycle, Without<Player>>,
    mut events: EventWriter<FireHookEvent>,
    time: Res<Time>,
) {
    for (entity, mut hook, transform, for_bicycle, parent) in query.iter_mut() {
        if let Ok(_) = is_not_player_query.get(for_bicycle.0) {
            if hook.ai_timer.tick(time.delta()).just_finished() {
                events.send(FireHookEvent { by: for_bicycle.0 });

                commands.entity(parent.get()).remove_children(&[entity]);
                commands.entity(entity).despawn_recursive();
            }
        }
    }
}

pub fn fire_hook_system(
    mut commands: Commands,
    mut by_query: Query<(Entity, &Rank, &GlobalTransform)>,
    mut target_query: Query<(Entity, &Rank, &GlobalTransform)>,
    mut events: EventReader<FireHookEvent>,
    assets: Res<AssetServer>,
) {
    for (event) in events.read() {
        let (by, by_rank, by_transform) = by_query.get_mut(event.by).unwrap();

        let mut target = None;

        for (entity, rank, transform) in target_query.iter_mut() {
            if rank.0 == by_rank.0 - 1 {
                target = Some((entity, transform.translation()));
                break;
            }
        }

        if let Some((target_entity, target_position)) = target {
            commands.spawn((
                DespawnMe,
                Hook {
                    target: target_entity,
                    by,
                },
                MoveToTarget {
                    target: target_entity,
                    speed: HOOK_SPEED,
                },
                SpriteBundle {
                    transform: Transform::from_translation(by_transform.translation()),
                    sprite: Sprite {
                        custom_size: Some(Vec2::new(1.0, 1.0)),
                        ..Default::default()
                    },
                    texture: assets.load("addons/Hook.png"),
                    ..Default::default()
                },
            ));
        }
    }
}

pub fn hook_hit_system(
    mut commands: Commands,
    mut events: EventReader<MovedToTargetEvent>,
    hook_query: Query<(&Hook)>,
    moving_back_query: Query<(&HookCaughtAndMovingBack)>,
    mut progress_query: Query<(&mut Progress)>,
) {
    for event in events.read() {
        if let Ok(hook) = hook_query.get(event.entity) {
            println!("Hook hit target");
            commands.entity(event.entity).despawn_recursive();

            commands.entity(hook.by).insert((
                MoveToTarget {
                    target: hook.target,
                    speed: HOOK_SPEED,
                },
                HookCaughtAndMovingBack,
            ));
        }

        if let Ok(hook) = moving_back_query.get(event.entity) {
            commands
                .entity(event.entity)
                .remove::<(HookCaughtAndMovingBack, MoveToTarget)>();

            let target_progress = progress_query.get_mut(event.target).unwrap().clone();
            let mut my_progress = progress_query.get_mut(event.entity).unwrap();
            *my_progress = target_progress;
        }
    }
}
