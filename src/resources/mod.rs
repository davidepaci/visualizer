use bevy::prelude::{Resource, Timer};
use robotics_lib::runner::Runner;

#[derive(Resource)]
pub struct MapInfo {
    pub(crate) size: u32,
    pub(crate) last_known_robot_position: (usize, usize),
    pub(crate) robot_moving: bool,
}

#[derive(Resource)]
pub struct RunnerTag(pub Runner);

unsafe impl Sync for RunnerTag {}
unsafe impl Send for RunnerTag {}

#[derive(Resource)]
pub struct GameTimer(pub Timer);