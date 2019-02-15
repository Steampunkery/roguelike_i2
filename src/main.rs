use tcod::RootConsole as Root;
use tcod::Console;
use tcod::input::KeyCode;
use tcod::BackgroundFlag;

/// The width of the map display area
pub const MAP_WIDTH: i32 = 80;
/// The height of the map display area
pub const MAP_HEIGHT: i32 = 50;

fn main() {
    use self::KeyCode::*;

    let mut console = Root::initializer()
        .size(MAP_WIDTH, MAP_HEIGHT)
        .title("Monochrome Rogue-like: The Original Masterpiece")
        .fullscreen(false)
        .init();

    let mut exit = false;
    while !(console.window_closed() || exit) {
        console.clear();
        console.put_char(40, 25, '@', BackgroundFlag::Set);
        console.flush();
        let keypress = console.wait_for_keypress(true);
        match keypress.code {
            Escape => exit = true,
            _ => ()
        }
    }
}
