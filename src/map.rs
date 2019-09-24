use crate::util::Point;
use rltk::RandomNumberGenerator;
use crate::MAP_WIDTH;
use crate::MAP_HEIGHT;
use std::convert::TryInto;
use std::ops::{Index, IndexMut};
use std::slice::Iter;

const MAX_ROOM_SIZE: i32 = 7;
const MIN_ROOM_SIZE: i32 = 4;
const MAX_ROOMS: usize = 7;

pub type Map = Vec<Vec<TileType>>;

#[derive(PartialEq, Copy, Clone)]
pub enum TileType {
    Wall,
    Floor,
    Stair,
}

#[derive(Clone, Copy, Debug)]
pub struct Rect {
    pub x1: i32,
    pub y1: i32,
    pub x2: i32,
    pub y2: i32,
}

impl Rect {
    /// Convenience method for creating new rectangles.
    pub fn new(x: i32, y: i32, w: i32, h: i32) -> Self {
        Rect { x1: x, y1: y, x2: x + w, y2: y + h }
    }

    /// Returns the center of the rectangle.
    pub fn center(&self) -> (i32, i32) {
        let center_x = (self.x1 + self.x2) / 2;
        let center_y = (self.y1 + self.y2) / 2;
        (center_x, center_y)
    }

    /// Checks if the rectangle collides with another.
    pub fn intersects_with(&self, other: &Rect) -> bool {
        // returns true if this rectangle intersects with another one
        (self.x1 <= other.x2) && (self.x2 >= other.x1) &&
            (self.y1 <= other.y2) && (self.y2 >= other.y1)
    }

    pub fn rand_point(&self, random: &mut RandomNumberGenerator) -> Point {
        Point {
            x: random.range(self.x1 + 1, self.x2),
            y: random.range(self.y1 + 1, self.y2),
        }
    }
}

pub struct DungeonMap {
    map: Map,
    pub rooms: Vec<Rect>,
    pub player_spawn: (i32, i32),
    pub stair: (i32, i32),
}

impl DungeonMap {
    pub fn new(random: &mut RandomNumberGenerator) -> DungeonMap {
        // fill map with wall tiles
        let mut map = vec![vec![TileType::Wall; MAP_WIDTH.try_into().unwrap()]; MAP_HEIGHT.try_into().unwrap()];
        let mut rooms = vec![];


        while rooms.len() < MAX_ROOMS {
            // random width and height
            let w = random.range(MIN_ROOM_SIZE, MAX_ROOM_SIZE + 1);
            let h = random.range(MIN_ROOM_SIZE, MAX_ROOM_SIZE + 1);
            // random position without going out of the boundaries of the map
            let x = random.range(1, MAP_WIDTH - w - 1);
            let y = random.range(1, MAP_HEIGHT - h - 1);

            let new_room = Rect::new(x, y, w, h);

            // run through the other rooms and see if they intersect with this one
            let failed = rooms.iter().any(|other_room| new_room.intersects_with(other_room));

            if !failed {
                // this means there are no intersections, so this room is valid
                Self::create_room(new_room, &mut map);
                let (new_x, new_y) = new_room.center();

                if !rooms.is_empty() {
                    // center coordinates of the previous room
                    let (prev_x, prev_y) = rooms[rooms.len() - 1].center();

                    // coin flip
                    if random.rand::<bool>() {
                        Self::create_h_tunnel(prev_x, new_x, prev_y, &mut map);
                        Self::create_v_tunnel(prev_y, new_y, new_x, &mut map);
                    } else {
                        Self::create_v_tunnel(prev_y, new_y, prev_x, &mut map);
                        Self::create_h_tunnel(prev_x, new_x, new_y, &mut map);
                    }
                }

                // finally, append the new room to the list
                rooms.push(new_room);
            }
        }

        let stair = rooms[rooms.len() - 1].center();

        map[stair.1 as usize][stair.0 as usize] = TileType::Stair;

        DungeonMap {
            player_spawn: rooms[0].center(),
            stair,
            map,
            rooms,
        }

    }

    pub fn iter(&self) -> Iter<Vec<TileType>> {
        self.map.iter()
    }

    fn create_room(room: Rect, map: &mut Map) {
        for y in (room.x1)..room.x2 {
            for x in (room.y1)..room.y2 {
                map[x as usize][y as usize] = TileType::Floor;
            }
        }
    }

    fn create_h_tunnel(x1: i32, x2: i32, y: i32, map: &mut Map) {
        for x in std::cmp::min(x1, x2)..(std::cmp::max(x1, x2) + 1) {
            map[y as usize][x as usize] = TileType::Floor;
        }
    }

    fn create_v_tunnel(y1: i32, y2: i32, x: i32, map: &mut Map) {
        for y in std::cmp::min(y1, y2)..(std::cmp::max(y1, y2) + 1) {
            map[y as usize][x as usize] = TileType::Floor;
        }

    }
}

impl Index<usize> for DungeonMap {
    type Output = Vec<TileType>;

    fn index(&self, idx: usize) -> &Self::Output {
        &self.map[idx]
    }
}

impl IndexMut<usize> for DungeonMap {
    fn index_mut(&mut self, idx: usize) -> &mut Self::Output {
        &mut self.map[idx]
    }
}