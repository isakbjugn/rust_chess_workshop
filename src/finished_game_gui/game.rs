use std::collections::{HashMap, HashSet};

use raylib::prelude::*;

use crate::finished_game::board_contract::BoardContract;
use crate::finished_game::color::Color;
use crate::finished_game::game_state::GameState;

use super::board_extended::BoardExtended;

/*
    NOTE(2021-09-14): Uferdig. Håndterer ikke sjakkmatt, sjakk eller patt. Dette er så langt
    jeg kom under kurset med dere og Online.
*/

fn piece_identifier(color: Color, piece_type: &str) -> String {
    format!("{color}{piece_type}")
}

struct Game {
    board: BoardExtended,
    turn: Color,
    _state: GameState,
    legal_squares: HashSet<(u8, u8)>,
    target_square: Option<(u8, u8)>,
}

impl Game {
    fn new() -> Self {
        Game {
            board: BoardExtended::new(),
            turn: Color::White,
            _state: GameState::Playing,
            legal_squares: HashSet::new(),
            target_square: None,
        }
    }

    fn play(&mut self) {
        let size = 500;
        let (mut rl, thread) = raylib::init()
            .size(size, size)
            .title("Hello, World")
            .build();

        // Bilder som blir brukt
        let images_path = "src/finished_game_gui/images/";
        let mut textures = HashMap::new();
        let image_names: HashMap<String, &str> = vec![
            (piece_identifier(Color::White, "King"), "white_king"),
            (piece_identifier(Color::White, "Queen"), "white_queen"),
            (piece_identifier(Color::White, "Rook"), "white_rook"),
            (piece_identifier(Color::White, "Bishop"), "white_bishop"),
            (piece_identifier(Color::White, "Knight"), "white_knight"),
            (piece_identifier(Color::White, "Pawn"), "white_pawn"),
            (piece_identifier(Color::Black, "King"), "black_king"),
            (piece_identifier(Color::Black, "Queen"), "black_queen"),
            (piece_identifier(Color::Black, "Rook"), "black_rook"),
            (piece_identifier(Color::Black, "Bishop"), "black_bishop"),
            (piece_identifier(Color::Black, "Knight"), "black_knight"),
            (piece_identifier(Color::Black, "Pawn"), "black_pawn"),
        ]
        .into_iter()
        .collect();
        let image_names = &image_names;

        for (_, name) in image_names {
            let path = format!("{}{}.png", images_path, name);
            let texture = rl.load_texture(&thread, path.as_str()).unwrap();
            textures.insert(name, texture);
        }
        let textures = textures;

        // Game-loop
        while !rl.window_should_close() {
            let mut d = rl.begin_drawing(&thread);
            let selected_square = self.get_selected_square_coordinates(&mut d);
            if d.is_mouse_button_released(MouseButton::MOUSE_LEFT_BUTTON) {
                let piece_color = self.board.get_square_color(&selected_square);
                if let Some(color) = piece_color {
                    if color == self.turn {
                        if self.legal_squares.is_empty() {
                            // Når en brikke skal velges
                            self.legal_squares = self.board.get_legal_squares(&selected_square);
                            self.target_square = Some(selected_square);
                        } else if !self.legal_squares.contains(&selected_square) {
                            // Når man avbryter valg av brikke
                            self.legal_squares.clear();
                            self.target_square = None;
                        }
                    }
                }

                if self.legal_squares.contains(&selected_square) {
                    // Når en brikke skal flyttes til en ny posisjon
                    let position_to_move_to = selected_square;
                    if self.legal_squares.contains(&position_to_move_to) {
                        // lovlig trekk
                        self.board
                            .move_piece(&self.target_square.unwrap(), position_to_move_to);
                        self.next_turn();
                    }

                    self.legal_squares.clear();
                    self.target_square = None;
                }
            }

            // TEGNING AV BRIKKER OG BRET

            d.clear_background(raylib::color::Color::BLACK);

            // Tegner brettet
            for i in 0..8 {
                for j in 0..8 {
                    let color = if (i + j) % 2 == 0 {
                        raylib::color::Color::WHITE
                    } else {
                        raylib::color::Color::BROWN
                    };
                    d.draw_rectangle(
                        (i * (size / 8)) as i32,
                        (j * (size / 8)) as i32,
                        size / 8,
                        size / 8,
                        color,
                    );
                }
            }

            // Tegner grønne ruter på lovlige trekk
            for square in &self.legal_squares {
                d.draw_rectangle(
                    (square.0 as i32 * (size / 8)) as i32,
                    ((7 - square.1 as i32) * (size / 8)) as i32,
                    size / 8,
                    size / 8,
                    raylib::color::Color::GREEN,
                );
            }

            // Tegner bokstaver og tall
            for i in 0..8 {
                d.draw_text(
                    format!("{}", i + 1).as_str(),
                    2,
                    ((7 - i) * (size / 8)) as i32 + 2,
                    20,
                    raylib::color::Color::BLACK,
                );
                d.draw_text(
                    format!("{}", (i + 97) as u8 as char).as_str(),
                    (i * (size / 8)) as i32 + 2,
                    size - 20 - 2,
                    20,
                    raylib::color::Color::BLACK,
                );
            }

            // Tegner brikker
            for (x, y, piece) in self.board.get_pieces() {
                let color = match piece.get_color() {
                    Color::White => raylib::color::Color::WHITE,
                    Color::Black => raylib::color::Color::WHITE,
                };

                let image_identifier = piece_identifier(piece.get_color(), piece.get_type());
                let image_name = image_names.get(&image_identifier).unwrap();
                let texture = textures.get(image_name).unwrap();

                d.draw_texture(
                    texture,
                    (x as i32 * (size / 8)) as i32,
                    ((7 - y as i32) * (size / 8)) as i32,
                    color,
                );

                // SLUTT PÅ TEGNING AV BRIKKER OG BRETT
            }
        }
    }

    fn get_selected_square_coordinates(&self, d: &mut RaylibDrawHandle) -> (u8, u8) {
        let cursor_pos = d.get_mouse_position();
        let mut x = (cursor_pos.x as i32 / (500 / 8)) as i32;
        if x > 7 {
            x = 7;
        }
        if x < 0 {
            x = 0;
        }
        let x = x as u8;
        let mut y = (cursor_pos.y as i32 / (500 / 8)) as i32;
        if y > 7 {
            y = 7;
        }
        if y < 0 {
            y = 0;
        }
        let y = 7 - y as u8;

        (x, y)
    }

    fn next_turn(&mut self) {
        self.turn = self.turn.opposite();
    }
}

pub fn main() {
    let mut game = Game::new();
    game.play();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn move_a_piece() {
        let mut game = Game::new();
        game.play();
    }
}
