use std::io::{stdout, Write};
use termion;
use termion::raw::IntoRawMode;

use crate::{spawner::Spawner, user::Player};

fn is_wall(row: u16, col: u16) -> bool {
    let size = termion::terminal_size().unwrap();
    row == 0 || row == size.1 - 4 || col == 0 || col == size.0 - 1
}

/*
 * Game Screen Render
 */
pub fn render(player: &Player, spawner: &mut Spawner, do_render: bool) {
    //setting up stdout and going into raw mode
    let mut stdout = stdout().into_raw_mode().unwrap();
    let terminal_size = termion::terminal_size().unwrap();
    // Create Screen Output String
    let mut screen_output = String::from("");
    for row in 0..terminal_size.1 - 3 {
        for col in 0..terminal_size.0 {
            let mut render_str = " ";

            // Render Output, priority highest last.
            if is_wall(row, col) {
                render_str = "\x1b[30m█\x1b[0m";
            }
            if player.base_attack_collides(row, col, true) {
                render_str = player.render_base_attack(row, col);
            }
            for xp_orb in &spawner.experience_orbs {
                if xp_orb.collides((col, row)) {
                    render_str = xp_orb.render();
                    break;
                }
            }
            for enemy in &mut spawner.enemies {
                if enemy.collides(row, col) {
                    render_str = enemy.render();
                    if player.base_attack_collides(row, col, false) {
                        enemy.take_damage(1);
                        render_str = "\x1b[31moͯ\x1b[0m";
                    }
                    break;
                }
            }

            if player.collides(row, col) {
                render_str = player.render();
            }
            // Attach Output string to line string
            screen_output.push_str(render_str);
        }
    }

    screen_output.push_str("\x1b[30m>\x1b[0m \x1b[31mDungeon\x1b[0m \x1b[33mC\x1b[0m\x1b[32mL\x1b[0m\x1b[36mI\x1b[0m \x1b[30m$\x1b[0m");

    screen_output.push_str(format!(" XP: {:?} ", player.xp).as_str());
    screen_output.push_str(format!(" Health: {:?} ", player.health).as_str());
    if player.health <= 0 {
        screen_output.push_str("  \x1b[31mGAME OVER\x1b[0m   ");
    }
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
