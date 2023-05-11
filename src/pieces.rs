use std::collections::HashSet;
use std::fmt::{Debug, Formatter};
#[cfg(feature = "gui")]
use std::fs::read;
#[cfg(feature = "gui")]
use egui_extras::RetainedImage;
use crate::enums::{Color, PieceType};
use crate::squares::{MoveDirections, Square, Squares};
use crate::utils::{get_south_east_diagonal, get_north_east_diagonal};

pub struct Piece {
    #[cfg(feature = "gui")]
    pub image: Option<RetainedImage>,
    pub color: Color,
    pub piece_type: PieceType,
    pub position: (u8, u8), // (row, column)
}

impl Clone for Piece {
    fn clone(&self) -> Self {
        Piece::new(self.color, self.piece_type, self.position)
    }
}

impl Debug for Piece {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?} {} at {:?}", self.color, self.piece_type, self.position)
    }
}

impl Piece {
    pub fn new(color: Color, piece_type: PieceType, position: (u8, u8)) -> Piece {
        #[cfg(feature = "gui")]
        let image_path = match (piece_type, color) {
            (PieceType::Pawn, Color::White) => "assets/pawn-white-48.png",
            (PieceType::Pawn, Color::Black) => "assets/pawn-black-48.png",
            (PieceType::Rook, Color::White) => "assets/rook-white-48.png",
            (PieceType::Rook, Color::Black) => "assets/rook-black-48.png",
            (PieceType::Knight, Color::White) => "assets/knight-white-48.png",
            (PieceType::Knight, Color::Black) => "assets/knight-black-48.png",
            (PieceType::Bishop, Color::White) => "assets/bishop-white-48.png",
            (PieceType::Bishop, Color::Black) => "assets/bishop-black-48.png",
            (PieceType::Queen, Color::White) => "assets/queen-white-48.png",
            (PieceType::Queen, Color::Black) => "assets/queen-black-48.png",
            (PieceType::King, Color::White) => "assets/king-white-48.png",
            (PieceType::King, Color::Black) => "assets/king-black-48.png",
        };
        Piece {
            color,
            piece_type,
            position,
            #[cfg(feature = "gui")]
            image: Some(RetainedImage::from_image_bytes(image_path, &read(image_path).unwrap()).unwrap()),
        }
    }

    pub fn get_piece_type(&self) -> PieceType {
        self.piece_type
    }

    pub fn get_position(&self) -> (u8, u8) {
        self.position
    }

    pub fn get_color(&self) -> Color {
        self.color
    }

    pub fn move_piece(&mut self, new_position: (u8, u8)) {
        self.position = new_position;
    }

    pub fn print(&self) -> char {
        match (self.piece_type, self.color) {
            (PieceType::Pawn, Color::White) => '♙',
            (PieceType::Pawn, Color::Black) => '♟',
            (PieceType::Rook, Color::White) => '♖',
            (PieceType::Rook, Color::Black) => '♜',
            (PieceType::Knight, Color::White) => '♘',
            (PieceType::Knight, Color::Black) => '♞',
            (PieceType::Bishop, Color::White) => '♗',
            (PieceType::Bishop, Color::Black) => '♝',
            (PieceType::Queen, Color::White) => '♕',
            (PieceType::Queen, Color::Black) => '♛',
            (PieceType::King, Color::White) => '♔',
            (PieceType::King, Color::Black) => '♚',
        }
    }

    pub fn get_moves(&self, team: &HashSet<(u8, u8)>, rival_team: &HashSet<(u8, u8)>) -> HashSet<(u8, u8)> {
        match self.piece_type {
            PieceType::Pawn => {
                let all_pieces = team.union(rival_team).cloned().collect();
                let moves: HashSet<(u8, u8)> = self.get_pawn_moves().difference(&all_pieces).cloned().collect();
                let capture_moves: HashSet<(u8, u8)> = self.get_pawn_capture_moves().intersection(rival_team).cloned().collect();
                moves.union(&capture_moves).cloned().collect()
            }
            PieceType::Rook => self.get_rook_moves().filter_move_directions(team, rival_team),
            PieceType::Knight => self.get_knight_moves().difference(team).cloned().collect(),
            PieceType::Bishop => self.get_bishop_moves().filter_move_directions(team, rival_team),
            PieceType::Queen => {
                let move_directions: HashSet<Vec<(u8, u8)>> = self.get_rook_moves().union(&self.get_bishop_moves()).cloned().collect();
                move_directions.filter_move_directions(team, rival_team)
            }
            PieceType::King => self.get_king_moves().difference(team).cloned().collect(),
        }
    }

    fn get_pawn_moves(&self) -> HashSet<(u8, u8)> {
        let (y, x) = self.position.as_i8();
        let moves: HashSet::<(i8, i8)> = match self.color {
            Color::White if self.position.0 == 1 => HashSet::from_iter([(2, x), (3, x)]),
            Color::White => HashSet::from_iter([(y + 1, x)]),
            Color::Black if self.position.0 == 6 => HashSet::from_iter([(5, x), (4, x)]),
            Color::Black => HashSet::from_iter([(y - 1, x)]),
        };
        moves.as_board_position()
    }
    fn get_pawn_capture_moves(&self) -> HashSet<(u8, u8)> {
        // TODO: Add possible en passant captures
        let (y , x) = self.position.as_i8();
        let capture_moves: HashSet<(i8 ,i8)> = match self.color {
            Color::White => HashSet::from_iter([(y + 1, x - 1), (y + 1, x + 1)]),
            Color::Black => HashSet::from_iter([(y - 1, x - 1), (y - 1, x + 1)]),
        };
        capture_moves.as_board_position()
    }

    fn get_rook_moves(&self) -> HashSet<Vec<(u8, u8)>> {
        let (y, x) = self.position;
        let vertical: Vec<(u8, u8)> = vec![(0, x), (1, x), (2, x), (3, x), (4, x), (5, x), (6, x), (7, x)];
        let horizontal: Vec<(u8, u8)> = vec![(y, 0), (y, 1), (y, 2), (y, 3), (y, 4), (y, 5), (y, 6), (y, 7)];

        let north = vertical.iter().cloned().filter(|&(new_y, _)| new_y > y).collect::<Vec<(u8, u8)>>();
        let south = vertical.iter().cloned().filter(|&(new_y, _)| new_y < y).rev().collect::<Vec<(u8, u8)>>();
        let east = horizontal.iter().cloned().filter(|&(_, new_x)| new_x > x).collect::<Vec<(u8, u8)>>();
        let west = horizontal.iter().cloned().filter(|&(_, new_x)| new_x < x).rev().collect::<Vec<(u8, u8)>>();

        HashSet::from_iter([north, south, east, west])
    }

    fn get_knight_moves(&self) -> HashSet<(u8, u8)> {
        let (y, x) = self.position.as_i8();
        let moves: HashSet<(i8, i8)> = HashSet::from_iter([(y + 2, x - 1), (y - 2, x - 1), (y + 2, x + 1), (y - 2, x + 1), (y - 1, x + 2), (y - 1, x - 2), (y + 1, x + 2), (y + 1, x - 2)]);
        moves.as_board_position()
    }

    fn get_bishop_moves(&self) -> HashSet<Vec<(u8, u8)>> {
        let (y, x) = self.position;
        let se_diag = get_south_east_diagonal(&self.position);
        let ne_diag = get_north_east_diagonal(&self.position);

        let south_east: Vec<(u8, u8)> = se_diag.iter().cloned().filter(|&(new_y, new_x)| new_y < y && new_x > x).collect();
        let north_west: Vec<(u8, u8)> = se_diag.iter().cloned().filter(|&(new_y, new_x)| new_y > y && new_x < x).rev().collect();
        let north_east: Vec<(u8, u8)> = ne_diag.iter().cloned().filter(|&(new_y, new_x)| new_y > y && new_x > x).collect();
        let south_west: Vec<(u8, u8)> = ne_diag.iter().cloned().filter(|&(new_y, new_x)| new_y < y && new_x < x).rev().collect();

        HashSet::from_iter([south_east, north_west, north_east, south_west])
    }

    fn get_king_moves(&self) -> HashSet<(u8, u8)> {
        let (y, x) = self.position.as_i8();
        let moves = HashSet::from_iter([(y + 1, x - 1),(y + 1, x), (y + 1, x + 1), (y, x - 1), (y, x + 1), (y - 1, x - 1), (y - 1, x), (y - 1, x + 1)]);
        moves.as_board_position()
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;
    use crate::enums::{Color, PieceType};
    use crate::pieces::Piece;
    use crate::squares::{Square, Squares};

    #[test]
    fn test_bishop_moves_1() {
        let bishop = Piece::new(Color::White, PieceType::Bishop, (0, 0));
        let positions = HashSet::new();
        let legal_moves = HashSet::from_iter([(1, 1), (2, 2), (3, 3), (4, 4), (5, 5), (6, 6), (7, 7)]);
        assert_eq!(bishop.get_moves(&positions, &positions), legal_moves)
    }

    #[test]
    fn test_bishop_moves_2() {
        let bishop = Piece::new(Color::White, PieceType::Bishop, (2, 3));
        let positions = HashSet::new();
        let legal_moves = HashSet::from_iter([(1, 2), (0, 1), (3, 4), (4, 5), (5, 6), (6, 7), (3, 2), (4, 1), (5, 0), (1, 4), (0, 5)]);
        assert_eq!(bishop.get_moves(&positions, &positions), legal_moves)
    }

    #[test]
    fn test_king_moves_edge() {
        let king = Piece::new(Color::White, PieceType::King, (0, 5));
        let positions = HashSet::new();
        let legal_moves = HashSet::from_iter([(0, 4), (1, 4), (1, 5), (1, 6), (0, 6)]);
        assert_eq!(king.get_moves(&positions, &positions), legal_moves)
    }

    #[test]
    fn test_king_moves_center() {
        let king = Piece::new(Color::White, PieceType::King, (4, 4));
        let positions = HashSet::new();
        let legal_moves = HashSet::from_iter([(5, 3), (5, 4), (5, 5), (4, 3), (4, 5), (3, 3), (3, 4), (3, 5)]);
        assert_eq!(king.get_moves(&positions, &positions), legal_moves)
    }

    #[test]
    fn test_knight_moves_edge() {
        let knight = Piece::new(Color::White, PieceType::Knight, (4, 0));
        let positions = HashSet::new();
        let legal_moves = HashSet::from_iter([(2, 1), (3, 2), (5, 2), (6, 1)]);
        assert_eq!(knight.get_moves(&positions, &positions), legal_moves)
    }

    #[test]
    fn test_knight_moves_center() {
        let knight = Piece::new(Color::White, PieceType::Knight, (4, 4));
        let positions = HashSet::new();
        let legal_moves = HashSet::from_iter([(5, 2), (3, 2), (6, 3), (2, 3), (6, 5), (2, 5), (5, 6), (3, 6)]);
        assert_eq!(knight.get_moves(&positions, &positions), legal_moves)
    }

    #[test]
    fn two_moves_for_e2_opening_move() {
        let pawn = Piece::new(Color::White, PieceType::Pawn,"e2".as_u8());
        let legal_moves = ["e3", "e4"].as_board_position();
        assert_eq!(pawn.get_pawn_moves(), legal_moves)
    }

    #[test]
    fn two_capture_moves_for_e2_opening_move() {
        let pawn = Piece::new(Color::White, PieceType::Pawn, "e2".as_u8());
        let legal_moves = ["d3", "f3"].as_board_position();
        assert_eq!(pawn.get_pawn_capture_moves(), legal_moves)
    }
}
