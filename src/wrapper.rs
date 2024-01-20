use robotics_lib::energy::Energy;
use robotics_lib::event::events::Event;
use robotics_lib::interface::{robot_map, where_am_i};
use robotics_lib::runner::backpack::BackPack;
use robotics_lib::runner::Runnable;
use robotics_lib::world::coordinates::Coordinate;
use robotics_lib::world::World;
use saver_bot::SaverBot;
use crate::{VISUALIZER_MAP, VISUALIZER_ROBOT_POSITION};

pub struct VisualizerRobotWrapper {
    saver_bot: SaverBot,
}

impl VisualizerRobotWrapper {
    pub fn new(saver_bot: SaverBot) -> Self {
        VisualizerRobotWrapper { saver_bot }
    }
}

impl Runnable for VisualizerRobotWrapper {
    fn process_tick(&mut self, world: &mut World) {
        self.saver_bot.process_tick(world);
        // save map data
        let mut data = VISUALIZER_MAP.lock().unwrap();
        *data = robot_map(world);
        // save robot position data
        let mut data_position = VISUALIZER_ROBOT_POSITION.lock().unwrap();
        *data_position = where_am_i(self, world).1;
    }

    fn handle_event(&mut self, event: Event) {
        println!();
        println!("{:?}", event);
        println!();
        self.saver_bot.handle_event(event)
    }

    fn get_energy(&self) -> &Energy {
        self.saver_bot.get_energy()
    }

    fn get_energy_mut(&mut self) -> &mut Energy {
        self.saver_bot.get_energy_mut()
    }

    fn get_coordinate(&self) -> &Coordinate {
        self.saver_bot.get_coordinate()
    }

    fn get_coordinate_mut(&mut self) -> &mut Coordinate {
        self.saver_bot.get_coordinate_mut()
    }

    fn get_backpack(&self) -> &BackPack {
        self.saver_bot.get_backpack()
    }

    fn get_backpack_mut(&mut self) -> &mut BackPack {
        self.saver_bot.get_backpack_mut()
    }
}