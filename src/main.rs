/*
visualizer
advanced programming course 23-24
university of trento

https://github.com/davidepaci
*/

// custom bevy stuff
mod entities;
mod systems;
mod resources;
mod components;
mod wrapper;

// robotics lib stuff
use crate::resources::GameTimer;
use crate::systems::{run_tick, setup_minimap};
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
use oxagaudiotool::sound_config::OxAgSoundConfig;
use worldgen_unwrap::public::WorldgeneratorUnwrap;

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
use systems::follow_robot_camera;
use systems::game_prestartup;

pub static TILE_PIXEL_SIZE: f32 = 64.0;
pub static TILE_PIXEL_OFFSET: f32 = 10.0;

lazy_static! {
    pub static ref VISUALIZER_MAP: Mutex<Option<Vec<Vec<Option<Tile>>>>> = Mutex::new(None);
    pub static ref VISUALIZER_ROBOT_POSITION: Mutex<(usize,usize)> = Mutex::new((0,0));
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
        .add_systems(PreStartup, game_prestartup)
        .add_systems(Startup, startup)
        //.add_systems(Startup, setup_minimap)
        .add_systems(Update, update_tilemap)
        .add_systems(Update, run_tick)
        .add_systems(Update, update_robot_position)
        .add_systems(Update, follow_robot_camera)
        .run();
}

fn main() {
    /*let robot = MyRobot(Robot::new());
    let mut generator = WorldGenerator::init(100);
    let run = Runner::new(Box::new(robot), &mut generator);
    match run {
        | Ok(mut r) => {
            let _ = r.game_tick();
            visualizer();
        }
        | Err(e) => println!("{:?}", e),
    }*/
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
        .insert_resource(GameTimer(Timer::from_seconds(
            2.0,
            TimerMode::Repeating,
        )))
        .insert_resource(MapInfo { size: 0, last_known_robot_position: (0,0), robot_moving: false})
        .add_systems(PreStartup, game_prestartup)
        .add_systems(Startup, startup)
        .add_systems(FixedUpdate, run_tick)
        //.add_systems(Startup, setup_minimap)
        .add_systems(Update, update_tilemap)
        .add_systems(Update, update_robot_position)
        .add_systems(Update, follow_robot_camera)
        .run();
}