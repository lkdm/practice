use eframe::egui::{self, ViewportBuilder, WindowLevel};

fn main() -> Result<(), eframe::Error> {
    let viewport = ViewportBuilder::default()
        .with_transparent(true)
        .with_decorations(false)
        .with_has_shadow(false)
        .with_title_shown(false)
        .with_taskbar(false)
        .with_resizable(false)
        .with_movable_by_background(false)
        .with_always_on_top()
        .with_mouse_passthrough(true);
    // .with_inner_size([320.0, 240.0]);

    let options = eframe::NativeOptions {
        viewport,
        ..Default::default()
    };

    eframe::run_native(
        "Multiple viewports",
        options,
        Box::new(|_cc| Ok(Box::<MyApp>::default())),
    )
}

struct MyApp;

impl Default for MyApp {
    fn default() -> Self {
        Self
    }
}

impl eframe::App for MyApp {
    fn clear_color(&self, _visuals: &egui::Visuals) -> [f32; 4] {
        egui::Rgba::TRANSPARENT.to_array()
    }

    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::Window::new("hello")
            .title_bar(false)
            .frame(egui::Frame::NONE) // removes background frame for full transparency
            .show(ctx, |ui| {
                ui.label("Timer overlay with transparency and mouse passthrough");
            });
        ctx.request_repaint();
    }
}
