use std::time;
pub struct Player {
    pub name: String,
    pub damage: f64,
}

pub trait MyClone {
    fn my_clone(&self) -> Self;
}

impl MyClone for Player {
    fn my_clone(&self) -> Self {
        Self {
            name: self.name.clone(),
            damage: self.damage,
        }
    }
}

#[test]
fn test_clone() {
    let player = Player {
        name: "Common Guy".to_string(),
        damage: 0.0,
    };

    let player_2 = player.my_clone();

    assert!(player_2.damage.abs() < f64::EPSILON);
    assert_eq!(player_2.name, "Common Guy".to_string());
}

pub trait MyDefault {
    fn my_default() -> Self;
}

impl MyDefault for Player {
    fn my_default() -> Self {
        Self { name: "".to_string(), damage: 0.0 }
    }
}

pub trait Bonus {
    fn apply_to_player(&self, player: &mut Player);

    fn remove_from_player(&self, player: &mut Player);

    fn effect_length(&self) -> time::Duration {
        time::Duration::from_secs(1)
    }
}

pub struct DamageBoost(f64);

impl Bonus for DamageBoost {
    fn apply_to_player(&self, player: &mut Player) {
        player.damage += self.0;
    }

    fn remove_from_player(&self, player: &mut Player) {
        player.damage -= self.0;
    }
}

pub struct NameReplacer {
    new_name: String,
}

impl Bonus for NameReplacer {
    fn effect_length(&self) -> time::Duration {
        time::Duration::MAX
    }

    fn apply_to_player(&self, player: &mut Player) {
        player.name = self.new_name.clone();
    }

    fn remove_from_player(&self, _player: &mut Player) {}
}

#[test]
fn test_bonus() {
    let damage_1 = DamageBoost(42.0);
    let damage_2 = DamageBoost(10.0);
    let name = NameReplacer {
        new_name: "XXX_KILLER_XXX".to_string(),
    };

    let bonuses: Vec<&dyn Bonus> = vec![&damage_1, &damage_2, &name];

    let mut player = Player {
        name: "Common Guy".to_string(),
        damage: 0.0,
    };

    for bonus in bonuses.iter() {
        bonus.apply_to_player(&mut player);
    }

    assert!((player.damage - 52.0).abs() < f64::EPSILON);
    assert_eq!(player.name, "XXX_KILLER_XXX".to_string());

    for bonus in bonuses.iter() {
        bonus.remove_from_player(&mut player);
    }

    assert!(player.damage.abs() < f64::EPSILON);
    assert_eq!(player.name, "XXX_KILLER_XXX".to_string());
}
