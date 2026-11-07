use gpui_kit::component::ActiveTheme as _;
use gpui_kit::component::button::{Button, ButtonVariants};
use gpui_kit::component::input::{Textarea, TextareaState};
use gpui_kit::component::{Theme, ThemeMode};
use gpui_kit::*;

use crate::editor::TextEditor;

pub mod editor;

struct App {
    editor: Entity<TextEditor>,
}

impl App {
    pub fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let editor = cx.new(|cx| TextEditor::new(window, cx));
        Self { editor }
    }
}

impl Render for App {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .bg(cx.theme().background)
            .text_color(cx.theme().foreground)
            .border_color(cx.theme().border)
            .flex()
            .flex_col()
            .size_full()
            .p_4()
            .gap_3()
            .items_center()
            .child(self.editor.clone())
            .child(Button::new("send").primary().label("Send").on_click({
                move |_, _, _| {
                    println!("Send clicked!");
                }
            }))
    }
}

fn main() {
    application().with_assets(assets::Assets).run(|cx| {
        init(cx);

        open_window(WindowOptions::default(), cx, |window, cx| {
            Theme::change(ThemeMode::Dark, None, cx);
            cx.new(|cx| App::new(window, cx))
        })
        .expect("Failed to open window");
    });
}
