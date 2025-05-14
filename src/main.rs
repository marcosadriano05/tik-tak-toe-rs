mod game;

use game::{Player, TikTakToe, render};

fn main() {
    let mut game = TikTakToe::new(3);
    dbg!(&game);
    let player = Player { value: 'X' };
    game.play_action(&player, 3);
    dbg!(&game);
    render(&game);
}
