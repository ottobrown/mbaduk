use std::path::PathBuf;

use mb_goban::Board;
use mb_goban::Stone;

use mb_sgf::SgfTree;

use eframe::egui;
use egui::Ui;

pub enum OptionalGame {
    Some(GameState),
    None(NewGameBuilder),
}

pub struct GameState {
    pub board: Board,
    pub turn: Stone,

    pub tree: SgfTree,
    /// The path to the sgf file being edited; None if the sgf file has not been saved.
    pub sgf_path: Option<PathBuf>,
}

pub struct NewGameBuilder {
    pub size: (usize, usize),
    pub tree: SgfTree,
    pub sgf_path: Option<PathBuf>,
}

impl NewGameBuilder {
    /// Creates the ui and returns the [GameState] if done.
    pub fn render(&mut self, ui: &mut Ui) -> Option<GameState> {
        egui::Frame::group(ui.style())
            .show(ui, |ui| {
                ui.heading("Load SGF file");

                if ui.button("Select file").clicked() {
                    let dialog = rfd::FileDialog::new()
                        .add_filter("sgf", &["sgf"])
                        .pick_file();

                    if let Some(p) = dialog {
                        self.sgf_path = Some(p.clone());

                        ui.label(format!("{}", &p.display()));

                        match std::fs::read_to_string(&p) {
                            Ok(s) => {
                                match mb_sgf::parse(&s) {
                                    Ok(t) => {
                                        self.tree = t;

                                        self.sgf_root_props();

                                        return Some(self.build());
                                    }
                                    Err(e) => {
                                        ui.label(format!("failed to parse sgf tree: {:?}", e));
                                    }
                                };
                            }
                            Err(e) => {
                                ui.label(format!("{:?}", e));
                            }
                        }
                    }
                }

                ui.heading("Board size:");

                ui.label("Width:");
                ui.add(egui::Slider::new(&mut self.size.0, 5..=50));

                ui.label("Height:");
                ui.add(egui::Slider::new(&mut self.size.1, 5..=50));

                if ui.button("Finish").clicked() {
                    return Some(self.build());
                }

                None
            })
            .inner
    }

    pub fn build(&self) -> GameState {
        return GameState {
            board: Board::empty(self.size.0, self.size.1),
            turn: Stone::Black,
            tree: SgfTree::default(),
            sgf_path: None,
        };
    }

    /// Edit properties of `Self` based on the props in the sgf root
    pub fn sgf_root_props(&mut self) {
        let root_node = &self.tree.nodes[0];

        #[allow(clippy::single_match)]
        for p in &root_node.props {
            match p.id.as_str() {
                "SZ" => self.size = mb_sgf::util::parse_board_size(&p.values[0]).unwrap(),

                _ => {}
            }
        }
    }
}

impl Default for NewGameBuilder {
    fn default() -> Self {
        Self {
            size: (19, 19),
            tree: SgfTree::default(),
            sgf_path: None,
        }
    }
}
