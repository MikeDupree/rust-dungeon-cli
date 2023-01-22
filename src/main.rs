use input::{handle_input, spawn_stdin_channel};
use interface::render;
use spawner::{control_swarm, Spawner};
use std::sync::{Arc, Mutex};

mod enemy;
mod input;
mod interface;
mod spawner;
mod user;

fn main() {
    let do_render = true;
    let player = Arc::new(Mutex::new(user::Player::create()));

    // spawn enemies
    let (position_update_tx, position_update_rx) = std::sync::mpsc::channel();
    let spawner = Arc::new(Mutex::new(Spawner::new(4, player.lock().unwrap().pos)));

    // Spawn Input Thread
    let stdin_channel = spawn_stdin_channel();

    loop {
        // Control Enemy Swarm
        let spawner_control_clone = Arc::clone(&spawner);
        control_swarm(&position_update_rx, spawner_control_clone);

        // Handle Input
        // If handle_input returns true kill the game.
        let player_clone = Arc::clone(&player);
        if handle_input(player_clone, &stdin_channel, &position_update_tx) {
            break;
        }

        // Update State
        update(&mut player.lock().unwrap(), &mut spawner.lock().unwrap());

        // Render screen
        render(&player.lock().unwrap(), &spawner.lock().unwrap(), do_render);
    }
}

/*
 * Game Update
 */
fn update(player: &mut user::Player, spawner: &mut Spawner) {
    player.update();
    spawner.update_swarm();
}

