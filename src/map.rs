use crate::game_state::{DespawnMe, MAP_DATA, MAPS, RaceConfig};
use crate::item_pickup::ItemPickup;
use crate::slow::Slow;
use crate::waypoint::Waypoint;
use avian2d::math::Vector;
use avian2d::prelude::{Collider, Position, RigidBody, Rotation, VhacdParameters};
use bevy::asset::io::embedded::EmbeddedAssetRegistry;
use bevy::prelude::*;
use bevy::render::camera::ScalingMode;
use itertools::Itertools;
use lyon::tessellation::geometry_builder::{Positions, SimpleBuffersBuilder};
use lyon::tessellation::{BuffersBuilder, FillOptions, FillVertex, VertexBuffers};
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
    race_config: Res<RaceConfig>,
) {
    // let texture_handle = asset_server.load("map.webp");
    // let transform = Transform::from_scale(Vec3::splat(1.0 / 20.0));
    // commands.spawn(SpriteBundle {
    //     texture: texture_handle,
    //     transform,
    //     ..Default::default()
    // });

    let map_idx = MAPS
        .iter()
        .position(|map| map == &race_config.map)
        .unwrap_or(0);

    let map = MAP_DATA[map_idx];

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
                    // let data = data_url::DataUrl::process(href).unwrap();
                    // let (vec, meta) = data.decode_to_vec().unwrap();
                    //
                    // assets.insert_asset(
                    //     "embedded_map.jpeg".into(),
                    //     &Path::new("embedded_map.jpeg"),
                    //     vec,
                    // );

                    let map_image = asset_server.load(format!("maps/{}", href));

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

                    commands.spawn((
                        SpriteBundle {
                            texture: map_image,
                            transform: map_transform,
                            sprite: Sprite {
                                custom_size: Some(Vec2::new(width, height)),
                                ..Default::default()
                            },
                            ..Default::default()
                        },
                        AudioBundle {
                            source: asset_server.load(format!("music/{}.mp3", race_config.map)),
                            settings: PlaybackSettings::LOOP,
                        },
                        DespawnMe,
                    ));

                    let camera_scale = 0.013;
                    let mut camera = Camera2dBundle {
                        transform: center,
                        ..Default::default()
                    };

                    camera.projection.scaling_mode = ScalingMode::FixedHorizontal(20.0);

                    // camera.projection.scaling_mode = ScalingMode::AutoMin {
                    //     min_width: width * camera_scale,
                    //     min_height: height * camera_scale,
                    // };
                    // camera.projection.scaling_mode;
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

                let mut first = None;
                let mut prev = None;

                if attrs.get("id").map(Deref::deref) == Some("track") {
                    for (idx, (x, y)) in points.iter().enumerate().rev() {
                        let mut entity = commands.spawn((
                            Name::new(format!("Waypoint {}", idx)),
                            Waypoint {
                                next: prev,
                                index: idx,
                            },
                            TransformBundle {
                                local: Transform::from_translation(Vec3::new(*x, *y, 0.0)),
                                ..Default::default()
                            },
                            VisibilityBundle::default(),
                            DespawnMe,
                        ));

                        if first.is_none() {
                            first = Some(entity.id());
                        }

                        prev = Some(entity.id());
                    }
                }

                if let Some(first) = first {
                    commands.entity(first).insert(Waypoint {
                        next: prev,
                        index: points.len(),
                    });
                }

                let class = attrs.get("class").map(Deref::deref);
                let classes = class
                    .map(|s| s.split(' ').collect::<Vec<_>>())
                    .unwrap_or_default();

                let mut buffers = VertexBuffers::new();

                let mut builder = SimpleBuffersBuilder::new(&mut buffers, Positions);

                let mut tessellator = lyon::tessellation::FillTessellator::new();
                let fill_options = FillOptions::default();
                let mut builder = tessellator.builder(&fill_options, &mut builder);
                points
                    .iter()
                    .map(|(x, y)| lyon::math::Point::new(*x, *y))
                    .enumerate()
                    .for_each(|(i, p)| {
                        if i == 0 {
                            builder.begin(p);
                        } else {
                            builder.line_to(p);
                        }
                    });

                builder.end(true);
                builder.build().unwrap();

                buffers.indices.iter().tuples().for_each(|(i, j, k)| {
                    // Triangle order is important!: https://github.com/Jondolf/avian/issues/368
                    let collider = Collider::triangle(
                        Vector::new(
                            buffers.vertices[*i as usize].x,
                            buffers.vertices[*i as usize].y,
                        ),
                        Vector::new(
                            buffers.vertices[*k as usize].x,
                            buffers.vertices[*k as usize].y,
                        ),
                        Vector::new(
                            buffers.vertices[*j as usize].x,
                            buffers.vertices[*j as usize].y,
                        ),
                    );

                    if classes.contains(&"collider") {
                        commands.spawn((RigidBody::Static, collider, DespawnMe));
                    } else if classes.contains(&"slow") {
                        commands.spawn((collider, Slow, DespawnMe));
                    }
                });
            }
            Event::Tag(tag::Circle, _, attrs) => {
                let cx = attrs.get("cx").unwrap().parse().unwrap();
                let cy = -attrs.get("cy").unwrap().parse::<f32>().unwrap();

                let collider = Collider::circle(0.5);

                if attrs.get("class").map(Deref::deref) == Some("pickup") {
                    let aspect = 782.0 / 868.0;

                    commands.spawn((
                        SpriteBundle {
                            transform: Transform::from_translation(Vec3::new(cx, cy, 0.0)),
                            sprite: Sprite {
                                custom_size: Some(Vec2::new(1.0, 1.0 / aspect)),
                                ..Default::default()
                            },
                            ..Default::default()
                        },
                        collider,
                        ItemPickup::default(),
                        DespawnMe,
                    ));
                }
            }
            _ => {}
        }
    }
}
