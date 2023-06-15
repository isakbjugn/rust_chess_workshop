use std::collections::HashSet;
use crate::finished_game::color::Color;

pub trait BoardContract {
    fn new() -> Self where Self: Sized;
    fn get_piece_name(&self, position: &(u8, u8)) -> String;
    fn get_square_color(&self, position: &(u8, u8)) -> Option<Color>;
    fn get_legal_squares(&self, position: &(u8, u8)) -> HashSet<(u8, u8)>;
    fn create_board(&self) -> Vec<Vec<char>>;
    fn move_piece(&mut self, position: &(u8, u8), target_square: (u8, u8));
    fn capture(&mut self, position: &(u8, u8), target_square: (u8, u8));
    fn get_positions(&self, color: Color) -> HashSet<(u8, u8)>;
    fn is_check(&self, color: Color) -> bool {
        false
        // todo!("Skal implementeres i oppgave 8)
    }
    fn is_checkmate(&self, color: Color) -> bool {
        false
        // todo!("Skal implementeres i oppgave 9)
    }
    fn print(&self, legal_squares: Option<&HashSet<(u8, u8)>>);
}