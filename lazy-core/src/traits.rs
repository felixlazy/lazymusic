use ratatui::{
    layout::Alignment,
    style::{Color, Modifier, Style},
    widgets::Borders,
};

pub trait HasTitleStyle {
    fn title_style(&self) -> Style;
    fn title_alignment(&self) -> Alignment;
    fn title_text(&self) -> &str;
}

pub trait HasBorderStyle {
    fn border_style(&self) -> Style;
    fn borders(&self) -> Borders;
    fn has_border(&self) -> bool;
}

pub trait HasTuiStyle {
    fn bg(&self) -> Color;
    fn fg(&self) -> Color;
}

pub trait HasTitleStyleSetter {
    fn set_title_text(&mut self, text: String);
    fn set_title_alignment(&mut self, alignment: Alignment);
    fn set_title_modifier(&mut self, modifier: Modifier);
    fn set_title_fg(&mut self, fg: Color);
    fn set_title_bg(&mut self, bg: Color);
}

pub trait HasBorderStyleSetter {
    fn set_border(&mut self, border: Borders);
    fn set_border_fg(&mut self, fg: Color);
    fn set_border_bg(&mut self, bg: Color);
}

pub trait HasTuiStyleSetter {
    fn set_tui_bg(&mut self, color: Color);
    fn set_tui_fg(&mut self, color: Color);
}
