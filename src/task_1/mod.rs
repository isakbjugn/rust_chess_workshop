use std::collections::HashSet;
use crate::task_1::color::Color;
use crate::task_1::piece::Piece;

mod color;
mod piece;
mod game;
mod board;

/// Oppgave 1
/// `Pawn`-strukten inneholoder allerede metodene `new` og `print`. I denne oppgaven skal vi utvide
/// `Pawn` med flere metoder vi har behov for.
/// Vi skal lage tre nyttefunksjoner, for å ha tilgang til private felt:
///    - `get_name` (gir ut brikkens navn, finnes som strengkonstant)
///    - `get_color` (gir ut brikkens farge)
///    - `get_position` (gir ut brikkens posisjon)
///
/// Samt tre metoder vi vil trenge på sikt:
///    - `move_piece` (endrer brikkens posisjon)
///    - `get_moves` (henter ut gyldige felt en brikke kan flytte til)
///
/// Les i Rust-boka om:
///    - [Metoder som muterer](https://doc.rust-lang.org/book/ch05-03-method-syntax.html?#defining-methods)
pub fn main() {
    println!("Kjører game::main() i task_1");
    game::main()
}

#[derive(Clone)]
struct Pawn {
    color: Color,
    position: (u8, u8),
}

impl Piece for Pawn {
    fn new(color: Color, position: (u8, u8)) -> Self {
        Pawn {
            color,
            position,
        }
    }
    fn print(&self) -> char {
        match self.color {
            Color::White => '♙',
            Color::Black => '♟',
        }
    }

    fn get_name(&self) -> String {
        todo!()
    }

    fn get_color(&self) -> Color {
        todo!()
    }

    fn get_position(&self) -> &(u8, u8) {
        &self.position
    }

    fn move_piece(&mut self, target: (u8, u8)) {
        todo!()
    }

    fn get_moves(&self, team: &HashSet<(u8, u8)>, rival_team: &HashSet<(u8, u8)>) -> HashSet<(u8, u8)> {
        todo!()
    }
}

impl Pawn {
    fn get_pawn_moves(&self) -> HashSet<(u8, u8)> {
        HashSet::new()
    }

    fn get_pawn_capture_moves(&self) -> HashSet<(u8, u8)> {
        HashSet::new()
    }
}