use crate::bike::Player;
use crate::bike_config::ForBicycle;
use crate::ranking::{Progress, Rank};
use bevy::input::keyboard::KeyboardInput;
use bevy::prelude::*;

pub struct LassoPlugin;

impl Plugin for LassoPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                player_lasso_control_system,
                fire_lasso_system,
                move_to_target_system,
                lasso_hit_system,
            ),
        )
        .add_event::<MovedToTargetEvent>()
        .add_event::<FireLassoEvent>();
    }
}

#[derive(Debug, Component, Default)]
pub struct LassoAddon;

pub const LASOO_SPEED: f32 = 0.1;

#[derive(Debug, Component)]
pub struct Lasso {
    target: Entity,
    by: Entity,
}

#[derive(Debug, Event)]
pub struct FireLassoEvent {
    by: Entity,
}

#[derive(Debug, Component)]
pub struct LassoCaughtAndMovingBack;

#[derive(Debug, Component)]
pub struct MoveToTarget {
    pub target: Entity,
    pub speed: f32,
}

#[derive(Debug, Event)]
pub struct MovedToTargetEvent {
    pub entity: Entity,
    pub target: Entity,
}

pub fn player_lasso_control_system(
    mut commands: Commands,
    mut query: Query<(&LassoAddon, &GlobalTransform, &ForBicycle)>,
    mut is_player_query: Query<&Player>,
    input: Res<ButtonInput<KeyCode>>,
    mut events: EventWriter<FireLassoEvent>,
) {
    for (lasso, transform, for_bicycle) in query.iter_mut() {
        if let Ok(_) = is_player_query.get(for_bicycle.0) {
            if input.just_pressed(KeyCode::Space) {
                events.send(FireLassoEvent { by: for_bicycle.0 });
            }
        }
    }
}

pub fn fire_lasso_system(
    mut commands: Commands,
    mut by_query: Query<(Entity, &Rank, &GlobalTransform)>,
    mut target_query: Query<(Entity, &Rank, &GlobalTransform)>,
    mut events: EventReader<FireLassoEvent>,
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
                Lasso {
                    target: target_entity,
                    by,
                },
                MoveToTarget {
                    target: target_entity,
                    speed: LASOO_SPEED,
                },
                SpriteBundle {
                    transform: Transform::from_translation(by_transform.translation()),
                    sprite: Sprite {
                        custom_size: Some(Vec2::new(1.0, 1.0)),
                        ..Default::default()
                    },
                    texture: assets.load("addons/Lasso.png"),
                    ..Default::default()
                },
            ));
        }
    }
}

pub fn move_to_target_system(
    mut query: Query<(Entity, &MoveToTarget, &mut Transform)>,
    target_query: Query<(&GlobalTransform)>,
    mut events: EventWriter<MovedToTargetEvent>,
) {
    for (entity, target, mut transform) in query.iter_mut() {
        let target_transform = target_query.get(target.target).unwrap();

        let direction = target_transform.translation() - transform.translation;
        let distance = direction.length();
        let direction = direction.normalize();

        transform.translation += direction * target.speed;

        if distance < 1.0 {
            events.send(MovedToTargetEvent { entity, target: target.target });
        }
    }
}

pub fn lasso_hit_system(
    mut commands: Commands,
    mut events: EventReader<MovedToTargetEvent>,
    lasso_query: Query<(&Lasso)>,
    moving_back_query: Query<(&LassoCaughtAndMovingBack)>,
    mut progress_query: Query<(&mut Progress)>,
) {
    for event in events.read() {
        if let Ok(lasso) = lasso_query.get(event.entity) {
            println!("Lasso hit target");
            commands.entity(event.entity).despawn_recursive();

            commands.entity(lasso.target).insert((
                MoveToTarget {
                    target: lasso.by,
                    speed: LASOO_SPEED,
                },
                LassoCaughtAndMovingBack,
            ));
        }

        if let Ok(_) = moving_back_query.get(event.entity) {
            commands
                .entity(event.entity)
                .remove::<(LassoCaughtAndMovingBack, MoveToTarget)>();

            let my_progress = progress_query.get_mut(event.target).unwrap().clone();
            let mut target_progress = progress_query.get_mut(event.entity).unwrap();
            *target_progress = my_progress;
        }
    }
}
