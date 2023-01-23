use rand::Rng;
use std::sync::mpsc::{Receiver, Sender};
use std::{thread::current, time::Instant};
use uuid::Uuid;

#[derive(Clone, Debug)]
pub struct Enemy {
    id: Uuid,
    xp_rewards: u16,
    health: u16,
    pub pos: (u16, u16),
    last_updated: Instant,
    health_updated: Instant,
}

impl Enemy {
    pub fn new(eid: Uuid) -> Enemy {
        let mut rng = rand::thread_rng();
        Enemy {
            id: eid,
            xp_rewards: 1,
            health: 3,
            pos: (rng.gen_range(2..70), rng.gen_range(2..70)),
            last_updated: Instant::now(),
            health_updated: Instant::now(),
        }
    }

    pub fn get_health(&self) -> u16 {
        self.health
    }

    pub fn render(&self) -> &str {
        "\x1b[92m%\x1b[0m"
    }

    pub fn collides(&self, row: u16, col: u16) -> bool {
        // todo handle damage case where collision
        // causes damage to enemy
        self.pos.0 == col && self.pos.1 == row
    }

    pub fn take_damage(&mut self, dmg: u16) {
        if self.health_updated.elapsed().as_millis() >= 350 {
            if self.health > 0 {
                self.health -= dmg;
                if self.health == 0 {
                    println!("Enemy Dead");
                }
                self.health_updated = Instant::now();
            }
        }
    }

    pub fn move_towards(&mut self, target_pos: (u16, u16)) {
        if self.last_updated.elapsed().as_millis() >= 350 {
            let current_pos = self.pos;

            // Y axis movement
            if self.pos.1 < target_pos.1 {
                self.pos.1 += 1
            } else if self.pos.1 > target_pos.1 {
                self.pos.1 -= 1
            }

            // X axis movement
            if self.pos.0 < target_pos.0 {
                self.pos.0 += 1
            } else if self.pos.0 > target_pos.0 {
                self.pos.0 -= 1
            }

            // TODO a way for an instance of enemy to know if there is something obstructing it.
            //if self.check_mob_collisions(enemies) {
            // if new move causes collision with another enemy
            // revert back to original pos, this enemy doesnt move.
            // TODO figure out better ai logic for enemy movement
            // self.pos = current_pos;
            //}
            self.last_updated = Instant::now();
        }
    }

    pub fn check_mob_collisions(&self, enemies: &Vec<Enemy>) -> bool {
        for enemy in enemies {
            if self.id != enemy.id && self.pos.0 == enemy.pos.0 && self.pos.1 == enemy.pos.1 {
                return true;
            }
        }
        false
    }

    fn clone(&self) -> Enemy {
        Enemy {
            id: self.id,
            xp_rewards: self.xp_rewards,
            health: self.health,
            pos: self.pos,
            last_updated: self.last_updated,
            health_updated: self.health_updated,
        }
    }
}
