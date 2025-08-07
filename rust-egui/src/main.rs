use dirs::download_dir;
use eframe::egui::{self, CentralPanel, Ui};
use rfd::FileDialog;
use std::path::PathBuf;

use crate::pdf::PDF;

pub mod pdf;

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
    pdf: Option<PDF>,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            picked_file: None,
            pdf: None,
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
            let document = PDF::new(file);
            // ui.label(format!("Page length: {}", document.length()));
            self.pdf = Some(document);
        }
    }

    fn show_split_button(&mut self, ui: &mut Ui) -> Result<(), Box<dyn std::error::Error>> {
        if ui.button("Split").clicked() {
            let downloads_dir = dirs::download_dir().expect("could not find downloads directory");

            if let Some(ref old_pdf) = self.pdf {
                // Split PDF into individual page PDFs
                let single_pages = old_pdf.split_into_single_pages()?;
                // Optionally save each single page PDF
                for (index, single_pdf) in single_pages.into_iter().enumerate() {
                    let mut output_path = downloads_dir.clone();
                    output_path.push(format!("page_{}.pdf", index + 1));
                    let mut pdf = single_pdf;
                    pdf.flush(&output_path)?;
                }
            }
        }
        Ok(())
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
            self.show_split_button(ui);
        });

        ctx.request_repaint(); // keep UI responsive
    }
}
