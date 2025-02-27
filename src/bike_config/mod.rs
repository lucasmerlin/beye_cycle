pub mod addon;
pub mod frame;

use crate::bike_config::addon::Addon;
use crate::bike_config::frame::BikeFrame;
use bevy::ecs::system::EntityCommands;
use bevy::math::Vec2;
use bevy::prelude::{AssetServer, Component, Entity, Event, Res, Resource};
use enum_iterator::{all, Sequence};
use rand::prelude::IteratorRandom;
use rand::thread_rng;
use rand_derive2::RandGen;

pub const FRAME_OFFSET: Vec2 = Vec2::new(0.0, 0.0);

#[derive(Debug, Resource, Default, RandGen)]
pub struct PlayerConfig(pub CharacterConfig);

#[derive(Debug, Event)]
pub struct PlayerConfigChangedEvent;

#[derive(Debug, Component)]
pub enum BicycleMod {
    Skin,
    Hat,
    RearWheel,
    Frame,
    Addon,
}

#[derive(Debug, Component)]
pub struct ForBicycle(pub Entity);

pub trait BicycleModTrait {
    fn name(&self) -> &'static str;
    fn asset_folder(&self) -> &'static str;
    fn has_asset(&self) -> bool {
        true
    }
    fn asset(&self, menu: bool) -> Option<String> {
        if self.has_asset() {
            Some(format!("{}/{}.webp", self.asset_folder(), self.name()))
        } else {
            None
        }
    }
    fn bg_asset(&self, menu: bool) -> Option<String> {
        if self.has_asset() {
            Some(format!("{}/{} White.webp", self.asset_folder(), self.name()))
        } else {
            None
        }
    }
    fn asset_res(&self, menu: bool) -> Vec2;
    fn asset_offset(&self, menu: bool) -> Vec2;
    fn z_order(&self) -> f32;

    fn spawn(&self, commands: &mut EntityCommands,  assets: &Res<AssetServer>) {}
}

pub trait Selectable: Sequence {
    fn next(&mut self);
    fn prev(&mut self);
    fn rand(&mut self);
}

impl<T> Selectable for T
where
    T: Sequence + PartialEq + Clone,
{
    fn next(&mut self) {
        *self = Sequence::next(self).unwrap_or(Sequence::first().unwrap());
    }

    fn prev(&mut self) {
        *self = Sequence::previous(self).unwrap_or(Sequence::last().unwrap());
    }

    fn rand(&mut self) {
        *self = all()
            .filter(|i| i != self)
            .choose(&mut thread_rng())
            .unwrap()
    }
}

#[derive(Debug, Clone, Default, RandGen)]
pub struct CharacterConfig {
    pub skin: Skin,
    pub hat: Hat,
    pub bike: BikeConfig,
}

#[derive(Debug, Clone, Default, Sequence, RandGen, PartialEq)]
pub enum Skin {
    #[default]
    Bob,
    Weyefu,
    Weyetleye,
    Fleye,
    Snek,
    EyeT,
}

impl BicycleModTrait for Skin {
    fn name(&self) -> &'static str {
        match self {
            Skin::Bob => "Bob",
            Skin::Weyefu => "Weyefu",
            Skin::Weyetleye => "Weyetleye",
            Skin::Fleye => "Fleye",
            Skin::Snek => "Snek",
            Skin::EyeT => "Eye-T",
        }
    }

    fn asset(&self, menu: bool) -> Option<String> {
        if menu {
            Some(format!("{}/{}.webp", self.asset_folder(), self.name()))
        } else {
            Some(format!("{}/{} Side.webp", self.asset_folder(), self.name()))
        }
    }

    fn bg_asset(&self, menu: bool) -> Option<String> {
        if menu {
            Some(format!("{}/{} White.webp", self.asset_folder(), self.name()))
        } else {
            Some(format!("{}/{} Side White.webp", self.asset_folder(), self.name()))
        }
    }

    fn asset_folder(&self) -> &'static str {
        "skins"
    }

    fn asset_res(&self, menu: bool) -> Vec2 {
        if menu {
            Vec2::new(1200.0, 1800.0)
        } else {
            Vec2::new(861.0, 1151.0)
        }
    }

    fn asset_offset(&self, menu: bool) -> Vec2 {
        if menu {
            Vec2::new(-0.9, 2.0)
        } else {
            Vec2::new(-1.8, -0.9) / 3.0 + FRAME_OFFSET
        }
    }

    fn z_order(&self) -> f32 {
        20.0
    }
}

#[derive(Debug, Clone, Default, Sequence, RandGen, PartialEq)]
pub enum Hat {
    #[default]
    None,
    Propeller,
    Crown,
    TopHat,
    PaperBag,
}

impl BicycleModTrait for Hat {
    fn name(&self) -> &'static str {
        match self {
            Hat::None => "None",
            Hat::Propeller => "Propeller",
            Hat::Crown => "Crown",
            Hat::TopHat => "Top Hat",
            Hat::PaperBag => "Paper Bag",
        }
    }

    fn asset_folder(&self) -> &'static str {
        "hats"
    }

    fn asset(&self, menu: bool) -> Option<String> {
        if !self.has_asset() {
            return None;
        }
        if menu {
            Some(format!("{}/{}.webp", self.asset_folder(), self.name()))
        } else {
            Some(format!("{}/{} Side.webp", self.asset_folder(), self.name()))
        }
    }

    fn bg_asset(&self, menu: bool) -> Option<String> {
        if !self.has_asset() {
            return None;
        }
        if menu {
            Some(format!("{}/{} White.webp", self.asset_folder(), self.name()))
        } else {
            Some(format!("{}/{} Side White.webp", self.asset_folder(), self.name()))
        }
    }

    fn has_asset(&self) -> bool {
        self != &Hat::None
    }

    fn asset_res(&self, menu: bool) -> Vec2 {
        if menu {
            Vec2::new(1143.0, 1365.0)
        } else {
            Vec2::new(1077.0, 1175.0)
        }
    }

    fn asset_offset(&self, menu: bool) -> Vec2 {
        if menu {
            Vec2::new(-1.5, 1.95)
        } else {
            Vec2::new(-1.1, -0.4)
        }
    }

    fn z_order(&self) -> f32 {
        30.0
    }
}

#[derive(Debug, Clone, Default, RandGen)]
pub struct BikeConfig {
    pub rear_wheel: RearWheel,
    pub frame: BikeFrame,
    pub addon: Addon,
}

#[derive(Debug, Clone, Default, Sequence, RandGen, PartialEq)]
pub enum RearWheel {
    #[default]
    Motorcycle,
    Skateboard,
    Tank,
    Icecube,
}

impl BicycleModTrait for RearWheel {
    fn name(&self) -> &'static str {
        match self {
            RearWheel::Motorcycle => "Motorcycle",
            RearWheel::Skateboard => "Skateboard",
            RearWheel::Tank => "Tank",
            RearWheel::Icecube => "Icecube",
        }
    }

    fn asset_folder(&self) -> &'static str {
        "rear_wheels"
    }

    fn asset_res(&self, menu: bool) -> Vec2 {
        Vec2::new(565.0, 515.0)
    }

    fn asset_offset(&self, menu: bool) -> Vec2 {
        Vec2::new(0.7, -1.9) / 3.0 + FRAME_OFFSET
    }

    fn z_order(&self) -> f32 {
        5.0
    }
}
