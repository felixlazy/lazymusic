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

        let left_haft_circle =
            Paragraph::new("")
                .alignment(Alignment::Right)
                .fg(if self.ratio == 0.0 {
                    self.style.bg()
                } else {
                    self.style.fg()
                });
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

#[cfg(test)]
mod tests {
    use super::*;
    use ratatui::{Terminal, backend::TestBackend};

    #[test]
    fn test_progress_tui_default() {
        let progress = ProgressTui::default();
        assert_eq!(progress.ratio(), 0.0);
        assert_eq!(progress.style.bg(), Color::Rgb(47, 51, 77));
        // Add more assertions for default styles if needed
    }

    #[test]
    fn test_progress_tui_set_ratio() {
        let mut progress = ProgressTui::default();

        progress.set_ratio(0.5);
        assert_eq!(progress.ratio(), 0.5);

        progress.set_ratio(1.5);
        assert_eq!(progress.ratio(), 1.0, "Ratio should be clamped at 1.0");

        progress.set_ratio(-0.5);
        assert_eq!(progress.ratio(), 0.0, "Ratio should be clamped at 0.0");
    }

    #[test]
    fn test_progress_tui_reset_ratio() {
        let mut progress = ProgressTui::default();
        progress.set_ratio(0.7);
        progress.reset_ratio();
        assert_eq!(progress.ratio(), 0.0);
    }

    #[test]
    fn test_progress_tui_render_smoke_test() {
        let backend = TestBackend::new(100, 30);
        let mut terminal = Terminal::new(backend).unwrap();
        let progress = ProgressTui::default();

        terminal
            .draw(|f| {
                progress.render(f, f.area());
            })
            .unwrap();
    }

    #[test]
    fn test_progress_tui_render_left_half_circle_color() {
        let backend = TestBackend::new(100, 30);
        let mut terminal = Terminal::new(backend).unwrap();
        let mut progress = ProgressTui::default();

        // Test when ratio is 0.0
        progress.set_ratio(0.0);
        terminal
            .draw(|f| {
                progress.render(f, f.area());
            })
            .unwrap();
        let buffer = terminal.backend().buffer();
        // The left half-circle is in row[0], which is 2 chars wide.
        // The rect for row[0] starts at x=0, y=0 (relative to inner area)
        // The inner area starts at (1,1) if there's a border.
        // row[0] will be at (1,1) with width 2.
        // The character '' is at (1,1).
        let cell = buffer.cell((1, 1));
        assert_eq!(
            cell.unwrap().fg,
            progress.style.bg(),
            "Left half-circle color should be background color when ratio is 0.0"
        );

        // Test when ratio is > 0.0
        progress.set_ratio(0.5);
        terminal
            .draw(|f| {
                progress.render(f, f.area());
            })
            .unwrap();
        let buffer = terminal.backend().buffer();
        let cell = buffer.cell((1, 1));
        assert_eq!(
            cell.unwrap().fg,
            progress.style.fg(),
            "Left half-circle color should be foreground color when ratio is > 0.0"
        );
    }
}

