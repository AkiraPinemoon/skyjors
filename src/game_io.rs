use crate::playerdata::PlayerData;
use async_trait::async_trait;

#[async_trait]
pub trait GameIO : Send + Sync {
    async fn start_turn(&mut self, player: &str);
    async fn update_playfields(&mut self, playerdata: &Vec<PlayerData>);
    async fn ask_yes_or_no(&mut self, msg: &str) -> bool;
    async fn ask_playfield_position(
        &mut self,
        msg: &str,
        playfield: Vec<[(bool, i8); 3]>,
        validator: fn(playfield: Vec<[(bool, i8); 3]>, pos: (usize, usize)) -> bool,
    ) -> (usize, usize);
    async fn draw_card(&mut self, card: i8);
    async fn take_card(&mut self, card: i8);
    async fn end_game(&mut self, playerdata: &Vec<PlayerData>);
}
