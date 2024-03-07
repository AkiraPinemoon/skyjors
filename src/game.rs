use crate::{playerdata::PlayerData, stack::Stack, util::{self, alphabet_to_num, ask_matrix_option, ask_yes_or_no}};
use std::fmt;

pub struct Game {
    player_data: Vec<PlayerData>,
    stack: Stack,
    current_player: usize,
    phase: GamePhase,
    last_played_card: Option<i8>,
}

impl Game {
    pub fn is_done(&self) -> bool {
        self.phase == GamePhase::Ended
    }
    pub fn turn(&mut self) {
        match self.phase {
            GamePhase::InitialReveal => {
                println!("It's your turn, {}", self.player_data[self.current_player].name);
                self.print_current_playfield();

                println!("Choose your first card to reveal (e.g., A3)");
                self.reveal_card();

                println!("Choose your second card to reveal");
                self.reveal_card();

                if self.current_player == self.player_data.len() - 1 {
                    self.phase = GamePhase::Play;
                }
            },
            GamePhase::Play => {
                self.print_current_playfield();
                let chosen_card = match self.last_played_card {
                    None => {
                        let card = self.stack.draw().unwrap();
                        println!("You drew an {}", card);
                        card
                    },
                    Some(last) => {
                        println!("You can either take the {} that was thrown away last or draw a random card, do you want to draw a card?", last);
                        let card = match ask_yes_or_no() {
                            true => {
                                let card = self.stack.draw().unwrap();
                                println!("You drew an {}", card);
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

                println!("Do you want to play this card (or throw it away)?");
                match ask_yes_or_no() {
                    true => {
                        println!("Sure. Pick what card to replace.");
                        self.replace_card(chosen_card);
                    },
                    false => {
                        println!("Ok. Pick a card to reveal.");
                        self.last_played_card = Some(chosen_card);
                        self.reveal_card();
                    },
                };
            },
            GamePhase::Ended => {

            },
        }

        self.current_player += 1;
        self.current_player %= self.player_data.len();
    }

    fn print_current_playfield(&self) {
        print!(" ");
        for x in 0..self.player_data[self.current_player].playfield.len() {
            print!("{:3}", x);
        }
        println!("");

        for row in 0..3 {
            print!("{}", util::num_to_alphabet(row));
            for card in self.player_data[self.current_player].playfield.iter() {
                if card[row].0 { print!("{:3}", card[row].1); }
                else { print!("  X"); }
            }
            println!("")
        }
    }

    fn reveal_card(&mut self) {
        let (x, y) = ask_matrix_option(0, self.player_data[self.current_player].playfield.len(), 0, 2);
    
        self.player_data[self.current_player].playfield[x][y].0 = true;

        self.print_current_playfield();
    }

    fn replace_card(&mut self, card: i8) {
        let (x, y) = ask_matrix_option(0, self.player_data[self.current_player].playfield.len(), 0, 2);
        
        self.last_played_card = Some(self.player_data[self.current_player].playfield[x][y].1);

        self.player_data[self.current_player].playfield[x][y].0 = true;
        self.player_data[self.current_player].playfield[x][y].1 = card;

        self.print_current_playfield();
    }
}

#[derive(PartialEq)]
pub enum GamePhase {
    InitialReveal,
    Play,
    Ended,
}

pub struct GameBuilder {
    player_data: Vec<PlayerData>,
    stack: Stack,
    current_player: usize,
    phase: GamePhase,
    last_played_card: Option<i8>,
}

impl GameBuilder {
    pub fn new() -> Self {
        Self {
            player_data: Vec::new(),
            stack: Stack::new(),
            current_player: 0,
            phase: GamePhase::InitialReveal,
            last_played_card: None,
        }
    }

    pub fn with_player(&mut self, name: String) -> Result<&mut Self, String> {
        self.player_data.push(PlayerData {
            name,
            playfield: self.stack.draw_playfield()?
        });
        Ok(self)
    }

    pub fn build(self) -> Game {
        Game {
            player_data: self.player_data,
            stack: self.stack,
            current_player: self.current_player,
            phase: self.phase,
            last_played_card: self.last_played_card,
        }
    }
}

impl fmt::Debug for Game {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Game").field("player_data", &self.player_data).field("stack", &self.stack).finish()
    }
}
