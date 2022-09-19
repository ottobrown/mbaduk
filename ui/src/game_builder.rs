use mb_goban::Board;

use eframe::egui;
use egui::Ui;

pub struct NewGameBuilder {
    pub size: (usize, usize),
}

impl NewGameBuilder {
    /// Creates the ui and returns the [Board] if done.
    pub fn build(&mut self, ui: &mut Ui) -> Option<Board> {
        ui.heading("Board size:");

        ui.label("Width:");
        ui.add(egui::Slider::new(&mut self.size.0, 5..=50));

        ui.label("Height:");
        ui.add(egui::Slider::new(&mut self.size.1, 5..=50));

        if ui.button("Finish").clicked() {
            return Some(Board::empty(self.size.0, self.size.1));
        }

        return None;
    }
}
        
impl Default for NewGameBuilder {
    fn default() -> Self {
        Self {
            size: (19, 19),
        }
    }
}
