use crate::battle::{BattleResult, battle};
use crate::enemy::Enemy;
use crate::player::{Class, Player};
use crate::prompts::{class_choice, name, read_line};
use crate::rng::range_inclusive;
use crate::ui::{color, hp_bar, legend, panel, welcome};
use std::fs;

const SAVE_FILE: &str = "savegame.txt";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Location {
    Haven,
    WhisperingWoods,
    OldMine,
    SunkenTemple,
    Citadel,
}

impl Location {
    fn title(self) -> &'static str {
        match self {
            Self::Haven => "Haven",
            Self::WhisperingWoods => "Whispering Woods",
            Self::OldMine => "Old Mine",
            Self::SunkenTemple => "Sunken Temple",
            Self::Citadel => "The Ashen Citadel",
        }
    }
}

pub struct Game {
    pub player: Player,
    companion: Player,
    location: Location,
    visited: Vec<Location>,
    shards: u8,
    defeated_guardian: bool,
    running: bool,
    quest_stage: u8,
    reputation: i32,
    explored: Vec<Location>,
}

impl Game {
    pub fn new() -> Self {
        welcome();
        println!("The sky is broken. Three star-shards are all that can seal the Ashen Citadel.");
        legend();
        let player = Player::new(name(), class_choice());
        Self {
            player,
            companion: Player::new("Rowan".into(), Class::Swordsman),
            location: Location::Haven,
            visited: vec![Location::Haven],
            shards: 0,
            defeated_guardian: false,
            running: true,
            quest_stage: 0,
            reputation: 0,
            explored: Vec::new(),
        }
    }

    pub fn run(&mut self) {
        println!("\nWelcome, {}. Type `help` for commands.", self.player.name);
        while self.running && self.player.health > 0 {
            self.hud();
            let Some(command) = read_line(&format!("{} > ", self.location.title())) else {
                break;
            };
            self.handle_command(&command);
        }
        if self.player.health == 0 {
            println!("The world remembers your sacrifice.");
        }
    }

    fn handle_command(&mut self, command: &str) {
        let mut parts = command.split_whitespace();
        match parts.next().unwrap_or_default() {
            "help" => self.help(),
            "look" | "l" => self.look(),
            "status" | "stats" => self.status(),
            "inventory" | "inv" => self.inventory(),
            "map" => self.map(),
            "quest" | "quests" => self.quest(),
            "talk" => self.talk(parts.next()),
            "travel" | "go" => self.travel(parts.next()),
            "explore" => self.explore(),
            "rest" => self.rest(),
            "buy" => self.buy(parts.next()),
            "save" => self.save(),
            "load" => self.load(),
            "quit" | "exit" => {
                self.running = false;
                println!("Your journey is saved in legend.");
            }
            "" => {}
            _ => println!("Unknown command. Type `help` to see available commands."),
        }
    }

    fn help(&self) {
        println!(
            "Commands: look, map, status, inventory, quest, talk <npc>, travel <place>, explore, rest, buy potion, save, load, quit"
        );
        println!(
            "Combat: attack, fists, spell, potion, inspect, run. Navigation: haven, woods, mine, temple, citadel."
        );
    }

    fn hud(&self) {
        let hp = hp_bar(self.player.health, self.player.max_health, 18);
        let location = color("\x1b[36m", self.location.title());
        let hp_text = color(
            if self.player.health < self.player.max_health / 3 {
                "\x1b[31m"
            } else {
                "\x1b[32m"
            },
            format_args!("{hp} {}/{} HP", self.player.health, self.player.max_health),
        );
        let gold = color("\x1b[33m", format_args!("{} gold", self.player.gold));
        let shards = color("\x1b[35m", format_args!("shards {}/3", self.shards));
        let hud = format!("{location}  {hp_text}  {gold}  {shards}");
        println!("{hud}");
    }

    fn map(&self) {
        let mark = |location| {
            if self.visited.contains(&location) {
                "●"
            } else {
                "○"
            }
        };
        panel(
            "REALM MAP",
            &[
                format!(
                    "{} Haven ── {} Whispering Woods",
                    mark(Location::Haven),
                    mark(Location::WhisperingWoods)
                ),
                "                 │".to_string(),
                format!(
                    "                 {} Old Mine ── {} Sunken Temple",
                    mark(Location::OldMine),
                    mark(Location::SunkenTemple)
                ),
                "                                      │".to_string(),
                format!(
                    "                                      {} Ashen Citadel",
                    mark(Location::Citadel)
                ),
            ],
        );
    }

    fn quest(&self) {
        let text = match self.quest_stage {
            0 => "The Shattered Sky: recover three star-shards from the dangerous regions.",
            1 => "The Shattered Sky: the woods have yielded a shard. Two more remain.",
            2 => "The Shattered Sky: two shards glow at your belt. One more remains.",
            _ => "The Shattered Sky: enter the Citadel and confront the Ashen Guardian.",
        };
        panel(
            "ACTIVE QUEST",
            &[text.into(), format!("Reputation: {}", self.reputation)],
        );
    }

    fn talk(&mut self, npc: Option<&str>) {
        if self.location != Location::Haven {
            println!("There is nobody here to talk to.");
            return;
        }
        match npc.unwrap_or("quartermaster") {
            "quartermaster" | "shop" => println!(
                "Quartermaster: `buy potion` keeps heroes alive. The woods hide old paths."
            ),
            "rowan" => println!(
                "Rowan: I have walked beside you this far. In the Citadel, I will not retreat."
            ),
            "oracle" => {
                println!("The Oracle is not in Haven. Its drowned prophecy waits in the temple.")
            }
            _ => println!("People to talk to: quartermaster, rowan."),
        }
        self.reputation += 1;
    }

    fn look(&self) {
        println!(
            "\n{} — {}",
            self.location.title(),
            match self.location {
                Location::Haven => "A lantern-lit refuge. A quartermaster sells supplies.",
                Location::WhisperingWoods =>
                    "Silver leaves whisper warnings. Something hunts between the trunks.",
                Location::OldMine =>
                    "Abandoned shafts descend beneath the mountain. The air tastes of iron.",
                Location::SunkenTemple => "A drowned sanctuary glows with impossible blue light.",
                Location::Citadel => "A fortress of black glass waits beyond the ruined gate.",
            }
        );
        println!(
            "Companion: {} {}/{} HP | reputation: {}",
            self.companion.name, self.companion.health, self.companion.max_health, self.reputation
        );
        if self.location == Location::Haven {
            println!("The quartermaster says: `buy potion` (10 gold).");
        }
        println!("Exits: haven, woods, mine, temple, citadel. Use `map` to see explored routes.");
    }

    fn status(&self) {
        println!(
            "{} the {:?} | level {} | {}/{} HP | {} gold | {} XP",
            self.player.name,
            self.player.class,
            self.player.level,
            self.player.health,
            self.player.max_health,
            self.player.gold,
            self.player.experience
        );
        println!(
            "Weapon: {} ({} damage, {} durability, range {}, spell-focus {}) | Shards: {}/3",
            self.player.weapon.name,
            self.player.weapon.attack_damage,
            self.player.weapon.durability,
            self.player.weapon.range,
            self.player.weapon.is_spell,
            self.shards
        );
    }

    fn inventory(&self) {
        panel(
            "INVENTORY",
            &[format!(
                "Gold: {} | Potions: {}",
                self.player.gold,
                self.player.consumables.len()
            )],
        );
        for item in &self.player.consumables {
            println!("- {} (restores {})", item.name, item.healing);
        }
        if self.player.consumables.is_empty() {
            println!("- Empty");
        }
    }

    fn travel(&mut self, destination: Option<&str>) {
        let next = match destination.unwrap_or_default() {
            "haven" => Location::Haven,
            "woods" | "forest" => Location::WhisperingWoods,
            "mine" => Location::OldMine,
            "temple" => Location::SunkenTemple,
            "citadel" => Location::Citadel,
            _ => {
                println!("Choose haven, woods, mine, temple, or citadel.");
                return;
            }
        };
        if next == Location::Citadel && self.shards < 3 {
            println!("The citadel gate rejects you. Three star-shards are required.");
            return;
        }
        self.location = next;
        if !self.visited.contains(&next) {
            self.visited.push(next);
        }
        println!("You travel to {}.", next.title());
        self.look();
        if next != Location::Haven && range_inclusive(1, 100) <= 35 {
            println!(
                "\n{}",
                crate::ui::color("\x1b[31m", "A roaming monster ambushes you on the road!")
            );
            self.random_encounter(next);
        }
    }

    fn explore(&mut self) {
        let first_clear = !self.explored.contains(&self.location);
        let mut enemy = match self.location {
            Location::WhisperingWoods => Enemy::new("Mossbound Stalker", 42, 8),
            Location::OldMine => Enemy::elite("Iron Warden", 65, 11, 2),
            Location::SunkenTemple => Enemy::elite("Drowned Oracle", 80, 14, 3),
            Location::Citadel if !self.defeated_guardian => {
                Enemy::elite("Ashen Guardian", 120, 18, 4)
            }
            Location::Citadel => {
                println!("The citadel is silent. You have already won.");
                return;
            }
            Location::Haven => {
                println!("Haven is safe. Travel somewhere dangerous to explore.");
                return;
            }
        };
        match battle(&mut self.player, &mut self.companion, &mut enemy) {
            BattleResult::Victory { gold, experience } => {
                self.player.gold += gold;
                if self.player.gain_experience(experience) {
                    println!("Level up! You are now level {}.", self.player.level);
                }
                if self.location != Location::Citadel && first_clear {
                    self.shards += 1;
                    self.quest_stage = self.shards;
                    self.reputation += 2;
                    self.explored.push(self.location);
                    println!("You recover a star-shard ({}/3).", self.shards);
                } else if self.location != Location::Citadel {
                    self.player.gold += 5;
                    println!("The area is picked clean. You salvage 5 extra gold.");
                } else {
                    self.defeated_guardian = true;
                    println!("The Ashen Citadel falls. You saved the realm!");
                    self.running = false;
                }
            }

            BattleResult::Fled => println!("The path remains dangerous."),
            BattleResult::Defeat => self.running = false,
        }
    }

    fn random_encounter(&mut self, location: Location) {
        let mut enemy = match location {
            Location::WhisperingWoods => Enemy::new("Roadside Ghoul", 30, 7),
            Location::OldMine => Enemy::new("Tunnel Skitterer", 38, 9),
            Location::SunkenTemple => Enemy::new("Temple Eel", 45, 10),
            Location::Citadel => Enemy::new("Cinder Scout", 55, 12),
            Location::Haven => return,
        };
        match battle(&mut self.player, &mut self.companion, &mut enemy) {
            BattleResult::Victory { gold, experience } => {
                self.player.gold += gold;
                if self.player.gain_experience(experience) {
                    println!("Level up! You are now level {}.", self.player.level);
                }
                println!("The road is clear. You salvage {} gold.", gold);
            }
            BattleResult::Fled => println!("You escape the ambush and reach {}.", location.title()),
            BattleResult::Defeat => self.running = false,
        }
    }

    fn rest(&mut self) {
        if self.location == Location::Haven {
            self.player.health = self.player.max_health;
            self.companion.health = self.companion.max_health;
            println!("You rest at the inn. The party is fully restored.");
        } else {
            println!("You need a safe inn in Haven to rest.");
        }
    }

    fn buy(&mut self, item: Option<&str>) {
        if self.location != Location::Haven {
            println!("There is no shop here.");
            return;
        }
        if item == Some("potion") && self.player.gold >= 10 {
            self.player.gold -= 10;
            self.player.add_potion();
            println!("Potion purchased.");
        } else {
            println!("The quartermaster sells `potion` for 10 gold.");
        }
    }

    fn save(&self) {
        let data = format!(
            "{}|{}|{}|{}|{}|{}|{}|{}|{}",
            self.player.name,
            self.player.health,
            self.player.max_health,
            self.player.gold,
            self.player.experience,
            self.player.level,
            self.shards,
            self.quest_stage,
            self.reputation
        );
        match fs::write(SAVE_FILE, data) {
            Ok(()) => println!("Game saved to {SAVE_FILE}."),
            Err(error) => println!("Unable to save: {error}"),
        }
    }

    fn load(&mut self) {
        match fs::read_to_string(SAVE_FILE) {
            Ok(data) => {
                let fields: Vec<&str> = data.split('|').collect();
                if fields.len() != 9 {
                    println!("Save file is invalid.");
                    return;
                }
                let parsed: Option<Vec<u32>> = fields[1..].iter().map(|v| v.parse().ok()).collect();
                if let Some(values) = parsed {
                    self.player.name = fields[0].to_string();
                    self.player.health = values[0];
                    self.player.max_health = values[1];
                    self.player.gold = values[2];
                    self.player.experience = values[3];
                    self.player.level = values[4];
                    self.shards = values[5] as u8;
                    self.quest_stage = values[6] as u8;
                    self.reputation = values[7] as i32;
                    println!("Game loaded. Welcome back, {}.", self.player.name);
                } else {
                    println!("Save file contains invalid numbers.");
                }
            }
            Err(error) => println!("Unable to load: {error}"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn new_game_starts_in_haven_with_three_shards_needed() {
        let game = Game {
            player: Player::new("Test".into(), Class::Swordsman),
            companion: Player::new("Rowan".into(), Class::Swordsman),
            location: Location::Haven,
            visited: vec![Location::Haven],
            shards: 0,
            defeated_guardian: false,
            running: true,
            quest_stage: 0,
            reputation: 0,
            explored: Vec::new(),
        };
        assert_eq!(game.location, Location::Haven);
        assert_eq!(game.shards, 0);
    }

    #[test]
    fn leveling_restores_health_and_improves_weapon() {
        let mut player = Player::new("Test".into(), Class::Swordsman);
        player.health = 1;
        let original_damage = player.weapon.attack_damage;
        assert!(player.gain_experience(100));
        assert_eq!(player.level, 2);
        assert_eq!(player.health, player.max_health);
        assert!(player.weapon.attack_damage > original_damage);
    }
}
