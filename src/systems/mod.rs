use std::path::PathBuf;
use crate::components::LastUpdate;
use crate::entities::VisualizerRobot;
use crate::entities::HUD;
use crate::entities::{BigBrother, ContentMap, MiniCamera};
use crate::entities::{DncRectangle, TileMap};
use crate::events::{CameraEvent, TickEvent};
use crate::resources::GameTimer;
use crate::resources::MapInfo;
use crate::resources::RunnerTag;
use crate::VISUALIZER_ENERGY;
use crate::{
    TILE_PIXEL_OFFSET, TILE_PIXEL_SIZE, VISUALIZER_MAP, VISUALIZER_ROBOT_POSITION, VISUALIZER_TIME,
};

use bevy::asset::{AssetServer, Handle};
use bevy::core_pipeline::clear_color::ClearColorConfig;

use bevy::ecs::query::Without;
use bevy::input::Input;
use bevy::math::UVec2;
use bevy::math::Vec3;
use bevy::prelude::{
    default, Assets, BuildChildren, Camera, Camera2dBundle, Color, ColorMaterial, Commands,
    EventReader, EventWriter, Image, KeyCode, NodeBundle, Query, Res, ResMut, SpriteBundle, Time,
    Transform, Vec2, With,
};
use bevy::render::camera::OrthographicProjection;
use bevy::render::camera::Viewport;

use bevy::sprite::Sprite;
use bevy::text::Text;
use bevy::text::TextSection;
use bevy::text::TextStyle;
use bevy::ui::node_bundles::TextBundle;
use bevy::ui::AlignContent;
use bevy::ui::AlignSelf;
use bevy::ui::FlexDirection;
use bevy::ui::JustifyContent;
use bevy::ui::PositionType;
use bevy::ui::Style;
use bevy::ui::UiRect;
use bevy::ui::Val;
use bevy::ui::ZIndex;

use bevy_ecs_tilemap::map::{TilemapId, TilemapSize, TilemapTexture, TilemapTileSize, TilemapType};
use bevy_ecs_tilemap::prelude::{TileBundle, TilePos, TileStorage, TileTextureIndex};
use bevy_ecs_tilemap::TilemapBundle;

use oxagaudiotool::sound_config::OxAgSoundConfig;

use robotics_lib::runner::Runner;
use robotics_lib::world::tile::Content;
use robotics_lib::world::tile::TileType;

use saver_bot::SaverBot;
use my_robot::MyRobot;

use std::env;

use worldgen_unwrap::public::WorldgeneratorUnwrap;
// 🌯 runner wrapper 🌯
use crate::wrapper::VisualizerRobotWrapper;

pub fn game_prestartup(mut commands: Commands, mut game_timer: ResMut<GameTimer>) {
    // Env args
    let args: Vec<String> = env::args().collect();
    // Pause timer
    game_timer.0.pause();

    // Create bot, world, play audio
    let coin_amount = &args[1];
    let bot_choice = &args[3];

    // Load background music
    let background_music = OxAgSoundConfig::new_looped_with_volume("assets/default/music.ogg", 2.0);

    // Create robot and world
    let world_path = &args[2];
    match bot_choice.as_str() {
        "0" => {
            let mut saver_bot = SaverBot::new(Some(coin_amount.parse::<usize>().unwrap()));
            // Play background music
            let _ = saver_bot.audio.play_audio(&background_music);
            let robot = VisualizerRobotWrapper::new(saver_bot);

            let mut worldgen = WorldgeneratorUnwrap::init(false, Some(PathBuf::from(world_path.clone())));

            // Process first tick, add runner to resource
            let runner = Runner::new(Box::new(robot), &mut worldgen);
            let _ = match runner {
                Ok(mut runner) => {
                    let _ = runner.game_tick();

                    commands.insert_resource(RunnerTag(runner));
                }
                Err(err) => panic!("Error: {:?}", err),
            };
        },
        "1" => {
            let anastasia_bot = MyRobot::new();
            let robot = VisualizerRobotWrapper::new(anastasia_bot);

            let mut worldgen = WorldgeneratorUnwrap::init(false, Some(PathBuf::from(world_path.clone())));

            // Process first tick, add runner to resource
            let runner = Runner::new(Box::new(robot), &mut worldgen);
            let _ = match runner {
                Ok(mut runner) => {
                    let _ = runner.game_tick();

                    commands.insert_resource(RunnerTag(runner));
                }
                Err(err) => panic!("Error: {:?}", err),
            };
        }
        _ => {}
    }

    game_timer.0.unpause();
}

pub fn run_tick(
    time: Res<Time>,
    mut runner: ResMut<RunnerTag>,
    mut timer: ResMut<GameTimer>,
    mut event: EventWriter<TickEvent>,
) {
    if timer.0.tick(time.delta()).just_finished() {
        println!("RUNNING");
        println!("RUNNING");
        println!("RUNNING");
        println!("RUNNING");
        println!("RUNNING");
        println!("=========");
        let _ = runner.0.game_tick();
        // send tick update event
        event.send(TickEvent);
    }
}

pub fn startup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut map_info: ResMut<MapInfo>,
    _materials: ResMut<Assets<ColorMaterial>>,
) {
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

    // spawn black rectangle for day/night cycle
    commands.spawn((
        DncRectangle,
        SpriteBundle {
            sprite: Sprite {
                color: Color::rgba(0., 0., 0., 0.),
                custom_size: Some(Vec2::new(10000000000.0, 10000000000.0)),
                ..default()
            },
            transform: Transform::from_translation(Vec3::new(0., 0., 50.)),
            ..default()
        },
    ));

    let map_size = TilemapSize {
        x: map_info.size,
        y: map_info.size,
    };
    let tile_size = TilemapTileSize {
        x: TILE_PIXEL_SIZE,
        y: TILE_PIXEL_SIZE,
    };
    let grid_size = tile_size.into();
    let map_type = TilemapType::default();
    // tiles
    let mut tile_storage = TileStorage::empty(map_size);
    let tilemap_entity = commands.spawn_empty().id();
    let texture_handle: Handle<Image> = asset_server.load("tiles_robotic_lib.png");

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
                    TileMap,
                ))
                .id();
            tile_storage.set(&tile_pos, tile_entity);
        }
    }

    // spawn tilemap
    commands.entity(tilemap_entity).insert(TilemapBundle {
        grid_size,
        map_type,
        size: map_size,
        storage: tile_storage,
        texture: TilemapTexture::Single(texture_handle),
        tile_size,
        //transform: get_tilemap_center_transform(&map_size, &grid_size, &map_type, 0.0),
        transform: Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)),
        ..Default::default()
    });

    // contents
    let texture_handle2: Handle<Image> = asset_server.load("contents_robotic_lib.png");
    let mut tile_storage2 = TileStorage::empty(map_size);
    let tilemap_entity2 = commands.spawn_empty().id();

    for y in (0..map_info.size).rev() {
        for x in 0..map_info.size {
            let tile_pos = TilePos { x, y };
            let tile_entity = commands
                .spawn((
                    TileBundle {
                        position: tile_pos,
                        tilemap_id: TilemapId(tilemap_entity2),
                        ..Default::default()
                    },
                    LastUpdate::default(),
                    ContentMap,
                ))
                .id();
            tile_storage2.set(&tile_pos, tile_entity);
        }
    }

    // spawn contents
    commands.entity(tilemap_entity2).insert(TilemapBundle {
        grid_size,
        map_type,
        size: map_size,
        storage: tile_storage2,
        texture: TilemapTexture::Single(texture_handle2),
        tile_size,
        //transform: get_tilemap_center_transform(&map_size, &grid_size, &map_type, 0.0),
        transform: Transform::from_translation(Vec3::new(0.0, 0.0, 5.0)),
        ..Default::default()
    });

    // spawn robot
    commands.spawn((
        VisualizerRobot,
        SpriteBundle {
            texture: asset_server.load("robot_64x64.png"),
            //transform: Transform::from_translation(Vec3::new(-288.0,295.0,10.0)),
            transform: Transform::from_translation(Vec3::new(0.0, 74.0, 10.0)),
            ..default()
        },
    ));

    // spawn camera
    commands.spawn((Camera2dBundle::default(), BigBrother));
}

pub fn setup_hud(mut commands: Commands) {
    let energy_bar_layout = commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Vw(95.0),
                height: Val::Vh(5.),
                border: UiRect::all(Val::Px(2.)),
                flex_direction: FlexDirection::Column,
                align_self: AlignSelf::FlexEnd,
                ..default()
            },

            background_color: Color::WHITE.into(),
            border_color: Color::BLACK.into(),
            ..default()
        })
        .id();
    let energy_bar_mask = commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.),
                    height: Val::Percent(100.),
                    align_self: AlignSelf::Start,
                    position_type: PositionType::Relative,
                    ..default()
                },
                background_color: Color::BLUE.into(),
                z_index: ZIndex::Local(10),
                ..default()
            },
            HUD,
        ))
        .id();
    let energy_bar_label = commands
        .spawn((
            TextBundle {
                text: Text::from_sections([
                    TextSection::new(
                        "0",
                        TextStyle {
                            font_size: 32.0,
                            color: Color::BLACK.into(),
                            ..default()
                        },
                    ),
                    TextSection::new(
                        "\n",
                        TextStyle {
                            font_size: 32.0,
                            color: Color::BLACK.into(),
                            ..default()
                        },
                    ),
                    TextSection::new(
                        "",
                        TextStyle {
                            font_size: 32.0,
                            color: Color::BLACK.into(),
                            ..default()
                        },
                    ),
                ]),
                ..default()
            },
            HUD,
        ))
        .id();
    let energy_bar_label_layout = commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.),
                height: Val::Percent(100.),
                position_type: PositionType::Absolute,
                align_content: AlignContent::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            z_index: ZIndex::Local(20),
            ..default()
        })
        .id();

    commands
        .entity(energy_bar_layout)
        .push_children(&[energy_bar_label_layout, energy_bar_mask]);
    commands
        .entity(energy_bar_label_layout)
        .add_child(energy_bar_label);
}

pub fn update_hud(
    mut query_hud_text: Query<&mut Text, With<HUD>>,
    mut query_style: Query<&mut Style, (With<HUD>, Without<Text>)>,
    mut events: EventReader<TickEvent>,
) {
    for _event in events.iter() {
        // get robot energy
        let data = *VISUALIZER_ENERGY.lock().unwrap() as f32;
        for mut text in query_hud_text.iter_mut() {
            text.sections[0].value = format!("Energy: {:?}", data as usize);
            text.sections[1].value = "/".into();
            text.sections[2].value = format!("{:?}", 1000);
        }
        for mut style in query_style.iter_mut() {
            style.width = Val::Percent(100. * (data as f32 / 1000.0))
        }
    }
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
                    physical_size: UVec2::new(512, 256),
                    ..default()
                }),
                ..default()
            },
            camera_2d: bevy::prelude::Camera2d {
                clear_color: ClearColorConfig::None,
                ..default()
            },
            projection: OrthographicProjection {
                scale: 5.0,
                ..default()
            },
            transform: Transform {
                translation: Vec3::new(0.0, 0.0, 500.0),
                ..default()
            },
            ..default()
        },
        MiniCamera,
    ));
}

// update day night cycle
pub fn update_dnc(
    mut query: Query<(&DncRectangle, &mut Sprite)>,
    mut events: EventReader<TickEvent>,
) {
    for _event in events.iter() {
        let (_dnc_rectangle, mut sprite) = query.single_mut();
        // get world time
        let data = VISUALIZER_TIME.lock().unwrap();
        // calc day night cycle
        // get time first
        match data
            .get_time_of_day_string()
            .split_once(":")
            .unwrap()
            .0
            .parse::<u8>()
        {
            Ok(number) => {
                let alpha: f32 = match number {
                    0..=4 => 0.8,
                    5 => 0.6,
                    6 => 0.3,
                    7 => 0.1,
                    8..=17 => 0.,
                    18 => 0.2,
                    19 => 0.4,
                    20 => 0.6,
                    21..=23 => 0.8,
                    _ => 0.0,
                };
                sprite.color.set_a(alpha);
            }
            _ => {}
        }
    }
}

// func to change sprite position based on actual robot position
pub fn update_robot_position(
    mut query: Query<(&VisualizerRobot, &mut Transform)>,
    mut map_info: ResMut<MapInfo>,
    mut events: EventReader<TickEvent>,
    mut event_w: EventWriter<CameraEvent>,
) {
    for _event in events.iter() {
        // get robot position
        let data = VISUALIZER_ROBOT_POSITION.lock().unwrap();
        // check if robot position is different. if it is, then update on screen
        if map_info.last_known_robot_position != *data {
            // change robot position in gui
            // robot's x = gui's y and viceversa
            let (_player, mut transform) = query.single_mut();
            println!(
                "map size: {} - tile_pixel_size {} - x {} - y {} - tile_pixel_offse {}",
                map_info.size, TILE_PIXEL_SIZE, data.0, data.1, TILE_PIXEL_OFFSET
            );
            transform.translation.x = (TILE_PIXEL_SIZE * (data.1 as f32)) + 5.0;
            transform.translation.y = (map_info.size as f32 * TILE_PIXEL_SIZE)
                - (TILE_PIXEL_SIZE * (data.0 as f32 + 1.0))
                + TILE_PIXEL_OFFSET;
            println!("{} {}", transform.translation.x, transform.translation.y);
            println!("{:?}", data);
            // save new robot position
            map_info.last_known_robot_position = *data;
        }
        // send event to update BigBrother camera
        event_w.send(CameraEvent);
    }
}

// update tiles
pub fn update_tilemap(
    time: ResMut<Time>,
    mut query: Query<(&TileMap, &mut TileTextureIndex, &mut LastUpdate)>,
    mut events: EventReader<TickEvent>,
) {
    for _event in events.iter() {
        // get visualizer map
        let data = VISUALIZER_MAP.lock().unwrap();
        // flatten it
        let flattened = data.clone().unwrap().concat();
        //println!("{:?}", flattened);
        let current_time = time.elapsed_seconds_f64();
        for (index, (_tilemap, mut tile, mut last_update)) in query.iter_mut().enumerate() {
            if (current_time - last_update.value) > 1.0 {
                //let data = VISUALIZER_MAP.lock().unwrap();
                //println!("Global variable value: {:?}", *data);
                if let Some(flattened_tile) = flattened.get(index) {
                    if let Some(tile_ref) = flattened_tile.as_ref() {
                        tile.0 = match tile_ref.tile_type {
                            TileType::Grass => 0,
                            TileType::Sand => 1,
                            TileType::Snow => 2,
                            TileType::Mountain => 3,
                            TileType::Teleport(false) => 4,
                            TileType::Wall => 5,
                            TileType::Hill => 6,
                            TileType::Street => 7,
                            TileType::Lava => 8,
                            TileType::DeepWater => 9,
                            TileType::ShallowWater => 10,
                            _ => 5,
                        }
                    } else {
                        tile.0 = 5
                    }
                }
                last_update.value = current_time;
            }
        }
    }
}

// update contents
pub fn update_contents(
    time: ResMut<Time>,
    mut query: Query<(&ContentMap, &mut TileTextureIndex, &mut LastUpdate)>,
    mut events: EventReader<TickEvent>,
) {
    for _event in events.iter() {
        // get visualizer map
        let data = VISUALIZER_MAP.lock().unwrap();
        // flatten it
        let flattened = data.clone().unwrap().concat();
        //println!("{:?}", flattened);
        let current_time = time.elapsed_seconds_f64();
        for (index, (_contentmap, mut tile, mut last_update)) in query.iter_mut().enumerate() {
            if (current_time - last_update.value) > 1.0 {
                if let Some(flattened_content) = flattened.get(index) {
                    if let Some(content_ref) = flattened_content.as_ref() {
                        tile.0 = match content_ref.content {
                            Content::None => 0,
                            Content::Water(_) => 1,
                            Content::Scarecrow => 2,
                            Content::Tree(_) => 3,
                            Content::Garbage(_) => 4,
                            Content::Bank(_) => 5,
                            Content::Crate(_) => 6,
                            Content::Fish(_) => 7,
                            Content::Market(_) => 8,
                            Content::Bush(_) => 9,
                            Content::Bin(_) => 10,
                            Content::Coin(_) => 11,
                            Content::Rock(_) => 12,
                            Content::JollyBlock(_) => 13,
                            Content::Fire => 14,
                            Content::Building => 15,
                            _ => 0,
                        }
                    } else {
                        tile.0 = 0
                    }
                }

                last_update.value = current_time;
            }
        }
    }
}

pub fn follow_robot_camera(
    mut query: Query<(&mut Transform, &mut OrthographicProjection), With<BigBrother>>,
    query_robot: Query<&Transform, (With<VisualizerRobot>, Without<BigBrother>)>,
    mut query_minimap: Query<
        &mut Transform,
        (
            With<MiniCamera>,
            Without<VisualizerRobot>,
            Without<BigBrother>,
        ),
    >,
    mut events: EventReader<CameraEvent>,
) {
    for _event in events.iter() {
        let robot_transform = query_robot.single();
        let (mut camera_transform, mut camera_ortho) = query.single_mut();

        let z = camera_transform.translation.z;
        camera_transform.translation.x = robot_transform.translation.x;
        camera_transform.translation.y = robot_transform.translation.y;
        camera_ortho.scale = 0.6;
        camera_transform.translation.z = z;

        let mut minimap_transform = query_minimap.single_mut();

        let z = minimap_transform.translation.z;
        minimap_transform.translation.x = robot_transform.translation.x;
        minimap_transform.translation.y = robot_transform.translation.y;
        minimap_transform.translation.z = z;
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
