use eframe::egui;

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions::default();

    eframe::run_native(
        "Test",
        options,
        Box::new(|_cc| Ok(Box::new(MyApp::default()))),
    )
}

#[derive(Default)]
struct MyApp {
    text: String,
}

impl eframe::App for MyApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        let mut layouter = |ui: &egui::Ui, text: &dyn egui::TextBuffer, wrap_width: f32| {
            let mut job = egui::text::LayoutJob::default();
            job.wrap.max_width = wrap_width;
            let indent = 24.0;

            for (i, paragraph) in text.as_str().split("\n").enumerate() {
                // Add back the line break
                if i > 0 {
                    job.append("\n", 0.0, egui::TextFormat::default());
                }
                // On new line, add a visual indent
                job.append(paragraph, indent, egui::TextFormat::default());
            }

            ui.fonts_mut(|f| f.layout_job(job))
        };

        let text_edit = egui::TextEdit::multiline(&mut self.text)
            .desired_width(f32::INFINITY)
            .layouter(&mut layouter);

        ui.add(text_edit);
    }
}
