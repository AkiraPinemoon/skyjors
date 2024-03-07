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
    pub fn is_done(&self) -> bool {
        self.phase == GamePhase::Ended
    }
    pub fn turn(&mut self) {
        match self.phase {
            GamePhase::InitialReveal => {
                self.io.start_turn(&self.playerdata[self.current_player].name);
                self.reveal_card("Choose your first card to reveal (e.g., A3)");
                self.reveal_card("Choose your second card to reveal");

                if self.current_player == self.playerdata.len() - 1 {
                    self.phase = GamePhase::Play;
                }
            },
            GamePhase::Play => {
                self.io.start_turn(&self.playerdata[self.current_player].name);
                let chosen_card = match self.last_played_card {
                    None => {
                        let card = self.stack.draw().unwrap();
                        self.io.draw_card(card);
                        card
                    },
                    Some(last) => {
                        let card = match self.io.ask_yes_or_no(&format!("You can either take the {} that was thrown away last or draw a random card, do you want to draw a card?", last)) {
                            true => {
                                let card = self.stack.draw().unwrap();
                                self.io.draw_card(card);
                                card
                            },
                            false => {
                                let card = last;
                                self.last_played_card = None;
                                card
                            },
                        };
                        card
                    },
                };

                match self.io.ask_yes_or_no("Do you want to play this card (or throw it away)?") {
                    true => {
                        self.replace_card(chosen_card, "Sure. Pick what card to replace.");
                    },
                    false => {
                        self.last_played_card = Some(chosen_card);
                        self.reveal_card("Ok. Pick a card to reveal.");
                    },
                };
            },
            GamePhase::Ended => {

            },
        }

        self.current_player += 1;
        self.current_player %= self.playerdata.len();
    }

    fn reveal_card(&mut self, msg: &str) {
        let (x, y) = self.io.ask_playfield_position(msg, &self.playerdata[self.current_player].playfield);
    
        self.playerdata[self.current_player].playfield[x][y].0 = true;

        self.io.update_playfields(&self.playerdata);
    }

    fn replace_card(&mut self, card: i8, msg: &str) {
        let (x, y) = self.io.ask_playfield_position(msg, &self.playerdata[self.current_player].playfield);
        
        self.last_played_card = Some(self.playerdata[self.current_player].playfield[x][y].1);

        self.playerdata[self.current_player].playfield[x][y].0 = true;
        self.playerdata[self.current_player].playfield[x][y].1 = card;

        self.io.update_playfields(&self.playerdata);
    }
}

#[derive(PartialEq)]
pub enum GamePhase {
    InitialReveal,
    Play,
    Ended,
}

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
            playfield: self.stack.draw_playfield()?
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
        f.debug_struct("Game").field("player_data", &self.playerdata).field("stack", &self.stack).finish()
    }
}
