use crate::battle::{BattleResult, battle};
use crate::enemy::Enemy;
use crate::player::{Class, Player};
use crate::prompts::{class_choice, name, read_line};
use crate::rng::range_inclusive;
use crate::ui::{color, hp_bar, legend, panel, welcome};
use std::collections::HashMap;
use std::fs;
use std::str::FromStr;

const SAVE_FILE: &str = "savegame.txt";
const SAVE_VERSION: &str = "2";
const TALKED_TO_QUARTERMASTER: u8 = 1;
const TALKED_TO_ROWAN: u8 = 2;

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

    fn key(self) -> &'static str {
        match self {
            Self::Haven => "haven",
            Self::WhisperingWoods => "woods",
            Self::OldMine => "mine",
            Self::SunkenTemple => "temple",
            Self::Citadel => "citadel",
        }
    }

    fn parse(value: &str) -> Option<Self> {
        match value {
            "haven" => Some(Self::Haven),
            "woods" => Some(Self::WhisperingWoods),
            "mine" => Some(Self::OldMine),
            "temple" => Some(Self::SunkenTemple),
            "citadel" => Some(Self::Citadel),
            _ => None,
        }
    }

    fn adjacent(self, other: Self) -> bool {
        matches!(
            (self, other),
            (Self::Haven, Self::WhisperingWoods)
                | (Self::WhisperingWoods, Self::Haven)
                | (Self::WhisperingWoods, Self::OldMine)
                | (Self::OldMine, Self::WhisperingWoods)
                | (Self::OldMine, Self::SunkenTemple)
                | (Self::SunkenTemple, Self::OldMine)
                | (Self::SunkenTemple, Self::Citadel)
                | (Self::Citadel, Self::SunkenTemple)
        )
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
    talked_to: u8,
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
            talked_to: 0,
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
                println!(
                    "Journey ended. Use `save` before quitting if you want to continue later."
                );
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
        let (bit, message) = match npc.unwrap_or("quartermaster") {
            "quartermaster" | "shop" => (
                TALKED_TO_QUARTERMASTER,
                "Quartermaster: `buy potion` keeps heroes alive. The woods hide old paths.",
            ),
            "rowan" => (
                TALKED_TO_ROWAN,
                "Rowan: I have walked beside you this far. In the Citadel, I will not retreat.",
            ),
            "oracle" => {
                println!("The Oracle is not in Haven. Its drowned prophecy waits in the temple.");
                return;
            }
            _ => {
                println!("People to talk to: quartermaster, rowan.");
                return;
            }
        };
        println!("{message}");
        if self.talked_to & bit == 0 {
            self.talked_to |= bit;
            self.reputation += 1;
            println!("Your reputation improves.");
        }
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
            println!(
                "The quartermaster says: `buy potion` ({} gold).",
                self.potion_price()
            );
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
        if next != self.location && !self.location.adjacent(next) {
            println!(
                "You cannot travel directly from {} to {}. Follow the marked route.",
                self.location.title(),
                next.title()
            );
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
                self.companion.gain_experience(experience);
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
                self.companion.gain_experience(experience);
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
            self.player.repair_weapon();
            self.companion.repair_weapon();
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
        let price = self.potion_price();
        if item == Some("potion") && self.player.gold >= price {
            self.player.gold -= price;
            self.player.add_potion();
            println!("Potion purchased.");
        } else {
            println!("The quartermaster sells `potion` for {price} gold.");
        }
    }

    fn potion_price(&self) -> u32 {
        10u32.saturating_sub(self.reputation.max(0) as u32).max(5)
    }

    fn save(&self) {
        let mut lines = vec![format!("version={SAVE_VERSION}")];
        save_player(&mut lines, "player", &self.player);
        save_player(&mut lines, "companion", &self.companion);
        lines.extend([
            format!("location={}", self.location.key()),
            format!("visited={}", locations_to_string(&self.visited)),
            format!("explored={}", locations_to_string(&self.explored)),
            format!("shards={}", self.shards),
            format!("defeated_guardian={}", self.defeated_guardian),
            format!("quest_stage={}", self.quest_stage),
            format!("reputation={}", self.reputation),
            format!("talked_to={}", self.talked_to),
        ]);
        let data = lines.join("\n");
        match fs::write(SAVE_FILE, data) {
            Ok(()) => println!("Game saved to {SAVE_FILE}."),
            Err(error) => println!("Unable to save: {error}"),
        }
    }

    fn load(&mut self) {
        match fs::read_to_string(SAVE_FILE) {
            Ok(data) => match parse_save(&data).and_then(|fields| self.apply_save(fields)) {
                Ok(()) => println!("Game loaded. Welcome back, {}.", self.player.name),
                Err(error) => println!("Unable to load save: {error}"),
            },
            Err(error) => println!("Unable to load: {error}"),
        }
    }

    fn apply_save(&mut self, fields: HashMap<String, String>) -> Result<(), String> {
        if fields.get("version").map(String::as_str) != Some(SAVE_VERSION) {
            return Err("unsupported save version".into());
        }
        let mut player = self.player.clone();
        let mut companion = self.companion.clone();
        load_player(&mut player, "player", &fields)?;
        load_player(&mut companion, "companion", &fields)?;
        let location = required_location(&fields, "location")?;
        let visited = required_locations(&fields, "visited")?;
        let explored = required_locations(&fields, "explored")?;
        let shards: u8 = parse_field(&fields, "shards")?;
        let defeated_guardian: bool = parse_field(&fields, "defeated_guardian")?;
        let quest_stage: u8 = parse_field(&fields, "quest_stage")?;
        let reputation: i32 = parse_field(&fields, "reputation")?;
        let talked_to: u8 = parse_field(&fields, "talked_to")?;

        if shards > 3
            || quest_stage != shards
            || explored.len() != shards as usize
            || visited.is_empty()
            || !visited.contains(&location)
            || explored.contains(&Location::Haven)
            || explored.len() > 3
            || explored
                .iter()
                .enumerate()
                .any(|(index, location)| explored[..index].contains(location))
            || explored.iter().any(|location| !visited.contains(location))
            || talked_to > (TALKED_TO_QUARTERMASTER | TALKED_TO_ROWAN)
            || defeated_guardian && shards != 3
            || player.health > player.max_health
            || companion.health > companion.max_health
            || player.max_health == 0
            || companion.max_health == 0
            || player.level == 0
            || companion.level == 0
        {
            return Err("save contains inconsistent game state".into());
        }
        self.player = player;
        self.companion = companion;
        self.location = location;
        self.visited = visited;
        self.explored = explored;
        self.shards = shards;
        self.defeated_guardian = defeated_guardian;
        self.quest_stage = quest_stage;
        self.reputation = reputation;
        self.talked_to = talked_to;
        self.running = true;
        Ok(())
    }
}

fn save_player(lines: &mut Vec<String>, prefix: &str, player: &Player) {
    lines.extend([
        format!("{prefix}_name={}", player.name),
        format!("{prefix}_class={}", class_key(player.class)),
        format!("{prefix}_health={}", player.health),
        format!("{prefix}_max_health={}", player.max_health),
        format!("{prefix}_gold={}", player.gold),
        format!("{prefix}_experience={}", player.experience),
        format!("{prefix}_level={}", player.level),
        format!("{prefix}_weapon_name={}", player.weapon.name),
        format!("{prefix}_weapon_durability={}", player.weapon.durability),
        format!("{prefix}_weapon_damage={}", player.weapon.attack_damage),
        format!("{prefix}_weapon_spell={}", player.weapon.is_spell),
        format!("{prefix}_weapon_range={}", player.weapon.range),
        format!("{prefix}_potions={}", player.consumables.len()),
    ]);
}

fn load_player(
    player: &mut Player,
    prefix: &str,
    fields: &HashMap<String, String>,
) -> Result<(), String> {
    let name = required(fields, &format!("{prefix}_name"))?.to_string();
    let class = parse_class(required(fields, &format!("{prefix}_class"))?)?;
    let mut restored = Player::new(name, class);
    restored.health = parse_field(fields, &format!("{prefix}_health"))?;
    restored.max_health = parse_field(fields, &format!("{prefix}_max_health"))?;
    restored.gold = parse_field(fields, &format!("{prefix}_gold"))?;
    restored.experience = parse_field(fields, &format!("{prefix}_experience"))?;
    restored.level = parse_field(fields, &format!("{prefix}_level"))?;
    restored.weapon.name = required(fields, &format!("{prefix}_weapon_name"))?.to_string();
    restored.weapon.durability = parse_field(fields, &format!("{prefix}_weapon_durability"))?;
    restored.weapon.attack_damage = parse_field(fields, &format!("{prefix}_weapon_damage"))?;
    restored.weapon.is_spell = parse_field(fields, &format!("{prefix}_weapon_spell"))?;
    restored.weapon.range = parse_field(fields, &format!("{prefix}_weapon_range"))?;
    let potions: usize = parse_field(fields, &format!("{prefix}_potions"))?;
    if potions > 100 {
        return Err(format!("save contains too many {prefix} potions"));
    }
    restored.consumables.clear();
    for _ in 0..potions {
        restored.add_potion();
    }
    *player = restored;
    Ok(())
}

fn parse_save(data: &str) -> Result<HashMap<String, String>, String> {
    let mut fields = HashMap::new();
    for line in data.lines() {
        let (key, value) = line
            .split_once('=')
            .ok_or_else(|| "save contains a malformed line".to_string())?;
        if key.is_empty() || fields.insert(key.to_string(), value.to_string()).is_some() {
            return Err("save contains duplicate or empty keys".into());
        }
    }
    Ok(fields)
}

fn required<'a>(fields: &'a HashMap<String, String>, key: &str) -> Result<&'a str, String> {
    fields
        .get(key)
        .map(String::as_str)
        .ok_or_else(|| format!("save is missing {key}"))
}

fn parse_field<T: FromStr>(fields: &HashMap<String, String>, key: &str) -> Result<T, String> {
    required(fields, key)?
        .parse()
        .map_err(|_| format!("save contains an invalid {key}"))
}

fn required_location(fields: &HashMap<String, String>, key: &str) -> Result<Location, String> {
    Location::parse(required(fields, key)?).ok_or_else(|| format!("save contains an invalid {key}"))
}

fn required_locations(
    fields: &HashMap<String, String>,
    key: &str,
) -> Result<Vec<Location>, String> {
    required(fields, key)?
        .split(',')
        .filter(|value| !value.is_empty())
        .map(|value| {
            Location::parse(value).ok_or_else(|| format!("save contains an invalid {key}"))
        })
        .collect()
}

fn locations_to_string(locations: &[Location]) -> String {
    locations
        .iter()
        .map(|location| location.key())
        .collect::<Vec<_>>()
        .join(",")
}

fn class_key(class: Class) -> &'static str {
    match class {
        Class::Swordsman => "swordsman",
        Class::Wizard => "wizard",
        Class::Healer => "healer",
    }
}

fn parse_class(value: &str) -> Result<Class, String> {
    match value {
        "swordsman" => Ok(Class::Swordsman),
        "wizard" => Ok(Class::Wizard),
        "healer" => Ok(Class::Healer),
        _ => Err("save contains an invalid class".into()),
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
            talked_to: 0,
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

    #[test]
    fn dangerous_regions_require_adjacent_travel() {
        assert!(Location::Haven.adjacent(Location::WhisperingWoods));
        assert!(!Location::Haven.adjacent(Location::SunkenTemple));
    }

    #[test]
    fn save_parser_rejects_duplicate_keys() {
        let result = parse_save("version=2\nversion=2");
        assert!(result.is_err());
    }

    #[test]
    fn reputation_reduces_potion_price_but_never_below_five() {
        let mut game = Game {
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
            talked_to: 0,
        };
        assert_eq!(game.potion_price(), 10);
        game.reputation = 20;
        assert_eq!(game.potion_price(), 5);
    }
}
