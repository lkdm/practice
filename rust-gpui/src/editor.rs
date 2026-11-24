use gpui_kit::{
    AppContext as _, Context, Entity, IntoElement, ParentElement as _, Render, Styled as _, Window,
    base::input::{language_config::LanguageConfig, set_language_config},
    component::{
        ActiveTheme as _,
        input::{Editor, EditorState},
    },
    div, px,
};

pub struct TextEditor {
    editor: Entity<EditorState>,
}

impl TextEditor {
    pub fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let rules = LanguageConfig::default();
        set_language_config("markdown", rules, cx);

        let editor = cx.new(|cx| {
            let mut state = EditorState::new(window, cx)
                .language("markdown")
                .line_number(false)
                .folding(false)
                .soft_wrap(true)
                .placeholder("To be or not to be...");

            state.set_highlighter("markdown".to_string(), cx);

            state
        });

        Self { editor }
    }
}

impl Render for TextEditor {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div().flex_1().min_h_0().w(px(640.0)).child(
            Editor::new(&self.editor)
                .font_family("CaskaydiaMono Nerd Font Mono")
                .text_size(px(18.))
                .h_full()
                .bordered(false)
                .bg(cx.theme().background),
        )
    }
}
