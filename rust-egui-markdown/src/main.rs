use dirs::download_dir;
use eframe::egui::{self, CentralPanel, ComboBox, Ui};
use rfd::FileDialog;
use std::path::PathBuf;

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions::default();

    eframe::run_native(
        "Markdown editor",
        options,
        Box::new(|_cc| Ok(Box::<MyApp>::default())),
    )
}

struct MyApp {
    picked_file: Option<PathBuf>,
    markdown_input: String,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            picked_file: None,
            markdown_input: "".into(),
        }
    }
}

impl MyApp {
    // This method handles the file picker UI and modifies self.picked_file
    fn show_file_picker(&mut self, ui: &mut Ui) {
        if ui.button("Open file picker").clicked() {
            let downloads_dir = dirs::download_dir().unwrap();
            if let Some(path) = FileDialog::new().set_directory(downloads_dir).pick_file() {
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
            ui.label("This is directly on the main frame without a window.");
            self.show_file_picker(ui);

            ui.text_edit_multiline(&mut self.markdown_input);
        });

        ctx.request_repaint(); // keep UI responsive
    }
}
