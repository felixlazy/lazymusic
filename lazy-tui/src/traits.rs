//! 该模块定义了 TUI 组件的 traits。

use lazy_core::traits::{HasBorderStyle, HasTitleStyle, HasTuiStyle};
use ratatui::{Frame, layout::Rect, widgets::Block};
use std::any::Any;

/// 可渲染为 `ratatui` `Block` 的组件的 trait。
pub trait TuiBlock: HasTitleStyle + HasBorderStyle + HasTuiStyle {
    /// 从组件的属性创建 `Block`。
    fn to_block(&self) -> Block<'_> {
        Block::default()
            .title(self.title_text())
            .title_style(self.title_style())
            .title_alignment(self.title_alignment())
            .borders(self.borders())
            .border_style(self.border_style())
            .style(self.tui_style())
    }

    /// 返回块的内部区域。
    fn get_inner(&self, rect: Rect) -> Rect {
        if self.has_border() {
            self.to_block().inner(rect)
        } else {
            rect
        }
    }
}

impl<U> TuiBlock for U where U: HasBorderStyle + HasTitleStyle + HasTuiStyle {}

/// 可在 TUI 中渲染的组件的 trait。
pub trait RenderTui: Any {
    /// 在给定的框架和区域中渲染组件。
    fn render(&self, frame: &mut Frame, rect: Rect);

    /// 以 `&dyn Any` 的形式返回组件。
    fn as_any(&self) -> &dyn Any;

    /// 以 `&mut dyn Any` 的形式返回组件。
    fn as_any_mut(&mut self) -> &mut dyn Any;
}

/// 包含其他小部件的组件的 trait。
pub trait HasWidgets {
    /// 返回对小部件向量的可变引用。
    fn get_widgets_mut(&mut self) -> &mut Vec<Box<dyn RenderTui>>;

    /// 返回对 `T` 类型特定小部件的可变引用。
    fn get_widget_mut<T: 'static>(&mut self) -> Option<&mut T> {
        self.get_widgets_mut()
            .iter_mut()
            .find_map(|widget| widget.as_any_mut().downcast_mut::<T>())
    }
}

