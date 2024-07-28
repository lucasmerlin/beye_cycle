use bevy::prelude::{Component, Entity};

pub struct LassoPlugin;


#[derive(Debug, Component)]
pub struct LassoAddon;

#[derive(Debug, Component)]
pub struct Lasso {
    target: Entity,
}


pub fn player_lasso_control_system(

) {

}