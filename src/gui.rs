use std::cmp::max;
use eframe::egui;
use egui::*;
use crate::sim::{Process, Instruction, Field};

const TEAM_COLORS: [Color32; 4] = [Color32::GREEN, Color32::LIGHT_BLUE, Color32::YELLOW, Color32::RED];

pub(crate) struct EmarsApp {
    pub(crate) core: Vec<Instruction>,
    pub(crate) coresize: usize,
    pub(crate) default_instruction: Instruction,
    pub(crate) core_view_size: usize,
    pub(crate) teams: u8,
    pub(crate) processes: Vec<Process>,
}

fn display_instruction(instruction: Instruction, coresize: usize) -> String {
    format!(
        // Example output:
        // MOV.AB  $-100,  $1
        // |----->||----->|
        "{op:<8}{a:<8}{b}",
        op = format!("{}.{}", instruction.opcode, instruction.modifier),
        a = display_field(instruction.field_a, coresize as isize),
        b = display_field(instruction.field_b, coresize as isize),
    )
}

fn display_field(field: Field, coresize_isize: isize) -> String {
    let mut value = field.value as isize;
    if value > coresize_isize / 2 {
        value -= coresize_isize
    }
    format!("{}{}", field.address_mode, value)
}

pub fn core_view(app: &mut EmarsApp, context: &egui::Context) {
    let default_size: Vec2;
    let label_font_size: f32;
    let square_size_inside: f32;
    let square_size_outside: f32;
    let x_margin: f32;
    let y_margin: f32;
    let stroke_size: f32;
    match app.core_view_size {
        0 => (default_size, label_font_size, square_size_inside, square_size_outside, x_margin, y_margin, stroke_size) = (vec2(519., 416.), 6., 4., 5., 20., 16., 1.),
        1 => (default_size, label_font_size, square_size_inside, square_size_outside, x_margin, y_margin, stroke_size) = (vec2(822., 655.), 8., 6., 8., 24., 17., 1.5),
        2 => (default_size, label_font_size, square_size_inside, square_size_outside, x_margin, y_margin, stroke_size) = (vec2(1026., 817.), 10., 8., 10., 28., 18., 2.),
        _ => panic!("Invalid core view size of {}", app.core_view_size)
    };
    egui::Window::new("Core View")
    .default_size(default_size)
    .show(context, |ui|{
        
        let painter_width = ui.available_width();
        let painter_height = max((((app.coresize * 5) as f32 / painter_width).ceil() + 5.) as usize, ui.available_height() as usize) as f32;
        
        let (response, painter) = ui.allocate_painter(vec2(painter_width, painter_height), Sense::hover());
        let window_width = response.rect.width();
        let window_height = response.rect.height();
        // println!("{window_width} x {window_height}s");

        painter.text(
            pos2(response.rect.min.x, response.rect.min.y + 16.),
            Align2::LEFT_TOP,
            "0000",
            FontId::monospace(label_font_size),
            Color32::LIGHT_GRAY
        );

        let hovered: bool;
        let mut hovered_text = String::new();
        let hover_pos: Pos2 = match response.hover_pos() {
            Some(pos) => pos,
            None => pos2(0., 0.)
        };
        if hover_pos == pos2(0., 0.) {
            hovered = false;
        } else {
            hovered = true;
        }

        // println!("hover_pos: {} x {}", hover_pos.x, hover_pos.y);

        let mut x = response.rect.min.x + x_margin; // calculates *objective* x position
        let mut y = response.rect.min.y + y_margin; // calculates *objective* y position

        for i in 0..app.coresize {
            let mut stroke = Stroke::NONE;

            // checks if square is being pointed to by any processes
            for process in &app.processes {
                if process.pointer == i as usize {
                    stroke = Stroke::new(stroke_size, TEAM_COLORS[process.team as usize]);
                }
            }

            // checks if this square is being hovered
            if hovered && x <= hover_pos.x && (x + square_size_outside) >= hover_pos.x && y <= hover_pos.y && (y + square_size_outside) >= hover_pos.y {
                hovered_text = display_instruction(app.core[i as usize], app.coresize);
                // println!("hovered_text: {hovered_text}");
                stroke = Stroke::new(stroke_size, Color32::YELLOW);
            }

            let instruction_color: Color32;
            if app.core[i as usize] == app.default_instruction {
                instruction_color = Color32::DARK_GRAY;
            } else {
                instruction_color = Color32::from_rgb(200, 0, 0);
            }

            // draws the rectangle at pos (x, y) and size (4, 4) in red
            painter.rect(
                Rect::from_min_size(pos2(x, y), vec2(square_size_inside, square_size_inside)), 
                CornerRadius::same(1), 
                instruction_color,
                stroke,
                StrokeKind::Inside
            );

            // moves next square's x 5 pixels to the left
            x += square_size_outside;
            if (x - response.rect.min.x) > window_width - square_size_inside { // if next square will overflow,
                x = response.rect.min.x + x_margin; // set it's x back to the beginning
                y += square_size_outside; // and move it down a row.

                let label_color: Color32;
                if ((y - response.rect.min.y)/square_size_outside).floor() % 4. == 0. {
                    label_color = Color32::LIGHT_GRAY;
                } else if ((y - response.rect.min.y)/square_size_outside).floor() % 2. == 0. {
                    label_color = Color32::GRAY;
                } else {
                    label_color = Color32::DARK_GRAY;
                }

                if i != 7999 { painter.text(
                    pos2(response.rect.min.x, y - 2.),
                    Align2::LEFT_TOP,
                    format!("{:04}", i + 1),
                    FontId::monospace(label_font_size),
                    label_color
                ); }

                if (y - response.rect.min.y) > window_height - square_size_inside { // if next row will overflow,
                    // println!("boo");
                    break; // stop rendering the squares.
                }
            }
        }

        painter.text(
            pos2(response.rect.min.x, response.rect.min.y),
            Align2::LEFT_TOP,
            hovered_text,
            FontId::monospace(12.),
            Color32::LIGHT_GRAY
        )

    });
}