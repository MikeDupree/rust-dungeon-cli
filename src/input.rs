use std::io::stdin;
use std::sync::mpsc::channel;
use std::sync::mpsc::TryRecvError;
use std::sync::mpsc::{Receiver, Sender};
use std::sync::{Arc, Mutex};
use std::thread;
use termion::event::Key;

use termion;
use termion::input::TermRead;

use crate::user;

/*
 * Input Capture Thread
 */
pub fn spawn_stdin_channel() -> Receiver<Key> {
    let (tx, rx) = channel::<Key>();
    thread::spawn(move || loop {
        let stdin = stdin();
        for c in stdin.keys() {
            tx.send(c.unwrap()).unwrap();
            break;
        }
    });
    rx
}

/*
 * Input Event Handler
 */
pub fn handle_input(
    player: Arc<Mutex<user::Player>>,
    stdin_channel: &Receiver<Key>,
    position_update_tx: &Sender<(u16, u16)>,
) -> bool {
    match stdin_channel.try_recv() {
        Ok(key) => {
            // TODO figure out issue 1.1 and remove this direct call on player
            let mut player_lock = player.lock().unwrap();
            player_lock.handle_input(key);
            position_update_tx.send(player_lock.pos).unwrap();
            match key {
                Key::Ctrl('c') => return true,
                Key::Ctrl('q') => return true,
                _ => return false,
            }
        }
        Err(TryRecvError::Empty) => (),
        Err(TryRecvError::Disconnected) => panic!("Channel disconnected"),
    }
    false
}
