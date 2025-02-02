use crate::{board, player, utils};
use std::{thread, time};

pub fn start(board: &mut [[i32; 3]; 3], turn: &mut i32, game_mode: &str) {
    let mut player1_name = String::new();
    let mut player2_name = String::from("Computer");

    match game_mode {
        "single" => {
            player::get_player_name(&mut player1_name);
        }
        "friend" => {
            player::get_player_name(&mut player1_name);
            player::get_player_name(&mut player2_name);
        }
        _ => println!("Invalid game option !!"),
    }

    loop {
        while board::check_winner(board).is_none() && !board::is_draw(board) {
            utils::clear_screen();
            board::render_board(board);

            println!(
                "{}'s turn! Enter row and column separated by a ',' (only values from 0 to 2): ",
                if *turn == 1 {
                    &player1_name
                } else {
                    &player2_name
                }
            );

            if game_mode == "single" && *turn == 2 {
                thread::sleep(time::Duration::from_millis(1000));

                player::handle_computer_turn(board);

                *turn = 1;
            } else {
                player::handle_human_player_turn(turn, board);
            }
        }

        utils::clear_screen();
        board::render_board(board);

        if let Some(winner) = board::check_winner(board) {
            println!(
                "{} wins!",
                if winner == 1 {
                    &player1_name
                } else {
                    &player2_name
                }
            );
        } else {
            println!("Draw!");
        }

        println!("Do you wish to play again? [S/N]: ");
        let option = utils::read_input::<String>()
            .expect("Failed to read input")
            .trim()
            .to_uppercase();
        if option != "S" {
            println!("Goodbye...");
            break;
        }
        board::restart_board(board);
    }
}
