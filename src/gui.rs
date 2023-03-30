use std::io;
use std::io::Write;
use egui::{Color32, Pos2, Rect};
use eframe::egui;
use egui_extras::RetainedImage;
use crate::board_trait::Board;
use crate::enums::Color;
use crate::utils::select_square;

const PIECE_SIZE: f32 = 48.0;

// impl eframe::App for Board {
//     fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
//
//         let my_frame = egui::containers::Frame {
//             fill: Color32::WHITE,
//             ..Default::default()
//         };
//
//         egui::CentralPanel::default().frame(my_frame).show(ctx, |ui| {
//             self.chess_board_image.show(ui);
//
//             for (position, piece) in self.pieces.iter() {
//                 let img = egui::Image::new(
//                     piece.image.texture_id(ctx),
//                     piece.image.size_vec2(),
//                 );
//
//                 img.paint_at(
//                     ui,
//                     Rect {
//                         min: Pos2 { x: PIECE_SIZE * position.0 as f32, y: PIECE_SIZE * position.1 as f32 },
//                         max: Pos2 { x: PIECE_SIZE * (position.0 + 1) as f32, y: PIECE_SIZE * (position.1 + 1) as f32 },
//                     },
//                 )
//             }
//         });
//     }
// }
