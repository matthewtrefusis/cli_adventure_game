#[derive(Debug)]
pub struct Consumable {
    pub name: String,
    pub healing: u32,
}

impl Consumable {
    pub fn healing_potion() -> Self {
        Self {
            name: "Healing Potion".to_string(),
            healing: 25,
        }
    }
}