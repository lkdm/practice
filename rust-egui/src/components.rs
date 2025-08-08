use dirs::download_dir;
use eframe::egui::{self, Color32, Frame, RichText, Ui};
use rfd::FileDialog;
use std::path::PathBuf;

use crate::pdf::PDF;

pub fn file_picker(ui: &mut Ui, picked_pdf: &mut Option<PDF>) {
    if ui.button("Browse").clicked() {
        let mut dialog = FileDialog::new().add_filter("PDF", &["pdf"]);

        if let Some(downloads) = download_dir() {
            dialog = dialog.set_directory(downloads);
        }

        if let Some(path) = dialog.pick_file() {
            *picked_pdf = Some(PDF::new(&path));
            // load PDF here if you want
        }
    }

    if let Some(pdf) = picked_pdf {
        ui.label(match pdf.path.as_ref() {
            Some(p) => format!("Picked file: {}", p.display()),
            None => "No path".to_string(),
        });
    }
}

pub fn file_picker_with_label(ui: &mut Ui, picked_files: &mut Vec<PathBuf>) {
    Frame::none()
        .fill(Color32::from_gray(30)) // subtle background, customize as you like
        .rounding(egui::Rounding::same(6))
        .show(ui, |ui| {
            // Button takes full width
            if ui
                .add(egui::Button::new("Browse").min_size(ui.available_size()))
                .clicked()
            {
                if let Some(files) = rfd::FileDialog::new()
                    .set_directory(dirs::download_dir().unwrap_or_else(|| ".".into()))
                    .pick_files()
                {
                    *picked_files = files;
                }
            }

            ui.add_space(4.0); // spacing between button and label

            let count = picked_files.len();
            let label_text = if count == 0 {
                "No files selected".to_owned()
            } else if count == 1 {
                format!("1 file selected")
            } else {
                format!("{} files selected", count)
            };

            ui.label(
                RichText::new(label_text)
                    .small()
                    .color(Color32::from_gray(180)),
            );
        });
}
