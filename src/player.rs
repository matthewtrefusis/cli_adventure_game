#[derive(Debug)]
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
    pub weapon: String,
    pub attack_damage: u32,
}

impl Player {
    pub fn new(name: String, class: Class) -> Self {
        let health = match class {
            Class::Swordsman => 120,
            Class::Wizard => 80,
            Class::Healer => 100,
        };

        let weapon: String = match class {
            Class::Swordsman => "Sword".to_string(),
            Class::Wizard => "Staff".to_string(),
            Class::Healer => "Dagger".to_string(),
        };

        let attack_damage = match weapon.as_str() {
            "Sword" => 12,
            "Staff" => 3,
            "Dagger" => 6,
            _ => 0,
        };

        Self {
            name,
            class,
            health,
            weapon,
            attack_damage,
        }
    }
}
