use dirs::download_dir;
use eframe::egui::{
    self, CentralPanel, Color32, ComboBox, Frame, KeyboardShortcut, Margin, RichText, Rounding,
    SelectableLabel, Ui,
};
use egui_material_icons::icons;
use rfd::FileDialog;
use std::path::PathBuf;

use crate::pdf::PDF;

// TODO:
// - Merge
// - Compress

pub mod pdf;

fn main() -> Result<(), eframe::Error> {
    let mut options = eframe::NativeOptions::default();

    eframe::run_native(
        "PDF Tools",
        options,
        Box::new(|_cc| {
            _cc.egui_ctx.set_pixels_per_point(1.5);
            egui_material_icons::initialize(&_cc.egui_ctx);
            Ok(Box::<MyApp>::default())
        }),
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

    fn show_nav(&mut self, ui: &mut Ui) {
        ui.vertical_centered(|ui| {
            ui.add_space(10.0);
            ui.label(
                RichText::new(format!("{} {}", icons::ICON_PICTURE_AS_PDF, "PDF Toolbox"))
                    .heading()
                    .color(Color32::from_rgb(180, 180, 180)),
            );
            ui.add_space(20.0);
        });

        for (action_val, label, icon, hint) in &[
            (
                Action::Split,
                "Split",
                icons::ICON_CALL_SPLIT,
                "Split a single PDF into multiple",
            ),
            (
                Action::Merge,
                "Merge",
                icons::ICON_CALL_MERGE,
                "Merge a group of PDFs into one",
            ),
            (
                Action::Compress,
                "Compress",
                icons::ICON_COMPRESS,
                "Compress PDFs",
            ),
        ] {
            let selected = self.action == *action_val;

            let button = egui::Button::new(
                RichText::new(format!("{}  {}", icon, label))
                    .size(14.0)
                    .color(if selected {
                        Color32::WHITE
                    } else {
                        Color32::from_rgb(180, 180, 180)
                    }),
            )
            .fill(if selected {
                Color32::from_rgb(70, 130, 180)
            } else {
                Color32::from_rgb(40, 40, 40)
            })
            .min_size(egui::vec2(ui.available_width(), 36.0));

            if ui.add(button).on_hover_text(*hint).clicked() {
                self.action = *action_val;
            }

            ui.add_space(8.0);
        }

        ui.with_layout(egui::Layout::bottom_up(eframe::egui::Align::Center), |ui| {
            ui.add_space(6.0);
            ui.label(
                RichText::new("v0.1.0")
                    .small()
                    .color(Color32::from_gray(100)),
            );
        });
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Action {
    Split,
    Merge,
    Compress,
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &eframe::egui::Context, _frame: &mut eframe::Frame) {
        egui::SidePanel::left("top_bar")
            .resizable(true)
            .default_width(180.0)
            .width_range(160.0..=240.0)
            .frame(
                Frame::new()
                    .fill(Color32::from_rgb(20, 20, 20))
                    .inner_margin(8),
            )
            .show(ctx, |ui| {
                egui::ScrollArea::both().show(ui, |ui| {
                    self.show_nav(ui);
                });
                // ui.add_space(10.0);
            });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.add_space(10.0);
            self.show_file_picker(ui);
            self.show_split_button(ui);
            match self.action {
                Action::Split => ui.label("Split"),
                Action::Merge => ui.label("Merge"),
                Action::Compress => ui.label("Compress"),
            }
        });

        ctx.request_repaint();
    }
}
