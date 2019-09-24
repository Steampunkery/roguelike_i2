use crate::map::{TileType, Map};

use rltk::VirtualKeyCode;

pub struct Player {
    pub position: (i32, i32),
    pub has_moved: bool,
    pub quit: bool,
}

impl Player {
    fn move_player(&mut self, delta_x: i32, delta_y: i32, map: &Map) {
        let current_position = self.position;
        let new_position = (current_position.0 + delta_x, current_position.1 + delta_y);
        if map[new_position.1 as usize][new_position.0 as usize] != TileType::Wall {
            self.position = new_position;
            self.has_moved = true;
        }
    }

    pub fn handle_input(&mut self, map: &Map, keycode: Option<VirtualKeyCode>) {
        match keycode {
            Some(key) => {
                match key {
                    // Numpad
                    VirtualKeyCode::Numpad8 => self.move_player(0, -1, map),
                    VirtualKeyCode::Numpad4 => self.move_player(-1, 0, map),
                    VirtualKeyCode::Numpad6 => self.move_player(1, 0, map),
                    VirtualKeyCode::Numpad2 => self.move_player(0, 1, map),

                    // Numpad diagonals
                    VirtualKeyCode::Numpad7 => self.move_player(-1, -1, map),
                    VirtualKeyCode::Numpad9 => self.move_player(1, -1, map),
                    VirtualKeyCode::Numpad1 => self.move_player(-1, 1, map),
                    VirtualKeyCode::Numpad3 => self.move_player(1, 1, map),

                    // Cursors
                    VirtualKeyCode::Up => self.move_player(0, -1, map),
                    VirtualKeyCode::Down => self.move_player(0, 1, map),
                    VirtualKeyCode::Left => self.move_player(-1, 0, map),
                    VirtualKeyCode::Right => self.move_player(1, 0, map),

                    VirtualKeyCode::Escape => self.quit = true,

                    _ => {}
                }
            }
            None => {}
        }
    }
}