use bura::game::{ GameManager, GameState };
use bura::deck::shuffler;
use bura::deck::builder;

fn main() {
    let state = GameState::new(Vec::new(), builder::bura_deck());
    let mut manager = GameManager::new(state, shuffler::shuffle);
    
    manager.start();
}
