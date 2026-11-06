use gpui_kit::component::ActiveTheme as _;
use gpui_kit::component::button::{Button, ButtonVariants};
use gpui_kit::component::input::{Textarea, TextareaState};
use gpui_kit::component::{Theme, ThemeMode};
use gpui_kit::*;

struct HelloWorld {
    message: Entity<TextareaState>,
}

impl HelloWorld {
    fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let message = cx.new(|cx| {
            TextareaState::new(window, cx)
                .auto_grow(2, 8)
                .placeholder("Write a message...")
        });

        Self { message }
    }
}

impl Render for HelloWorld {
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
            .child(
                div().flex_1().min_h_0().w(px(640.0)).child(
                    Textarea::new(&self.message)
                        .h_full()
                        .bordered(false)
                        .bg(cx.theme().background),
                ),
            )
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
            cx.new(|cx| HelloWorld::new(window, cx))
        })
        .expect("Failed to open window");
    });
}
