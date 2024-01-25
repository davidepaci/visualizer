/*
visualizer
advanced programming course 23-24
university of trento

https://github.com/davidepaci
*/

// custom bevy stuff
mod components;
mod entities;
mod events;
mod resources;
mod systems;
mod wrapper;

// robotics lib stuff
use crate::resources::GameTimer;
use crate::systems::setup_hud;
use crate::systems::update_contents;
use crate::systems::update_dnc;
use crate::systems::update_hud;
use crate::systems::{run_tick, setup_minimap};

use robotics_lib;

use robotics_lib::world::environmental_conditions::EnvironmentalConditions;
use robotics_lib::world::environmental_conditions::WeatherType::Sunny;
use robotics_lib::world::tile::Tile;

// bevy engine
use bevy::{
    diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
    prelude::*,
};

use bevy_ecs_tilemap::prelude::*;

// static mut blasphemy
use lazy_static::lazy_static;
use std::sync::Mutex;

// custom bevy stuff pt. 2
use crate::events::{CameraEvent, TickEvent};

use resources::MapInfo;

use systems::follow_robot_camera;
use systems::game_prestartup;
use systems::startup;
use systems::update_robot_position;
use systems::update_tilemap;

pub static TILE_PIXEL_SIZE: f32 = 64.0;
pub static TILE_PIXEL_OFFSET: f32 = 10.0;

lazy_static! {
    pub static ref VISUALIZER_MAP: Mutex<Option<Vec<Vec<Option<Tile>>>>> = Mutex::new(None);
    pub static ref VISUALIZER_ROBOT_POSITION: Mutex<(usize, usize)> = Mutex::new((0, 0));
    pub static ref VISUALIZER_TIME: Mutex<EnvironmentalConditions> =
        Mutex::new(EnvironmentalConditions::new(&[Sunny], 0, 0).unwrap());
    pub static ref VISUALIZER_ENERGY: Mutex<usize> = Mutex::new(0);
}

fn main() {
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
        .insert_resource(GameTimer(Timer::from_seconds(2.0, TimerMode::Repeating)))
        .insert_resource(MapInfo {
            size: 0,
            last_known_robot_position: (0, 0),
            current_time: (0, 0),
        })
        .add_event::<TickEvent>()
        .add_event::<CameraEvent>()
        .add_systems(PreStartup, game_prestartup)
        .add_systems(Startup, startup)
        .add_systems(Startup, setup_minimap)
        .add_systems(Startup, setup_hud)
        .add_systems(FixedUpdate, run_tick)
        .add_systems(Update, update_tilemap)
        .add_systems(Update, update_contents)
        .add_systems(Update, update_robot_position)
        .add_systems(Update, update_dnc)
        .add_systems(Update, follow_robot_camera)
        .add_systems(Update, update_hud)
        .run();
}
