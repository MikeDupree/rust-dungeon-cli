use std::io::{stdin, stdout, Write};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

mod interface;

#[derive(Debug)]
struct Pos(u16, u16);

// Player.rs
#[derive(Debug)]
struct Player {
    pub health: u16,
    pub xp: u16,
    pub speed: u16,
    pub pos: Pos,
    pub marker: char,
    pub vulnerable: bool,
}

impl Player {
    fn create() -> Player {
        Player {
            health: 10,
            xp: 0,
            pos: Pos { 0: 10, 1: 10 },
            speed: 1,
            marker: '&',
            vulnerable: true,
        }
    }
    fn render(&self) {
        let mut stdout = stdout().into_raw_mode().unwrap();
        write!(stdout, "\x1b[92m&\x1b[0m").ok().unwrap();
    }
    fn collides(&self, row: u16, col: u16) -> bool {
        self.pos.0 == col && self.pos.1 == row
    }
}

fn main() {
    let terminal_size = termion::terminal_size().unwrap();
    let stdin = stdin();

    //setting up stdout and going into raw mode
    let mut stdout = stdout().into_raw_mode().unwrap();
    //printing welcoming message, clearing the screen and going to left top corner with the cursor
    stdout.flush().unwrap();
    let player = Player::create();

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
    for row in 0..terminal_size.1 - 3 {
        for col in 0..terminal_size.0 {
            if interface::is_wall(row, col) {
                write!(stdout, "\x1b[33m#\x1b[0m").unwrap();
            } else if player.collides(row, col) {
                //todo use the inst
                player.render();
            } else {
                write!(stdout, " ").unwrap();
            }
        }
    }
    writeln!(stdout, "{:?}", terminal_size).unwrap();
    //detecting keydown events
    /*
    for c in stdin.keys() {
        //i reckon this speaks for itself
        match c.unwrap() {
            Key::Ctrl('h') => println!("Hello world!"),
            Key::Ctrl('q') => break,
            Key::Alt('t') => println!("termion is cool"),
            _ => (),
        }

        stdout.flush().unwrap();
    }
    */
}
