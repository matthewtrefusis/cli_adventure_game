use crate::enemy::Enemy;
use crate::player::Class;
use crate::spell::{Spell, SpellKind};
use std::io;

pub enum BattleChoice {
    Weapon,
    Fists,
    Spell,
    Consumable,
    Flee,
}

pub enum HealTarget {
    SelfTarget,
    Companion,
}

fn prompt() -> String {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    input.trim().to_lowercase()
}

pub fn intro() {
    println!("Hello brave explorer! Want to embark on an adventure? y/n");
    match prompt().as_str() {
        "y" => {
            println!("Yay! Your aspirations will give you some very good karma later in your life.")
        }

        "n" => println!("Boo, you're such a buzzkill!"),
        _ => {
            println!("Not a valid choice. Try again");
            intro()
        }
    }
}

pub fn class_choice() -> Class {
    println!("Now it's time to choose your class");
    println!("What do you choose?");
    println!("(S)wordsman, (W)izard or (H)ealer");

    match prompt().as_str() {
        "s" => Class::Swordsman,
        "w" => Class::Wizard,
        "h" => Class::Healer,
        _ => {
            println!("Not a valid choice. Try again");
            class_choice()
        }
    }
}

pub fn name() -> String {
    println!("It's time to choose a name");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    input.trim().to_string()
}

pub fn first_encounter() -> Enemy {
    let enemy = Enemy::new("Cave Rat", 30, 5);

    println!("\nA {} jumps out of the darkness!", enemy.name);
    println!("Enemy health: {}", enemy.health);
    println!("Enemy attack damage: {}", enemy.attack_damage);

    enemy
}

pub fn battle_choice(has_spells: bool, has_consumables: bool) -> BattleChoice {
    loop {
        println!("\nWhat will you do?");
        println!("(W)eapon, (F)ists, or (R)un");
        if has_spells {
            println!("(S)pell");
        }
        if has_consumables {
            println!("(C)onsumable");
        }

        match prompt().as_str() {
            "w" => return BattleChoice::Weapon,
            "f" => return BattleChoice::Fists,
            "s" if has_spells => return BattleChoice::Spell,
            "c" if has_consumables => return BattleChoice::Consumable,
            "r" => return BattleChoice::Flee,
            _ => println!("Not a valid choice. Try again."),
        }
    }
}

pub fn spell_choice(spells: &[Spell]) -> usize {
    loop {
        println!("Choose a spell:");

        for (index, spell) in spells.iter().enumerate() {
            let effect = match spell.kind {
                SpellKind::Damage => format!("{} damage", spell.power),
                SpellKind::Heal => format!("{} healing", spell.power),
            };
            println!("({}) {} ({})", index + 1, spell.name, effect);
        }

        if let Ok(choice) = prompt().parse::<usize>() {
            if (1..=spells.len()).contains(&choice) {
                return choice - 1;
            }
        }

        println!("Not a valid spell choice. Try again.");
    }
}

pub fn heal_target(has_companion: bool) -> HealTarget {
    loop {
        println!("Who should be healed?");
        println!("(S)elf");
        if has_companion {
            println!("(C)ompanion");
        }

        match prompt().as_str() {
            "s" => return HealTarget::SelfTarget,
            "c" if has_companion => return HealTarget::Companion,
            _ => println!("Not a valid healing target. Try again."),
        }
    }
}
