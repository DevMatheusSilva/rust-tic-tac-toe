pub fn restart_board(board: &mut [[i32; 3]; 3]) {
    for row in board.iter_mut() {
        row.fill(0);
    }
}

pub fn render_board(board: &[[i32; 3]; 3]) {
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

pub fn check_for_win_or_block_movement(
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

pub fn check_winner(board: &[[i32; 3]; 3]) -> Option<i32> {
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

pub fn is_draw(board: &[[i32; 3]; 3]) -> bool {
    board.iter().all(|row| row.iter().all(|&cell| cell != 0))
}
