use crate::bike_config::BicycleModTrait;
use bevy::math::Vec2;
use enum_iterator::Sequence;
use rand_derive2::RandGen;

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

    fn asset_res(&self) -> Vec2 {
        Vec2::new(728.0, 868.0)
    }

    fn asset_offset(&self) -> Vec2 {
        Vec2::new(-1.5, -2.9) / 3.0
    }

    fn z_order(&self) -> f32 {
        30.0
    }
}
