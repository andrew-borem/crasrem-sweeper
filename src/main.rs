use eframe::{egui, IconData};
use egui::{Vec2, Color32, Stroke, Rect};
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
            |cc| Box::new(MyApp::new(cc))
        )
    );
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

#[derive(Default)]
struct MyApp {
    background_texture_id: Option<egui::TextureId>
}

impl MyApp {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        Self::default()
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui: &mut egui::Ui| {
            let test: u16 = GameBoard::new();
            let test2 = GameBoard::populate_board(test);
    
            for vec in test2 {
                draw_rect(ui, vec);
            }
        });
    }
}

fn draw_rect(ui: &mut egui::Ui, position: Vec2) {
    let mut rectangle = Rect::NOTHING;

    let image = match load_image_from_path(std::path::Path::new("src/assets/tile_background.png")) {
        Ok(image) => image,
        Err(e) => {
            eprintln!("Error loading image: {:?}", e);

            return;
        }
    };

    

    rectangle.set_top(position[0] * 32.0);
    rectangle.set_left(position[1] * 32.0);
    rectangle.set_width(32.0);
    rectangle.set_height(32.0);

    let response = ui.allocate_rect(rectangle, egui::Sense::click());

    let painter = ui.painter();
    let color = Color32::from_rgb(100, 150, 200);
    let default_stroke = Color32::from_rgb(0, 0, 0);
    let default_fill = if response.hovered() {
        color.gamma_multiply(0.5)
    } else {
        color
    };

    if response.clicked() {
        println!("you clicked the rect at position {:?}", position);
        println!("left value: {}", position[0] * 32.0);
        println!("top value: {}", position[1] * 32.0);
        println!("rectangle information: {:?}", rectangle);
    }

    &painter.rect_stroke(rectangle, 0.0, Stroke{width: 1.0, color: default_stroke});
    &painter.rect_filled(rectangle, 0.0, default_fill);
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

fn load_image_from_path(path: &std::path::Path) -> Result<egui::ColorImage, image::ImageError> {
    let image = image::io::Reader::open(path)?.decode()?;
    let dimensional = &image.to_rgba8();
    let size = [dimensional.width() as _, dimensional.height() as _];
    let image_buffer = image.to_rgba8();
    let pixels = image_buffer.as_flat_samples();
    Ok(egui::ColorImage::from_rgba_unmultiplied(
        size,
        pixels.as_slice(),
    ))
}