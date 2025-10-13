//! 视图路由组件模块，定义了 TUI 的主内容区域，可根据状态切换不同的子视图。

use lazy_core::structs::{BorderStyle, TitleStyle, TuiStyle};
use lazy_macro::DeriveHasTuiStyle;
use ratatui::{Frame, layout::Rect};

use crate::{
    traits::{HasWidgets, RenderTui, TuiBlock, TuiEventHandle},
    types::TuiEnent,
};

/// `RouterViewTui` 是一个多功能视图容器，扮演“视图路由”的角色。
#[derive(DeriveHasTuiStyle)]
pub struct RouterViewTui {
    /// 组件的标题样式。
    title: TitleStyle,
    /// 组件的边框样式。
    border: BorderStyle,
    /// 组件的通用样式（背景、前景颜色等）。
    style: TuiStyle,
    /// 包含的所有可切换的子组件（视图）。
    widgets: Vec<Box<dyn RenderTui>>,
}

impl Default for RouterViewTui {
    /// 创建一个默认的 `RouterViewTui` 实例。
    fn default() -> Self {
        Self {
            title: Default::default(),
            border: Default::default(),
            style: Default::default(),
            widgets: vec![],
        }
    }
}

/// 为 `RouterViewTui` 实现 `RenderTui` trait，使其能够被渲染。
impl RenderTui for RouterViewTui {
    /// 渲染 `RouterViewTui` 组件。
    fn render(&self, frame: &mut Frame, rect: Rect) {
        // 获取去掉边框的内部区域
        let inner = self.get_inner(rect);
        // 渲染根组件边框和标题
        frame.render_widget(self.to_block(), rect);

        // TODO: 在此实现活动子组件的渲染逻辑
    }

    fn as_event(&self) -> Option<&dyn crate::traits::TuiEventHandle> {
        Some(self)
    }

    fn as_event_mut(&mut self) -> Option<&mut dyn crate::traits::TuiEventHandle> {
        Some(self)
    }

    fn as_border_mut(&mut self) -> Option<&mut dyn lazy_core::traits::HasBorderStyleSetter> {
        Some(self)
    }
}

/// 为 `RouterViewTui` 实现 `HasWidgets` trait，使其能够管理子组件。
impl HasWidgets for RouterViewTui {
    /// 获取对 `widgets` 向量的可变引用。
    fn get_widgets_mut(&mut self) -> &mut Vec<Box<dyn RenderTui>> {
        &mut self.widgets
    }

    /// 获取对 `widgets` 向量的不可变引用。
    fn get_widgets(&self) -> &Vec<Box<dyn RenderTui>> {
        &self.widgets
    }
}

impl TuiEventHandle for RouterViewTui {
    fn event_handle(&mut self, event: TuiEnent) {}
}
