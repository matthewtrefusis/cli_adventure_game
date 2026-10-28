use crate::enemy::Enemy;
use crate::player::Player;
use crate::prompts::{self, BattleChoice, HealTarget};
use crate::spell::SpellKind;

pub fn battle(player: &mut Player, companion: &mut Player, enemy: &mut Enemy) {
    println!("\nBattle started: {} vs {}", player.name, enemy.name);
    println!("{} is fighting alongside {}.", companion.name, player.name);

    let mut fled = false;
    let mut enemy_targets_companion = false;

    while player.health > 0 && enemy.health > 0 {
        match prompts::battle_choice(!player.spells.is_empty(), player.has_consumables()) {
            BattleChoice::Weapon => {
                enemy.health = enemy.health.saturating_sub(player.weapon.attack_damage);
                println!(
                    "{} attacks with {} for {} damage. {} health: {}",
                    player.name,
                    player.weapon.name,
                    player.weapon.attack_damage,
                    enemy.name,
                    enemy.health
                );
            }
            BattleChoice::Fists => {
                const FIST_DAMAGE: u32 = 2;

                enemy.health = enemy.health.saturating_sub(FIST_DAMAGE);
                println!(
                    "{} attacks with their fists for {} damage. {} health: {}",
                    player.name, FIST_DAMAGE, enemy.name, enemy.health
                );
            }
            BattleChoice::Spell => {
                let spell_index = prompts::spell_choice(&player.spells);
                let spell = player.spells[spell_index].clone();

                match spell.kind {
                    SpellKind::Damage => {
                        enemy.health = enemy.health.saturating_sub(spell.power);
                        println!(
                            "{} casts {} for {} damage. {} health: {}",
                            player.name, spell.name, spell.power, enemy.name, enemy.health
                        );
                    }
                    SpellKind::Heal => {
                        let target = prompts::heal_target(companion.health > 0);
                        let (target_name, healing) = match target {
                            HealTarget::SelfTarget => {
                                let healing = player.heal(spell.power);
                                (player.name.clone(), healing)
                            }
                            HealTarget::Companion => {
                                let healing = companion.heal(spell.power);
                                (companion.name.clone(), healing)
                            }
                        };

                        println!(
                            "{} casts {} on {} and restores {} health.",
                            player.name, spell.name, target_name, healing
                        );
                    }
                }
            }
            BattleChoice::Consumable => {
                if let Some((name, healing)) = player.use_consumable() {
                    println!(
                        "{} uses {} and restores {} health. Health: {}",
                        player.name, name, healing, player.health
                    );
                } else {
                    println!("{} is already at full health.", player.name);
                    continue;
                }
            }
            BattleChoice::Flee => {
                println!("{} fled from the battle.", player.name);
                fled = true;
                break;
            }
        }

        if enemy.health == 0 {
            break;
        }

        if enemy_targets_companion && companion.health > 0 {
            companion.health = companion.health.saturating_sub(enemy.attack_damage);
            println!(
                "{} attacks for {} damage. {} health: {}",
                enemy.name, enemy.attack_damage, companion.name, companion.health
            );
        } else {
            player.health = player.health.saturating_sub(enemy.attack_damage);
            println!(
                "{} attacks for {} damage. {} health: {}",
                enemy.name, enemy.attack_damage, player.name, player.health
            );
        }
        enemy_targets_companion = !enemy_targets_companion;
    }

    if fled {
        println!("The {} remains undefeated.", enemy.name);
    } else if enemy.health == 0 {
        println!("{} wins!", player.name);
    } else {
        println!("{} wins!", enemy.name);
    }
}
