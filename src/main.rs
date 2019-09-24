extern crate rltk;

mod map;
mod util;
mod player;

use map::{TileType, DungeonMap};
use util::{idx_xy, xy_idx};
use rltk::{Rltk, GameState, Console, RandomNumberGenerator};
use rltk::{Point, BaseMap, Algorithm2D, RGB, GRAY39};
use std::convert::TryInto;
use crate::player::Player;

pub const MAP_HEIGHT: i32 = 50;
pub const MAP_WIDTH: i32 = 80;

const SHOW: bool = false;

pub struct State {
    player: Player,
    map: DungeonMap,
    random: RandomNumberGenerator,
    visible: Vec<Vec<bool>>,
    discovered: Vec<Vec<bool>>,
}

impl State {
    pub fn new() -> State {
        let mut random = RandomNumberGenerator::new();
        let map = DungeonMap::new(&mut random);

        // Basically just false
        let visible = vec![vec![false; MAP_WIDTH.try_into().unwrap()]; MAP_HEIGHT.try_into().unwrap()];
        let discovered = vec![vec![false; MAP_WIDTH.try_into().unwrap()]; MAP_HEIGHT.try_into().unwrap()];
        let state = State {
            player: Player {
                has_moved: true,
                quit: false,
                position: map.player_spawn,
            },
            map,
            random,
            visible,
            discovered,
        };
        state
    }
}

impl GameState for State {
    fn tick(&mut self, ctx : &mut Rltk) {
        self.player.handle_input(&self.map.map, ctx.key);

        if self.player.quit {
            ctx.quit();
        }

        if self.player.has_moved {
            for y in self.visible.iter_mut() {
                for x in y {
                    *x = false;
                }
            }


            let player_point = Point::new(self.player.position.0, self.player.position.1);
            let fov: Vec<Point> = rltk::field_of_view(player_point, 8, self);

            for pt in fov {
                self.visible[pt.y as usize][pt.x as usize] = true;
                self.discovered[pt.y as usize][pt.x as usize] = true;
            }
        }

        ctx.cls();

        for (i, y) in self.map.map.iter().enumerate() {
            for (j, x) in y.iter().enumerate() {
                if self.visible[i][j] || SHOW {
                    match x {
                        TileType::Floor => ctx.print(j as i32, i as i32, "."),
                        TileType::Wall => ctx.print(j as i32, i as i32, "+")
                    }
                }
                if self.discovered[i][j] && !self.visible[i][j]{
                    match x {
                        TileType::Floor => ctx.print_color(j as i32, i as i32, RGB::named(GRAY39), RGB::new(), "."),
                        TileType::Wall => ctx.print_color(j as i32, i as i32, RGB::named(GRAY39), RGB::new(), "+")
                    }
                }
            }
        }

        ctx.print(self.player.position.0, self.player.position.1, "@");
    }
}

impl BaseMap for State {
    // We'll use this one - if its a wall, we can't see through it
    fn is_opaque(&self, idx: i32) -> bool {
        let (x, y) = idx_xy(idx as usize);
        self.map.map[y as usize][x as usize] == TileType::Wall
    }
    fn get_available_exits(&self, _idx: i32) -> Vec<(i32, f32)> {
        Vec::new()
    }
    fn get_pathing_distance(&self, _idx1: i32, _idx2: i32) -> f32 {
        0.0
    }
}

impl Algorithm2D for State {
    // Point translations that we need for field-of-view. Fortunately, we've already written them!
    fn point2d_to_index(&self, pt: Point) -> i32 {
        xy_idx(pt.x, pt.y) as i32
    }
    fn index_to_point2d(&self, idx: i32) -> Point {
        Point::new(idx % 80, idx / 80)
    }
}

fn main() {
    let context = Rltk::init_simple8x8(80, 50, "Hello RLTK World", "resources");
    let gs = State::new();
    rltk::main_loop(context, gs);
}