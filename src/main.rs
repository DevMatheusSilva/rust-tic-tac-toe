extern crate rand;
use rand::Rng;
use std::{io, thread, time};

fn main() {
    println!("Welcome to Tic Tac Toe !!");

    let mut board = [[0; 3]; 3];
    let mut turn = 1;

    let mut game_mode = get_game_mode();
    while game_mode != "single" && game_mode != "friend" {
        game_mode = get_game_mode();
    }

    game_main_loop(&mut board, &mut turn, &game_mode[..]);
}

fn game_main_loop(board: &mut [[i32; 3]; 3], turn: &mut i32, game_mode: &str) {
    let mut player1_name = String::new();
    let mut player2_name = String::from("Computer");

    match game_mode {
        "single" => {
            get_player_name(&mut player1_name);
        }
        "friend" => {
            get_player_name(&mut player1_name);
            get_player_name(&mut player2_name);
        }
        _ => println!("Invalid game option !!"),
    }

    loop {
        while check_winner(board).is_none() && !is_draw(board) {
            clear_screen();
            render_board(board);

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

                handle_computer_turn(board);

                *turn = 1;
            } else {
                handle_human_player_turn(turn, board);
            }
        }

        clear_screen();
        render_board(board);

        if let Some(winner) = check_winner(board) {
            println!(
                "{} venceu!",
                if winner == 1 {
                    &player1_name
                } else {
                    &player2_name
                }
            );
        } else {
            println!("Empate!");
        }

        println!("Deseja jogar novamente? [S/N]: ");
        let option = read_input::<String>()
            .expect("Failed to read input")
            .trim()
            .to_uppercase();
        if option != "S" {
            println!("Goodbye...");
            break;
        }
        restart_board(board);
    }
}

fn get_player_name(player_name: &mut String) {
    println!("Enter player's name: ");
    *player_name = read_input::<String>()
        .expect("Failed to read input")
        .trim()
        .to_string();
}

fn get_game_mode() -> String {
    println!("Do you want to play single or with a friend ? (single/friend): ");
    read_input::<String>().expect("Failed to read input")
}

fn read_input<T>() -> Result<T, String>
where
    T: std::str::FromStr,
{
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .map_err(|_| "Failed to read input".to_string())?;
    input
        .trim()
        .parse::<T>()
        .map_err(|_| "Failed to parse input".to_string())
}

fn restart_board(board: &mut [[i32; 3]; 3]) {
    for row in board.iter_mut() {
        row.fill(0);
    }
}

fn render_board(board: &[[i32; 3]; 3]) {
    for (i, row) in board.iter().enumerate() {
        for (j, &element) in row.iter().enumerate() {
            print!(" {} ", cell_to_symbol(element));
            if j < 2 {
                print!("|");
            }
        }
        println!();
        if i < 2 {
            println!("---+---+---");
        }
    }
}

fn cell_to_symbol(value: i32) -> char {
    match value {
        1 => 'X',
        2 => 'O',
        _ => ' ',
    }
}

fn handle_human_player_turn(turn: &mut i32, board: &mut [[i32; 3]; 3]) {
    let (row, column) = get_player_move();
    if board[row][column] == 0 {
        board[row][column] = *turn;
        *turn = if *turn == 1 { 2 } else { 1 };
    }
}

fn handle_computer_turn(board: &mut [[i32; 3]; 3]) {
    let mut choice: Option<(usize, usize)> = check_for_win_or_block_movement(board, true);

    if choice.is_none() {
        choice = check_for_win_or_block_movement(board, false);
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

fn check_for_win_or_block_movement(
    board: &[[i32; 3]; 3],
    check_for_win: bool,
) -> Option<(usize, usize)> {
    let check_number = if check_for_win { 2 } else { 1 };

    for i in 0..board.len() {
        for j in 0..board[i].len() {
            if board[i][j] == 0 {
                let mut temp_board = *board;
                temp_board[i][j] = check_number;
                if check_winner(&temp_board) == Some(check_number) {
                    return Some((i, j));
                }
            }
        }
    }

    None
}

fn get_player_move() -> (usize, usize) {
    loop {
        let user_input = read_input::<String>().expect("Failed to read input");
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

fn check_winner(board: &[[i32; 3]; 3]) -> Option<i32> {
    for i in 0..3 {
        if check_row(i, board) {
            return Some(board[i][0]);
        }
        if check_col(i, board) {
            return Some(board[0][i]);
        }
    }
    if check_diagonals(board) {
        return Some(board[1][1]);
    }
    None
}

fn check_row(i: usize, board: &[[i32; 3]; 3]) -> bool {
    board[i][0] != 0 && board[i][0] == board[i][1] && board[i][1] == board[i][2]
}

fn check_col(i: usize, board: &[[i32; 3]; 3]) -> bool {
    board[0][i] != 0 && board[0][i] == board[1][i] && board[1][i] == board[2][i]
}

fn check_diagonals(board: &[[i32; 3]; 3]) -> bool {
    (board[0][0] != 0 && board[0][0] == board[1][1] && board[1][1] == board[2][2])
        || (board[0][2] != 0 && board[0][2] == board[1][1] && board[1][1] == board[2][0])
}

fn is_draw(board: &[[i32; 3]; 3]) -> bool {
    board.iter().all(|row| row.iter().all(|&cell| cell != 0))
}

fn clear_screen() {
    print!("\x1B[2J\x1B[1;1H");
    io::Write::flush(&mut io::stdout()).unwrap();
}
