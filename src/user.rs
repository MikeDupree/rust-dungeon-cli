use std::io::{stdout, Write};
use termion::{event::Key, raw::IntoRawMode};

// Player.rs
#[derive(Debug)]
pub struct Player {
    pub health: u16,
    pub xp: u16,
    pub speed: u16,
    pub pos: (u16, u16),
    pub marker: char,
    pub vulnerable: bool,
}

impl Player {
    pub fn create() -> Player {
        Player {
            health: 10,
            xp: 0,
            pos: (10, 10),
            speed: 1,
            marker: '&',
            vulnerable: true,
        }
    }
    pub fn render(&self) -> &str{
        "\x1b[92m&\x1b[0m"
    }
    pub fn collides(&self, row: u16, col: u16) -> bool {
        self.pos.0 == col && self.pos.1 == row
    }

    pub fn detect_input(key: &Key) {
        println!("key {:?}", key);
    }
}
