use crate::{console_io::ConsoleIO, game::GameBuilder};

mod game;
mod playerdata;
mod stack;
mod util;
mod console_io;
mod game_io;

fn main() {
    println!("Welcome to SkyJo!");

    let mut builder = GameBuilder::new();

    builder.with_player("Paul".to_owned()).unwrap().with_player("Joey".to_owned()).unwrap();
    
    let io = ConsoleIO::new();
    let mut game = builder.build(Box::new(io));

    game.start();

    while !game.is_done() {
        game.turn();
    }
}
