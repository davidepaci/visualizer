use bevy::prelude::Resource;

#[derive(Resource)]
pub struct MapInfo {
    pub(crate) size: u32,
    pub(crate) last_known_robot_position: (usize, usize),
    pub(crate) robot_moving: bool,
}