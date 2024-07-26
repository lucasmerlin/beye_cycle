use bevy::prelude::*;

pub fn spawn_map_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let texture_handle = asset_server.load("map.png");
    let transform = Transform::from_scale(Vec3::splat(1.0 / 20.0));
    commands.spawn(SpriteBundle {
        texture: texture_handle,
        transform,
        ..Default::default()
    });
}