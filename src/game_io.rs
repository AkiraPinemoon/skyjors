use crate::playerdata::PlayerData;

pub trait GameIO {
    fn update_playfields(&mut self, playerdata: Vec<PlayerData>);
    fn ask_yes_or_no(&mut self, msg: &str) -> bool;
    fn ask_playfield_position(&mut self, msg: &str, playfield: Vec<[(bool, i8); 3]>) -> (usize, usize);
}