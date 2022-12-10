#![deny(clippy::pedantic)]
#![allow(clippy::similar_names)]
#![allow(clippy::cast_sign_loss)]

use mancala::{Board, Player};
fn main() {
    println!("Setting up the game");
    let mut board = Board::new();
    let mut winner = None;
    let mut next_player = Player::A;

    while winner.is_none() {
        println!("{board}");
        println!();
        println!("Player {next_player:?}, select a hole:");

        let selected_hole = get_selected_hole();
        let selected_hole: usize = match selected_hole {
            Ok(num) => num,
            Err(_) => continue,
        };
        println!("Chose hole {selected_hole}");
        next_player = board.player_choses_hole(next_player, selected_hole);
        winner = board.test_if_won();
    }

    println!("{board}");
    println!();
    println!("Congratulations {:?}, you won!", winner.unwrap());
}

fn get_selected_hole() -> Result<usize, std::num::ParseIntError> {
    let mut user_input = String::new();
    std::io::stdin()
        .read_line(&mut user_input)
        .expect("Failed to read line");
    user_input.trim().parse()
}
