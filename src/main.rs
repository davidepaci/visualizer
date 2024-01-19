// custom bevy stuff
mod entities;
mod systems;
mod resources;
mod components;

// robotics lib stuff
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

// bevy engine
use bevy::{
    diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
    prelude::*,
};
use bevy_ecs_tilemap::prelude::*;
use rand::{thread_rng, Rng};
use bevy::{input::Input, math::Vec3, prelude::*, render::camera::Camera};

// static mut blasphemy
use lazy_static::lazy_static;
use std::sync::Mutex;

// custom bevy stuff pt. 2
use entities::VisualizerRobot;
use components::LastUpdate;
use resources::MapInfo;
use systems::camera_movement;
use systems::startup;
use systems::update_robot_position;
use systems::update_tilemap;
/*
visualizer
advanced programming course 23-24
university of trento

https://github.com/davidepaci
*/

pub static TILE_PIXEL_SIZE: f32 = 64.0;
pub static TILE_PIXEL_OFFSET: f32 = 10.0;

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
        .insert_resource(MapInfo { size: 0, last_known_robot_position: (0,0), robot_moving: false})
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