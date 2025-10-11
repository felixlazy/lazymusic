//! 此模块提供了一个用于显示进度条的 TUI 组件。
//!
//! `ProgressTui` 结构体负责渲染一个带有圆角的进度条。
//! 它使用 `ratatui` 库中的 `Gauge` 小部件来显示进度。

use lazy_core::{structs::TuiStyle, traits::HasTuiStyle};
use lazy_macro::DeriveHasTuiStyle;
use ratatui::{
    layout::{Alignment, Constraint, Layout},
    style::{Color, Stylize},
    widgets::{Gauge, Paragraph},
};

use crate::traits::RenderTui;

/// 一个用于显示进度条的 TUI 组件。
///
/// 它由一个用于进度条本身的中央 `Gauge` 小部件，
/// 和两个用于圆角的 `Paragraph` 小部件组成。
#[derive(DeriveHasTuiStyle)]
pub struct ProgressTui {
    style: TuiStyle,
    ratio: f64,
}

impl Default for ProgressTui {
    /// 创建一个具有默认样式的新的 `ProgressTui`。
    ///
    fn default() -> Self {
        let mut style = TuiStyle::default();
        style.set_bg(Color::Rgb(47, 51, 77));
        Self { style, ratio: 0.0 }
    }
}

impl RenderTui for ProgressTui {
    /// 将进度条渲染到框架上。
    ///
    /// 进度条在给定的 `rect` 内渲染。
    /// 它由三部分组成：一个左半圆、进度计量器和一个右半圆。
    fn render(&self, frame: &mut ratatui::Frame, rect: ratatui::prelude::Rect) {
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
        .split(rect);

        frame.render_widget(left_haft_circle, row[0]);
        frame.render_widget(bars, row[1]);
        frame.render_widget(right_haft_circle, row[2]);
    }

    /// 以 `dyn Any` 的形式返回对组件的引用。
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    /// 以 `dyn Any` 的形式返回对组件的可变引用。
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
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

