use std::collections::HashSet;
use crate::finished_game::color::Color;
use crate::finished_game::piece::Piece;
use crate::square::{Square, Squares};

const KNIGHT_NAME: &str = "springar";

#[derive(Clone)]
pub struct Knight {
    color: Color,
    position: (u8, u8),
}

impl Piece for Knight {
    fn new(color: Color, position: (u8, u8)) -> Self {
        Knight {
            color,
            position,
        }
    }
    fn print(&self) -> char {
        match self.color {
            Color::White => '♘',
            Color::Black => '♞',
        }
    }
    fn get_name(&self) -> String {
        String::from(KNIGHT_NAME)
    }
    fn get_color(&self) -> Color {
        self.color
    }
    fn get_position(&self) -> &(u8, u8) {
        &self.position
    }
    fn move_piece(&mut self, target: (u8, u8)) {
        self.position = target;
    }
    fn get_moves(&self, team: &HashSet<(u8, u8)>, _: &HashSet<(u8, u8)>) -> HashSet<(u8, u8)> {
        let (x, y) = self.position.as_i8().unwrap();
        let moves: HashSet<(i8, i8)> = HashSet::from_iter([
            (x - 2, y + 1), (x - 2, y - 1), // to kolonner til venstre
            (x - 1, y + 2), (x - 1, y - 2), // én kolonne til venstre
            (x + 1, y + 2), (x + 1, y - 2), // én kolonne til høyre
            (x + 2, y + 1), (x + 2, y - 1), // to kolonner til høyre
        ]);
        moves.as_board_positions().difference(team).cloned().collect()
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;
    use crate::assert_eq_set;
    use crate::finished_game::color::Color;
    use crate::finished_game::piece::knight::Knight;
    use crate::finished_game::piece::Piece;
    use crate::square::{Square, Squares};

    #[test]
    fn test_knight_moves_edge() {
        let knight = Knight::new(Color::White, "a5".as_u8().unwrap());
        let positions = HashSet::new();
        let legal_moves = ["b3", "c4", "c6", "b7"].as_board_positions();
        assert_eq_set!(knight.get_moves(&positions, &positions), legal_moves)
    }

    #[test]
    fn test_knight_moves_center() {
        let knight = Knight::new(Color::White, "e5".as_u8().unwrap());
        let positions = HashSet::new();
        let legal_moves = ["c6", "c4", "d7", "d3", "f7", "f3", "g6", "g4"].as_board_positions();
        assert_eq_set!(knight.get_moves(&positions, &positions), legal_moves)
    }
}