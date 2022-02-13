mod configuration;
mod game;
mod util;

use crate::game::run;

fn main() {
    run::Game::new().run();
}
