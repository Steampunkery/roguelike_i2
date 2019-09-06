extern crate rltk;
use rltk::{Rltk, GameState, Console, VirtualKeyCode};

#[derive(PartialEq, Copy, Clone)]
enum TileType {
    Wall,
    Floor
}

struct State {
    player_position: usize,
    map: Vec<TileType>
}

impl State {
    pub fn new(player_position: (i32, i32)) -> State {
        let map = vec![TileType::Floor; 80*50];
        let state = State {
            player_position: xy_idx(player_position.0, player_position.1),
            map
        };
        state
    }

    fn move_player(&mut self, delta_x: i32, delta_y: i32) {
        let current_position = idx_xy(self.player_position);
        let new_position = (current_position.0 + delta_x, current_position.1 + delta_y);
        self.player_position = xy_idx(new_position.0, new_position.1);
    }
}

pub fn xy_idx(x: i32, y: i32) -> usize {
    (y as usize * 80) + x as usize
}

pub fn idx_xy(idx: usize) -> (i32, i32) {
    (idx as i32 % 80, idx as i32 / 80)
}

impl GameState for State {
    fn tick(&mut self, ctx : &mut Rltk) {
        match ctx.key {
            Some(key) => {
                match key {
                    // Numpad
                    VirtualKeyCode::Numpad8 => self.move_player(0, -1),
                    VirtualKeyCode::Numpad4 => self.move_player(-1, 0),
                    VirtualKeyCode::Numpad6 => self.move_player(1, 0),
                    VirtualKeyCode::Numpad2 => self.move_player(0, 1),

                    // Numpad diagonals
                    VirtualKeyCode::Numpad7 => self.move_player(-1, -1),
                    VirtualKeyCode::Numpad9 => self.move_player(1, -1),
                    VirtualKeyCode::Numpad1 => self.move_player(-1, 1),
                    VirtualKeyCode::Numpad3 => self.move_player(1, 1),

                    // Cursors
                    VirtualKeyCode::Up => self.move_player(0, -1),
                    VirtualKeyCode::Down => self.move_player(0, 1),
                    VirtualKeyCode::Left => self.move_player(-1, 0),
                    VirtualKeyCode::Right => self.move_player(1, 0),

                    VirtualKeyCode::Escape => ctx.quit(),

                    _ => {}
                }
            }
            None => {}
        }

        ctx.cls();

        for i in 0..80*50 {
            let (x, y) = idx_xy(i);
            if self.map[i] == TileType::Floor{
                ctx.print(x, y, ".");
            }
        }

        let (player_x, player_y) = idx_xy(self.player_position);
        ctx.print(player_x, player_y, "@");
    }
}

fn main() {
    let context = Rltk::init_simple8x8(80, 50, "Hello RLTK World", "resources");
    let gs = State::new((40, 25));
    rltk::main_loop(context, gs);
}