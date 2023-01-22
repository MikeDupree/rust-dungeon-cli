use std::sync::mpsc::{Receiver, Sender};
use std::sync::{Arc, Mutex};
use uuid::Uuid;

use crate::enemy::Enemy;

pub struct Spawner {
    pub enemies: Vec<Enemy>,
    enemies_limit: u16,
}

impl Spawner {
    pub fn new(enemies_limit: u16) -> Spawner {
        // spawn enemies
        let mut enemies: Vec<Enemy> = vec![];
        for n in 0..enemies_limit {
            enemies.push(Enemy::new(Uuid::new_v4()));
        }

        Spawner {
            enemies,
            enemies_limit,
        }
    }

}
impl Clone for Spawner {
    fn clone(&self) -> Spawner {
        Spawner {
            enemies: self.enemies.clone(),
            enemies_limit: self.enemies_limit,
        }
    }
}
