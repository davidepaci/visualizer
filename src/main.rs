
use std::collections::HashMap;

use robotics_lib;
use robotics_lib::energy::Energy;
use robotics_lib::event::events::Event;
use robotics_lib::interface::{Direction, go, robot_map, where_am_i};
use robotics_lib::interface::Direction::{Down, Left, Right, Up};
use robotics_lib::runner::{Robot, Runnable, Runner};
use robotics_lib::runner::backpack::BackPack;
use robotics_lib::world::coordinates::Coordinate;
use robotics_lib::world::environmental_conditions::EnvironmentalConditions;
use robotics_lib::world::environmental_conditions::WeatherType::{Rainy, Sunny};
use robotics_lib::world::tile::{Content, Tile, TileType};
use robotics_lib::world::World;
use robotics_lib::world::world_generator::Generator;

use bevy::{
    diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
    prelude::*,
};
use bevy_ecs_tilemap::prelude::*;
use rand::{thread_rng, Rng};
use bevy::{input::Input, math::Vec3, prelude::*, render::camera::Camera};

use lazy_static::lazy_static;
use std::sync::Mutex;

/*
visualizer
advanced programming course 23-24
university of trento

https://github.com/davidepaci
*/

static TILE_PIXEL_SIZE: f32 = 64.0;
static TILE_PIXEL_OFFSET: f32 = 10.0;

lazy_static! {
    static ref VISUALIZER_MAP: Mutex<Option<Vec<Vec<Option<Tile>>>>> = Mutex::new(None);
    static ref VISUALIZER_ROBOT_POSITION: Mutex<(usize,usize)> = Mutex::new((0,0));
}
struct MyRobot(Robot);

struct WorldGenerator {
    size: usize,
}

impl WorldGenerator {
    fn init(size: usize) -> Self {
        WorldGenerator { size }
    }
}

impl Generator for WorldGenerator {
    fn gen(&mut self) -> (Vec<Vec<Tile>>, (usize, usize), EnvironmentalConditions, f32, Option<HashMap<Content, f32>>) {
        let mut map: Vec<Vec<Tile>> = Vec::with_capacity(self.size);
        for i in 0..10 {
            map.push(vec![]);
            for l in 0..10 {
                if (thread_rng().gen_range(0..2) == 0) {
                    map[i].push( Tile {tile_type: TileType::Grass, content: Content::Coin(1), elevation: 10});
                } else {
                    map[i].push( Tile {tile_type: TileType::Snow, content: Content::None, elevation: 10});
                }
            }
        }
        /*map[2][0].content = Content::Coin(1);
        map[6][2].content = Content::Coin(1);
        map[2][1].content = Content::Coin(1);
        map[1][1].content = Content::Coin(1);*/
        // map[2][1] = Tile{tile_type: TileType::Wall, content: Content::None, elevation: 2 * 10};
        //map[2][0].content = Content::Tree(0);

        let environmental_conditions = EnvironmentalConditions::new(&vec![Sunny, Rainy], 15, 12);
        return (map, (1, 1), environmental_conditions.unwrap(), 100.0, None);
    }
}

impl Runnable for MyRobot {
    fn process_tick(&mut self, world: &mut World) {
        //go(self, world, Left);
        //go(self, world, Up);
        go(self, world, Up);
        go(self, world, Right);
        go(self, world, Right);
        /*go(self, world, Right);
        go(self, world, Right);
        go(self, world, Right);
        go(self, world, Right);
        go(self, world, Right);
        go(self, world, Right);
        go(self, world, Right);
        go(self, world, Right);
        go(self, world, Right);
        go(self, world, Right);
        go(self, world, Right);
        go(self, world, Right);*/
        println!("{:?}", where_am_i(self, world));
        println!("{:?}", self.get_energy());
        println!("{:?}", where_am_i(self, world));

        // save map data
        let mut data = VISUALIZER_MAP.lock().unwrap();
        *data = robot_map(world);
        // save robot position data
        let mut data_position = VISUALIZER_ROBOT_POSITION.lock().unwrap();
        *data_position = where_am_i(self, world).1;
        println!("{:?}", *data);
    }

    fn handle_event(&mut self, event: Event) {
        println!();
        println!("{:?}", event);
        println!();
    }

    fn get_energy(&self) -> &Energy {
        &self.0.energy
    }
    fn get_energy_mut(&mut self) -> &mut Energy {
        &mut self.0.energy
    }

    fn get_coordinate(&self) -> &Coordinate {
        &self.0.coordinate
    }
    fn get_coordinate_mut(&mut self) -> &mut Coordinate {
        &mut self.0.coordinate
    }

    fn get_backpack(&self) -> &BackPack {
        &self.0.backpack
    }
    fn get_backpack_mut(&mut self) -> &mut BackPack {
        &mut self.0.backpack
    }
}

#[derive(Component)]
struct VisualizerRobot;

#[derive(Resource)]
struct MapInfo {
    size: u32,
}

// func to change sprite position based on actual robot position
fn update_robot_position(mut query: Query<(&VisualizerRobot, &mut Transform)>, map_info: Res<MapInfo>) {
    // get robot position
    let data = VISUALIZER_ROBOT_POSITION.lock().unwrap();
    // change robot position in gui
    // robot's x = gui's y and viceversa
    let (player, mut transform) = query.single_mut();
    println!("map size: {} - tile_pixel_size {} - x {} - y {} - tile_pixel_offse {}", map_info.size, TILE_PIXEL_SIZE, data.0, data.1, TILE_PIXEL_OFFSET);
    transform.translation.x = (TILE_PIXEL_SIZE * (data.1 as f32)) + 5.0;
    transform.translation.y = (map_info.size as f32 * TILE_PIXEL_SIZE) - (TILE_PIXEL_SIZE * (data.0 as f32 + 1.0)) + TILE_PIXEL_OFFSET;
    println!("{} {}", transform.translation.x, transform.translation.y);
    println!("{:?}", data);
}

fn startup(mut commands: Commands, asset_server: Res<AssetServer>, mut map_info: ResMut<MapInfo>) {
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

#[derive(Default, Component)]
struct LastUpdate {
    value: f64,
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
fn update_tilemap(time: ResMut<Time>, mut query: Query<(&mut TileTextureIndex, &mut LastUpdate)>) {
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

/*#[derive(Resource)]
struct RunnerResource {
    runner: Runner
}*/

fn visualizer() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: String::from("Visualizer"),
                        ..Default::default()
                    }),
                    ..default()
                })
                .set(ImagePlugin::default_nearest()),
        )
        .add_plugins(LogDiagnosticsPlugin::default())
        .add_plugins(FrameTimeDiagnosticsPlugin)
        .add_plugins(TilemapPlugin)
        .insert_resource(MapInfo { size: 0})
        .add_systems(Startup, startup)
        .add_systems(Update, update_tilemap)
        .add_systems(Update, update_robot_position)
        .add_systems(Update, camera_movement)
        .run();
}

fn main() {
    let robot = MyRobot(Robot::new());
    let mut generator = WorldGenerator::init(100);
    let run = Runner::new(Box::new(robot), &mut generator);
    match run {
        | Ok(mut r) => {
            let _ = r.game_tick();
            visualizer();
        }
        | Err(e) => println!("{:?}", e),
    }
}