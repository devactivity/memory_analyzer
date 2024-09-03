#[cfg(feature = "analyzer")]
pub mod analyzer;

#[cfg(feature = "game_engine")]
pub mod game_engine;

#[cfg(feature = "analyzer")]
fn run_memory_analyzer() {
    analyzer::main();
}

#[cfg(feature = "game_engine")]
fn run_game_engine() {
    game_engine::main();
}

fn main() {
    #[cfg(feature = "analyzer")]
    run_memory_analyzer();

    #[cfg(feature = "game_engine")]
    run_game_engine();
}
