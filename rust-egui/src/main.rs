use dirs::download_dir;
use eframe::egui::{
    self, CentralPanel, Color32, ComboBox, Frame, KeyboardShortcut, Margin, Rounding,
    SelectableLabel, Ui,
};
use rfd::FileDialog;
use std::path::PathBuf;

use crate::pdf::PDF;

pub mod pdf;

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions::default();

    eframe::run_native(
        "PDF Tools",
        options,
        Box::new(|_cc| Ok(Box::<MyApp>::default())),
    )
}

struct MyApp {
    action: Action,
    picked_file: Option<PathBuf>,
    pdf: Option<PDF>,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            action: Action::Split,
            picked_file: None,
            pdf: None,
        }
    }
}

impl MyApp {
    // This method handles the file picker UI and modifies self.picked_file
    fn show_file_picker(&mut self, ui: &mut egui::Ui) {
        let open_shortcut = egui::KeyboardShortcut::new(egui::Modifiers::COMMAND, egui::Key::O);

        let shortcut_triggered = ui
            .ctx()
            .input_mut(|input| input.consume_shortcut(&open_shortcut));

        if ui.button("Open file picker").clicked() || shortcut_triggered {
            let downloads_dir =
                dirs::download_dir().unwrap_or_else(|| std::path::PathBuf::from("."));
            if let Some(path) = FileDialog::new().set_directory(downloads_dir).pick_file() {
                self.picked_file = Some(path);
            }
        }

        if let Some(ref file) = self.picked_file {
            ui.label(format!("Picked file: {}", file.display()));
            let document = PDF::new(file);
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

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Action {
    Split,
    Merge,
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &eframe::egui::Context, _frame: &mut eframe::Frame) {
        ctx.set_pixels_per_point(1.5);

        egui::TopBottomPanel::top("top_bar")
            .min_height(24.0)
            .frame(
                Frame::new()
                    .fill(Color32::from_rgb(50, 50, 50))
                    .inner_margin(8),
            )
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    ui.selectable_value(&mut self.action, Action::Split, "Split");
                    ui.selectable_value(&mut self.action, Action::Merge, "Merge");
                });
                // ui.add_space(10.0);
            });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.add_space(10.0);
            self.show_file_picker(ui);
            self.show_split_button(ui);
        });

        ctx.request_repaint();
    }
}
