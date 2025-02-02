use super::board;
use super::utils;
use rand::Rng;

pub fn get_player_name(player_name: &mut String) {
    println!("Enter player's name: ");
    *player_name = utils::read_input::<String>()
        .expect("Failed to read input")
        .trim()
        .to_string();
}

pub fn handle_human_player_turn(turn: &mut i32, board: &mut [[i32; 3]; 3]) {
    let (row, column) = get_player_move();
    if board[row][column] == 0 {
        board[row][column] = *turn;
        *turn = if *turn == 1 { 2 } else { 1 };
    }
}

pub fn handle_computer_turn(board: &mut [[i32; 3]; 3]) {
    let mut choice: Option<(usize, usize)> = board::check_for_win_or_block_movement(board, true);

    if choice.is_none() {
        choice = board::check_for_win_or_block_movement(board, false);
    }

    if let Some((row, column)) = choice {
        board[row][column] = 2;
    } else {
        let mut row = rand::rng().random_range(1..=board.len() - 1);
        let mut column = rand::rng().random_range(1..=board.len() - 1);
        while board[row][column] != 0 {
            row = rand::rng().random_range(1..=board.len() - 1);
            column = rand::rng().random_range(1..=board.len() - 1);
        }
        board[row][column] = 2;
    }
}

pub fn get_player_move() -> (usize, usize) {
    loop {
        let user_input = utils::read_input::<String>().expect("Failed to read input");
        let coordinates: Vec<usize> = user_input
            .trim()
            .split(',')
            .filter_map(|coordinate| coordinate.trim().parse().ok())
            .collect();

        if coordinates.len() == 2 && coordinates[0] < 3 && coordinates[1] < 3 {
            return (coordinates[0], coordinates[1]);
        }
        println!("Invalid coordinates! Must be two numbers (0-2) separated by a comma.");
    }
}
