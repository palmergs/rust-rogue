extern crate tcod;
extern crate rand;

use tcod::{ Console, RootConsole, BackgroundFlag };
use tcod::input::KeyCode;
use tcod::colors::Color;
use rand::{ thread_rng, Rng };

mod bound;
use bound::{ Point, Bound, Contains };


fn render(root: &mut RootConsole, char_pt: &Point, dog_pt: &Point) {
    root.clear();
    root.put_char(char_pt.col, char_pt.row, '@', BackgroundFlag::Set);
    root.put_char(dog_pt.col, dog_pt.row, 'd', BackgroundFlag::Set);
    root.flush();
}

fn main() {

    let window_bounds = Bound { min: Point { row: 0, col: 0 }, max: Point { row: 50i32, col: 80i32 } };

    let mut root = RootConsole::initializer().size(window_bounds.max.col, window_bounds.max.row).init();
    let mut rng = thread_rng();
    let mut exit = false;
    
    // set default colors
    root.set_default_background(Color::new(0, 0, 0));
    root.set_default_foreground(Color::new(255, 255, 255));

    // initialize character
    let mut char_pt = Point { col: 40i32, row: 25i32 };
    let mut dog_pt = Point { col: 38i32, row: 23i32 };

    // initialize screen
    render(&mut root, &char_pt, &dog_pt);

    while !root.window_closed() && !exit {

        // wait for input
        let keypress = root.wait_for_keypress(true);

        // update game
        let mut offset = Point { row: 0, col: 0 };
        match keypress.code {
            KeyCode::Escape =>  exit = true,
            KeyCode::Up =>      offset.row = -1,
            KeyCode::NumPad8 => offset.row = -1,
            KeyCode::Down =>    offset.row = 1,
            KeyCode::NumPad2 => offset.row = 1,
            KeyCode::Left =>    offset.col = -1,
            KeyCode::NumPad4 => offset.col = -1,
            KeyCode::Right =>   offset.col = 1,
            KeyCode::NumPad6 => offset.col = 1,
            KeyCode::NumPad7 => { offset.col = -1; offset.row = -1 },
            KeyCode::NumPad9 => { offset.col = 1; offset.row = -1 },
            KeyCode::NumPad1 => { offset.col = -1; offset.row = 1 },
            KeyCode::NumPad3 => { offset.col = 1; offset.row = 1 },
            _ => {}
        }

        match window_bounds.contains(char_pt.offset(&offset)) { 
            Contains::DoesContain => char_pt = char_pt.offset(&offset),
            Contains::DoesNotContain => {},
        }

        let offset = Point { row: rng.gen_range(0, 3i32) - 1, col: rng.gen_range(0, 3i32) - 1 };
        match window_bounds.contains(dog_pt.offset(&offset)) {
            Contains::DoesContain => dog_pt = dog_pt.offset(&offset),
            Contains::DoesNotContain => {},
        }

        // render
        render(&mut root, &char_pt, &dog_pt);
    }
}
