use std::collections::HashSet;
use std::io;
use std::cmp::{min, max};

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
    if square.chars().count() != 2 { return None }
    let mut chars = square.chars();
    let col = chars.next().unwrap().to_ascii_lowercase() as u8 - 97;
    let row = chars.next().unwrap().to_digit(10).unwrap() as u8 - 1;

    if col < 8 && row < 8 {
        return Some((row, col));
    }
    None
}

pub fn get_valid_moves(moves: &mut HashSet<(i8, i8)>) -> HashSet<(u8, u8)> {
    moves.retain(|(y, x)| (0..8).contains(y) && (0..8).contains(x));
    moves.iter().map(|&(y, x)| (y as u8, x as u8)).collect()
}

pub fn to_move_lines(moves: &HashSet<(u8, u8)>) -> HashSet<Vec<(u8, u8)>> {
    HashSet::from_iter(moves.iter().map(|&position| vec![position]))
}

pub fn to_moves(move_lines: HashSet<Vec<(u8, u8)>>) -> HashSet<(u8, u8)> {
    move_lines.into_iter().flatten().collect()
}

pub fn get_north_west_diagonal(position: &(u8, u8)) -> Vec<(u8, u8)> {
    let sum = position.0 + position.1;
    match sum {
        0 => vec![(0, 0)],
        1 => vec![(1, 0), (0, 1)],
        2 => vec![(2, 0), (1, 1), (2, 0)],
        3 => vec![(3, 0), (2, 1), (1, 2), (0, 3)],
        4 => vec![(4, 0), (3, 1), (2, 2), (1, 3), (0, 4)],
        5 => vec![(5, 0), (4, 1), (3, 2), (2, 3), (1, 4), (0, 5)],
        6 => vec![(6, 0), (5, 1), (4, 2), (3, 3), (2, 4), (1, 5), (0, 6)],
        7 => vec![(7, 0), (6, 1), (5, 2), (4, 3), (3, 4), (2, 5), (1, 6), (0, 7)],
        8 => vec![(7, 1), (6, 2), (5, 3), (4, 4), (3, 5), (2, 6), (1, 7)],
        9 => vec![(7, 2), (6, 3), (5, 4), (4, 5), (3, 6), (2, 7)],
        10 => vec![(7, 3), (6, 4), (5, 5), (4, 6), (3, 7)],
        11 => vec![(7, 4), (6, 5), (5, 6), (4, 7)],
        12 => vec![(7, 5), (6, 6), (5, 7)],
        13 => vec![(7, 6), (6, 7)],
        14 => vec![(7, 7)],
        _ => panic!()
    }
}

pub fn get_south_west_diagonal(position: &(u8, u8)) -> Vec<(u8, u8)> {
    let difference = position.0 as i8 - position.1 as i8;
    match difference {
        7 => vec![(7, 0)],
        6 => vec![(6, 0), (7, 1)],
        5 => vec![(5, 0), (6, 1), (7, 0)],
        4 => vec![(4, 0), (5, 1), (6, 2), (7, 3)],
        3 => vec![(3, 0), (4, 1), (5, 2), (6, 3), (7, 4)],
        2 => vec![(2, 0), (3, 1), (4, 2), (5, 3), (6, 4), (7, 5)],
        1 => vec![(1, 0), (2, 1), (3, 2), (4, 3), (5, 4), (6, 5), (7, 6)],
        0 => vec![(0, 0), (1, 1), (2, 2), (3, 3), (4, 4), (5, 5), (6, 6), (7, 7)],
        -1 => vec![(0, 1), (1, 2), (2, 3), (3, 4), (4, 5), (5, 6), (6, 7)],
        -2 => vec![(0, 2), (1, 3), (2, 4), (3, 5), (4, 6), (5, 7)],
        -3 => vec![(0, 3), (1, 4), (2, 5), (3, 6), (4, 7)],
        -4 => vec![(0, 4), (1, 5), (2, 6), (3, 7)],
        -5 => vec![(0, 5), (1, 6), (2, 7)],
        -6 => vec![(0, 6), (1, 7)],
        -7 => vec![(0, 7)],
        _ => panic!()
    }
}

fn _get_diagonals_by_formula(position: &(u8, u8)) -> HashSet<(u8, u8)> {
    let sum = position.0 + position.1;
    let mut upper_left_lower_right = HashSet::new();
    for i in 0..=(7 - u8::abs_diff(7, sum)) {
        upper_left_lower_right.insert((min(7, sum) - i, max(7, sum) - 7 + i));
    }

    let difference = position.0 as i8 - position.1 as i8;
    let mut lower_left_upper_right = HashSet::new();
    for i in 0..=(7 - i8::abs(difference )) as u8 {
        lower_left_upper_right.insert((max(0, difference) as u8 + i, max(0, -difference) as u8 + i));
    }
    upper_left_lower_right.extend(lower_left_upper_right);
    upper_left_lower_right
}
