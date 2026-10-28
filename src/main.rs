mod battle;
mod consumable;
mod enemy;
mod game;
mod player;
mod prompts;
mod rng;
mod spell;
mod ui;
mod weapon;

fn main() {
    game::Game::new().run();
}
