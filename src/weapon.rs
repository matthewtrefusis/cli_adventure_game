#[derive(Debug, Clone)]
pub struct Weapon {
    pub name: String,
    pub durability: u32,
    pub attack_damage: u32,
    pub is_spell: bool,
    pub range: u32,
}
