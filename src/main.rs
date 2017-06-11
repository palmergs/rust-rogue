extern crate tcod;
extern crate rand;

use tcod::{ Console, RootConsole, BackgroundFlag };
use tcod::input::KeyCode;
use tcod::colors::Color;
use rand::{ thread_rng, Rng };

fn render(root: &mut RootConsole, char_x: i32, char_y: i32, dog_x: i32, dog_y: i32) {
    root.clear();
    root.put_char(char_x, char_y, '@', BackgroundFlag::Set);
    root.put_char(dog_x, dog_y, 'd', BackgroundFlag::Set);
    root.flush();
}

fn main() {

    let rows = 50i32;
    let cols = 80i32;

    let mut root = RootConsole::initializer().size(cols, rows).init();
    let mut rng = thread_rng();
    let mut exit = false;
    
    // set default colors
    root.set_default_background(Color::new(0, 0, 0));
    root.set_default_foreground(Color::new(255, 255, 255));

    // initialize character
    let mut char_x = 40i32;
    let mut char_y = 25i32;

    // initialize dog
    let mut dog_x = 38i32;
    let mut dog_y = 23i32;

    // initialize screen
    render(&mut root, char_x, char_y, dog_x, dog_y);

    while !root.window_closed() && !exit {

        // wait for input
        let keypress = root.wait_for_keypress(true);
        match keypress.code {
            KeyCode::Escape => { exit = true },
            KeyCode::Up => { 
                if char_y > 0 { 
                    char_y -= 1;
                }
            },
            KeyCode::Down => {
                if char_y < (rows - 1) {
                    char_y += 1;
                }
            },
            KeyCode::Left => {
                if char_x > 0 {
                    char_x -= 1;
                }
            },
            KeyCode::Right => {
                if char_x < (cols - 1) {
                    char_x += 1;
                }
            },
            _ => {}
        }

        // update game
        let dog_move_x = rng.gen_range(0, 3i32) - 1;
        if (dog_x + dog_move_x) > 0 && (dog_x + dog_move_x) < (cols - 1) {
            dog_x += dog_move_x;
        }

        let dog_move_y = rng.gen_range(0, 3i32) - 1;
        if (dog_y + dog_move_y) > 0 && (dog_y + dog_move_y) < (rows - 1) {
            dog_y += dog_move_y;
        }

        // render
        render(&mut root, char_x, char_y, dog_x, dog_y);
    }
}
