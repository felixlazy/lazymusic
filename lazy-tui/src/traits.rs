use lazy_core::traits::{HasBorderStyle, HasTitleStyle, HasTuiStyle};
use ratatui::{Frame, layout::Rect, widgets::Block};
use std::any::Any;

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
pub trait RenderTui: Any {
    fn render(&self, frame: &mut Frame, rect: Rect);
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
}

pub trait HasWidgets {
    fn get_widgets_mut(&mut self) -> &mut Vec<Box<dyn RenderTui>>;

    fn get_widget_mut<T: 'static>(&mut self) -> Option<&mut T> {
        self.get_widgets_mut()
            .iter_mut()
            .find_map(|widget| widget.as_any_mut().downcast_mut::<T>())
    }
}