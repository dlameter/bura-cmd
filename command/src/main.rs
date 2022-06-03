use bura::deck::builder;
use bura::deck::shuffler;
use bura::game::step;
use bura::game::{GameManager, GameState};

fn main() {
    let state = GameState::new(Vec::new(), builder::bura_deck());
    let mut manager = GameManager::new(state, shuffler::rng_shuffle);
    manager.add_step(step::clear_screen);
    manager.add_step(step::confirm_player);
    manager.add_step(step::draw);
    manager.add_step(step::play);
    manager.add_step(step::change_player);
    manager.add_step(step::clear_screen);
    manager.add_step(step::confirm_player);
    manager.add_step(step::draw);
    manager.add_step(step::play);
    manager.add_step(step::clear_screen);
    manager.add_step(step::change_player);
    manager.add_step(step::score_trick);
    manager.add_step(step::declare_win);
    manager.add_step(step::change_player);
    manager.add_step(step::declare_win);
    manager.add_step(step::change_player);

    manager.start();

    runner_test();
}

fn runner_test() {
    let mut runner = Runner::new();
    runner.add(Box::new(FuncRunnable::new(step::clear_screen)));
    runner.add(Box::new(Counter { count: 0 }));
    runner.run();
}

trait Runnable {
    fn run(&mut self, game_state: &mut GameState);
}

struct Runner {
    runnables: Vec<Box<dyn Runnable>>
}

impl Runner {
    fn new() -> Runner {
        Runner { runnables: Vec::new() }
    }

    fn add(&mut self, runnable: Box<dyn Runnable>) {
        self.runnables.push(runnable);
    }

    fn run(&mut self) {
        let mut game_state = GameState::new(Vec::new(), builder::bura_deck());

        for runnable in &mut self.runnables {
            runnable.run(&mut game_state);
        }
    }
}

struct Counter {
    count: i32
}
impl Runnable for Counter {
    fn run(&mut self, _game_state: &mut GameState) {
        self.count += 1;
    }
}

struct FuncRunnable {
    func: fn(&mut GameState)
}
impl FuncRunnable {
    fn new(func: fn(&mut GameState)) -> FuncRunnable {
        FuncRunnable { func }
    }
}
impl From<fn(&mut GameState)> for FuncRunnable {
    fn from(func: fn(&mut GameState)) -> Self {
        FuncRunnable::new(func)
    }
}
impl Runnable for FuncRunnable {
    fn run(&mut self, game_state: &mut GameState) {
        (self.func)(game_state);
    }
}