use crate::{game_io::GameIO, util};

pub struct ConsoleIO {
}

impl GameIO for ConsoleIO {
    fn update_playfields(&mut self, player_datas: &Vec<crate::playerdata::PlayerData>) {
        for player_data in player_datas {
            println!("{}", player_data.name);
            print!(" ");
            for x in 0..player_data.playfield.len() {
                print!("{:3}", x);
            }
            println!("");

            for row in 0..3 {
                print!("{}", util::num_to_alphabet(row));
                for card in player_data.playfield.iter() {
                    if card[row].0 { print!("{:3}", card[row].1); }
                    else { print!("  X"); }
                }
                println!("")
            }
        }
    }

    fn ask_yes_or_no(&mut self, msg: &str) -> bool {
        println!("{}", msg);
        let mut input_line = String::new();
        std::io::stdin().read_line(&mut input_line).expect("Failed to read from stdin");
        match input_line.split_whitespace().take(1).last().unwrap() {
            "yes" | "y" => true,
            "no" | "n" => false,
            _ => {
                println!("Invalid input. Type yes/y/no/n");
                self.ask_yes_or_no("")
            },
        }
    }

    fn ask_playfield_position(&mut self, msg: &str, playfield: &Vec<[(bool, i8); 3]>) -> (usize, usize) {
        println!("{}", msg);
        let mut input_line = String::new();
        std::io::stdin().read_line(&mut input_line).expect("Failed to read from stdin");

        let y = match input_line.trim().chars().next() {
            Some(c) => {
                let y = util::alphabet_to_num(c);
                if y <= 2 {
                    Some(y)
                } else {
                    println!("Invalid input. try a lower y coordinate.");
                    None
                }
            },
            None => {
                println!("Invalid input. no letter found.");
                None
            }
        };

        let x = match input_line.trim()[1..].parse::<usize>() {
            Ok(digit) if digit <= playfield.len() => Some(digit),
            _ => {
                println!("Invalid input. try a lower x coordinate.");
                None
            }
        };

        match (x, y) {
            (Some(x), Some(y)) => (x, y),
            _ => self.ask_playfield_position("", playfield)
        }
    }
    
    fn start_turn(&mut self, player: &str) {
        println!("It's your turn {}.", player);
    }
    
    fn draw_card(&mut self, card: i8) {
        println!("You drew an {}", card);
    }
}
