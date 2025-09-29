use lazy_macro::Accessor;
use ratatui::{
    layout::Alignment,
    style::{Color, Modifier},
};

#[derive(Accessor)]
pub struct TitleStyle {
    text: String,
    #[Accessor(Copy)]
    alignment: Alignment,
    #[Accessor(Copy)]
    modifier: Modifier,
    #[Accessor(Copy)]
    fg: Color,
    #[Accessor(Copy)]
    bg: Color,
}
impl Default for TitleStyle {
    fn default() -> Self {
        Self {
            text: Default::default(),
            alignment: Alignment::Center,
            modifier: Modifier::ITALIC,
            fg: Color::Rgb(130, 170, 255), // #82aaff
            bg: Color::Rgb(34, 36, 54),    // #222436
        }
    }
}
