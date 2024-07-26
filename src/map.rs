use crate::waypoint::Waypoint;
use avian2d::math::Vector;
use avian2d::prelude::{Collider, RigidBody};
use bevy::asset::io::embedded::EmbeddedAssetRegistry;
use bevy::prelude::*;
use bevy::render::camera::ScalingMode;
use itertools::Itertools;
use std::ops::Deref;
use std::path::{Path, PathBuf};
use svg::node::element::tag;
use svg::node::element::tag::Type;
use svg::node::Value;
use svg::parser::Event;

pub fn spawn_map_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    assets: ResMut<EmbeddedAssetRegistry>,
) {
    // let texture_handle = asset_server.load("map.png");
    // let transform = Transform::from_scale(Vec3::splat(1.0 / 20.0));
    // commands.spawn(SpriteBundle {
    //     texture: texture_handle,
    //     transform,
    //     ..Default::default()
    // });

    let map = include_str!("../assets/level1.svg");
    let svg = svg::read(map).unwrap();

    //let mut view_box = None;

    for item in svg {
        match item {
            Event::Tag(tag::SVG, _, attrs) => {
                // view_box = attrs.get("viewBox").map(Deref::deref).map(|s| {
                //     let mut parts = s.split(' ').map(|s| s.parse().unwrap());
                //     let x = parts.next().unwrap();
                //     let y = parts.next().unwrap();
                //     let width = parts.next().unwrap();
                //     let height = parts.next().unwrap();
                //     (x, y, width, height)
                // });
            }
            Event::Tag(tag::Image, _, attrs) => {
                if let Some(href) = attrs.get("xlink:href") {
                    println!("Loading image: {}", href);
                    let data = data_url::DataUrl::process(href).unwrap();
                    let (vec, meta) = data.decode_to_vec().unwrap();

                    assets.insert_asset("map".into(), &Path::new("map"), vec);

                    let map = asset_server.load("map.png");

                    let width = attrs.get("width").unwrap().parse().unwrap();
                    let height = attrs.get("height").unwrap().parse::<f32>().unwrap();

                    let center = Transform::from_translation(Vec3::new(
                        width / 2.0,
                        // Invert height because the SVG coordinate system is flipped in comparison to Bevy's
                        -height / 2.0,
                        0.0,
                    ));

                    let mut map_transform = center.clone();
                    map_transform.translation.z = -1.0;

                    commands.spawn(SpriteBundle {
                        texture: map,
                        transform: map_transform,
                        sprite: Sprite {
                            custom_size: Some(Vec2::new(width, height)),
                            ..Default::default()
                        },
                        ..Default::default()
                    });

                    let mut camera = Camera2dBundle {
                        transform: center,
                        ..Default::default()
                    };
                    camera.projection.scaling_mode = ScalingMode::AutoMin {
                        min_width: width,
                        min_height: height,
                    };
                    commands.spawn((camera,));
                }
            }
            Event::Tag(tag::Polygon, _, attrs) => {
                let points = attrs.get("points").unwrap();
                let points: Vec<(f32, f32)> = points
                    .split(' ')
                    .tuples()
                    .map(|(x, y)| {
                        (
                            x.parse().unwrap(),
                            // Because the SVG coordinate system is flipped in comparison to Bevy's
                            -y.parse::<f32>().unwrap(),
                        )
                    })
                    .collect();

                let mut prev = None;

                if attrs.get("id").map(Deref::deref) == Some("track") {
                    for (x, y) in points.iter().rev() {
                        let mut entity = commands.spawn((
                            Waypoint { next: prev },
                            TransformBundle {
                                local: Transform::from_translation(Vec3::new(*x, *y, 0.0)),
                                ..Default::default()
                            },
                            VisibilityBundle::default(),
                        ));

                        entity.with_children(|commands| {
                            commands.spawn((SpriteBundle {
                                texture: asset_server.load("ducky.png"),
                                transform: Transform::from_scale(Vec3::splat(0.01)),
                                ..Default::default()
                            },));
                        });

                        prev = Some(entity.id());
                    }
                }

                let class = attrs.get("class").map(Deref::deref);
                let classes = class
                    .map(|s| s.split(' ').collect::<Vec<_>>())
                    .unwrap_or_default();

                if classes.contains(&"collider") {
                    commands.spawn((
                        RigidBody::Static,
                        Collider::convex_hull(
                            points.iter().map(|(x, y)| Vector::new(*x, *y)).collect(),
                        ).unwrap(),
                    ));
                }
            }
            _ => {}
        }
    }
}
