#[derive(Debug, PartialEq)]
pub enum State {
    Win(WinData),
    Draw,
    Continue,
}

#[derive(Debug, PartialEq)]
pub struct WinData {
    pub board_indexes: Vec<usize>,
}

#[derive(PartialEq)]
pub struct Player {
    pub value: bool,
}

#[derive(Debug)]
pub struct TikTakToe {
    board: [Option<bool>; 9],
    pub state: State,
}

impl TikTakToe {
    pub fn new() -> Self {
        TikTakToe {
            board: [None; 9],
            state: State::Continue,
        }
    }

    pub fn get_board(&self) -> &[Option<bool>] {
        &self.board
    }

    pub fn play_action(&mut self, player: &Player, index: usize) {
        let State::Continue = self.state else {
            return;
        };

        if let Some(item) = self.board.get_mut(index)
            && item.is_none()
        {
            *item = Some(player.value);
            self.state = self.calculate_state(player);
        }
    }

    fn calculate_state(&self, player: &Player) -> State {
        let square = 3;

        let mut indexes_main_diagonal = vec![0; square];
        let mut current_main_diagonal_index = 0;
        let mut indexes_secondary_diagonal = vec![0; square];
        let mut current_secondary_diagonal_index = square - 1;

        for i in 0..square {
            let horizontal_indexes: Vec<usize> = ((i * square)..(i * square + square)).collect();
            if let Some(value) = self.has_winner(horizontal_indexes, player) {
                return State::Win(value);
            }

            let vertical_indexes: Vec<usize> = (0..square).map(|item| item * square + i).collect();
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
        indexes
            .iter()
            .all(|&item| self.board[item].is_some_and(|x| !(x ^ player.value)))
            .then_some(WinData {
                board_indexes: indexes,
            })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn user_play_action() {
        let mut game = TikTakToe::new();
        let player = Player { value: true };
        assert_eq!(game.board.iter().all(|&item| item.is_none()), true);
        game.play_action(&player, 3);
        assert_eq!(game.board.get(3), Some(&Some(true)));
    }

    #[test]
    fn only_act_if_game_state_is_continue() {
        let mut game = TikTakToe::new();
        let player = Player { value: true };
        game.board = [
            Some(true),
            Some(false),
            Some(false),
            None,
            Some(true),
            Some(false),
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
        let player = Player { value: false };
        game.play_action(&player, 3);
        assert_eq!(game.board.get(3), Some(&None));
    }

    #[test]
    fn horizontal_win() {
        let mut game = TikTakToe::new();
        game.board = [
            Some(true),
            Some(true),
            Some(false),
            Some(false),
            Some(true),
            Some(false),
            Some(false),
            Some(false),
            Some(false),
        ];
        let player = Player { value: false };
        assert_eq!(
            game.calculate_state(&player),
            State::Win(WinData {
                board_indexes: vec![6, 7, 8]
            })
        );
    }

    #[test]
    fn vertical_win() {
        let mut game = TikTakToe::new();
        game.board = [
            Some(false),
            Some(true),
            Some(false),
            Some(false),
            Some(true),
            Some(false),
            Some(false),
            Some(false),
            Some(true),
        ];
        let player = Player { value: false };
        assert_eq!(
            game.calculate_state(&player),
            State::Win(WinData {
                board_indexes: vec![0, 3, 6]
            })
        );
    }

    #[test]
    fn main_diagonal_win() {
        let mut game = TikTakToe::new();
        game.board = [
            Some(false),
            Some(true),
            Some(false),
            Some(false),
            Some(false),
            Some(true),
            Some(true),
            Some(false),
            Some(false),
        ];
        let player = Player { value: false };
        assert_eq!(
            game.calculate_state(&player),
            State::Win(WinData {
                board_indexes: vec![0, 4, 8]
            })
        );
    }

    #[test]
    fn secondary_diagonal_win() {
        let mut game = TikTakToe::new();
        game.board = [
            Some(false),
            Some(true),
            Some(false),
            Some(true),
            Some(false),
            Some(true),
            Some(false),
            Some(true),
            Some(true),
        ];
        let player = Player { value: false };
        assert_eq!(
            game.calculate_state(&player),
            State::Win(WinData {
                board_indexes: vec![2, 4, 6]
            })
        );
    }

    #[test]
    fn draw() {
        let mut game = TikTakToe::new();
        game.board = [
            Some(false),
            Some(false),
            Some(true),
            Some(true),
            Some(true),
            Some(false),
            Some(false),
            Some(true),
            Some(false),
        ];
        let player = Player { value: false };
        assert_eq!(game.calculate_state(&player), State::Draw);
    }

    #[test]
    fn continue_game() {
        let mut game = TikTakToe::new();
        game.board = [Some(false), None, None, None, None, None, None, None, None];
        let player = Player { value: false };
        assert_eq!(game.calculate_state(&player), State::Continue);
    }
}
