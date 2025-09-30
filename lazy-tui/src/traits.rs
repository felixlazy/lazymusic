use lazy_core::traits::{HasBorderStyle, HasTitleStyle, HasTuiStyle};
use ratatui::{Frame, layout::Rect, widgets::Block};

pub trait TuiBlock: HasTitleStyle + HasBorderStyle + HasTuiStyle {
    fn to_block(&self) -> Block<'_> {
        Block::default()
            .title(self.title_text())
            .title_style(self.title_style())
            .title_alignment(self.title_alignment())
            .borders(self.borders())
            .border_style(self.border_style())
            .style(self.tui_style())
    }
    fn get_inner(&self, rect: Rect) -> Rect {
        if self.has_border() {
            self.to_block().inner(rect)
        } else {
            rect
        }
    }
}

impl<U> TuiBlock for U where U: HasBorderStyle + HasTitleStyle + HasTuiStyle {}
pub trait RenderTui {
    fn render(&self, frame: &mut Frame, rect: Rect);
}
