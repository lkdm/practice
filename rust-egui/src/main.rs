use dirs::desktop_dir;
use eframe::egui::{self, Color32, Frame, KeyboardShortcut, RichText, Ui};
use egui_material_icons::icons;
use std::{ops::Range, path::PathBuf};

use crate::pdf::PDF;

mod components;
pub mod pdf;

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions::default();

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

struct SplitViewState {
    picked_pdf: Option<PDF>,

    ranges: Vec<Range<usize>>,
}

impl Default for SplitViewState {
    fn default() -> Self {
        Self {
            picked_pdf: None,
            ranges: Vec::new(),
        }
    }
}

struct Config {
    /// The default directory to open
    default_directory: Option<PathBuf>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            default_directory: dirs::download_dir().or(desktop_dir()),
        }
    }
}

struct MyApp {
    action: Action,
    split_view_state: SplitViewState,
    config: Config,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            action: Action::Split,
            split_view_state: SplitViewState::default(),
            config: Config::default(),
        }
    }
}

impl MyApp {
    fn show_split_view(&mut self, ui: &mut egui::Ui) {
        // File picker to load PDF
        components::file_picker(ui, &mut self.split_view_state.picked_pdf);

        ui.separator();
    }

    fn show_extract_view(&mut self, ui: &mut egui::Ui) {
        components::file_picker(ui, &mut self.split_view_state.picked_pdf);
    }

    fn show_merge_view(&mut self, ui: &mut egui::Ui) {
        // file_picker_with_label(ui);
        ui.label("Merge functionality coming soon...");
        // merge UI code here
    }

    fn show_compress_view(&mut self, ui: &mut egui::Ui) {
        ui.label("Compress functionality coming soon...");
        // compress UI code here
    }

    // fn show_split_button(&mut self, ui: &mut Ui) -> Result<(), Box<dyn std::error::Error>> {
    //     if ui.button("Split").clicked() {
    //         let downloads_dir = dirs::download_dir().expect("could not find downloads directory");
    //
    //         if let Some(ref old_pdf) = self.pdf {
    //             // Split PDF into individual page PDFs
    //             let single_pages = old_pdf.split_into_single_pages()?;
    //             // Optionally save each single page PDF
    //             for (index, single_pdf) in single_pages.into_iter().enumerate() {
    //                 let mut output_path = downloads_dir.clone();
    //                 output_path.push(format!("page_{}.pdf", index + 1));
    //                 let mut pdf = single_pdf;
    //                 pdf.flush(&output_path)?;
    //             }
    //         }
    //     }
    //     Ok(())
    // }

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

        for action in Action::all() {
            let selected = self.action == action.clone();

            let button = egui::Button::new(
                RichText::new(format!(" {}  {}", action.icon(), action.label()))
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

            if ui.add(button).on_hover_text(action.hint()).clicked() {
                self.action = action.clone();
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Action {
    Split,
    Extract,
    Merge,
    Compress,
}

type ActionViewFn = fn(&mut MyApp, &mut egui::Ui);

impl Action {
    /// Return all supported actions
    pub fn all() -> &'static [Action] {
        &[
            Action::Split,
            Action::Extract,
            Action::Merge,
            Action::Compress,
        ]
    }

    pub fn label(&self) -> &'static str {
        match self {
            Action::Split => "Split",
            Action::Extract => "Extract pages",
            Action::Merge => "Merge",
            Action::Compress => "Compress",
        }
    }

    pub fn icon(&self) -> &'static str {
        match self {
            Action::Split => icons::ICON_CALL_SPLIT,
            Action::Extract => icons::ICON_COLORIZE,
            Action::Merge => icons::ICON_CALL_MERGE,
            Action::Compress => icons::ICON_COMPRESS,
        }
    }

    pub fn hint(&self) -> &'static str {
        match self {
            Action::Split => "Split a single PDF into multiple",
            Action::Extract => "Extract a page or range of pages from a PDF",
            Action::Merge => "Merge a group of PDFs into one",
            Action::Compress => "Compress PDFs",
        }
    }

    pub fn view(&self) -> ActionViewFn {
        match self {
            Action::Split => MyApp::show_split_view,
            Action::Extract => MyApp::show_extract_view,
            Action::Merge => MyApp::show_merge_view,
            Action::Compress => MyApp::show_compress_view,
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &eframe::egui::Context, _frame: &mut eframe::Frame) {
        // egui::TopBottomPanel::top("top_bar").show(ctx, |ui| {
        //     egui::MenuBar::new().ui(ui, |ui| {
        //         ui.menu_button("Application", |ui| {
        //             if ui.button("Quit").clicked() {
        //                 ui.ctx().send_viewport_cmd(egui::ViewportCommand::Close);
        //             }
        //         });
        //     });
        // });

        egui::SidePanel::left("left_bar")
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
            (self.action.view())(self, ui);
        });

        ctx.request_repaint();
    }
}
