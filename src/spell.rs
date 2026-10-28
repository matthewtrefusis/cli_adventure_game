#[derive(Debug, Clone, Copy)]
pub enum SpellKind {
    Damage,
    Heal,
}

#[derive(Debug, Clone)]
pub struct Spell {
    pub name: String,
    pub power: u32,
    pub kind: SpellKind,
}

impl Spell {
    pub fn damage(name: &str, power: u32) -> Self {
        Self {
            name: name.to_string(),
            power,
            kind: SpellKind::Damage,
        }
    }

    pub fn healing(name: &str, power: u32) -> Self {
        Self {
            name: name.to_string(),
            power,
            kind: SpellKind::Heal,
        }
    }
}
