use std::sync::mpsc::Receiver;
use std::sync::mpsc::TryRecvError;
use std::sync::{Arc, Mutex};
use uuid::Uuid;

use crate::enemy::Enemy;

pub struct Spawner {
    pub enemies: Vec<Enemy>,
    enemies_limit: u16,
    player_pos: (u16, u16),
}

impl Spawner {
    pub fn new(enemies_limit: u16, player_pos: (u16, u16)) -> Spawner {
        // spawn enemies
        let mut enemies: Vec<Enemy> = vec![];
        for n in 0..enemies_limit {
            enemies.push(Enemy::new(Uuid::new_v4()));
        }

        Spawner {
            enemies,
            enemies_limit,
            player_pos,
        }
    }

    pub fn update_player_pos(&mut self, pos: (u16, u16)) {
        self.player_pos = pos;
    }

    pub fn update_swarm(&mut self) {
        // Check for deaths
        self.enemies = self.enemies.clone().into_iter().filter(|enemy| enemy.get_health() > 0).collect();

        // Move swarm towards player
        // TODO should avoid obstacles. should circle player (not group up into line)
        for enemy in &mut self.enemies {
            enemy.move_towards(self.player_pos);
        }
    }
}

pub fn control_swarm(position_update_rx: &Receiver<(u16, u16)>, spawner: Arc<Mutex<Spawner>>) {
    match position_update_rx.try_recv() {
        Ok((x, y)) => {
            let mut spawner_lock = spawner.lock().unwrap();
            spawner_lock.update_player_pos((x, y));
        }
        Err(TryRecvError::Empty) => (),
        Err(TryRecvError::Disconnected) => panic!("Channel disconnected"),
    }
}
/*
impl Clone for Spawner {
    fn clone(&self) -> Spawner {
        Spawner {
            enemies: self.enemies.clone(),
            enemies_limit: self.enemies_limit,
            pos_update_rx: self.pos_update_rx
        }
    }
}
*/
