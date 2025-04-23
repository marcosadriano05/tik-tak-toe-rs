#[derive(Debug, PartialEq)]
enum State {
    Win(WinData),
    Draw,
    Continue,
}

#[derive(Debug, PartialEq)]
struct WinData {
    board_indexes: Vec<usize>,
}

pub struct Player {
    pub value: char,
}

pub struct TikTakToe {
    square: usize,
    board: Vec<Option<char>>,
}

fn check_eq_from_option_char(left: &Option<char>, right: &char) -> bool {
    match left {
        Some(v) => v.eq_ignore_ascii_case(right),
        None => false,
    }
}

impl TikTakToe {
    pub fn new(square: usize) -> Self {
        TikTakToe {
            square,
            board: vec![None; square.pow(2)],
        }
    }

    pub fn play_action(&mut self, player: &Player, index: usize) {
        if let Some(item) = self.board.get_mut(index) {
            match item {
                None => {
                    *item = Some(player.value);
                }
                _ => (),
            }
        }
    }

    fn has_win(&self, player: &Player) -> State {
        let square = &self.square;

        let mut indexes_main_diagonal = vec![0; *square];
        let mut accumulator_main_diagonal = 0;
        let mut indexes_secondary_diagonal = vec![0; *square];
        let mut accumulator_secondary_diagonal = square - 1;

        for i in 0..(*square) {
            let win = self.board[(i * square)..(i * square + square)]
                .iter()
                .all(|&item| check_eq_from_option_char(&item, &player.value));
            if win {
                return State::Win(WinData {
                    board_indexes: vec![i * square, i * square + 1, i * square + 2],
                });
            }

            let mut vertical_indexes = Vec::new();
            for j in 0..(*square) {
                vertical_indexes.push(j * square + i);
            }
            let win = vertical_indexes
                .iter()
                .all(|&item| check_eq_from_option_char(&self.board[item], &player.value));
            if win {
                return State::Win(WinData {
                    board_indexes: vec![i, i + square, i + square + square],
                });
            }

            indexes_main_diagonal[i] = accumulator_main_diagonal;
            accumulator_main_diagonal = accumulator_main_diagonal + square + 1;
            indexes_secondary_diagonal[i] = accumulator_secondary_diagonal;
            accumulator_secondary_diagonal = accumulator_secondary_diagonal + square - 1;
        }

        let win = indexes_main_diagonal
            .iter()
            .all(|&item| check_eq_from_option_char(&self.board[item], &player.value));
        if win {
            return State::Win(WinData {
                board_indexes: vec![
                    indexes_main_diagonal[0],
                    indexes_main_diagonal[1],
                    indexes_main_diagonal[2],
                ],
            });
        }

        let win = indexes_secondary_diagonal
            .iter()
            .all(|&item| check_eq_from_option_char(&self.board[item], &player.value));
        if win {
            return State::Win(WinData {
                board_indexes: vec![
                    indexes_secondary_diagonal[0],
                    indexes_secondary_diagonal[1],
                    indexes_secondary_diagonal[2],
                ],
            });
        }

        if self.board.iter().all(|&item| item.is_some()) {
            return State::Draw;
        }

        State::Continue
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn user_play_action() {
        let mut game = TikTakToe::new(3);
        let player = Player { value: 'O' };
        assert_eq!(game.board.iter().all(|&item| item.is_none()), true);
        game.play_action(&player, 3);
        assert_eq!(game.board.get(3), Some(&Some('O')));
    }

    #[test]
    fn horizontal_win() {
        let mut game = TikTakToe::new(3);
        game.board = vec![
            Some('X'),
            Some('X'),
            Some('O'),
            Some('O'),
            Some('X'),
            Some('O'),
            Some('O'),
            Some('O'),
            Some('O'),
        ];
        let player = Player { value: 'O' };
        assert_eq!(
            game.has_win(&player),
            State::Win(WinData {
                board_indexes: vec![6, 7, 8]
            })
        );
    }

    #[test]
    fn vertical_win() {
        let mut game = TikTakToe::new(3);
        game.board = vec![
            Some('O'),
            Some('X'),
            Some('O'),
            Some('O'),
            Some('X'),
            Some('O'),
            Some('O'),
            Some('O'),
            Some('X'),
        ];
        let player = Player { value: 'O' };
        assert_eq!(
            game.has_win(&player),
            State::Win(WinData {
                board_indexes: vec![0, 3, 6]
            })
        );
    }

    #[test]
    fn main_diagonal_win() {
        let mut game = TikTakToe::new(3);
        game.board = vec![
            Some('O'),
            Some('X'),
            Some('O'),
            Some('O'),
            Some('O'),
            Some('X'),
            Some('X'),
            Some('O'),
            Some('O'),
        ];
        let player = Player { value: 'O' };
        assert_eq!(
            game.has_win(&player),
            State::Win(WinData {
                board_indexes: vec![0, 4, 8]
            })
        );
    }

    #[test]
    fn secondary_diagonal_win() {
        let mut game = TikTakToe::new(3);
        game.board = vec![
            Some('O'),
            Some('X'),
            Some('O'),
            Some('X'),
            Some('O'),
            Some('X'),
            Some('O'),
            Some('X'),
            Some('X'),
        ];
        let player = Player { value: 'O' };
        assert_eq!(
            game.has_win(&player),
            State::Win(WinData {
                board_indexes: vec![2, 4, 6]
            })
        );
    }

    #[test]
    fn draw() {
        let mut game = TikTakToe::new(3);
        game.board = vec![
            Some('O'),
            Some('O'),
            Some('X'),
            Some('X'),
            Some('X'),
            Some('O'),
            Some('O'),
            Some('X'),
            Some('O'),
        ];
        let player = Player { value: 'O' };
        assert_eq!(game.has_win(&player), State::Draw);
    }

    #[test]
    fn continue_game() {
        let mut game = TikTakToe::new(3);
        game.board = vec![Some('O'), None, None, None, None, None, None, None, None];
        let player = Player { value: 'O' };
        assert_eq!(game.has_win(&player), State::Continue);
    }
}
