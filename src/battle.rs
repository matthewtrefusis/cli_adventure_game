use crate::enemy::Enemy;
use crate::player::Player;
use crate::prompts::read_line;
use crate::rng::around;
use crate::spell::SpellKind;

pub enum BattleResult {
    Victory { gold: u32, experience: u32 },
    Fled,
    Defeat,
}

pub fn battle(player: &mut Player, companion: &mut Player, enemy: &mut Enemy) -> BattleResult {
    println!("\n╭─ COMBAT ─────────────────────────────────────────╮");
    println!("│ {} enters combat with {}!", player.name, enemy.name);
    println!(
        "│ Enemy: {} HP  |  attack {}  |  armor {}",
        enemy.health, enemy.attack_damage, enemy.armor
    );
    println!("╰───────────────────────────────────────────────────╯");
    let mut round = 1;
    let mut companion_turn = false;

    while player.health > 0 && enemy.health > 0 {
        println!(
            "\nRound {round}  |  {}: {}/{} HP  |  {}: {}/{} HP",
            player.name,
            player.health,
            player.max_health,
            enemy.name,
            enemy.health,
            enemy.max_health
        );
        println!("[a]ttack  [f]ists  [s]pell  [p]otion  [r]un  [i]nspect");
        let Some(choice) = read_line("combat> ") else {
            return BattleResult::Fled;
        };
        let mut acted = true;
        match choice.as_str() {
            "a" | "attack" => {
                let damage = around(player.weapon.attack_damage);
                let dealt = enemy.take_damage(damage);
                println!("You strike with {} for {dealt} damage.", player.weapon.name);
            }
            "f" | "fists" => {
                let damage = around(2);
                let dealt = enemy.take_damage(damage);
                println!("You fight bare-handed for {dealt} damage.");
            }
            "s" | "spell" if !player.spells.is_empty() => {
                for (i, spell) in player.spells.iter().enumerate() {
                    println!("{}: {} ({})", i + 1, spell.name, spell.power);
                }
                let Some(index) = read_line("spell> ").and_then(|v| v.parse::<usize>().ok()) else {
                    println!("Invalid spell.");
                    continue;
                };
                if let Some(spell) = player.spells.get(index.saturating_sub(1)).cloned() {
                    match spell.kind {
                        SpellKind::Damage => {
                            let damage = around(spell.power);
                            let dealt = enemy.take_damage(damage);
                            println!("{} burns the enemy for {dealt} damage.", spell.name);
                        }
                        SpellKind::Heal => {
                            let healed = player.heal(spell.power);
                            println!("{} restores {healed} HP.", spell.name);
                        }
                    }
                } else {
                    println!("Invalid spell.");
                    acted = false;
                }
            }
            "p" | "potion" => {
                if let Some((name, healed)) = player.use_consumable() {
                    println!("{name} restores {healed} HP.");
                } else {
                    println!("You cannot use a potion at full health or have none left.");
                    acted = false;
                }
            }
            "r" | "run" => {
                println!("You retreat from {}.", enemy.name);
                return BattleResult::Fled;
            }
            "i" | "inspect" => {
                println!(
                    "{} has {}/{} HP, {} attack, {} armor.",
                    enemy.name, enemy.health, enemy.max_health, enemy.attack_damage, enemy.armor
                );
                acted = false;
            }
            _ => {
                println!("Unknown combat command.");
                acted = false;
            }
        }
        if !acted || enemy.health == 0 {
            continue;
        }

        if companion.health > 0 && companion_turn {
            let damage = around(companion.weapon.attack_damage);
            let dealt = enemy.take_damage(damage);
            println!("{} follows up for {dealt} damage.", companion.name);
        } else {
            let target = if companion.health > 0 && round % 3 == 0 {
                &mut *companion
            } else {
                &mut *player
            };
            let damage = enemy.roll_attack_damage();
            let dealt = target.take_damage(damage);
            println!("{} hits {} for {dealt} damage.", enemy.name, target.name);
        }
        companion_turn = !companion_turn;
        round += 1;
    }

    if enemy.health == 0 {
        println!(
            "Victory! You earn {} gold and {} XP.",
            enemy.reward_gold, enemy.reward_xp
        );
        BattleResult::Victory {
            gold: enemy.reward_gold,
            experience: enemy.reward_xp,
        }
    } else {
        println!("You have fallen. The expedition ends here.");
        BattleResult::Defeat
    }
}
