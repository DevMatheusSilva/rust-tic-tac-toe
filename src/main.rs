extern crate rand;
mod board;
mod game;
mod player;
mod utils;

fn main() {
    println!("Welcome to Tic Tac Toe !!");

    let mut board = [[0; 3]; 3];
    let mut turn = 1;

    let mut game_mode = utils::get_game_mode();
    while game_mode != "single" && game_mode != "friend" {
        game_mode = utils::get_game_mode();
    }

    game::start(&mut board, &mut turn, &game_mode[..]);
}
