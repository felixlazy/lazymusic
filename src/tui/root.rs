use ratatui::{
    layout::Alignment,
    style::{Color, Modifier, Style},
    widgets::Borders,
};

use crate::tui::traits::{DrawTui, TuiBorder, TuiTitle};

// ==================== 边框样式 ====================
#[derive(Debug, Default)]
pub(crate) struct BorderStyle {
    enabled: bool,
    theme: Color,
}

// ==================== 标题样式 ====================
#[derive(Debug, Default)]
pub(crate) struct TitleStyle {
    enabled: bool,
    text: String,
    fg: Color,
    bg: Color,
    italic: bool,
    alignment: Alignment,
}

// ==================== 根 UI 组件 ====================
#[derive(Debug, Default)]
pub struct RootTui {
    pub border: BorderStyle,
    pub title: TitleStyle,
}

impl DrawTui for RootTui {}

// ==================== TuiBorder ====================
impl TuiBorder for RootTui {
    fn set_border(&mut self, enable: bool) {
        self.border.enabled = enable;
    }

    fn has_border(&self) -> bool {
        self.border.enabled
    }

    fn set_border_theme(&mut self, theme: Color) {
        self.border.theme = theme;
    }

    fn border_theme(&self) -> Color {
        self.border.theme
    }

    fn borders(&self) -> Borders {
        if self.border.enabled {
            Borders::ALL
        } else {
            Borders::NONE
        }
    }

    fn border_style(&self) -> Style {
        Style::default().fg(self.border.theme)
    }
}

// ==================== TuiTitle ====================
impl TuiTitle for RootTui {
    fn set_title(&mut self, title: String) {
        self.title.text = title;
    }

    fn enable_title(&mut self, enable: bool) {
        self.title.enabled = enable;
    }

    fn has_title(&self) -> bool {
        self.title.enabled
    }

    fn set_title_foreground(&mut self, fg: Color) {
        self.title.fg = fg;
    }

    fn title_foreground(&self) -> Color {
        self.title.fg
    }

    fn set_title_background(&mut self, bg: Color) {
        self.title.bg = bg;
    }

    fn title_background(&self) -> Color {
        self.title.bg
    }

    fn set_title_italic(&mut self, enable: bool) {
        self.title.italic = enable;
    }

    fn title_italic(&self) -> bool {
        self.title.italic
    }

    fn title(&self) -> Option<&str> {
        if self.title.enabled {
            Some(self.title.text.as_str())
        } else {
            None
        }
    }

    fn set_title_alignment(&mut self, alignment: Alignment) {
        self.title.alignment = alignment;
    }

    fn title_alignment(&self) -> Alignment {
        self.title.alignment
    }

    fn title_style(&self) -> Style {
        let mut style = Style::default().fg(self.title.fg).bg(self.title.bg);
        if self.title.italic {
            style = style.add_modifier(Modifier::ITALIC);
        }
        style
    }
}
