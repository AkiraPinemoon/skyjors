use crate::{playerdata::PlayerData, stack::Stack, util::{self, alphabet_to_num}};
use std::fmt;

pub struct Game {
    player_data: Vec<PlayerData>,
    stack: Stack,
    current_player: usize,
    phase: GamePhase,
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
        let mut input_line = String::new();
        std::io::stdin().read_line(&mut input_line).expect("Failed to read from stdin");
    
        let y = match input_line.trim().chars().next() {
            Some(c) => alphabet_to_num(c),
            None => {
                println!("Invalid input. Please enter a character and a 1-digit integer.");
                return;
            }
        };
    
        let x = match input_line.trim()[1..].parse::<u8>() {
            Ok(digit) if digit < 10 => digit as usize,
            _ => {
                println!("Invalid input. Please enter a valid 1-digit integer.");
                return;
            }
        };
    
        self.player_data[self.current_player].playfield[x][y].0 = true;

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
}

impl GameBuilder {
    pub fn new() -> Self {
        Self {
            player_data: Vec::new(),
            stack: Stack::new(),
            current_player: 0,
            phase: GamePhase::InitialReveal,
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
        }
    }
}

impl fmt::Debug for Game {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Game").field("player_data", &self.player_data).field("stack", &self.stack).finish()
    }
}
