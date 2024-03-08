use crate::playerdata::PlayerData;

pub trait GameIO {
    fn start_turn(&mut self, player: &str);
    fn update_playfields(&mut self, playerdata: &Vec<PlayerData>);
    fn ask_yes_or_no(&mut self, msg: &str) -> bool;
    fn ask_playfield_position(&mut self, msg: &str, playfield: &Vec<[(bool, i8); 3]>, validator: fn(playfield: &Vec<[(bool, i8); 3]>, pos: (usize, usize)) -> bool) -> (usize, usize);
    fn draw_card(&mut self, card: i8);
    fn take_card(&mut self, card: i8);
    fn end_game(&mut self, playerdata: &Vec<PlayerData>);
}
