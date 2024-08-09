use std::any::type_name;
use std::collections::HashSet;
use downcast_rs::{Downcast, impl_downcast};

use dyn_clonable::clonable;

use crate::finished_game::color::Color;

pub mod pawn;
pub mod rook;
pub mod knight;
pub mod bishop;
pub mod queen;
pub mod king;

#[clonable]
pub trait Piece: Clone + Downcast {
    fn new(color: Color, position: (u8, u8)) -> Self where Self: Sized;
    fn print(&self) -> char;
    fn get_type(&self) -> &'static str {
        type_name::<Self>().rsplit("::").next().unwrap()
    }
    fn get_color(&self) -> Color;
    fn get_position(&self) -> &(u8, u8);
    fn move_piece(&mut self, target: (u8, u8));
    fn get_moves(&self, team: &HashSet<(u8, u8)>, rival_team: &HashSet<(u8, u8)>) -> HashSet<(u8, u8)>;
}

impl_downcast!(Piece);