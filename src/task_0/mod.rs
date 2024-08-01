mod board;
mod game;

pub fn main() {
    println!("Køyrer game::main() i oppgåve 0");
    game::main()
}

/// Definer en `enum` som heter `Color`:
enum Color {
    White,
    Black,
}

/// Definer en `struct` som heter `Pawn`:
struct Pawn {
    color: Color,
    position: (u8, u8),
}

impl Pawn {
    fn new(color: Color, position: (u8, u8)) -> Self {
        Pawn { color, position }
    }
    fn print(&self) -> char {
        match self.color {
            Color::White => '♟',
            Color::Black => '♙',
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{Color, Pawn};

    #[test]
    fn pawn_struct_and_color_enum_exists() {
        let white_pawn = Pawn::new(Color::White, (0, 1));
        assert_eq!(white_pawn.print(), '♟');
        assert_eq!(white_pawn.position, (0, 1));
        let black_pawn = Pawn::new(Color::Black, (0, 6));
        assert_eq!(black_pawn.print(), '♙');
        assert_eq!(black_pawn.position, (0, 6));
    }
}