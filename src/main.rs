use enemy::Enemy;
use event_system::create_event_system;
use spawner::Spawner;
use std::cell::{RefCell, RefMut};
use std::io::{stdin, stdout, Stdout, Write};
use std::rc::Rc;
use std::sync::mpsc::TryRecvError;
use std::sync::mpsc::{Receiver, Sender};
use std::sync::{mpsc, MutexGuard};
use std::sync::{Arc, Mutex};
use std::thread::sleep;
use std::time::{Duration, Instant, SystemTime};
use std::{thread, time};
use termion;
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::{IntoRawMode, RawTerminal};
use uuid::Uuid;

mod enemy;
mod interface;
mod spawner;
mod user;

const MS_PER_UPDATE: u32 = 15000;
fn main() {
    let do_render = true;

    //stdout.flush().unwrap();
    let mut player = user::Player::create();
    // TODO issue: 1.1 register handle_input
    //render_event.register_key_down(player.handle_input);

    // spawn enemies
    let (position_update_tx, position_update_rx) = std::sync::mpsc::channel();
    let mut spawner = Arc::new(Mutex::new(Spawner::new(4)));
    let spawner_clone = spawner.clone();

    // Spawn Input Thread
    let stdin_channel = spawn_stdin_channel();
    let mut last_updated = Instant::now();
    loop {
        // Get Input
        match stdin_channel.try_recv() {
            Ok(key) => {
                // TODO figure out issue 1.1 and remove this direct call on player
                player.handle_input(key);

                match key {
                    Key::Ctrl('c') => break,
                    Key::Ctrl('q') => break,
                    _ => (),
                }
            }
            Err(TryRecvError::Empty) => (),
            Err(TryRecvError::Disconnected) => panic!("Channel disconnected"),
        }

        // Update State
        if update(&mut player, &position_update_tx) {
            println!("Calling pos update");
            position_update_tx.send(player.pos).unwrap();
            last_updated = Instant::now();
        }

        // Render Screen
        render(&player, &spawner_clone.lock().unwrap(), do_render);
    }
}

fn update(player: &mut user::Player, position_update_tx: &Sender<(u16, u16)>) -> bool {
    player.update();
    true
}

fn render(player: &user::Player, spawner: &Spawner, do_render: bool) {
    //setting up stdout and going into raw mode
    let mut stdout = stdout().into_raw_mode().unwrap();
    let terminal_size = termion::terminal_size().unwrap();
    // Create Screen Output String
    let mut screen_output = String::from("");
    for row in 0..terminal_size.1 - 3 {
        for col in 0..terminal_size.0 {
            if interface::is_wall(row, col) {
                screen_output.push_str("\x1b[33m█\x1b[0m")
            } else if player.collides(row, col) {
                screen_output.push_str(player.render());
            } else if player.base_attack_collides(row, col) {
                screen_output.push_str(player.render_base_attack());
            } else {
                let mut enemy_rendered = false;
                for enemy in &spawner.enemies {
                    if enemy.collides(row, col) {
                        screen_output.push_str(enemy.render());
                        enemy_rendered = true;
                        break;
                    }
                }
                if !enemy_rendered {
                    screen_output.push_str(" ");
                }
            }
        }
    }

    if do_render {
        //clearing the screen and going to top left corner
        /*write!(
                stdout,
                "{}{}",
                termion::cursor::Goto(1, 1),
                termion::clear::All
            )
            .unwrap();
        */
        println!("{}", screen_output);
    }

    //sleep(100);
}

fn spawn_stdin_channel() -> Receiver<Key> {
    let (tx, rx) = mpsc::channel::<Key>();
    thread::spawn(move || loop {
        let stdin = stdin();
        for c in stdin.keys() {
            tx.send(c.unwrap()).unwrap();
            break;
        }
    });
    rx
}

fn spawn_stdin_channel() -> Receiver<Key> {
    let (tx, rx) = mpsc::channel::<Key>();
    thread::spawn(move || loop {
        let stdin = stdin();
        for c in stdin.keys() {
            tx.send(c.unwrap()).unwrap();
            break;
        }
    });
    rx
}

fn sleepFor(millis: u64) {
    let duration = time::Duration::from_millis(millis);
    thread::sleep(duration);
}
