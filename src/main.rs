mod game_state;
mod states;
mod player;

use game_state::GlobalGameState;

fn main() {
    let mut g_game_state: GlobalGameState = GlobalGameState::new();

    g_game_state.run_game();
}
