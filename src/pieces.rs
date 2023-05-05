use std::collections::HashSet;
use crate::chess_board::ChessBoard;
use crate::board::Board;
use std::fmt::{Debug, Formatter};
#[cfg(feature = "gui")]
use std::fs::read;
#[cfg(feature = "gui")]
use egui_extras::RetainedImage;
use crate::enums::{Color, PieceType};
use crate::squares::Squares;
use crate::utils::{get_south_east_diagonal, get_north_east_diagonal};

pub struct Piece {
    #[cfg(feature = "gui")]
    pub image: Option<RetainedImage>,
    pub color: Color,
    pub piece_type: PieceType,
    pub position: (u8, u8), // (row, column)
    pub moved: bool
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
            (PieceType::Knight, Color::White) => "assets/knight-white-48.png",
            (PieceType::Knight, Color::Black) => "assets/knight-black-48.png",
            (PieceType::Queen, Color::White) => "assets/queen-white-48.png",
            (PieceType::Queen, Color::Black) => "assets/queen-black-48.png",
            (PieceType::Rook, Color::White) => "assets/rook-white-48.png",
            (PieceType::Rook, Color::Black) => "assets/rook-black-48.png",
            (PieceType::Bishop, Color::White) => "assets/bishop-white-48.png",
            (PieceType::Bishop, Color::Black) => "assets/bishop-black-48.png",
            (PieceType::Pawn, Color::White) => "assets/pawn-white-48.png",
            (PieceType::Pawn, Color::Black) => "assets/pawn-black-48.png",
            (PieceType::King, Color::White) => "assets/king-white-48.png",
            (PieceType::King, Color::Black) => "assets/king-black-48.png",
        };
        Piece {
            color,
            piece_type,
            position,
            moved: false,
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
        self.moved = true;
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

    pub fn get_moves(&self, board: &Board) -> HashSet<(u8, u8)> {
        match self.piece_type {
            PieceType::Pawn => {
                let mut moves = self.get_pawn_moves();
                if let Some(captures) = self.get_pawn_capture_moves(board) {
                    moves.extend(captures)
                }
                moves.into_iter()
                    .filter(|square| board.get_square_color(square) != Some(self.color))
                    .collect()
            },
            PieceType::Rook => {
                let move_directions = self.get_rook_moves();
                board.filter_move_directions(&move_directions, self.color)
            },
            PieceType::Knight => self.get_knight_moves(),
            PieceType::Bishop => {
                let move_directions = self.get_bishop_moves();
                board.filter_move_directions(&move_directions, self.color)
            },
            PieceType::Queen => {
                let mut move_directions = self.get_rook_moves();
                move_directions.extend(self.get_bishop_moves());
                board.filter_move_directions(&move_directions, self.color)
            },
            PieceType::King => self.get_king_moves(),
        }
    }
    fn get_pawn_moves(&self) -> HashSet<(u8, u8)> {
        let (y, x) = self.position;
        match self.color {
            Color::White => {
                match self.moved {
                    true => HashSet::from_iter([(y + 1, x)]),
                    false => HashSet::from_iter([(2, x), (3, x)])
                }
            },
            Color::Black => {
                match self.moved {
                    true => HashSet::from_iter([(y - 1, x)]),
                    false => HashSet::from_iter([(5, x), (4, x)])
                }
            }
        }
    }
    fn get_pawn_capture_moves(&self, board: &Board) -> Option<HashSet<(u8, u8)>> {
        // TODO: Add possible en passant captures
        let (y, x) = self.position;
        let mut captures = HashSet::new();
        match self.color {
            Color::White if y < 7 => {
                let capture_y = y + 1;
                if x > 1 && board.get_square_color(&(capture_y, x - 1)) == Some(Color::Black) {
                    captures.insert((capture_y, x - 1));
                }
                if let Some(Color::Black) = board.get_square_color(&(capture_y, x + 1)) {
                    captures.insert((capture_y, x + 1));
                }
                Some(captures)
            },
            Color::Black if y > 0 => {
                let capture_y = y - 1;
                if x > 1 && board.get_square_color(&(capture_y, x - 1)) == Some(Color::Black) {
                    captures.insert((capture_y, x - 1));
                }
                if let Some(Color::White) = board.get_square_color(&(capture_y, x + 1)) {
                    captures.insert((capture_y, x + 1));
                }
                Some(captures)
            },
            _ => None,
        }
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
        let y = self.position.0 as i8;
        let x = self.position.1 as i8;
        let moves: HashSet<(i8, i8)> = HashSet::from_iter([(y + 2, x - 1), (y - 2, x - 1), (y + 2, x + 1), (y - 2, x + 1), (y - 1, x + 2), (y - 1, x - 2), (y + 1, x + 2), (y + 1, x - 2)]);
        moves.chess_board_filter()
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

    fn get_queen_moves(&self) -> HashSet<Vec<(u8, u8)>> {
        let mut moves = self.get_rook_moves();
        moves.extend(self.get_bishop_moves());
        moves
    }
    fn get_king_moves(&self) -> HashSet<(u8, u8)> {
        let y = self.position.0 as i8;
        let x = self.position.1 as i8;
        let moves = HashSet::from_iter([(y + 1, x - 1),(y + 1, x), (y + 1, x + 1), (y, x - 1), (y, x + 1), (y - 1, x - 1), (y - 1, x), (y - 1, x + 1)]);
        moves.chess_board_filter()
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;
    use crate::chess_board::ChessBoard;
    use crate::board::Board;
    use crate::enums::{Color, PieceType};
    use crate::pieces::Piece;

    #[test]
    fn test_bishop_moves_1() {
        let bishop = Piece::new(Color::White, PieceType::Bishop, (0, 0));
        let board = Board::empty();
        let legal_moves = HashSet::from_iter([(1, 1), (2, 2), (3, 3), (4, 4), (5, 5), (6, 6), (7, 7)]);
        assert_eq!(bishop.get_moves(&board), legal_moves)
    }

    #[test]
    fn test_bishop_moves_2() {
        let bishop = Piece::new(Color::White, PieceType::Bishop, (2, 3));
        let board = Board::empty();
        let legal_moves = HashSet::from_iter([(1, 2), (0, 1), (3, 4), (4, 5), (5, 6), (6, 7), (3, 2), (4, 1), (5, 0), (1, 4), (0, 5)]);
        assert_eq!(bishop.get_moves(&board), legal_moves)
    }

    #[test]
    fn test_king_moves_edge() {
        let king = Piece::new(Color::White, PieceType::King, (0, 5));
        let legal_moves = HashSet::from_iter([(0, 4), (1, 4), (1, 5), (1, 6), (0, 6)]);
        assert_eq!(king.get_king_moves(), legal_moves)
    }

    #[test]
    fn test_king_moves_center() {
        let king = Piece::new(Color::White, PieceType::King, (4, 4));
        let legal_moves = HashSet::from_iter([(5, 3), (5, 4), (5, 5), (4, 3), (4, 5), (3, 3), (3, 4), (3, 5)]);
        assert_eq!(king.get_king_moves(), legal_moves)
    }

    #[test]
    fn test_knight_moves_edge() {
        let knight = Piece::new(Color::White, PieceType::Knight, (4, 0));
        let legal_moves = HashSet::from_iter([(2, 1), (3, 2), (5, 2), (6, 1)]);
        assert_eq!(knight.get_knight_moves(), legal_moves)
    }

    #[test]
    fn test_knight_moves_center() {
        let knight = Piece::new(Color::White, PieceType::Knight, (4, 4));
        let legal_moves = HashSet::from_iter([(5, 2), (3, 2), (6, 3), (2, 3), (6, 5), (2, 5), (5, 6), (3, 6)]);
        assert_eq!(knight.get_knight_moves(), legal_moves)
    }
}
