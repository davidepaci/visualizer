use crate::entities::Minimap;
use bevy::math::UVec2;
use bevy::ecs::query::Without;
use bevy::asset::{AssetServer, Handle};
use bevy::input::Input;
use bevy::math::Vec3;
use bevy::prelude::{Camera, Camera2dBundle, Commands, default, Image, KeyCode, Query, Res, ResMut, SpriteBundle, Time, Transform, With};
use crate::resources::MapInfo;
use crate::components::LastUpdate;
use bevy_ecs_tilemap::map::{TilemapId, TilemapSize, TilemapTexture, TilemapTileSize, TilemapType};
use bevy_ecs_tilemap::prelude::{TileBundle, TilePos, TileStorage, TileTextureIndex};
use bevy_ecs_tilemap::TilemapBundle;
use bevy::render::camera::OrthographicProjection;
use rand::{Rng, thread_rng};
use robotics_lib::world::tile::TileType;
use crate::{TILE_PIXEL_OFFSET, TILE_PIXEL_SIZE, VISUALIZER_MAP, VISUALIZER_ROBOT_POSITION};
use crate::entities::VisualizerRobot;
use bevy::render::camera::Viewport;

pub fn startup(mut commands: Commands, asset_server: Res<AssetServer>, mut map_info: ResMut<MapInfo>) {
    // get visualizer map
    let data = VISUALIZER_MAP.lock().unwrap();
    if let Some(rows) = &*data {
        println!("num_rows {}", rows.len());
        println!("num_rows {}", rows.len());
        println!("num_rows {}", rows.len());
        println!("num_rows {}", rows.len());
        println!("num_rows {}", rows.len());
        // save size in resource
        map_info.size = rows.len() as u32;
    }

    commands.spawn(Camera2dBundle::default());

    let texture_handle: Handle<Image> = asset_server.load("tiles_robotic_lib.png");

    let map_size = TilemapSize { x: map_info.size, y: map_info.size };
    let mut tile_storage = TileStorage::empty(map_size);
    let tilemap_entity = commands.spawn_empty().id();

    for y in (0..map_info.size).rev() {
        for x in 0..map_info.size {
            let tile_pos = TilePos { x, y };
            let tile_entity = commands
                .spawn((
                    TileBundle {
                        position: tile_pos,
                        tilemap_id: TilemapId(tilemap_entity),
                        ..Default::default()
                    },
                    LastUpdate::default(),
                ))
                .id();
            tile_storage.set(&tile_pos, tile_entity);
        }
    }

    let tile_size = TilemapTileSize { x: TILE_PIXEL_SIZE, y: TILE_PIXEL_SIZE };
    let grid_size = tile_size.into();
    let map_type = TilemapType::default();

    // spawn tilemap
    commands.entity(tilemap_entity).insert(TilemapBundle {
        grid_size,
        map_type,
        size: map_size,
        storage: tile_storage,
        texture: TilemapTexture::Single(texture_handle),
        tile_size,
        //transform: get_tilemap_center_transform(&map_size, &grid_size, &map_type, 0.0),
        transform: Transform::from_translation(Vec3::new(0.0,0.0,0.0)),
        ..Default::default()
    });

    // spawn robot
    commands.spawn(
        (
            VisualizerRobot,
            SpriteBundle {
                texture: asset_server.load("robot_64x64.png"),
                //transform: Transform::from_translation(Vec3::new(-288.0,295.0,10.0)),
                transform: Transform::from_translation(Vec3::new(0.0,74.0,10.0)),
                ..default()
            })
    );
}

pub fn setup_minimap(mut commands: Commands) {
    commands.spawn((
        Camera2dBundle {
            camera: Camera {
                // renders after / on top of other cameras
                order: 2,
                // set the viewport to a 256x256 square in the top left corner
                viewport: Some(Viewport {
                    physical_position: UVec2::new(0, 0),
                    physical_size: UVec2::new(256, 256),
                    ..default()
                }),
                ..default()
            },
            ..default()
        },
        Minimap,
    ));
}

// func to change sprite position based on actual robot position
pub fn update_robot_position(mut query: Query<(&VisualizerRobot, &mut Transform)>, mut map_info: ResMut<MapInfo>) {
    // get robot position
    let data = VISUALIZER_ROBOT_POSITION.lock().unwrap();
    // check if robot position is different. if it is, then update on screen
    if (map_info.last_known_robot_position != *data) {
        // change robot position in gui
        // robot's x = gui's y and viceversa
        let (player, mut transform) = query.single_mut();
        println!("map size: {} - tile_pixel_size {} - x {} - y {} - tile_pixel_offse {}", map_info.size, TILE_PIXEL_SIZE, data.0, data.1, TILE_PIXEL_OFFSET);
        transform.translation.x = (TILE_PIXEL_SIZE * (data.1 as f32)) + 5.0;
        transform.translation.y = (map_info.size as f32 * TILE_PIXEL_SIZE) - (TILE_PIXEL_SIZE * (data.0 as f32 + 1.0)) + TILE_PIXEL_OFFSET;
        println!("{} {}", transform.translation.x, transform.translation.y);
        println!("{:?}", data);
        // save new robot position
        map_info.last_known_robot_position = *data;
    }
}

// In this example it's better not to use the default `MapQuery` SystemParam as
// it's faster to do it this way:
fn random(time: ResMut<Time>, mut query: Query<(&mut TileTextureIndex, &mut LastUpdate)>) {
    let current_time = time.elapsed_seconds_f64();
    let mut random = thread_rng();
    for (mut tile, mut last_update) in query.iter_mut() {
        if (current_time - last_update.value) > 0.2 {
            tile.0 = random.gen_range(0..6);
            last_update.value = current_time;
        }
    }
}

// update tiles
pub fn update_tilemap(time: ResMut<Time>, mut query: Query<(&mut TileTextureIndex, &mut LastUpdate)>) {
    // get visualizer map
    let data = VISUALIZER_MAP.lock().unwrap();
    // flatten it
    let flattened = data.clone().unwrap().concat();
    println!("{:?}", flattened);
    let current_time = time.elapsed_seconds_f64();
    for (index, (mut tile, mut last_update)) in query.iter_mut().enumerate() {
        if (current_time - last_update.value) > 1.0 {
            //let data = VISUALIZER_MAP.lock().unwrap();
            //println!("Global variable value: {:?}", *data);
            if let Some(flattened_tile) = flattened.get(index) {
                if let Some(tile_ref) = flattened_tile.as_ref() {
                    match tile_ref.tile_type {
                        TileType::Grass => tile.0 = 0,
                        TileType::Sand => tile.0 = 1,
                        TileType::Snow => tile.0 = 2,
                        TileType::Mountain => tile.0 = 3,
                        TileType::Teleport(false) => tile.0 = 4,
                        TileType::Wall => tile.0 = 5,
                        TileType::Hill => tile.0 = 6,
                        TileType::Street => tile.0 = 7,
                        TileType::Lava => tile.0 = 8,
                        TileType::DeepWater => tile.0 = 9,
                        TileType::ShallowWater => tile.0 = 10,
                        _ => tile.0 = 5,
                    }
                } else {
                    tile.0 = 5
                }
            }
            println!("{:?}", tile);
            last_update.value = current_time;
        }
    }
}

fn update_tilemap_debug(time: ResMut<Time>, mut query: Query<(&mut TileTextureIndex, &mut LastUpdate)>) {
    let current_time = time.elapsed_seconds_f64();
    let mut i = 0;
    for (index, (mut tile, mut last_update)) in query.iter_mut().enumerate() {
        if (current_time - last_update.value) > 1.0 {
            //let data = VISUALIZER_MAP.lock().unwrap();
            //println!("Global variable value: {:?}", *data);
            tile.0 = i;
            i += 1;
            if i > 5 { i = 0};
            println!("{:?}", tile);
            last_update.value = current_time;
        }
    }
}

pub fn follow_robot_camera(
    mut query: Query<(&mut Transform, &mut OrthographicProjection), With<Camera>>,
    mut query_robot: Query<&Transform, (With<VisualizerRobot>, Without<Camera>)>) {
    let robot_transform = query_robot.single();
    let (mut camera_transform, mut camera_ortho) = query.single_mut();

    camera_transform.translation.x = robot_transform.translation.x;
    camera_transform.translation.y = robot_transform.translation.y;
    camera_ortho.scale = 0.6;
}

pub fn camera_movement(
    time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&mut Transform, &mut OrthographicProjection), With<Camera>>,
) {
    for (mut transform, mut ortho) in query.iter_mut() {
        let mut direction = Vec3::ZERO;

        if keyboard_input.pressed(KeyCode::A) {
            direction -= Vec3::new(1.0, 0.0, 0.0);
        }

        if keyboard_input.pressed(KeyCode::D) {
            direction += Vec3::new(1.0, 0.0, 0.0);
        }

        if keyboard_input.pressed(KeyCode::W) {
            direction += Vec3::new(0.0, 1.0, 0.0);
        }

        if keyboard_input.pressed(KeyCode::S) {
            direction -= Vec3::new(0.0, 1.0, 0.0);
        }

        if keyboard_input.pressed(KeyCode::Z) {
            ortho.scale += 0.1;
        }

        if keyboard_input.pressed(KeyCode::X) {
            ortho.scale -= 0.1;
        }

        /*if ortho.scale < 0.5 {
            ortho.scale = 0.5;
        }*/

        let z = transform.translation.z;
        transform.translation += time.delta_seconds() * direction * 500.;
        // Important! We need to restore the Z values when moving the camera around.
        // Bevy has a specific camera setup and this can mess with how our layers are shown.
        transform.translation.z = z;
    }
}