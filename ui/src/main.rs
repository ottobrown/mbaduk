#![allow(clippy::needless_return)]
#![allow(clippy::redundant_field_names)]

use eframe::egui;
use eframe::App;
use eframe::NativeOptions;

use mb_goban::Board;
use mb_goban::Stone;

mod board;
mod game_builder;

use board::{render_board, BoardStyle};
use game_builder::NewGameBuilder;

fn main() {
    let ops = NativeOptions::default();

    eframe::run_native("MBaduk", ops, Box::new(|cc| Box::new(State::new(cc))));
}

struct State {
    board: Option<Board>,
    builder: NewGameBuilder,

    style: BoardStyle,
    turn: Stone,
}

impl State {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self {
            board: None,
            builder: NewGameBuilder::default(),
            style: BoardStyle::default(),
            turn: Stone::Black,
        }
    }
}

impl App for State {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            egui::ScrollArea::both().show(ui, |ui| {
                main_ui(ui, self);
            })
        });
    }
}

fn main_ui(ui: &mut egui::Ui, state: &mut State) {
    if let Some(b) = state.board.as_mut() {
        let size = egui::vec2(800.0, 800.0);

        render_board(ui, b, size, &state.style, &mut state.turn);
    } else {
        state.board = state.builder.build(ui);
    }
}
