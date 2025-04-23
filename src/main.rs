mod game;

use game::{Player, TikTakToe};

fn main() {
    let mut game = TikTakToe::new(3);
    let player = Player { value: 'X' };
    game.play_action(&player, 3);
}
