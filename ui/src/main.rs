#![allow(clippy::needless_return)]
#![allow(clippy::redundant_field_names)]

use eframe::egui;
use eframe::App;
use eframe::NativeOptions;

use mb_goban::Board;
use mb_goban::Stone;

mod board;

use board::{render_board, BoardStyle};

fn main() {
    let ops = NativeOptions::default();

    eframe::run_native("MBaduk", ops, Box::new(|cc| Box::new(State::new(cc))));
}

struct State {
    board: Board,
    style: BoardStyle,
    turn: Stone,
}

impl State {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        let board = Board::empty(19, 19);
        let style = BoardStyle::default();

        Self {
            board: board,
            style: style,
            turn: Stone::Black,
        }
    }
}

impl App for State {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            egui::ScrollArea::both().show(ui, |ui| {
                render_board(
                    ui,
                    &mut self.board,
                    egui::vec2(800.0, 800.0),
                    &self.style,
                    &mut self.turn,
                );
            })
        });
    }
}
