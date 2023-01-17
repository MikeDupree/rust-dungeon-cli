use std::time::Instant;

use event_system::create_event_system;
use termion::event::Key;

// Player.rs
#[derive(Debug)]
pub struct Player {
    pub health: u16,
    pub xp: u16,
    pub speed: u16,
    pub pos: (u16, u16),
    pub attack_pos: Vec<(u16, u16)>,
    pub attack_render_count: u16,
    pub marker: char,
    pub vulnerable: bool,
    pub last_updated: Instant,
}

impl Player {
    pub fn create() -> Player {
        Player {
            health: 10,
            xp: 0,
            pos: (10, 10),
            attack_pos: vec![(10, 10)],
            attack_render_count: 0,
            speed: 1,
            marker: '&',
            vulnerable: true,
            last_updated: Instant::now()
        }
    }

    pub fn render(&self) -> &str {
        "\x1b[92m♟\x1b[0m"
    }

    pub fn collides(&self, row: u16, col: u16) -> bool {
        self.pos.0 == col && self.pos.1 == row
    }

    pub fn render_base_attack(&self) -> &str {
        "\x1b[92m*\x1b[0m"
    }

    pub fn base_attack_collides(&self, row: u16, col: u16) -> bool {
        for a in &self.attack_pos {
            if a.0 == col && a.1 == row {
                return true;
            }
        }

        false
    }

    pub fn update(&mut self) {
        if self.last_updated.elapsed().as_millis() >= 200 {
            if self.attack_render_count >= 5 {
                for (_i, el) in self.attack_pos.iter_mut().enumerate() {
                  el.0 = self.pos.0;
                  el.1 = self.pos.1;
                }
                self.attack_render_count = 0;
                return;
            }
            for (i, el) in self.attack_pos.iter_mut().enumerate() {
                match i {
                    0 => el.0 += 1,
                    _ => (),
                }
            }
            self.attack_render_count += 1;
            self.last_updated = Instant::now();
        }
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
