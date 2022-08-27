use eframe::App;
use eframe::NativeOptions;
use eframe::egui;

fn main() {
    let ops = NativeOptions::default();

    eframe::run_native(
        "MBaduk",
        ops, 
        Box::new( |cc| Box::new(State::new(cc)) )
    );
}

struct State {}

impl State {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self {}
    }
}

impl App for State {
   fn update(&mut self, _ctx: &egui::Context, _frame: &mut eframe::Frame) {}
}
