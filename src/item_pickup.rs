use crate::bike::{spawn_selectable, Bicycle, ModContainer};
use crate::bike_config::addon::{Addon, AddonComponent};
use crate::bike_config::{BicycleMod, BicycleModTrait};
use avian2d::prelude::Collision;
use bevy::prelude::*;
use enum_iterator::all;
use rand::prelude::IteratorRandom;
use rand::{random, thread_rng};

pub struct ItemPickupPlugin;

impl Plugin for ItemPickupPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (item_collision_system, respawn_item_system));
    }
}

#[derive(Component, Debug)]
pub struct ItemPickup {
    pub item: Option<Addon>,
    pub timer: Timer,
}

impl Default for ItemPickup {
    fn default() -> Self {
        let mut timer = Timer::from_seconds(1.0, TimerMode::Once);
        Self { item: None, timer }
    }
}

pub fn item_collision_system(
    mut commands: Commands,
    mut collisions: EventReader<Collision>,
    mut pickup_query: Query<(Entity, &mut ItemPickup, &mut Visibility)>,
    mut bicycle_query: Query<
        (Entity, &Children),
        (Without<ItemPickup>, With<Bicycle>, Without<ModContainer>),
    >,
    mut addon_container_query: Query<&Children, With<ModContainer>>,
    has_addon_query: Query<&AddonComponent>,
    assets: Res<AssetServer>,
) {
    for Collision(contact) in collisions.read() {
        if let Some((entity, mut pickup, mut visibility)) = pickup_query
            .iter_many_mut([contact.entity1, contact.entity2])
            .fetch_next()
        {
            if let Some((bicycle_entity, bicycle_children)) = bicycle_query
                .iter_many_mut([contact.entity1, contact.entity2])
                .fetch_next()
            {
                if pickup.item.is_none() {
                    continue;
                }

                // The pickup still gets hidden even if the player already has an addon
                let item = pickup.item.take();
                *visibility = Visibility::Hidden;
                pickup.timer.reset();

                let container = bicycle_children.first().unwrap();

                let (container_children) = addon_container_query.get(*container).unwrap();

                // Check if the container already has an addon
                if has_addon_query.iter_many(container_children.iter()).count() > 0 {
                    continue;
                }

                if let Some(item) = item {
                    spawn_selectable(
                        bicycle_entity,
                        &mut commands.entity(*container),
                        &item,
                        &assets,
                        BicycleMod::Addon,
                    );
                }
            }
        }
    }
}

pub fn respawn_item_system(
    time: Res<Time>,
    mut pickup_query: Query<(&mut ItemPickup, &mut Visibility, &mut Handle<Image>)>,
    assets: Res<AssetServer>,
) {
    for ((mut pickup, mut visibility, mut image)) in pickup_query.iter_mut() {
        if pickup.timer.tick(time.delta()).just_finished() {
            let item: Addon = all::<Addon>()
                .filter(|a| a != &Addon::None)
                .choose(&mut thread_rng())
                .unwrap();
            *image = assets.load(format!("addons/{} Bubble.png", item.name()));
            pickup.item = Some(item);

            *visibility = Visibility::Visible;
        }
    }
}
