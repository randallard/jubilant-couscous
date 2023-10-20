// main.rs
//
// runs the game
//
// treat the board like an led array? all on or off except for players positions 

#[derive(Debug)]
pub enum GameState {
    Ready,
}

fn main() {
    let game_state = GameState::Ready;
    println!("Game {:?}",game_state);
}
