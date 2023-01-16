

pub struct Enemy {
    id: u32,
    rewardXP: u16,
    health: u16,
    pos: (u16, u16)
}

impl Enemy {
    pub fn create() -> Enemy {
        Enemy {
            id: 1, //todo generate id,
            rewardXP: 1,
            health: 3,
            pos: (25, 25),
        }
    }
   
    pub fn render(&self) -> &str {
        "\x1b[92m%\x1b[0m"
    }

    pub fn collides(&self, row: u16, col: u16) -> bool {
        // todo handle damage case where collision 
        // causes damage to enemy
        self.pos.0 == col && self.pos.1 == row
    }
    
    pub fn move_towards_player() {

    }
}
