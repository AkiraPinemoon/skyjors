use crate::game::GameBuilder;

mod game;
mod playerdata;
mod stack;
mod util;

fn main() {
    println!("Welcome to SkyJo!");

    let mut builder = GameBuilder::new();
    builder.with_player("Paul".to_owned()).unwrap();

    let mut game = builder.build();

    println!("{:?}", game);

    while !game.is_done() {
        game.turn();
    }
}
