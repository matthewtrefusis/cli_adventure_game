use crate::weapon::Weapon;

#[derive(Debug, Clone, Copy)]
pub enum Class {
    Swordsman,
    Wizard,
    Healer,
}

#[derive(Debug)]
#[allow(dead_code)]
pub struct Player {
    pub name: String,
    pub class: Class,
    pub health: u32,
    pub weapon: Weapon,
}

impl Player {
    pub fn new(name: String, class: Class) -> Self {
        let (health, weapon) = match class {
            Class::Swordsman => (
                120,
                Weapon {
                    name: "Sword".to_string(),
                    durability: 100,
                    attack_damage: 12,
                    is_spell: false,
                    range: 1,
                },
            ),
            Class::Wizard => (
                80,
                Weapon {
                    name: "Fireball".to_string(),
                    durability: 70,
                    attack_damage: 15,
                    is_spell: true,
                    range: 10,
                },
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
            ),
        };

        Self {
            name,
            class,
            health,
            weapon,
        }
    }
}
