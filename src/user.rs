use std::time::Instant;
use termion::event::Key;

use crate::spawner::ExperienceOrb;

// Player.rs
#[derive(Debug)]
pub struct Player {
    pub health: u16,
    pub xp: u32,
    pub speed: u16,
    pub pos: (u16, u16),
    pub attack_pos: Vec<(u16, u16)>,
    pub attack_render_count: u16,
    pub marker: char,
    pub vulnerable: bool,
    pub last_updated: Instant,
    pub last_damaged: Instant,
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
            last_updated: Instant::now(),
            last_damaged: Instant::now(),
        }
    }

    pub fn render(&self) -> &str {
        if self.health <= 0 {
            return "\u{001b}[31;1m⛼\u{001b}[0m";
        }
        "\u{001b}[34;1m♟\u{001b}[0m"
    }

    pub fn collides(&self, row: u16, col: u16) -> bool {
        self.pos.0 == col && self.pos.1 == row
    }

    pub fn render_base_attack(&self, row: u16, col: u16) -> &str {
        let mut attack_display = "";
        for (i, a) in self.attack_pos.iter().enumerate() {
            if a.0 == col && a.1 == row {
                match i {
                    0 => attack_display = "\u{001b}[32;1m↣\u{001b}[0m",
                    1 => attack_display = "\u{001b}[32;1m↢\u{001b}[0m",
                    _ => (),
                }
            }
        }

        attack_display
    }

    pub fn base_attack_collides(&self, row: u16, col: u16, is_display: bool) -> bool {
        if self.health <= 0 {
            return false;
        };
        let mut hit_radius = 1;
        if is_display {
            hit_radius = 0;
        }
        for a in &self.attack_pos {
            if (a.0 + hit_radius == col || a.0 == col || a.0 - hit_radius == col) && (a.1 == row) {
                return true;
            }
        }

        false
    }

    pub fn update(&mut self) {
        if self.health <= 0 {
            return;
        };

        if self.last_updated.elapsed().as_millis() >= 200 {
            // if render count >= to attack range reset attack pos.
            if self.attack_render_count >= 5 {
                for (_i, el) in self.attack_pos.iter_mut().enumerate() {
                    el.0 = self.pos.0;
                    el.1 = self.pos.1;
                }
                self.attack_render_count = 0;
                return;
            }

            // Update attack pos
            for (i, el) in self.attack_pos.iter_mut().enumerate() {
                match i {
                    0 => el.0 += 1,
                    1 => el.0 -= 1,
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
        if self.health <= 0 {
            return;
        };
        match direction {
            "u" => self.pos.1 -= 1,
            "d" => self.pos.1 += 1,
            "l" => self.pos.0 -= 1,
            "r" => self.pos.0 += 1,
            _ => (),
        }
    }

    pub fn collect_experience(&mut self, experience_orb: &mut ExperienceOrb) {
        self.xp += experience_orb.get_amount();
        if self.xp > 2 && self.attack_pos.len() < 2 {
            // increase players base attack to shoot 2 arrows
            self.attack_pos.push(self.pos)
        }
        experience_orb.collect();
    }

    pub fn take_damage(&mut self, damage: u16) {
        if self.last_damaged.elapsed().as_millis() >= 200 && self.health > 0 {
            self.health -= damage;
            self.last_damaged = Instant::now();
        }
    }
}
