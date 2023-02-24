use std::io;

pub fn select_square() -> Option<(u8, u8)> {
    let mut square = String::new();
    let stdin = io::stdin();
    stdin.read_line(&mut square).unwrap();
    while square.ends_with('\n') || square.ends_with('\r') {
        square.pop();
    }
    square_name_to_position(&square[..])
}

pub fn square_name_to_position(square: &str) -> Option<(u8, u8)> {
    if square.chars().count() != 2 {return None}
    let mut chars = square.chars();
    let col = chars.next().unwrap().to_ascii_lowercase() as u8 - 97;
    let row = chars.next().unwrap().to_digit(10).unwrap() as u8 - 1;

    if col < 8 && row < 8 {
        return Some((row, col));
    }
    None
}