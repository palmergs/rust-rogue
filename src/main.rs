extern crate tcod;

use tcod::{ Console, RootConsole, BackgroundFlag };
use tcod::input::KeyCode;
use tcod::colors::Color;

fn main() {

    let mut root = RootConsole::initializer().size(80, 50).init();
    root.set_default_background(Color::new(0, 0, 0));
    root.set_default_foreground(Color::new(255, 255, 255));
    let mut exit = false;
    
    while !root.window_closed() && !exit {
        root.clear();
        root.put_char(40, 25, '@', BackgroundFlag::Set);
        root.flush();
        
        let keypress = root.wait_for_keypress(true);
        match keypress.code {
            KeyCode::Escape => { exit = true },
            _ => {}
        }
    }
}
