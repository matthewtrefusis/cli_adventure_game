#[derive(Debug)]
pub struct Enemy {
    pub name: String,
    pub health: u32,
    pub attack_damage: u32,
}

impl Enemy {
    pub fn new(name: &str, health: u32, attack_damage: u32) -> Self {
        Self {
            name: name.to_string(),
            health,
            attack_damage,
        }
    }
}