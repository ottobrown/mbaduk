#![allow(clippy::needless_return)]
#![allow(clippy::redundant_field_names)]

use eframe::egui;
use eframe::App;
use eframe::NativeOptions;

mod board;
mod game;

use board::{render_board, BoardStyle};
use game::{NewGameBuilder, OptionalGame};

fn main() {
    let ops = NativeOptions::default();

    eframe::run_native("MBaduk", ops, Box::new(|cc| Box::new(State::new(cc))));
}

struct State {
    game: OptionalGame,

    style: BoardStyle,
}

impl State {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self {
            game: OptionalGame::None(NewGameBuilder::default()),
            style: BoardStyle::default(),
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
    match &mut state.game {
        OptionalGame::Some(ref mut g) => {
            let size = egui::vec2(800.0, 800.0);

            render_board(ui, &mut g.board, size, &state.style, &mut g.turn);
        }

        OptionalGame::None(ref mut b) => {
            if let Some(g) = b.render(ui) {
                state.game = OptionalGame::Some(g);
            }
        }
    }
}
