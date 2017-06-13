extern crate tcod;

use tcod::console::*;
use tcod::Color;
use tcod::colors;
use tcod::input::Key;
use tcod::input::KeyCode::*;

const SCREEN_WIDTH: i32 = 80;
const SCREEN_HEIGHT: i32 = 50;
const LIMIT_FPS: i32 = 20;

struct Object {
    x: i32,
    y: i32,
    char: char,
    color: Color,
}

impl Object {
    pub fn new(x: i32, y: i32, char: char, color: Color) -> Self {
        Object {
            x: x,
            y: y,
            char: char,
            color: color,
        }
    }

    pub fn move_by(&mut self, dx: i32, dy: i32) {
        self.x += dx;
        self.y += dy;
    }

    pub fn draw(&self, con: &mut Console) {
        con.set_default_foreground(self.color);
        con.put_char(self.x, self.y, self.char, BackgroundFlag::None);
    }

    pub fn clear(&self, con: &mut Console) {
        con.put_char(self.x, self.y, ' ', BackgroundFlag::None);
    }
}

fn handle_keys(root: &mut Root, player: &mut Object) -> bool {
    let key = root.wait_for_keypress(true);
    match key {
        Key { code: Up, .. } => player.y -= 1,
        Key { code: Down, .. } => player.y += 1,
        Key { code: Left, .. } => player.x -= 1,
        Key { code: Right, .. } => player.x += 1,
        Key { code: Enter, alt: true, .. } => {
            let fullscreen = root.is_fullscreen();
            root.set_fullscreen(!fullscreen);
        },
        Key { code: Escape, .. } => return true,
        _ => {},
    }
    false
}

fn main() {
    let mut root = Root::initializer()
            .font("arial10x10.png", FontLayout::Tcod)
            .font_type(FontType::Greyscale)
            .size(SCREEN_WIDTH, SCREEN_HEIGHT)
            .title("Tutorial")
            .init();
    let mut con = Offscreen::new(SCREEN_WIDTH, SCREEN_HEIGHT);

    tcod::system::set_fps(LIMIT_FPS);

    let mut player_x = SCREEN_WIDTH / 2;
    let mut player_y = SCREEN_HEIGHT / 2;

    let player = Object::new(SCREEN_WIDTH / 2, 
            SCREEN_HEIGHT / 2, 
            '@', 
            colors::WHITE);
    let npc = Object::new(SCREEN_WIDTH / 2 - 5, 
            SCREEN_HEIGHT / 2, 
            '@', 
            colors::YELLOW);
    let mut objects = [ player, npc ];

    while !root.window_closed() {
        con.set_default_foreground(colors::WHITE);

        for object in &objects {
            object.clear(&mut con)
        }
        
        let exit = handle_keys(&mut root, &mut objects[0]);
        if exit {
            break
        }

        for object in &objects {
            object.draw(&mut con)
        }

        blit(&mut con, 
                (0, 0), 
                (SCREEN_WIDTH, SCREEN_HEIGHT), 
                &mut root, 
                (0, 0), 
                1.0, 1.0);
        root.flush();
    }
}
