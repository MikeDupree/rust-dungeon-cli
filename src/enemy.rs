use std::time::Instant;
use rand::Rng;

pub struct Enemy {
    id: u32,
    rewardXP: u16,
    health: u16,
    pos: (u16, u16),
    last_updated: Instant,
}

impl Enemy {
    pub fn create(eid: u32) -> Enemy {
    let mut rng = rand::thread_rng();
        Enemy {
            id: eid, //todo generate id,
            rewardXP: 1,
            health: 3,
            pos: (rng.gen_range(2..70), rng.gen_range(2..70)),
            last_updated: Instant::now(),
        }
    }

    pub fn render(&self) -> &str {
        "\x1b[92m%\x1b[0m"
    }

    pub fn collides(&self, row: u16, col: u16) -> bool {
        // todo handle damage case where collision
        // causes damage to enemy
        self.pos.0 == col && self.pos.1 == row
    }

    pub fn move_towards(&mut self, target_pos: (u16, u16)) {
        if self.last_updated.elapsed().as_millis() >= 350 {
            if self.pos.0 < target_pos.0 {
                self.pos.0 += 1
            } else {
                self.pos.0 -= 1
            }

            if self.pos.1 < target_pos.1 {
                self.pos.1 += 1
            } else {
                self.pos.1 -= 1
            }
            self.last_updated = Instant::now();
        }
    }
}
