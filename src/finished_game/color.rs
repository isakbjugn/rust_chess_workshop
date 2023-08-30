use std::fmt;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Color {
    White,
    Black,
}

impl Color {
    pub fn opposite(&self) -> Color {
        match self {
            Color::White => Color::Black,
            Color::Black => Color::White,
        }
    }

    pub fn print_capitalised(&self) -> &str {
        match self {
            Color::White => "Kvit",
            Color::Black => "Svart",
        }
    }
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Color::White => write!(f, "kvit"),
            Color::Black => write!(f, "svart"),
        }
    }
}
