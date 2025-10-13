//! 此模块提供了一个用于显示进度条的 TUI 组件。
//!
//! `ProgressTui` 结构体负责渲染一个带有圆角的进度条。
//! 它使用 `ratatui` 库中的 `Gauge` 小部件来显示进度。

use lazy_core::{
    structs::{BorderStyle, TitleStyle, TuiStyle},
    traits::{HasBorderStyleSetter, HasTuiStyle},
};
use lazy_macro::DeriveHasTuiStyle;
use ratatui::{
    layout::{Alignment, Constraint, Layout, Rect},
    style::{Color, Stylize},
    widgets::{Gauge, Paragraph},
};

use crate::traits::{RenderTui, TuiBlock};

/// 一个用于显示进度条的 TUI 组件。
///
/// 它由一个用于进度条本身的中央 `Gauge` 小部件，
/// 和两个用于圆角的 `Paragraph` 小部件组成。
#[derive(DeriveHasTuiStyle)]
pub struct ProgressTui {
    title: TitleStyle,   // 标题样式
    border: BorderStyle, // 边框样式
    style: TuiStyle,     // 通用样式（颜色、对齐等）
    ratio: f64,
}

impl Default for ProgressTui {
    /// 创建一个具有默认样式的新的 `ProgressTui`。
    ///
    fn default() -> Self {
        let mut style = TuiStyle::default();
        style.set_bg(Color::Rgb(47, 51, 77));
        Self {
            title: Default::default(),
            style,
            border: Default::default(),
            ratio: 0.0,
        }
    }
}

impl RenderTui for ProgressTui {
    /// 将进度条渲染到框架上。
    ///
    /// 进度条在给定的 `rect` 内渲染。
    /// 它由三部分组成：一个左半圆、进度计量器和一个右半圆。
    fn render(&self, frame: &mut ratatui::Frame, rect: Rect) {
        // 获取去掉边框的内部区域
        let inner = self.get_inner(rect);
        // 渲染根组件边框和标题
        frame.render_widget(self.to_block().bg(self.border.bg()), rect);

        let left_haft_circle = Paragraph::new("").alignment(Alignment::Right);
        let bars = Gauge::default()
            .gauge_style(self.tui_style())
            .label("")
            .ratio(self.ratio);
        let right_haft_circle = Paragraph::new("")
            .alignment(Alignment::Left)
            .fg(self.style.bg());
        let row = Layout::horizontal([
            Constraint::Min(2),
            Constraint::Percentage(98),
            Constraint::Min(2),
        ])
        .split(inner);

        frame.render_widget(left_haft_circle, row[0]);
        frame.render_widget(bars, row[1]);
        frame.render_widget(right_haft_circle, row[2]);
    }

    fn as_border_mut(&mut self) -> Option<&mut dyn HasBorderStyleSetter> {
        Some(self)
    }
}

impl ProgressTui {
    /// 设置进度条的进度比率。
    ///
    /// `ratio` 应为 0.0 到 1.0 之间的值。
    pub(crate) fn set_ratio(&mut self, ratio: f64) {
        self.ratio = ratio.clamp(0.0, 1.0);
    }

    /// 返回进度条的当前进度比率。
    pub(crate) fn ratio(&self) -> f64 {
        self.ratio
    }

    pub(crate) fn reset_ratio(&mut self) {
        self.ratio = 0.0;
    }
}
