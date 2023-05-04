use std::collections::HashSet;

pub trait Squares {
    fn chess_board_filter(&self) -> HashSet<(u8, u8)>;
}

impl Squares for HashSet<(i8, i8)> {
    fn chess_board_filter(&self) -> HashSet<(u8, u8)> {
        self.iter()
            .filter(|(y, x)| (0..8).contains(y) && (0..8).contains(x))
            .map(|&(y, x)| (y as u8, x as u8))
            .collect()
    }
}