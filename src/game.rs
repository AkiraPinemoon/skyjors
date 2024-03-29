use crate::{game_io::GameIO, playerdata::PlayerData, stack::Stack};
use std::fmt;

pub struct Game {
    playerdata: Vec<PlayerData>,
    stack: Stack,
    current_player: usize,
    phase: GamePhase,
    last_played_card: Option<i8>,
    io: Box<dyn GameIO>,
}

impl Game {
    pub async fn start(&mut self) {
        self.io.update_playfields(&self.playerdata).await;
    }

    pub fn is_done(&self) -> bool {
        self.phase == GamePhase::Ended
    }

    pub async fn turn(&mut self) {
        match self.phase {
            GamePhase::InitialReveal => {
                self.io
                    .start_turn(&self.playerdata[self.current_player].name).await;
                self.reveal_card("Choose your first card to reveal (e.g., A3)").await;
                self.reveal_card("Choose your second card to reveal").await;

                if self.current_player == self.playerdata.len() - 1 {
                    self.phase = GamePhase::Play;
                }
            }
            GamePhase::Play => {
                self.io
                    .start_turn(&self.playerdata[self.current_player].name).await;
                let chosen_card = match self.last_played_card {
                    None => {
                        let card = self.stack.draw().unwrap();
                        self.io.draw_card(card).await;
                        card
                    }
                    Some(last) => {
                        let card = match self.io.ask_yes_or_no(&format!("You can either take the {} that was thrown away last or draw a random card, do you want to draw a card?", last)).await {
                            true => {
                                let card = self.stack.draw().unwrap();
                                self.io.draw_card(card).await;
                                card
                            },
                            false => {
                                let card = last;
                                self.last_played_card = None;
                                self.io.take_card(card).await;
                                card
                            },
                        };
                        card
                    }
                };

                match self
                    .io
                    .ask_yes_or_no("Do you want to play this card (or throw it away)?").await
                {
                    true => {
                        self.replace_card(chosen_card, "Sure. Pick what card to replace.").await;
                    }
                    false => {
                        self.last_played_card = Some(chosen_card);
                        self.reveal_card("Ok. Pick a card to reveal.").await;
                    }
                };

                self.remove_done_columns().await;
                if !self.has_unrevealed_cards(&self.playerdata[self.current_player].playfield) {
                    self.phase = GamePhase::Ended;
                    self.reveal_all_cards();
                    self.io.end_game(&self.playerdata).await;
                }
            }
            GamePhase::Ended => {}
        }

        self.current_player += 1;
        self.current_player %= self.playerdata.len();
    }

    pub async fn run(&mut self) {
        self.start().await;
        while !self.is_done() {
            self.turn().await;
        }
    }

    async fn remove_done_columns(&mut self) {
        for p in self.playerdata.iter_mut() {
            p.playfield = p
                .playfield
                .iter()
                .copied()
                .filter(|column| {
                    let keep = match column {
                        [(true, x), (true, y), (true, z)] => !(x == y && y == z),
                        _ => true,
                    };
                    if !keep { self.last_played_card = None; }
                    keep
                })
                .collect();
        }

        self.io.update_playfields(&self.playerdata).await;
    }

    fn has_unrevealed_cards(&self, playfield: &Vec<[(bool, i8); 3]>) -> bool {
        let mut res = false;
        playfield.iter().for_each(|column| {
            column.iter().for_each(|card| {
                if !card.0 {
                    res = true;
                }
            })
        });
        res
    }

    fn reveal_all_cards(&mut self) {
        self.playerdata.iter_mut().for_each(|player| {
            player.playfield.iter_mut().for_each(|column| {
                column.iter_mut().for_each(|card| {
                    card.0 = true;
                })
            });
        })
    }

    async fn reveal_card(&mut self, msg: &str) {
        let (x, y) = self.io.ask_playfield_position(
            msg,
            self.playerdata[self.current_player].playfield.clone(),
            |playfield, (x, y)| !playfield[x][y].0,
        ).await;

        self.playerdata[self.current_player].playfield[x][y].0 = true;

        self.io.update_playfields(&self.playerdata).await;
    }

    async fn replace_card(&mut self, card: i8, msg: &str) {
        let (x, y) = self.io.ask_playfield_position(
            msg,
            self.playerdata[self.current_player].playfield.clone(),
            |_, _| true,
        ).await;

        self.last_played_card = Some(self.playerdata[self.current_player].playfield[x][y].1);

        self.playerdata[self.current_player].playfield[x][y].0 = true;
        self.playerdata[self.current_player].playfield[x][y].1 = card;

        self.io.update_playfields(&self.playerdata).await;
    }
}

#[derive(PartialEq, Clone)]
pub enum GamePhase {
    InitialReveal,
    Play,
    Ended,
}

#[derive(Clone)]
pub struct GameBuilder {
    playerdata: Vec<PlayerData>,
    stack: Stack,
    current_player: usize,
    phase: GamePhase,
    last_played_card: Option<i8>,
}

impl GameBuilder {
    pub fn new() -> Self {
        Self {
            playerdata: Vec::new(),
            stack: Stack::new(),
            current_player: 0,
            phase: GamePhase::InitialReveal,
            last_played_card: None,
        }
    }

    pub fn with_player(&mut self, name: String) -> Result<&mut Self, String> {
        self.playerdata.push(PlayerData {
            name,
            playfield: self.stack.draw_playfield()?,
        });
        Ok(self)
    }

    pub fn build(self, io: Box<dyn GameIO>) -> Game {
        Game {
            playerdata: self.playerdata,
            stack: self.stack,
            current_player: self.current_player,
            phase: self.phase,
            last_played_card: self.last_played_card,
            io,
        }
    }
}

impl fmt::Debug for Game {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Game")
            .field("player_data", &self.playerdata)
            .field("stack", &self.stack)
            .finish()
    }
}
