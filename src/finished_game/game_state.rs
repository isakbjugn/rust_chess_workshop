#![allow(unused)]
use crate::finished_game::color::Color;

#[derive(PartialEq)]
pub enum GameState {
    Playing,
    Checkmate(Color),
    Draw(Color),
    Quit,
}