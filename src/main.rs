extern crate rltk;

mod map;
mod util;
mod player;
mod level;

use map::{TileType};
use util::{idx_xy, xy_idx};
use rltk::{Rltk, GameState, Console, RandomNumberGenerator};
use rltk::{Point, BaseMap, Algorithm2D, RGB, GRAY39};
use crate::player::Player;
use level::Level;

pub const MAP_HEIGHT: i32 = 50;
pub const MAP_WIDTH: i32 = 80;

const SHOW: bool = false;

pub struct State {
    player: Player,
    level: Level,
    random: RandomNumberGenerator,
}

impl State {
    pub fn new() -> State {
        let mut random = RandomNumberGenerator::new();
        let level = Level::new(&mut random);

        let state = State {
            player: Player {
                has_moved: true,
                quit: false,
                position: level.map.player_spawn,
                new_level: false
            },
            level,
            random,
        };
        state
    }

    pub fn new_level(&mut self) {
        let level = Level::new(&mut self.random);

        self.player.has_moved = true;
        self.player.new_level = false;
        self.player.position = level.map.player_spawn;
        self.level = level;
    }
}

impl GameState for State {
    fn tick(&mut self, ctx : &mut Rltk) {
        self.player.handle_input(&self.level.map, ctx.key);

        if self.player.quit {
            ctx.quit();
        }

        if self.player.new_level {
            self.new_level();
        }

        if self.player.has_moved {
            for y in self.level.visible.iter_mut() {
                for x in y {
                    *x = false;
                }
            }


            let player_point = Point::new(self.player.position.0, self.player.position.1);
            let fov: Vec<Point> = rltk::field_of_view(player_point, 8, self);

            for pt in fov {
                self.level.visible[pt.y as usize][pt.x as usize] = true;
                self.level.discovered[pt.y as usize][pt.x as usize] = true;
            }
        }

        ctx.cls();

        for (i, y) in self.level.map.iter().enumerate() {
            for (j, x) in y.iter().enumerate() {
                if self.level.visible[i][j] || SHOW {
                    match x {
                        TileType::Floor => ctx.print(j as i32, i as i32, "."),
                        TileType::Wall => ctx.print(j as i32, i as i32, "+"),
                        TileType::Stair => ctx.print(j as i32, i as i32, ">"),
                    }
                }
                if self.level.discovered[i][j] && !self.level.visible[i][j]{
                    match x {
                        TileType::Floor => ctx.print_color(j as i32, i as i32, RGB::named(GRAY39), RGB::new(), "."),
                        TileType::Wall => ctx.print_color(j as i32, i as i32, RGB::named(GRAY39), RGB::new(), "+"),
                        TileType::Stair => ctx.print_color(j as i32, i as i32, RGB::named(GRAY39), RGB::new(), ">")
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
        self.level.map[y as usize][x as usize] == TileType::Wall
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