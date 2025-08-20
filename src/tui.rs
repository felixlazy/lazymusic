use crate::tui::{theme::Theme, traits::Draw};

mod theme;
pub mod traits;
mod types;

#[derive(Default)]
pub struct Tui {
    theme: Theme,
}
impl Draw for Tui {
    fn draw(&mut self, frame: &mut ratatui::Frame) {}
}
