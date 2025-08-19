mod game;

use std::io;

use game::renderer::render;
use game::{Player, TikTakToe};

fn main() {
    let size = 5;
    let number_of_boards = size * size;
    let mut game = TikTakToe::new(size);
    let player_one = Player { value: 'X' };
    let player_two = Player { value: 'O' };
    let mut current_player = &player_one;
    loop {
        println!("TikTakToe");
        println!("Select the position:");
        render(&game);
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Error to get user input.");
        let input = input.trim();
        if input == "exit" {
            break;
        }
        let input: usize = match input.parse() {
            Ok(value) => {
                if !(1..=number_of_boards).contains(&value) {
                    println!(
                        "Only number between 1 and {} are allowed.",
                        &number_of_boards
                    );
                    continue;
                }
                value
            }
            Err(_) => {
                println!(
                    "Only number between 1 and {} are allowed.",
                    &number_of_boards
                );
                continue;
            }
        };
        dbg!(&game);
        game.play_action(current_player, input - 1);
        if *current_player == player_one {
            current_player = &player_two;
        } else {
            current_player = &player_one;
        }
        dbg!(&game);
    }
}
