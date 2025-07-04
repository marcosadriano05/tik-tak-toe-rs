pub mod renderer;

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

#[derive(PartialEq)]
pub struct Player {
    pub value: char,
}

#[derive(Debug)]
pub struct TikTakToe {
    square: usize,
    board: Vec<Option<char>>,
    state: State,
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
            state: State::Continue,
        }
    }

    pub fn get_board(&self) -> &Vec<Option<char>> {
        &self.board
    }

    pub fn play_action(&mut self, player: &Player, index: usize) {
        let State::Continue = self.state else {
            return;
        };

        if let Some(item) = self.board.get_mut(index) {
            if item.is_none() {
                *item = Some(player.value);
                self.state = self.calculate_state(player);
            }
        }
    }

    fn calculate_state(&self, player: &Player) -> State {
        let square = &self.square;

        let mut indexes_main_diagonal = vec![0; *square];
        let mut current_main_diagonal_index = 0;
        let mut indexes_secondary_diagonal = vec![0; *square];
        let mut current_secondary_diagonal_index = square - 1;

        for i in 0..(*square) {
            let horizontal_indexes: Vec<usize> = ((i * square)..(i * square + square)).collect();
            if let Some(value) = self.has_winner(horizontal_indexes, player) {
                return State::Win(value);
            }

            let vertical_indexes: Vec<usize> =
                (0..(*square)).map(|item| item * square + i).collect();
            if let Some(value) = self.has_winner(vertical_indexes, player) {
                return State::Win(value);
            }

            indexes_main_diagonal[i] = current_main_diagonal_index;
            current_main_diagonal_index = current_main_diagonal_index + square + 1;
            indexes_secondary_diagonal[i] = current_secondary_diagonal_index;
            current_secondary_diagonal_index = current_secondary_diagonal_index + square - 1;
        }

        if let Some(value) = self.has_winner(indexes_main_diagonal, player) {
            return State::Win(value);
        }

        if let Some(value) = self.has_winner(indexes_secondary_diagonal, player) {
            return State::Win(value);
        }

        if self.board.iter().all(|&item| item.is_some()) {
            return State::Draw;
        }

        State::Continue
    }

    fn has_winner(&self, indexes: Vec<usize>, player: &Player) -> Option<WinData> {
        let win = indexes
            .iter()
            .all(|&item| check_eq_from_option_char(&self.board[item], &player.value));
        if win {
            return Some(WinData {
                board_indexes: indexes,
            });
        }

        None
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
    fn only_act_if_game_state_is_continue() {
        let mut game = TikTakToe::new(3);
        let player = Player { value: 'X' };
        game.board = vec![
            Some('X'),
            Some('O'),
            Some('O'),
            None,
            Some('X'),
            Some('O'),
            None,
            None,
            None,
        ];
        game.play_action(&player, 8);
        assert_eq!(
            game.state,
            State::Win(WinData {
                board_indexes: vec![0, 4, 8]
            })
        );
        let player = Player { value: 'O' };
        game.play_action(&player, 3);
        assert_eq!(game.board.get(3), Some(&None));
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
            game.calculate_state(&player),
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
            game.calculate_state(&player),
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
            game.calculate_state(&player),
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
            game.calculate_state(&player),
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
        assert_eq!(game.calculate_state(&player), State::Draw);
    }

    #[test]
    fn continue_game() {
        let mut game = TikTakToe::new(3);
        game.board = vec![Some('O'), None, None, None, None, None, None, None, None];
        let player = Player { value: 'O' };
        assert_eq!(game.calculate_state(&player), State::Continue);
    }
}
