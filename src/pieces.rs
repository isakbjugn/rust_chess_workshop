use std::collections::HashSet;
use std::fmt;
use std::cmp::{min, max};
use crate::utils::get_valid_moves;

#[derive(Clone, Copy, Debug)]
pub struct Piece {
    pub color: Color,
    pub piece_type: Type,
    pub position: (u8, u8), // (row, column)
    pub moved: bool
}

impl Piece {
    pub fn new(color: Color, piece_type: Type, position: (u8, u8)) -> Piece {
        Piece {color, piece_type, position, moved: false}
    }
    pub fn move_piece(&mut self, new_position: (u8, u8)) {
        self.position = new_position;
        self.moved = true;
    }
    pub fn print(&self) -> char {
        match self.color {
            Color::White => self.piece_type.to_string().chars().next().unwrap().to_ascii_uppercase(),
            Color::Black => self.piece_type.to_string().chars().next().unwrap().to_ascii_lowercase(),
        }
    }

    pub fn get_moves(&self) -> HashSet<(u8, u8)> {
        let mut moves = match self.piece_type {
            Type::Pawn => self.get_pawn_moves(),
            Type::Rook => self.get_rook_moves(),
            Type::Knight => self.get_knight_moves(),
            Type::Bishop => self.get_bishop_moves(),
            Type::Queen => self.get_queen_moves(),
            Type::King => self.get_king_moves()
        };
        moves.retain(|&square| square != self.position);
        moves
    }

    fn get_pawn_moves(&self) -> HashSet<(u8, u8)> {
        let mut squares = HashSet::new();
        match self.color {
            Color::White => {
                squares.insert((2, self.position.1));
                if !self.moved { squares.insert((3, self.position.1)); }
            },
            Color::Black => {
                squares.insert((5, self.position.1));
                if !self.moved { squares.insert((4, self.position.1)); }
            }
        }
        squares
    }
    fn get_rook_moves(&self) -> HashSet<(u8, u8)> {
        let mut moves = HashSet::new();
        for row_or_col in 0..8 {
            moves.insert((row_or_col, self.position.1));
            moves.insert((self.position.0, row_or_col));
        }
        moves
    }
    fn get_knight_moves(&self) -> HashSet<(u8, u8)> {
        let y = self.position.0 as i8;
        let x = self.position.1 as i8;
        let mut all_squares= HashSet::from_iter([(y + 2, x - 1), (y - 2, x - 1), (y + 2, x + 1), (y - 2, x + 1), (y - 1, x + 2), (y - 1, x - 2), (y + 1, x + 2), (y + 1, x - 2)]);
        get_valid_moves(&mut all_squares)
    }
    fn get_bishop_moves(&self) -> HashSet<(u8, u8)> {
        let mut moves = HashSet::new();
        let (x, y) = self.position;

        // Øvre høgre diagonal
        for i in 1..=7 {
            if x + i > 7 || y + i > 7 {
                break;
            }
            moves.insert((x + i, y + i));
        }

        // Øvre venstre diagonal
        for i in 1..=7 {
            if x + i > 7 || y < i {
                break;
            }
            moves.insert((x + i, y - i));
        }

        // Nedre høgre diagonal
        for i in 1..=7 {
            if x < i || y + i > 7 {
                break;
            }
            moves.insert((x - i, y + i));
        }

        // Nedre venstre diagonal
        for i in 1..=7 {
            if x < i || y < i {
                break;
            }
            moves.insert((x - i, y - i));
        }

        moves
    }

    fn get_bishop_moves_2(&self) -> HashSet<(u8, u8)> {
        let sum = self.position.0 + self.position.1;
        let mut upper_left_lower_right = match sum {
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
        };

        let difference = self.position.0 as i8 - self.position.1 as i8;
        let mut lower_left_upper_right = match difference {
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
        };

        upper_left_lower_right.append(&mut lower_left_upper_right);
        HashSet::from_iter(upper_left_lower_right)
    }

    fn get_bishop_moves_3(&self) -> HashSet<(u8, u8)> {
        let sum = self.position.0 + self.position.1;
        let mut upper_left_lower_right = HashSet::new();
        for i in 0..=(7 - u8::abs_diff(7, sum)) {
            upper_left_lower_right.insert((min(7, sum) - i, max(7, sum) - 7 + i));
        }

        let difference = self.position.0 as i8 - self.position.1 as i8;
        let mut lower_left_upper_right = HashSet::new();
        for i in 0..=(7 - i8::abs(difference )) as u8 {
            lower_left_upper_right.insert((max(0, difference) as u8 + i, max(0, -difference) as u8 + i));
        }
        upper_left_lower_right.extend(lower_left_upper_right);
        upper_left_lower_right
    }

    fn get_queen_moves(&self) -> HashSet<(u8, u8)> {
        let mut moves = self.get_rook_moves();
        moves.extend(self.get_bishop_moves());
        moves
    }
    fn get_king_moves(&self) -> HashSet<(u8, u8)> {
        let mut moves = HashSet::new();
        let (y, x) = self.position;

        // trekk på øverside
        if y > 0 {
            moves.insert((y - 1, x));
            if x > 0 {
                moves.insert((y - 1, x - 1));
            }
            if x < 7 {
                moves.insert((y - 1, x + 1));
            }
        }

        // trekk på nedside
        if y < 7 {
            moves.insert((y + 1, x));
            if x > 0 {
                moves.insert((y + 1, x - 1));
            }
            if x < 7 {
                moves.insert((y + 1, x + 1));
            }
        }

        // trekk til venstre
        if x > 0 {
            moves.insert((y, x - 1));
        }

        // trekk til høgre
        if x < 7 {
            moves.insert((y, x + 1));
        }

        moves
    }
    fn get_king_moves_2(&self) -> HashSet<(u8, u8)> {
        match self.position {
            (7, 0) => HashSet::from_iter([(7, 1), (6, 1), (6, 0)]),
            (7, 7) => HashSet::from_iter([(6, 7), (6, 6), (7, 6)]),
            (0, 7) => HashSet::from_iter([(0, 6), (1, 6), (1, 7)]),
            (0, 0) => HashSet::from_iter([(1, 0), (1, 1), (0, 1)]),
            (7, x) => HashSet::from_iter([(7, x - 1), (7, x + 1), (6, x - 1), (6, x), (6, x + 1)]),
            (0, x) => HashSet::from_iter([(1, x - 1), (1, x), (1, x + 1), (0, x - 1), (0, x + 1)]),
            (y, 0) => HashSet::from_iter([(y + 1, 0), (y + 1, 1), (y, 1), (y - 1, 0), (y - 1, 1)]),
            (y, 7) => HashSet::from_iter([(y + 1, 6), (y + 1, 7), (y, 6), (y - 1, 6), (y - 1, 7)]),
            (y, x) => HashSet::from_iter([(y + 1, x - 1),(y + 1, x), (y + 1, x + 1), (y, x - 1), (y, x + 1), (y - 1, x - 1), (y - 1, x), (y - 1, x + 1)])
        }
    }
    fn get_king_moves_3(&self) -> HashSet<(u8, u8)> {
        let y = self.position.0 as i8;
        let x = self.position.1 as i8;
        let mut all_squares = HashSet::from_iter([(y + 1, x - 1),(y + 1, x), (y + 1, x + 1), (y, x - 1), (y, x + 1), (y - 1, x - 1), (y - 1, x), (y - 1, x + 1)]);
        get_valid_moves(&mut all_squares)
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Color {
    White,
    Black
}

#[derive(Copy, Clone, Debug)]
pub enum Type {
    Pawn,
    Rook,
    Knight,
    Bishop,
    Queen,
    King
}

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Type::Knight => write!(f, "Night"),
            _ => write!(f, "{:?}", self)
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;
    use crate::pieces::{Piece, Color, Type};

    #[test]
    fn test_bishop_moves_1() {
        let bishop = Piece::new(Color::White, Type::Bishop, (0, 0));
        let legal_moves = HashSet::from_iter(vec![(1, 1), (2, 2), (3, 3), (4, 4), (5, 5), (6, 6), (7, 7)]);
        assert_eq!(bishop.get_moves(), legal_moves)
    }

    #[test]
    fn test_bishop_moves_2() {
        let bishop = Piece::new(Color::White, Type::Bishop, (2, 3));
        let legal_moves = HashSet::from_iter(vec![(0, 1), (1, 2), (3, 4), (4, 5), (5, 6), (6, 7), (5, 0), (4, 1), (3, 2), (1, 4), (0, 5)]);
        assert_eq!(bishop.get_moves(), legal_moves)
    }

    #[test]
    fn test_king_moves_edge() {
        let bishop = Piece::new(Color::White, Type::King, (0, 5));
        let legal_moves = HashSet::from_iter([(0, 4), (1, 4), (1, 5), (1, 6), (0, 6)]);
        assert_eq!(bishop.get_moves(), legal_moves)
    }

    #[test]
    fn test_king_moves_center() {
        let bishop = Piece::new(Color::White, Type::King, (4, 4));
        let legal_moves = HashSet::from_iter([(5, 3), (5, 4), (5, 5), (4, 3), (4, 5), (3, 3), (3, 4), (3, 5)]);
        assert_eq!(bishop.get_moves(), legal_moves)
    }

    #[test]
    fn test_knight_moves_edge() {
        let bishop = Piece::new(Color::White, Type::Knight, (4, 0));
        let legal_moves = HashSet::from_iter([(2, 1), (3, 2), (5, 2), (6, 1)]);
        assert_eq!(bishop.get_moves(), legal_moves)
    }

    #[test]
    fn test_knight_moves_center() {
        let bishop = Piece::new(Color::White, Type::Knight, (4, 4));
        let legal_moves = HashSet::from_iter([(5, 2), (3, 2), (6, 3), (2, 3), (6, 5), (2, 5), (5, 6), (3, 6)]);
        assert_eq!(bishop.get_moves(), legal_moves)
    }
}

