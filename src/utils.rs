use std::io;

pub fn get_game_mode() -> String {
    println!("Do you want to play single or with a friend ? (single/friend): ");
    read_input::<String>().expect("Failed to read input")
}

pub fn read_input<T>() -> Result<T, String>
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

pub fn clear_screen() {
    print!("\x1B[2J\x1B[1;1H");
    io::Write::flush(&mut io::stdout()).unwrap();
}
