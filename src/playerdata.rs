use std::fmt;

pub struct PlayerData {
    pub name: String,
    pub playfield: Vec<[(bool, i8); 3]>,
}

impl fmt::Debug for PlayerData {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("PlayerData").field("name", &self.name).field("playfield", &self.playfield).finish()
    }
}
