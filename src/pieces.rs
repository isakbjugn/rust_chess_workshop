use std::collections::HashSet;
use std::fmt::{Debug, Formatter};
#[cfg(feature = "gui")]
use std::fs::read;
#[cfg(feature = "gui")]
use egui_extras::RetainedImage;
use crate::enums::{Color, PieceType};
use crate::utils::{get_south_east_diagonal, get_north_east_diagonal, get_valid_moves, to_move_lines};

pub struct Piece {
    #[cfg(feature = "gui")]
    pub image: RetainedImage,
    pub color: Color,
    pub piece_type: PieceType,
    pub position: (u8, u8), // (row, column)
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
            #[cfg(feature = "gui")]
            image: RetainedImage::from_image_bytes(image_path, &read(image_path).unwrap()).unwrap(),
        }
    }
    pub fn move_piece(&mut self, new_position: (u8, u8)) {
        self.position = new_position;
    }
    pub fn print(&self) -> char {
        if let PieceType::Knight = self.piece_type {
            return match self.color {
                Color::White => 'N',
                Color::Black => 'n',
            }
        }
        match self.color {
            Color::White => self.piece_type.to_string().chars().next().unwrap().to_ascii_uppercase(),
            Color::Black => self.piece_type.to_string().chars().next().unwrap().to_ascii_lowercase(),
        }
    }

    pub fn get_moves(&self) -> HashSet<Vec<(u8, u8)>> {
        let mut move_lines = match self.piece_type {
            PieceType::Rook => self.get_rook_moves(),
            PieceType::Bishop => self.get_bishop_moves(),
            PieceType::Queen => self.get_queen_moves(),
            PieceType::Pawn => HashSet::from_iter([self.get_pawn_moves()]),
            PieceType::Knight => to_move_lines(&self.get_knight_moves()),
            PieceType::King => to_move_lines(&self.get_king_moves()),
        };
        move_lines.retain(|line| !line.is_empty());
        move_lines
    }

    fn get_pawn_moves(&self) -> Vec<(u8, u8)> {
        let (y, x) = self.position;
        match self.color {
            Color::White => {
                match self.is_initial_pawn_position() {
                    true => vec![(y + 1, x)],
                    false => vec![(2, x), (3, x)]
                }
            },
            Color::Black => {
                match self.is_initial_pawn_position() {
                    true => vec![(y - 1, x)],
                    false => vec![(5, x), (4, x)]
                }
            }
        }
    }

    fn is_initial_pawn_position(&self) -> bool {
        if self.piece_type != PieceType::Pawn {
            panic!("is_initial_pawn_position was called on a non-pawn PieceType")
        }
        return match self.color {
            Color::White => self.position.1 == 1,
            Color::Black => self.position.1 == 6,
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
        let mut moves= HashSet::from_iter([(y + 2, x - 1), (y - 2, x - 1), (y + 2, x + 1), (y - 2, x + 1), (y - 1, x + 2), (y - 1, x - 2), (y + 1, x + 2), (y + 1, x - 2)]);
        get_valid_moves(&mut moves)
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
        let mut moves = HashSet::from_iter([(y + 1, x - 1),(y + 1, x), (y + 1, x + 1), (y, x - 1), (y, x + 1), (y - 1, x - 1), (y - 1, x), (y - 1, x + 1)]);
        get_valid_moves(&mut moves)
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;
    use crate::enums::{Color, PieceType};
    use crate::pieces::Piece;
    use crate::utils::to_move_lines;

    #[test]
    fn test_bishop_moves_1() {
        let bishop = Piece::new(Color::White, PieceType::Bishop, (0, 0));
        let legal_moves = HashSet::from_iter([vec![(1, 1), (2, 2), (3, 3), (4, 4), (5, 5), (6, 6), (7, 7)]]);
        assert_eq!(bishop.get_moves(), legal_moves)
    }

    #[test]
    fn test_bishop_moves_2() {
        let bishop = Piece::new(Color::White, PieceType::Bishop, (2, 3));
        let legal_moves = HashSet::from_iter([vec![(1, 2), (0, 1)], vec![(3, 4), (4, 5), (5, 6), (6, 7)], vec![(3, 2), (4, 1), (5, 0)], vec![(1, 4), (0, 5)]]);
        assert_eq!(bishop.get_moves(), legal_moves)
    }

    #[test]
    fn test_king_moves_edge() {
        let bishop = Piece::new(Color::White, PieceType::King, (0, 5));
        let legal_moves = HashSet::from_iter([(0, 4), (1, 4), (1, 5), (1, 6), (0, 6)].into_iter().map(|position| vec![position]));
        assert_eq!(bishop.get_moves(), legal_moves)
    }

    #[test]
    fn test_king_moves_center() {
        let bishop = Piece::new(Color::White, PieceType::King, (4, 4));
        let legal_moves = to_move_lines(&HashSet::from_iter([(5, 3), (5, 4), (5, 5), (4, 3), (4, 5), (3, 3), (3, 4), (3, 5)]));
        assert_eq!(bishop.get_moves(), legal_moves)
    }

    #[test]
    fn test_knight_moves_edge() {
        let bishop = Piece::new(Color::White, PieceType::Knight, (4, 0));
        let legal_moves = to_move_lines(&HashSet::from_iter([(2, 1), (3, 2), (5, 2), (6, 1)]));
        assert_eq!(bishop.get_moves(), legal_moves)
    }

    #[test]
    fn test_knight_moves_center() {
        let bishop = Piece::new(Color::White, PieceType::Knight, (4, 4));
        let legal_moves = to_move_lines(&HashSet::from_iter([(5, 2), (3, 2), (6, 3), (2, 3), (6, 5), (2, 5), (5, 6), (3, 6)]));
        assert_eq!(bishop.get_moves(), legal_moves)
    }
}

