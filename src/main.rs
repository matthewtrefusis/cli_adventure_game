mod player;
mod prompts;

use player::Player;

fn main() {
    prompts::intro();
    let class = prompts::class_choice();
    let player = Player::new(prompts::name(), class);

    println!("{player:#?}");
}
