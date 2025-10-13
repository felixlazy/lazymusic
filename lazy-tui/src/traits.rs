//! 该模块定义了 TUI 组件的 traits。

use lazy_core::traits::{HasBorderStyle, HasBorderStyleSetter, HasTitleStyle, HasTuiStyle};
use ratatui::{Frame, layout::Rect, widgets::Block};
use std::any::Any;

use crate::types::TuiEnent;

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

    /// 尝试将组件作为 `&dyn TuiEnentHandle` 的不可变引用返回。
    ///
    /// 主要用于在运行时动态检查一个 `RenderTui` 对象是否也实现了 `TuiEnentHandle`。
    /// 默认返回 `None`，需要事件处理的组件应重写此方法。
    fn as_enent(&self) -> Option<&dyn TuiEnentHandle> {
        None
    }

    /// 尝试将组件作为 `&mut dyn TuiEnentHandle` 的可变引用返回。
    ///
    /// 允许对实现了 `TuiEnentHandle` 的组件进行可变操作，如处理事件。
    /// 默认返回 `None`，需要事件处理的组件应重写此方法。
    fn as_enent_mut(&mut self) -> Option<&mut dyn TuiEnentHandle> {
        None
    }

    /// 尝试将组件作为 `&mut dyn HasBorderStyleSetter` 的可变引用返回。
    ///
    /// 主要用于在运行时动态地对组件边框进行操作。
    /// 默认返回 `None`，需要边框操作的组件应重写此方法。
    fn as_border_mut(&mut self) -> Option<&mut dyn HasBorderStyleSetter> {
        None
    }
}

/// 包含其他小部件的组件的 trait。
pub trait HasWidgets {
    /// 返回对小部件向量的可变引用。
    fn get_widgets_mut(&mut self) -> &mut Vec<Box<dyn RenderTui>>;

    /// 返回对小部件向量的可变引用。
    fn get_widgets(&self) -> &Vec<Box<dyn RenderTui>>;

    /// 返回对 `T` 类型特定小部件的可变引用。
    fn get_widget_mut<T: 'static>(&mut self) -> Option<&mut T> {
        self.get_widgets_mut()
            .iter_mut()
            .find_map(|widget| widget.as_any_mut().downcast_mut::<T>())
    }

    fn get_widget<T: 'static>(&self) -> Option<&T> {
        self.get_widgets()
            .iter()
            .find_map(|widget| widget.as_any().downcast_ref::<T>())
    }
}

pub trait TuiEnentHandle {
    fn enent_handle(&mut self, event: TuiEnent);
}
