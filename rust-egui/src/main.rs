use eframe::egui::{self, CentralPanel, Ui};
use rfd::FileDialog;
use std::path::PathBuf;

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions::default();

    eframe::run_native(
        "PDF Tool",
        options,
        Box::new(|_cc| Ok(Box::<MyApp>::default())),
    )
}

struct MyApp {
    picked_file: Option<PathBuf>,
}

impl Default for MyApp {
    fn default() -> Self {
        Self { picked_file: None }
    }
}

impl MyApp {
    // This method handles the file picker UI and modifies self.picked_file
    fn show_file_picker(&mut self, ui: &mut Ui) {
        if ui.button("Open file picker").clicked() {
            if let Some(path) = FileDialog::new().pick_file() {
                self.picked_file = Some(path);
            }
        }

        if let Some(ref file) = self.picked_file {
            ui.label(format!("Picked file: {}", file.display()));
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &eframe::egui::Context, _frame: &mut eframe::Frame) {
        ctx.set_pixels_per_point(1.5);
        CentralPanel::default().show(ctx, |ui| {
            ui.heading("PDF tool");
            ui.label("This is directly on the main frame without a window.");

            // Call the helper method here
            self.show_file_picker(ui);
        });

        ctx.request_repaint(); // keep UI responsive
    }
}
