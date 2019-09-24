use crate::map::{DungeonMap};
use crate::{MAP_WIDTH, MAP_HEIGHT};

use std::convert::TryInto;
use rltk::RandomNumberGenerator;

pub struct Level {
    pub map: DungeonMap,
    pub visible: Vec<Vec<bool>>,
    pub discovered: Vec<Vec<bool>>,
    pub stair: (i32, i32),
}

impl Level {
    pub fn new(random: &mut RandomNumberGenerator) -> Level {
        let map = DungeonMap::new(random);

        // Basically just false
        let visible = vec![vec![false; MAP_WIDTH.try_into().unwrap()]; MAP_HEIGHT.try_into().unwrap()];
        let discovered = vec![vec![false; MAP_WIDTH.try_into().unwrap()]; MAP_HEIGHT.try_into().unwrap()];
        Level {
            stair: map.stair,
            map,
            visible,
            discovered,
        }
    }
}