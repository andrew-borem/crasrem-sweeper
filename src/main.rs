use eframe::{egui, IconData};
use egui::{Vec2, Rect, Pos2, Color32, Stroke};
use image::ImageError;
use image::io::Reader as ImageReader;

fn main() {
    let mut native_options = eframe::NativeOptions::default();
    native_options.initial_window_size = Some(Vec2{ x: 512.0, y: 512.0});
    native_options.resizable = false;

    match load_icon("src/assets/icon.png") {
        Ok(icon) => native_options.icon_data = Some(icon),
        Err(e) => eprintln!("Error loading icon: {:?}", e),
    }

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

        for _i in 1..=area {
            vector.push(Vec2{x: x_position, y: y_position});

            y_position += 1.0;

            if y_position > 15.0 {
                y_position = 0.0;
                x_position += 1.0;
            }
        }

        return vector;
    }
}


impl eframe::App for MyEguiApp {

    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui: &mut egui::Ui| {
            let painter = ui.painter();
            let default_fill = Color32::from_rgb(100, 150, 200);
            let default_stroke = Color32::from_rgb(0, 0, 0);

            let test: u16 = GameBoard::new();
            let test2 = GameBoard::populate_board(test);
    
            for vec in test2 {
                let mut rectangle = Rect::NOTHING;

                rectangle.set_top(vec[0] * 32.0);
                rectangle.set_left(vec[1] * 32.0);
                rectangle.set_width(32.0);
                rectangle.set_height(32.0);
                &painter.rect_stroke(rectangle, 0.0, Stroke{width: 1.0, color: default_stroke});
                &painter.rect_filled(rectangle, 0.0, default_fill);
                println!("{:?}", rectangle);
            }
        });
    }
}

fn load_icon(path: &str) -> Result<IconData, ImageError> {
    let image = ImageReader::open(path)?
        .decode()?
        .to_rgba8();

    let (width, height) = image.dimensions();
    let rgba = image.into_raw();
    Ok(IconData {
        rgba,
        width: width as u32,
        height: height as u32,
    })
}
