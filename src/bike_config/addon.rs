use bevy::audio::{AudioBundle, Volume};
use crate::addons::giraffe::Giraffe;
use crate::addons::hook::HookAddon;
use crate::addons::lasso::LassoAddon;
use crate::addons::rocket::RocketAddon;
use crate::bike_config::{BicycleModTrait, FRAME_OFFSET};
use bevy::ecs::system::EntityCommands;
use bevy::math::Vec2;
use bevy::prelude::{AssetServer, Component, PlaybackSettings, Res};
use enum_iterator::Sequence;
use rand_derive2::RandGen;

#[derive(Debug, Component)]
pub struct AddonComponent;

#[derive(Debug, Clone, Default, Sequence, RandGen, PartialEq)]
pub enum Addon {
    #[default]
    None,
    Giraffe,
    Hook,
    Lasso,
    Rocket,
}

impl BicycleModTrait for Addon {
    fn name(&self) -> &'static str {
        match self {
            Addon::None => "None",
            Addon::Giraffe => "Giraffe",
            Addon::Hook => "Hook",
            Addon::Lasso => "Lasso",
            Addon::Rocket => "Rocket",
        }
    }

    fn asset_folder(&self) -> &'static str {
        "addons"
    }

    fn has_asset(&self) -> bool {
        self != &Addon::None
    }

    fn asset_res(&self, menu: bool) -> Vec2 {
        Vec2::new(728.0, 868.0)
    }

    fn asset_offset(&self, menu: bool) -> Vec2 {
        Vec2::new(-1.5, -2.9) / 3.0 + FRAME_OFFSET
    }

    fn z_order(&self) -> f32 {
        30.0
    }

    fn spawn(&self, commands: &mut EntityCommands, assets: &Res<AssetServer>) {
        commands.insert(AddonComponent);
        match self {
            Addon::None => {}
            Addon::Giraffe => {
                commands.insert(Giraffe::default());
            }
            Addon::Hook => {
                commands.insert(HookAddon::default());
            }
            Addon::Lasso => {
                commands.insert(LassoAddon::default());
            }
            Addon::Rocket => {
                commands.insert((
                    RocketAddon::default(),
                    AudioBundle {
                        source: assets.load("sounds/rocket.mp3"),
                        settings: PlaybackSettings::REMOVE.with_volume(Volume::new(0.5)),
                    },
                ));
            }
        }
    }
}
