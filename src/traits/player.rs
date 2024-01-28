pub const UNIDENTIFIED_MALE: Player = Player {
    name: "John Doe",
    health: 0,
    energy: 0,
};

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Player {
    name: &'static str,
    health: u8,
    energy: u8,
}

impl Player {
    pub fn new(name: &'static str) -> Player {
        Player {
            name: name,
            health: 100,
            energy: 100,
        }
    }
    pub fn is_dead(&self) -> bool {
        self.health <= 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_dead_player() {
        let mut player = Player::new("John Doe");
        player.health = 0;
        assert_eq!(player.is_dead(), true);
        player.energy = 0;
        assert_eq!(player, UNIDENTIFIED_MALE);
    }
}
