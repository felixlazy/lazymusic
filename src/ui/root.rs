use crate::ui::{
    theme::Theme,
    traits::{BorderControl, Renderable, TitleControl, UiAppearance, UiTheme},
};
use ratatui::style::Color;

#[derive(Default)]
pub(crate) struct RootUi {
    border_enabled: bool,
    title_enabled: bool,
    theme: Theme,
    title: String,
}

impl TitleControl for RootUi {
    fn set_title(&mut self, title: impl Into<String>) {
        self.title = title.into();
    }
    fn set_title_enabled(&mut self, enabled: bool) {
        self.title_enabled = enabled;
    }
    fn title(&self) -> &str {
        &self.title
    }
    fn is_title_enabled(&self) -> bool {
        self.title_enabled
    }
}

impl BorderControl for RootUi {
    fn set_border_enabled(&mut self, enabled: bool) {
        self.border_enabled = enabled;
    }
    fn is_border_enabled(&self) -> bool {
        self.border_enabled
    }
}
impl UiAppearance for RootUi {}
impl UiTheme for RootUi {
    fn fg(&self) -> Color {
        self.theme.fg
    }
    fn bg(&self) -> Color {
        self.theme.bg
    }
    fn blue(&self) -> Color {
        self.theme.blue
    }
}
impl Renderable for RootUi {
    fn render_content(&mut self, frame: &mut ratatui::Frame, area: ratatui::layout::Rect) {}
}
