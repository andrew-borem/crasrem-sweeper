use eframe::egui;
use egui::Vec2;

fn main() {
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "Crasrem Sweeper", 
        native_options, 
        Box::new(
            |cc| Box::new(MyEguiApp::new(cc))
        )
    );
}

#[derive(Default)]
struct MyEguiApp {}

impl MyEguiApp {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // Customize egui here with cc.egui_ctx.set_fonts and cc.egui_ctx.set_visuals.
        // Restore app state using cc.storage (requires the "persistence" feature).
        // Use the cc.gl (a glow::Context) to create graphics shaders and buffers that you can use
        // for e.g. egui::PaintCallback.

        Self::default()
    }
}

struct GameBoard {
    area: (u16, u16),
}

impl GameBoard {
    const X_CELLS: u16 = 16;
    const Y_CELLS: u16 = 16;
    pub const ALL_CELLS: u16 = Self::X_CELLS * Self::Y_CELLS; 

    pub fn new() -> u16 {
        Self::ALL_CELLS
    }

    fn populate_board(area: u16) -> Vec<Vec2> {
        let mut vector: Vec<Vec2> = vec![];
        let mut x_position: f32 = 0.0;
        let mut y_position: f32 = 0.0;

        for i in 1..=area {
            vector.push(Vec2{x: x_position, y: y_position});

            y_position += 1.0;

            if y_position > 15.0 {
                y_position = 0.0;
                x_position += 1.0;
            }
        }

        println!("{:?}", vector);

        return vector;
    }
}


enum Cell {
    Blank,
    Mine,
    Flag
}

impl eframe::App for MyEguiApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        frame.set_window_size(Vec2{x: 512.0, y: 512.0});

        egui::CentralPanel::default().show(ctx, |ui: &mut egui::Ui| {
            let test: u16 = GameBoard::new();
            let test2 = GameBoard::populate_board(test);
        });
    }
}