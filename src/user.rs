use crate::EventKeyDown;
use termion::event::Key;

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

    pub fn render(&self) -> &str {
        "\x1b[92m♟\x1b[0m"
    }

    pub fn collides(&self, row: u16, col: u16) -> bool {
        self.pos.0 == col && self.pos.1 == row
    }

    pub fn handle_input(&mut self, key: Key) -> bool {
        match key {
            Key::Char('e') => self.move_direction("u"),
            Key::Char('d') => self.move_direction("d"),
            Key::Char('s') => self.move_direction("l"),
            Key::Char('f') => self.move_direction("r"),
            _ => (),
        }
        true
    }
    pub fn move_direction(&mut self, direction: &str) {
        match direction {
            "u" => self.pos.1 -= 1,
            "d" => self.pos.1 += 1,
            "l" => self.pos.0 -= 1,
            "r" => self.pos.0 += 1,
            _ => (),
        }
    }

}
