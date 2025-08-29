use crate::tui::{theme::Theme, traits::Draw};

pub mod event;
mod theme;
pub mod traits;
pub mod types;

#[derive(Default)]
pub struct Tui {
    theme: Theme,
}
impl Draw for Tui {
    fn draw(&mut self, frame: &mut ratatui::Frame) {}
}
