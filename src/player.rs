use crate::consumable::Consumable;
use crate::spell::Spell;
use crate::weapon::Weapon;

#[derive(Debug, Clone, Copy)]
pub enum Class {
    Swordsman,
    Wizard,
    Healer,
}

#[derive(Debug)]
pub struct Player {
    pub name: String,
    pub class: Class,
    pub health: u32,
    pub max_health: u32,
    pub weapon: Weapon,
    pub spells: Vec<Spell>,
    pub consumables: Vec<Consumable>,
}

impl Player {
    pub fn new(name: String, class: Class) -> Self {
        let (health, weapon, spells) = match class {
            Class::Swordsman => (
                120,
                Weapon {
                    name: "Sword".to_string(),
                    durability: 100,
                    attack_damage: 12,
                    is_spell: false,
                    range: 1,
                },
                Vec::new(),
            ),
            Class::Wizard => (
                80,
                Weapon {
                    name: "Staff".to_string(),
                    durability: 70,
                    attack_damage: 7,
                    is_spell: false,
                    range: 2,
                },
                vec![
                    Spell::damage("Fireball", 15),
                    Spell::damage("Lightning", 20),
                ],
            ),
            Class::Healer => (
                100,
                Weapon {
                    name: "Dagger".to_string(),
                    durability: 90,
                    attack_damage: 6,
                    is_spell: false,
                    range: 1,
                },
                vec![Spell::healing("Heal", 25), Spell::damage("Holy Light", 10)],
            ),
        };

        Self {
            name,
            class,
            health,
            max_health: health,
            weapon,
            spells,
            consumables: vec![Consumable::healing_potion()],
        }
    }

    pub fn heal(&mut self, amount: u32) -> u32 {
        let previous_health = self.health;
        self.health = self.health.saturating_add(amount).min(self.max_health);
        self.health - previous_health
    }

    pub fn has_consumables(&self) -> bool {
        !self.consumables.is_empty()
    }

    pub fn use_consumable(&mut self) -> Option<(String, u32)> {
        if self.health == self.max_health {
            return None;
        }

        let consumable = self.consumables.pop()?;
        let previous_health = self.health;
        self.health = self
            .health
            .saturating_add(consumable.healing)
            .min(self.max_health);

        Some((consumable.name, self.health - previous_health))
    }
}
