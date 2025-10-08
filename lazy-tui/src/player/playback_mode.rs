//! `PlaybackModeTui` 模块，用于在 TUI 中显示播放模式。

use ratatui::{Frame, layout::Rect};
use crate::traits::RenderTui;

/// `PlaybackModeTui` 用于在 TUI 中显示播放模式（占位）。
#[derive(Default)]
pub struct PlaybackModeTui;

impl RenderTui for PlaybackModeTui {
    /// 渲染函数（当前为空）。
    fn render(&self, _frame: &mut Frame, _rect: Rect) {
        // 之后会实现
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}
