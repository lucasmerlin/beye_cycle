use crate::addons::giraffe::PooCollision;
use crate::addons::rocket::RocketAddon;
use crate::bike_config::{
    BicycleMod, BicycleModTrait, BikeConfig, CharacterConfig, ForBicycle, PlayerConfig, Selectable,
    FRAME_OFFSET,
};
use crate::game_state::{DespawnMe, GameState, RaceConfig};
use crate::ranking::{Progress, Rank};
use crate::slow::Slow;
use crate::waypoint::{Waypoint, WaypointAi};
use avian2d::math::Vector;
use avian2d::prelude::*;
use bevy::ecs::system::EntityCommands;
use bevy::prelude::*;
use bevy_egui::egui;
use bevy_egui::egui::{lerp, Id, Ui};
use bevy_inspector_egui::inspector_egui_impls::InspectorPrimitive;
use bevy_inspector_egui::reflect_inspector::InspectorUi;
use rand::random;
use std::any::Any;
use std::f32::consts::PI;

#[derive(Component, Debug)]
pub struct Bicycle;

#[derive(Component, Debug)]
pub struct Player;

#[derive(Component, Debug)]
pub struct ModContainer;

pub fn spawn_bikes(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    player_config: Res<PlayerConfig>,
    waypoint: Query<(Entity, &Waypoint, &Transform)>,

    children_query: Query<&Children>,
    menu: Res<State<GameState>>,
    race_config: Res<RaceConfig>,
) {
    let menu = matches!(**menu, GameState::MainMenu);

    let (first_waypoint_entity, first_waypoint, start_post) = waypoint
        .iter()
        .sort_by_key::<&Waypoint, _>(|data| data.index)
        .next()
        .unwrap();

    let (next_waypoint_entity, next_waypoint, next_waypoint_transfrom) =
        waypoint.get(first_waypoint.next.unwrap()).unwrap();

    let direction = -(next_waypoint_transfrom.translation - start_post.translation).xy();

    let mut spawn = |player: bool, offset: Vec2| {
        let (player_id, container_id) = {
            let mut entity = commands.spawn((
                Name::new(if player { "Player" } else { "Bot" }),
                Bicycle,
                BicycleControl {
                    turn: 0.0,
                    acceleration: 1.0,
                },
                Progress {
                    next_checkpoint: first_waypoint_entity,
                    round: 0,
                    checkpoint_idx: 0,
                    distance_to_next_checkpoint: 0.0,
                },
                Rank(0),
                DespawnMe,
                RigidBody::Dynamic,
                VisibilityBundle::default(),
                Collider::capsule(GAME_BICYCLE_LENGTH / 20.0, GAME_BICYCLE_LENGTH),
                // This was just the initial size I chose when first testing bike params
                Mass(0.2 * 2.0),
                ExternalForce::default(),
                TransformBundle {
                    local: start_post.clone()
                        * Transform::from_translation(Vec3::new(offset.x, offset.y, 0.0))
                        * Transform::from_rotation(Quat::from_rotation_z(
                            -direction.angle_between(-Vec2::Y),
                        )),
                    ..Default::default()
                },
                LinearDamping(10.0),
                AngularDamping(10.0),
            ));
            if player {
                entity.insert(Player);
            } else {
                entity.insert(WaypointAi);
            }

            let mut container_id = None;

            entity.with_children(|commands| {
                container_id = Some(
                    commands
                        .spawn((
                            TransformBundle {
                                local: Transform::from_translation(Vec3::new(-0.9, 0.0, 0.0)),
                                ..Default::default()
                            },
                            VisibilityBundle::default(),
                            ModContainer,
                        ))
                        .id(),
                );
            });

            (entity.id(), container_id.unwrap())
        };

        if player {
            apply_config(
                &mut commands,
                player_id,
                container_id,
                &player_config.0,
                &asset_server,
                &children_query,
                menu,
            );
        } else {
            let mut config: CharacterConfig = random();
            while config.skin == player_config.0.skin {
                config = random();
            }
            apply_config(
                &mut commands,
                player_id,
                container_id,
                &config,
                &asset_server,
                &children_query,
                menu,
            );
        }
    };

    let direction_right = direction.normalize().rotate(Vec2::from_angle(PI / 2.0));

    // Places enemies in a F1 like  grid
    for i in 0..race_config.ai_count + 1 {
        // This makes a mess but is better than bikes off the track
        let offset_i = usize::min(i, 8);
        let offset = direction.normalize() * (offset_i as f32 * 1.4)
            + direction_right * (offset_i as f32 % 2.0);
        spawn(i == 0, offset);
    }
}

pub const TEXTURE_BICYCLE_LENGTH: f32 = 1250.0;
pub const GAME_BICYCLE_LENGTH: f32 = 2.0;

pub fn apply_config_to_player(
    mut commands: Commands,
    mut query: Query<(Entity, &mut BicycleParams, &Children), With<Player>>,
    config: Res<PlayerConfig>,
    assets: Res<AssetServer>,

    children_query: Query<&Children>,
    menu: Res<State<GameState>>,
) {
    println!("apply_config_to_player");

    let player = query.iter_mut().next();
    if let Some((player, params, children)) = player {
        let container = *children.first().unwrap();

        apply_config(
            &mut commands,
            player,
            container,
            &config.0,
            &assets,
            &children_query,
            matches!(**menu, GameState::MainMenu),
        );
    }
}

pub fn apply_config(
    commands: &mut Commands,
    entity: Entity,
    container_entity: Entity,
    config: &CharacterConfig,
    assets: &Res<AssetServer>,
    children_query: &Query<&Children>,
    menu: bool,
) {
    {
        let mut entity_commands = commands.entity(entity);

        entity_commands.insert(BicycleParams::default() * config.bike.frame.params());
    }

    if let Ok(children) = children_query.get(container_entity) {
        for child in children.iter() {
            commands.entity(*child).despawn_recursive();
        }
    }

    let mut container_commands = commands.entity(container_entity);

    container_commands.clear_children();

    spawn_selectable(
        entity,
        &mut container_commands,
        &config.bike.frame,
        &assets,
        BicycleMod::Frame,
        menu,
    );

    spawn_selectable(
        entity,
        &mut container_commands,
        &config.bike.rear_wheel,
        &assets,
        BicycleMod::RearWheel,
        menu,
    );

    spawn_selectable(
        entity,
        &mut container_commands,
        &config.bike.addon,
        &assets,
        BicycleMod::Addon,
        menu,
    );

    spawn_selectable(
        entity,
        &mut container_commands,
        &config.skin,
        &assets,
        BicycleMod::Skin,
        menu,
    );

    spawn_selectable(
        entity,
        &mut container_commands,
        &config.hat,
        &assets,
        BicycleMod::Hat,
        menu,
    );
}

pub fn spawn_selectable(
    bicycle: Entity,
    commands: &mut EntityCommands,
    selectable: &impl BicycleModTrait,
    assets: &Res<AssetServer>,
    mod_type: BicycleMod,
    menu: bool,
) {
    let texture_res = selectable.asset_res(menu);
    let offset = selectable.asset_offset(menu);
    let z_order = selectable.z_order();
    let aspect = texture_res.x / texture_res.y;
    let game_length = texture_res.x / TEXTURE_BICYCLE_LENGTH * GAME_BICYCLE_LENGTH;
    let game_size = Vec2::new(game_length, game_length / aspect);

    let rotation = Quat::from_rotation_z(PI / 2.0);

    commands.with_children(|commands| {
        if let Some(asset) = selectable.asset(menu) {
            let mut entity = commands.spawn((
                SpriteBundle {
                    texture: assets.load(asset),
                    transform: Transform::from_translation(Vec3::new(offset.x, offset.y, z_order))
                        * Transform::from_rotation(rotation),
                    sprite: Sprite {
                        custom_size: Some(game_size),
                        ..Default::default()
                    },
                    ..Default::default()
                },
                mod_type,
                ForBicycle(bicycle),
            ));

            selectable.spawn(&mut entity);

            if let Some(bg) = selectable.bg_asset(menu) {
                entity.with_children(|commands| {
                    commands.spawn(
                        (SpriteBundle {
                            texture: assets.load(bg),
                            transform: Transform::from_translation(Vec3::new(
                                0.0,
                                0.0,
                                1.0 - z_order,
                            )),
                            sprite: Sprite {
                                custom_size: Some(game_size),
                                ..Default::default()
                            },
                            ..Default::default()
                        }),
                    );
                });
            }
        }
    });
}

#[derive(Component, Debug)]
pub struct BicycleControl {
    pub(crate) acceleration: f32,
    pub(crate) turn: f32,
}

#[derive(Component, Debug, Reflect)]
pub struct BicycleParams {
    pub acceleration: f32,
    pub max_speed: f32,
    pub turn: f32,
    // The lower, the driftier
    pub drift: f32,
}

pub const DEFAULT_MAX_SPEED: f32 = 5.0;

impl Default for BicycleParams {
    fn default() -> Self {
        Self {
            max_speed: DEFAULT_MAX_SPEED,
            acceleration: 25.0,
            turn: 0.02,
            drift: 1.0,
        }
    }
}

impl std::ops::Mul<BicycleParams> for BicycleParams {
    type Output = Self;

    fn mul(self, rhs: BicycleParams) -> Self::Output {
        Self {
            max_speed: self.max_speed * rhs.max_speed,
            acceleration: self.acceleration * rhs.acceleration,
            turn: self.turn * rhs.turn,
            drift: self.drift * rhs.drift,
        }
    }
}

impl InspectorPrimitive for BicycleParams {
    fn ui(&mut self, ui: &mut Ui, options: &dyn Any, id: Id, env: InspectorUi<'_, '_>) -> bool {
        ui.label("Max Speed");
        ui.add(egui::Slider::new(&mut self.max_speed, 0.0..=100.0).text("Max Speed"));

        ui.label("Acceleration");
        ui.add(egui::Slider::new(&mut self.acceleration, 0.0..=100.0).text("Acceleration"));

        ui.label("Turn");
        ui.add(egui::Slider::new(&mut self.turn, 0.0..=0.1).text("Turn"));

        ui.label("Drift");
        ui.add(egui::Slider::new(&mut self.drift, 0.0..=10.0).text("Drift"));

        false
    }

    fn ui_readonly(&self, ui: &mut Ui, options: &dyn Any, id: Id, env: InspectorUi<'_, '_>) {
        todo!()
    }
}

pub fn bike_controller_system(
    mut query: Query<(
        Entity,
        &BicycleControl,
        &BicycleParams,
        &LinearVelocity,
        &mut Transform,
        &mut ExternalForce,
        &mut LinearDamping,
        &Children,
    )>,
    children_query: Query<&Children>,
    has_rocket_query: Query<(&RocketAddon)>,
    spatial_query: SpatialQuery,
    mut slow_query: Query<(&Slow)>,
) {
    for (entity, control, params, velocity, mut transform, mut ext_force, mut damping, container) in
        query.iter_mut()
    {
        ext_force.clear();

        let container = container.iter().next().unwrap();
        let container_children = children_query.get(*container).unwrap();

        let has_rocket = container_children
            .iter()
            .any(|entity| has_rocket_query.get(*entity).is_ok());

        let intersections = spatial_query.point_intersections(
            Vector::new(transform.translation.x, transform.translation.y),
            SpatialQueryFilter::default(),
        );
        let slow = intersections
            .iter()
            .any(|entity| slow_query.get(*entity).is_ok());

        let mut max_speed = if slow {
            params.max_speed * 0.5
        } else {
            params.max_speed
        };
        let mut acceleration = if slow {
            params.acceleration * 0.5
        } else {
            params.acceleration
        };

        if has_rocket {
            max_speed *= 2.0;
            acceleration *= 2.0;
        }

        let bike_forward = (transform.rotation * Vec3::Y).xy();

        let forward_velocity = velocity.dot(bike_forward);

        let slower_than_max_speed =
            forward_velocity < max_speed && forward_velocity > -max_speed * 0.5;

        let mut control_acceleration_clamped = control.acceleration.clamp(-1.0, 1.0);

        if !slower_than_max_speed && slow {
            control_acceleration_clamped = -0.8;
        } else if !slower_than_max_speed {
            control_acceleration_clamped = 0.0;
        }

        let acceleration = control_acceleration_clamped * acceleration;
        let current_rotation = transform.rotation * Vec3::Y;
        ext_force.apply_force(Vec2::new(acceleration, 0.0).rotate(current_rotation.xy()));

        let slow_turn_factor = (forward_velocity / DEFAULT_MAX_SPEED).clamp(-1.0, 1.0);
        let turn_clamped = control.turn.clamp(-1.0, 1.0);
        let turn = turn_clamped * params.turn * slow_turn_factor;
        transform.rotate_z(turn);

        if control.acceleration == 0.0 {
            **damping = FloatExt::lerp(**damping, 3.0, 0.01);
        } else {
            damping.0 = 0.0;
        }

        if slow {
            damping.0 *= 1.2;
        }
    }
}

/// Basically kills the orthogonal velocity of the bike, as explained here: https://youtu.be/DVHcOS1E5OQ?si=UgpKyHxYqsRehCeZ&t=559
pub fn drift_factor_system(mut query: Query<(&mut LinearVelocity, &Transform, &BicycleParams)>) {
    for (mut lin_vel, transform, params) in query.iter_mut() {
        let drift = 1.0 - (0.08 * (1.0 / params.drift));

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
    mut query: Query<(&mut BicycleControl), (With<Player>, Without<PooCollision>)>,
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

// if the bike is going to the left, mirror it vertically
pub fn mirror_bike_system(
    mut query: Query<(&mut Transform, &Children), With<Bicycle>>,
    mut child_query: Query<(&mut Transform), Without<Bicycle>>,
) {
    for ((mut transform, children)) in query.iter_mut() {
        let rotation = transform.up().angle_between(Vec3::X);

        for child in children.iter() {
            if let Ok(mut child_transform) = child_query.get_mut(*child) {
                if rotation > PI / 2.0 {
                    if child_transform.scale.x > -1.0 {
                        child_transform.scale.x -= 0.1;
                        child_transform.translation.x =
                            lerp(-0.45..=0.45, (-1.0 * child_transform.scale.x) / 2.0 + 0.5);
                    }
                } else {
                    if child_transform.scale.x < 1.0 {
                        child_transform.scale.x += 0.1;
                        child_transform.translation.x =
                            lerp(0.45..=-0.45, child_transform.scale.x / 2.0 + 0.5);
                    }
                }
            }
        }
    }
}

/// Sets z index based on vertical position
pub fn apply_z_order(mut query: Query<(&GlobalTransform, &mut Transform), With<ModContainer>>) {
    let mut vec: Vec<_> = query.iter_mut().collect();

    vec.sort_by(|(a, _), (b, _)| {
        b.translation()
            .y
            .partial_cmp(&a.translation().y)
            .unwrap_or(std::cmp::Ordering::Equal)
    });

    for (i, ((_, mut transform))) in vec.into_iter().enumerate() {
        transform.translation.z = i as f32 * 50.0;
    }
}
