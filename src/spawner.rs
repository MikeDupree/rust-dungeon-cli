use std::sync::mpsc::Receiver;
use std::sync::mpsc::TryRecvError;
use std::sync::{Arc, Mutex};
use uuid::Uuid;

use crate::enemy::Enemy;

#[derive(Clone)]
pub struct ExperienceOrb {
    collected: bool,
    amount: u32,
    pos: (u16, u16),
}

impl ExperienceOrb {
    pub fn collides(&self, pos: (u16, u16)) -> bool {
        self.pos.0 == pos.0 && self.pos.1 == pos.1
    }

    pub fn render(&self) -> &str {
        "\x1b[33m☠\x1b[0m"
    }

    pub fn get_amount(&self) -> u32 {
        self.amount
    }

    pub fn collect(&mut self) {
        self.collected = true;
    }

    fn clone(&self) -> ExperienceOrb {
        ExperienceOrb {
            collected: self.collected,
            amount: self.amount,
            pos: self.pos,
        }
    }
}

pub struct Spawner {
    pub enemies: Vec<Enemy>,
    pub experience_orbs: Vec<ExperienceOrb>,
    enemies_limit: u16,
    player_pos: (u16, u16),
}

impl Spawner {
    pub fn new(enemies_limit: u16, player_pos: (u16, u16)) -> Spawner {
        // spawn enemies
        let mut enemies: Vec<Enemy> = vec![];
        for _n in 0..enemies_limit {
            enemies.push(Enemy::new(Uuid::new_v4()));
        }

        Spawner {
            enemies,
            experience_orbs: vec![],
            enemies_limit,
            player_pos,
        }
    }

    pub fn update_player_pos(&mut self, pos: (u16, u16)) {
        self.player_pos = pos;
    }

    pub fn update_swarm(&mut self) {
        // Remove collected experience_orbs
        let remaining_experience: Vec<ExperienceOrb> = self
            .experience_orbs
            .clone()
            .into_iter()
            .filter(|orb| !orb.collected)
            .collect();
        self.experience_orbs = remaining_experience;

        // Check for deaths
        let dead_enemies: Vec<Enemy> = self
            .enemies
            .clone()
            .into_iter()
            .filter(|enemy| enemy.get_health() <= 0)
            .collect();

        // Spawn experience_orbs for each dead enemy
        for dead_enemy in dead_enemies {
            self.experience_orbs.push(ExperienceOrb {
                collected: false,
                amount: dead_enemy.get_xp_rewards(),
                pos: dead_enemy.pos,
            });
        }

        // Spawn new enemies
        if self.enemies.len() < self.enemies_limit as usize {
            for _i in 0..5 - self.enemies.len() {
                self.enemies.push(Enemy::new(Uuid::new_v4()));
            }
        }

        self.enemies = self
            .enemies
            .clone()
            .into_iter()
            .filter(|enemy| enemy.get_health() > 0)
            .collect();

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
