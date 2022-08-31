use eframe::egui;
use egui::{Color32, Response, Ui};

use mb_goban::Board;
use mb_goban::Stone;

#[derive(Clone, Copy)]
pub struct BoardStyle {
    /// As a fraction of the whole board size
    pub padding: f32,
    /// In egui screen units
    pub line_thickness: f32,
    pub background_color: Color32,

    /// As a fraction of the minimus of the width and height of square.
    pub stone_radius: f32,
}
impl Default for BoardStyle {
    fn default() -> Self {
        BoardStyle {
            padding: 0.05,
            line_thickness: 3.0,
            background_color: Color32::from_rgb(0xDE, 0xB8, 0x87),
            stone_radius: 0.46,
        }
    }
}

#[derive(Clone)]
pub struct BoardUi {
    style: BoardStyle,

    size: egui::Vec2,
    board: Board,
}
impl BoardUi {
    pub fn new(style: BoardStyle, size: egui::Vec2, board: &Board) -> Self {
        Self {
            style: style,
            size: size,
            board: board.clone(),
        }
    }
}

impl egui::Widget for BoardUi {
    fn ui(self, ui: &mut Ui) -> Response {
        let (response, painter) = ui.allocate_painter(self.size, egui::Sense::drag());

        // draw background color
        painter.rect_filled(egui::Rect::EVERYTHING, 0.0, self.style.background_color);

        let padding = self.size * self.style.padding;

        let (w, h) = self.board.size();

        let inner_w = self.size.x - 2.0 * padding.x;
        let distance_x = inner_w / ((w - 1) as f32);

        // draw vertical lines
        for x in 0..w {
            let x_pos = response.rect.min.x + padding.x + (x as f32) * distance_x;
            let top = response.rect.min.y + padding.y;
            let bottom = response.rect.min.y + self.size.y - padding.y;

            painter.line_segment(
                [egui::pos2(x_pos, top), egui::pos2(x_pos, bottom)],
                egui::Stroke::new(self.style.line_thickness, Color32::BLACK),
            );
        }

        let inner_h = self.size.y - 2.0 * padding.y;
        let distance_y = inner_h / ((h - 1) as f32);

        // draw horizontal lines
        for y in 0..h {
            let y_pos = response.rect.min.y + padding.y + (y as f32) * distance_y;
            let left = response.rect.min.x + padding.x;
            let right = response.rect.min.x + self.size.x - padding.x;

            painter.line_segment(
                [egui::pos2(left, y_pos), egui::pos2(right, y_pos)],
                egui::Stroke::new(self.style.line_thickness, Color32::BLACK),
            );
        }

        for x in 0..w {
            for y in 0..h {
                let x_pos = response.rect.min.x + padding.x + (x as f32) * distance_x;
                let y_pos = response.rect.min.y + padding.y + (y as f32) * distance_y;

                let r = distance_x.min(distance_y) * self.style.stone_radius;

                match self.board.get(x, y) {
                    Ok(Stone::Black) => {
                        painter.circle_filled(egui::pos2(x_pos, y_pos), r, Color32::BLACK);
                    }
                    Ok(Stone::White) => {
                        painter.circle_filled(egui::pos2(x_pos, y_pos), r, Color32::WHITE);
                    }

                    _ => {}
                }
            }
        }

        return response;
    }
}
