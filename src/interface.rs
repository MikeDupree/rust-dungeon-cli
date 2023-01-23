use std::{io::{stdout, Write}, mem::swap};
use termion;
use termion::raw::IntoRawMode;

use crate::{user::Player, spawner::Spawner};

fn is_wall(row: u16, col: u16) -> bool {
    let size = termion::terminal_size().unwrap();
    row == 0 || row == size.1 - 4 || col == 0 || col == size.0 - 1
}


/*
 * Game Screen Render
 */
pub fn render(player: &Player, spawner: &Spawner, do_render: bool) {
    //setting up stdout and going into raw mode
    let mut stdout = stdout().into_raw_mode().unwrap();
    let terminal_size = termion::terminal_size().unwrap();
    // Create Screen Output String
    let mut screen_output = String::from("");
    for row in 0..terminal_size.1 - 3 {
        for col in 0..terminal_size.0 {
            if is_wall(row, col) {
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

    screen_output.push_str(format!("Swarm Size: {:?}", spawner.enemies.len()).as_str()); 

    if do_render {
        //clearing the screen and going to top left corner
        write!(
            stdout,
            "{}{}",
            termion::cursor::Goto(1, 1),
            termion::clear::All
        )
        .unwrap();
        println!("{}", screen_output);
    }

}

