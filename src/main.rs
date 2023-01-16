use std::io::{stdin, stdout, Write};
use std::sync::mpsc;
use std::sync::mpsc::Receiver;
use std::sync::mpsc::TryRecvError;
use std::{thread, time};

use event_system::create_event_system;

use termion;
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

mod enemy;
mod interface;
mod user;

create_event_system! {
    RenderEvent
    KeyDown {
        key: Key,
    }
}

fn main() {
    let do_render = true;
    let mut render_event = RenderEvent::new();
    let terminal_size = termion::terminal_size().unwrap();

    //setting up stdout and going into raw mode
    let mut stdout = stdout().into_raw_mode().unwrap();
    //stdout.flush().unwrap();
    let mut player = user::Player::create();
    // TODO issue: 1.1 register handle_input
    //render_event.register_key_down(player.handle_input);

    let mut enemy = enemy::Enemy::create();

    //clearing the screen and going to top left corner
    if do_render {}

    // Spawn Input Thread
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
                // TODO figure out issue 1.1 and remove this direct call on player
                player.handle_input(key);

                // Fire key down event
                render_event.fire_key_down(EventKeyDown { key: key });
                match key {
                    Key::Ctrl('q') => break,
                    _ => (),
                }
            }
            Err(TryRecvError::Empty) => (),
            Err(TryRecvError::Disconnected) => panic!("Channel disconnected"),
        }

        // Create Screen Output String
        let mut screen_output = String::from("");
        for row in 0..terminal_size.1 - 3 {
            for col in 0..terminal_size.0 {
                if interface::is_wall(row, col) {
                    screen_output.push_str("\x1b[33m█\x1b[0m")
                } else if player.collides(row, col) {
                    screen_output.push_str(player.render());
                } else if enemy.collides(row, col) {
                    screen_output.push_str(enemy.render());
                } else {
                    screen_output.push_str(" ");
                }
            }
        }

        if do_render {
            write!(
                stdout,
                "{}{}",
                termion::cursor::Goto(1, 1),
                termion::clear::All
            )
            .unwrap();
            println!("{}", screen_output);
        }

        // Fire render event

        //sleep(100);
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
