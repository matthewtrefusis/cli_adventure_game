#[derive(Debug, Clone)]
pub struct Enemy {
    pub name: String,
    pub health: u32,
    pub max_health: u32,
    pub attack_damage: u32,
    pub armor: u32,
    pub reward_gold: u32,
    pub reward_xp: u32,
}

impl Enemy {
    pub fn new(name: &str, health: u32, attack_damage: u32) -> Self {
        Self {
            name: name.to_string(),
            health,
            max_health: health,
            attack_damage,
            armor: 0,
            reward_gold: 10,
            reward_xp: 10,
        }
    }

    pub fn elite(name: &str, health: u32, attack_damage: u32, armor: u32) -> Self {
        Self {
            name: name.to_string(),
            health,
            max_health: health,
            attack_damage,
            armor,
            reward_gold: 35,
            reward_xp: 30,
        }
    }

    pub fn take_damage(&mut self, amount: u32) -> u32 {
        let damage = amount.saturating_sub(self.armor).max(1);
        let before = self.health;
        self.health = self.health.saturating_sub(damage);
        before - self.health
    }

    pub fn roll_attack_damage(&self) -> u32 {
        crate::rng::around(self.attack_damage)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn randomized_attack_stays_within_expected_range() {
        let enemy = Enemy::new("Test", 10, 20);
        for _ in 0..100 {
            assert!((15..=25).contains(&enemy.roll_attack_damage()));
        }
    }
}
