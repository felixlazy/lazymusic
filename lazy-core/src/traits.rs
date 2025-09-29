use ratatui::{
    layout::Alignment,
    style::{Color, Style},
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
