use crate::enemy::Enemy;
use crate::player::Player;

pub fn battle(player: &mut Player, enemy: &mut Enemy) {
    println!("\nBattle started: {} vs {}", player.name, enemy.name);

    while player.health > 0 && enemy.health > 0 {
        enemy.health = enemy.health.saturating_sub(player.weapon.attack_damage);
        println!(
            "{} attacks with {} for {} damage. {} health: {}",
            player.name,
            player.weapon.name,
            player.weapon.attack_damage,
            enemy.name,
            enemy.health
        );

        if enemy.health == 0 {
            break;
        }

        player.health = player.health.saturating_sub(enemy.attack_damage);
        println!(
            "{} attacks for {} damage. {} health: {}",
            enemy.name, enemy.attack_damage, player.name, player.health
        );
    }

    if player.health > 0 {
        println!("{} wins!", player.name);
    } else {
        println!("{} wins!", enemy.name);
    }
}