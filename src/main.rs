mod battle;
mod consumable;
mod enemy;
mod player;
mod prompts;
mod spell;
mod weapon;

use battle::battle;
use enemy::Enemy;
use player::{Class, Player};
use std::thread;
use std::time::Duration;

fn main() {
    prompts::intro();
    let class = prompts::class_choice();
    let player = Player::new(prompts::name(), class);

    println!("{player:#?}");

    thread::sleep(Duration::from_secs(2));

    let mut player = player;
    let mut companion = Player::new("Rowan".to_string(), Class::Swordsman);
    let mut enemy: Enemy = prompts::first_encounter();

    battle(&mut player, &mut companion, &mut enemy);
}
