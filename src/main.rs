use std::fmt::write;
use std::io;
use std::io::Read;
use std::io::{stdin, stdout, Stdin, Write};
use std::sync::mpsc;
use std::sync::mpsc::Receiver;
use std::sync::mpsc::TryRecvError;
use std::{thread, time};

use termion;
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

use crate::user::Player;

mod interface;
mod user;

fn main() {
    let terminal_size = termion::terminal_size().unwrap();
    let stdin = stdin();

    //setting up stdout and going into raw mode
    let mut stdout = stdout().into_raw_mode().unwrap();
    //printing welcoming message, clearing the screen and going to left top corner with the cursor
    stdout.flush().unwrap();
    let player = user::Player::create();

    //clearing the screen and going to top left corner
    /*
    write!(
        stdout,
        "{}{}",
        termion::cursor::Goto(1, 1),
        termion::clear::All
    )
    .unwrap();
    */
    let stdin_channel = spawn_stdin_channel();
    loop {
        write!(
            stdout,
            "{}{}",
            termion::cursor::Goto(1, 1),
            termion::clear::All
        )
        .unwrap();
        match stdin_channel.try_recv() {
            Ok(key) => {
                println!("hit {:?}", key);
                match key {
                    Key::Char('e') => println!("u"),
                    Key::Char('d') => println!("u"),
                    Key::Char('s') => println!("u"),
                    Key::Char('f') => println!("u"),
                    Key::Ctrl('q') => break,
                    Key::Alt('t') => println!("termion is cool"),
                    _ => (),
                }
            }
            Err(TryRecvError::Empty) => (),
            Err(TryRecvError::Disconnected) => panic!("Channel disconnected"),
        }

        let mut screen_output = String::from("");
        for row in 0..terminal_size.1 - 3 {
            for col in 0..terminal_size.0 {
                if interface::is_wall(row, col) {
                    screen_output.push_str("\x1b[33m#\x1b[0m")
                } else if player.collides(row, col) {
                    screen_output.push_str(player.render());
                } else {
                    screen_output.push_str(" ");
                }
            }
        }
        println!("{}", screen_output);
        sleep(100);
    }
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

fn sleep(millis: u64) {
    let duration = time::Duration::from_millis(millis);
    thread::sleep(duration);
}
