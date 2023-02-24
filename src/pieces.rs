use std::fmt;

#[derive(Clone, Copy, Debug)]
pub struct Piece {
    pub color: Color,
    pub piece_type: Type,
    pub position: (u8, u8),
}

impl Piece {
    pub fn move_piece(&mut self, new_position: (u8, u8)) {
        self.position = new_position;
    }
    pub fn print(&self) -> char {
        match self.color {
            Color::White => self.piece_type.to_string().chars().next().unwrap().to_ascii_uppercase(),
            Color::Black => self.piece_type.to_string().chars().next().unwrap().to_ascii_lowercase(),
        }
    }
}

#[derive(Copy, Clone, Debug)]
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