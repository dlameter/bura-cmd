use bura::game::{ GameManager, GameState };
use bura::deck::shuffler;
use bura::deck::builder;
use bura::game::step;

fn main() {
    let state = GameState::new(Vec::new(), builder::bura_deck());
    let mut manager = GameManager::new(state, shuffler::shuffle);
    manager.add_step(step::draw);
    manager.add_step(step::play);
    manager.add_step(step::change_player);
    manager.add_step(step::draw);
    manager.add_step(step::play);
    manager.add_step(step::change_player);
    manager.add_step(step::score_trick);
    
    manager.start();
}
