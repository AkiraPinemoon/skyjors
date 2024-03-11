use crate::{game_io::GameIO, util};
use async_trait::async_trait;
pub struct ConsoleIO {}

impl ConsoleIO {
    pub fn new() -> Self {
        println!("\x1B[2J");
        Self {}
    }
    fn set_cusor_pos(&self, x: usize, y: usize) {
        print!("\x1B[{};{}H", y, x);
    }
    fn clear_line(&self, y: usize) {
        print!("\x1B[{};0H\x1B[2K", y);
    }
}

#[async_trait]
impl GameIO for ConsoleIO {
    async fn update_playfields(&mut self, player_datas: &Vec<crate::playerdata::PlayerData>) {
        self.clear_line(1);
        self.clear_line(2);
        self.clear_line(3);
        self.clear_line(4);
        self.set_cusor_pos(1, 1);

        for (player_id, player_data) in player_datas.iter().enumerate() {
            self.set_cusor_pos(player_id * 17 + 1, 1);
            println!("{}", player_data.name);
            self.set_cusor_pos(player_id * 17 + 1, 2);
            print!(" ");
            for x in 0..player_data.playfield.len() {
                print!("{:3}", x);
            }
            println!("");

            for row in 0..3 {
                self.set_cusor_pos(player_id * 17 + 1, 3 + row);
                print!("{}", util::num_to_alphabet(row));
                for card in player_data.playfield.iter() {
                    if card[row].0 {
                        print!("{:3}", card[row].1);
                    } else {
                        print!("  X");
                    }
                }
                println!("")
            }
        }
    }

    async fn ask_yes_or_no(&mut self, msg: &str) -> bool {
        self.clear_line(7);
        self.clear_line(8);
        self.clear_line(9);
        self.set_cusor_pos(1, 7);

        println!("{}", msg);
        let mut input_line = String::new();
        std::io::stdin()
            .read_line(&mut input_line)
            .expect("Failed to read from stdin");
        match input_line.split_whitespace().take(1).last().unwrap() {
            "yes" | "y" => true,
            "no" | "n" => false,
            _ => {
                println!("Invalid input. Type yes/y/no/n");
                self.ask_yes_or_no("").await
            }
        }
    }

    async fn ask_playfield_position(
        &mut self,
        msg: &str,
        playfield: Vec<[(bool, i8); 3]>,
        validator: fn(playfield: Vec<[(bool, i8); 3]>, pos: (usize, usize)) -> bool,
    ) -> (usize, usize) {
        self.clear_line(7);
        self.clear_line(8);
        self.clear_line(9);
        self.set_cusor_pos(1, 7);

        println!("{}", msg);
        let mut input_line = String::new();
        std::io::stdin()
            .read_line(&mut input_line)
            .expect("Failed to read from stdin");

        if input_line.len() < 2 {
            return self.ask_playfield_position("", playfield, validator.clone()).await;
        }

        let y = match input_line.trim().chars().next() {
            Some(c) => {
                let y = util::alphabet_to_num(c);
                if y <= 2 {
                    Some(y)
                } else {
                    println!("Invalid input. try a lower y coordinate.");
                    None
                }
            }
            None => {
                println!("Invalid input. no letter found.");
                None
            }
        };

        let x = match input_line.trim()[1..].parse::<usize>() {
            Ok(digit) if digit < playfield.len() => Some(digit),
            _ => {
                println!("Invalid input. try a lower x coordinate.");
                None
            }
        };

        match (x, y) {
            (Some(x), Some(y)) => {
                if validator(playfield.clone(), (x, y)) {
                    return (x, y);
                }
                println!("Invalid input. try another coordinate.");
                self.ask_playfield_position("", playfield, validator).await
            }
            _ => self.ask_playfield_position("", playfield, validator).await,
        }
    }

    async fn start_turn(&mut self, player: &str) {
        self.clear_line(6);
        self.clear_line(7);
        self.clear_line(8);
        self.clear_line(9);

        self.set_cusor_pos(1, 6);
        println!("It's your turn {}.", player);
    }

    async fn draw_card(&mut self, card: i8) {
        self.clear_line(6);
        self.set_cusor_pos(1, 6);

        println!("You drew an {:2}", card);
    }

    async fn take_card(&mut self, card: i8) {
        self.clear_line(6);
        self.set_cusor_pos(1, 6);

        println!("You took the {:2}", card);
    }

    async fn end_game(&mut self, playerdata: &Vec<crate::playerdata::PlayerData>) {
        self.update_playfields(playerdata).await;
        self.clear_line(6);
        self.clear_line(7);
        self.clear_line(8);
        self.clear_line(9);

        self.set_cusor_pos(1, 7);

        println!("Game ended!");

        let mut scores = Vec::new();
        playerdata.iter().for_each(|player| {
            let mut score = 0;
            player.playfield.iter().for_each(|column| {
                column.iter().for_each(|card| {
                    score += card.1;
                })
            });
            scores.push((player.name.clone(), score));
        });

        scores.iter().for_each(|(playername, score)| {
            println!("{} scored {}", playername, score);
        });
    }
}
