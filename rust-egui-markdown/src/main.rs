use eframe::egui::{self, CentralPanel, ComboBox, Ui};

use crate::doc::DocumentManager;

pub mod doc;
pub mod fs;

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions::default();

    eframe::run_native(
        "Markdown editor",
        options,
        Box::new(|_cc| {
            _cc.egui_ctx.set_pixels_per_point(1.5);
            // egui_material_icons::initialize(&_cc.egui_ctx);
            Ok(Box::<MarkdownEditor>::default())
        }),
    )
}

struct MarkdownEditor {
    document_manager: DocumentManager,
}

impl Default for MarkdownEditor {
    fn default() -> Self {
        Self {
            document_manager: DocumentManager::default(),
        }
    }
}

// impl MyApp {
//     // This method handles the file picker UI and modifies self.picked_file
//     fn show_file_picker(&mut self, ui: &mut Ui) {
//         if ui.button("Open file picker").clicked() {
//             let downloads_dir = dirs::download_dir().unwrap();
//             if let Some(path) = FileDialog::new().set_directory(downloads_dir).pick_file() {
//                 self.picked_file = Some(path.clone());
//                 match std::fs::read_to_string(&path) {
//                     Ok(content) => self.markdown_input = content,
//                     Err(e) => {
//                         eprintln!("Error reading file: {}", e);
//                     }
//                 }
//             }
//         }
//
//         if let Some(ref file) = self.picked_file {
//             ui.label(format!("Picked file: {}", file.display()));
//         }
//     }
// }

impl eframe::App for MarkdownEditor {
    fn update(&mut self, ctx: &eframe::egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("top_bar").show(ctx, |ui| {
            egui::MenuBar::new().ui(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("New").clicked() {
                        ui.ctx().send_viewport_cmd(egui::ViewportCommand::Close);
                    }
                    if ui.button("Open").clicked() {
                        ui.ctx().send_viewport_cmd(egui::ViewportCommand::Close);
                    }
                    if ui.button("Save").clicked() {
                        ui.ctx().send_viewport_cmd(egui::ViewportCommand::Close);
                    }
                    if ui.button("Quit").clicked() {
                        ui.ctx().send_viewport_cmd(egui::ViewportCommand::Close);
                    }
                });

                ui.menu_button("Edit", |ui| {
                    if ui.button("Cut").clicked() {
                        ui.ctx()
                            .send_viewport_cmd(egui::ViewportCommand::RequestCut);
                    }
                    if ui.button("Copy").clicked() {
                        ui.ctx()
                            .send_viewport_cmd(egui::ViewportCommand::RequestCopy);
                    }
                    if ui.button("Paste").clicked() {
                        ui.ctx()
                            .send_viewport_cmd(egui::ViewportCommand::RequestPaste);
                    }
                });
            });
        });

        CentralPanel::default().show(ctx, |ui| {
            ui.label("This is directly on the main frame without a window.");
            // self.show_file_picker(ui);

            // ui.text_edit_multiline(&mut self.markdown_input);
        });

        ctx.request_repaint(); // keep UI responsive
    }
}
