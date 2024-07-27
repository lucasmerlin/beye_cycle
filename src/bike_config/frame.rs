use crate::bike::BicycleParams;
use crate::bike_config::{BicycleModTrait, Selectable};
use bevy::prelude::Vec2;
use enum_iterator::{all, Sequence};
use rand::prelude::IteratorRandom;
use rand::thread_rng;
use rand_derive2::RandGen;

#[derive(Debug, Clone, Default, Sequence, RandGen, PartialEq)]
pub enum BikeFrame {
    #[default]
    Fast,
    Princess,
    Banana,
    Flames,
}

impl BicycleModTrait for BikeFrame {
    fn name(&self) -> &'static str {
        match self {
            BikeFrame::Fast => "Fast",
            BikeFrame::Princess => "Princess",
            BikeFrame::Banana => "Banana",
            BikeFrame::Flames => "Flames",
        }
    }

    fn asset_folder(&self) -> &'static str {
        "frames"
    }

    fn asset_res(&self) -> Vec2 {
        Vec2::new(1250.0, 800.0)
    }

    fn asset_offset(&self) -> Vec2 {
        Vec2::new(0.0, 0.0)
    }

    fn z_order(&self) -> f32 {
        10.0
    }
}

impl BikeFrame {
    pub fn params(&self) -> BicycleParams {
        match self {
            BikeFrame::Fast => BicycleParams {
                max_speed: 1.0,
                acceleration: 1.0,
                turn: 1.0,
                drift: 0.5,
            },
            BikeFrame::Princess => BicycleParams {
                max_speed: 0.9,
                acceleration: 1.8,
                turn: 1.2,
                drift: 1.5,
            },
            BikeFrame::Banana => BicycleParams {
                max_speed: 0.8,
                acceleration: 1.53,
                turn: 1.0,
                drift: 1.0,
            },
            BikeFrame::Flames => BicycleParams {
                max_speed: 1.2,
                acceleration: 1.0,
                turn: 0.8,
                drift: 1.00,
            },
        }
    }
}