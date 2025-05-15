use super::{State, TikTakToe};

pub fn render(game: &TikTakToe) {
    println!("{}", make_display(game))
}

fn make_display(game: &TikTakToe) -> String {
    let board = game.get_board();
    let mut display = String::new();
    let board_size = board.len().isqrt();

    match &game.state {
        State::Win(data) => {
            for i in 0..board_size {
                ((i * board_size)..(i * board_size + board_size)).for_each(|value| {
                    let value = board[value].unwrap_or(' ').to_string();
                    let win_position = data.board_indexes.contains(&i);
                    let content = if win_position {
                        format!("!{}!", value)
                    } else {
                        format!("|{}|", value)
                    };
                    display.push_str(&content);
                });
                display.push('\n');
            }
            display
        }
        State::Draw => {
            for i in 0..board_size {
                ((i * board_size)..(i * board_size + board_size)).for_each(|value| {
                    let value = board[value].unwrap_or(' ').to_string();
                    let content = format!("[{}]", value);
                    display.push_str(&content);
                });
                display.push('\n');
            }
            display
        }
        State::Continue => {
            for i in 0..board_size {
                ((i * board_size)..(i * board_size + board_size)).for_each(|value| {
                    let value = board[value].unwrap_or(' ').to_string();
                    display.push_str(&format!("|{}|", value));
                });
                display.push('\n');
            }
            display
        }
    }
}
