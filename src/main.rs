
use std::collections::HashMap;

use robotics_lib;
use robotics_lib::energy::Energy;
use robotics_lib::event::events::Event;
use robotics_lib::interface::{Direction, go, where_am_i};
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

/*
visualizer
advanced programming course 23-24
university of trento

https://github.com/davidepaci
*/
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
        for i in 0..100 {
            map.push(vec![]);
            for l in 0..100 {
                map[i].push( Tile {tile_type: TileType::Grass, content: Content::None, elevation: 10});
            }
        }
        /*map[2][0].content = Content::Coin(1);
        map[6][2].content = Content::Coin(1);
        map[2][1].content = Content::Coin(1);
        map[1][1].content = Content::Coin(1);*/
        // map[2][1] = Tile{tile_type: TileType::Wall, content: Content::None, elevation: 2 * 10};
        map[2][0].content = Content::Tree(0);

        let environmental_conditions = EnvironmentalConditions::new(&vec![Sunny, Rainy], 15, 12);
        return (map, (1, 1), environmental_conditions.unwrap(), 100.0, None);
    }
}

impl Runnable for MyRobot {
    fn process_tick(&mut self, world: &mut World) {
        go(self, world, Left);
        go(self, world, Up);
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
        go(self, world, Right);
        go(self, world, Right);
        println!("{:?}", where_am_i(self, world));
        println!("{:?}", self.get_energy());
        println!("{:?}", where_am_i(self, world));
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

fn startup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());

    let texture_handle: Handle<Image> = asset_server.load("tiles.png");

    let map_size = TilemapSize { x: 320, y: 320 };
    let mut tile_storage = TileStorage::empty(map_size);
    let tilemap_entity = commands.spawn_empty().id();

    for x in 0..320u32 {
        for y in 0..320u32 {
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

    let tile_size = TilemapTileSize { x: 16.0, y: 16.0 };
    let grid_size = tile_size.into();
    let map_type = TilemapType::default();

    commands.entity(tilemap_entity).insert(TilemapBundle {
        grid_size,
        map_type,
        size: map_size,
        storage: tile_storage,
        texture: TilemapTexture::Single(texture_handle),
        tile_size,
        transform: get_tilemap_center_transform(&map_size, &grid_size, &map_type, 0.0),
        ..Default::default()
    });
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

pub fn movement(
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

        if ortho.scale < 0.5 {
            ortho.scale = 0.5;
        }

        let z = transform.translation.z;
        transform.translation += time.delta_seconds() * direction * 500.;
        // Important! We need to restore the Z values when moving the camera around.
        // Bevy has a specific camera setup and this can mess with how our layers are shown.
        transform.translation.z = z;
    }
}

fn main() {
    let robot = MyRobot(Robot::new());
    let mut generator = WorldGenerator::init(100);
    let run = Runner::new(Box::new(robot), &mut generator);
    match run {
        | Ok(mut r) => {
            let _ = r.game_tick();
        }
        | Err(e) => println!("{:?}", e),
    }

    App::new()
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: String::from("Random Map Example"),
                        ..Default::default()
                    }),
                    ..default()
                })
                .set(ImagePlugin::default_nearest()),
        )
        .add_plugins(LogDiagnosticsPlugin::default())
        .add_plugins(FrameTimeDiagnosticsPlugin)
        .add_plugins(TilemapPlugin)
        .add_systems(Startup, startup)
        .add_systems(Update, movement)
        .add_systems(Update, random)
        .run();
}