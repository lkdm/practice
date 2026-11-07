use gpui_kit::{
    AppContext as _, Context, Entity, IntoElement, ParentElement as _, Render, Styled as _, Window,
    base::input::TextareaState,
    component::{ActiveTheme as _, input::Textarea},
    div, px,
};

pub struct TextEditor {
    content: Entity<TextareaState>,
}

impl TextEditor {
    pub fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let content = cx.new(|cx| {
            TextareaState::new(window, cx)
                // .auto_grow(2, 8)
                .placeholder("To be or not to be...")
        });

        Self { content }
    }
}

impl Render for TextEditor {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div().flex_1().min_h_0().w(px(640.0)).child(
            Textarea::new(&self.content)
                .h_full()
                .bordered(false)
                .bg(cx.theme().background),
        )
    }
}
