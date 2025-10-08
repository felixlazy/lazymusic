//! `PlaybackProgressTui` 模块，用于在 TUI 中显示播放进度。

use crate::traits::RenderTui;
use lazy_core::{structs::TuiStyle, traits::HasTuiStyle};
use lazy_macro::DeriveHasTuiStyle;
use ratatui::{
    Frame,
    layout::{Alignment, Rect},
    style::{Color, Stylize},
    text::{Line, Span},
    widgets::Paragraph,
};
use std::time::Duration;

/// `PlaybackProgressTui` 用于在 TUI 中以 `MM:SS/MM:SS` 格式显示播放进度。
#[derive(DeriveHasTuiStyle)]
pub struct PlaybackProgressTui {
    /// 组件的 TUI 样式。
    style: TuiStyle,
    /// 当前播放进度。
    progress: Duration,
    /// 当前曲目的总时长。
    duration: Duration,
}

impl Default for PlaybackProgressTui {
    /// 创建一个默认的 `PlaybackProgressTui` 实例。
    fn default() -> Self {
        // 初始化默认样式
        let mut style = TuiStyle::default();
        // 默认文本左对齐
        style.set_alignment(Alignment::Left);
        Self {
            style,
            progress: Duration::ZERO,
            duration: Duration::ZERO,
        }
    }
}

impl RenderTui for PlaybackProgressTui {
    /// 渲染播放进度。
    fn render(&self, frame: &mut Frame, rect: Rect) {
        // 创建一个由多个样式片段（Span）组成的行（Line）
        let line = Line::from(vec![
            // 第一个片段：当前进度
            Span::raw(Self::format_duration(self.progress)).fg(self.style.fg()),
            // 第二个片段：分隔符
            Span::raw(" / ").fg(Color::White),
            // 第三个片段：总时长
            Span::raw(Self::format_duration(self.duration)).fg(self.style.fg()),
        ]);

        // 将行包装在 Paragraph 小部件中，并设置整体样式和对齐
        let widget = Paragraph::new(line)
            .style(self.tui_style())
            .alignment(self.tui_alignment());

        // 在指定区域渲染小部件
        frame.render_widget(widget, rect);
    }

    /// 将 `self` 转换为 `&dyn Any`，用于动态类型转换。
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    /// 将 `self` 转换为 `&mut dyn Any`，用于动态类型转换。
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}

impl PlaybackProgressTui {
    /// 将 Duration 格式化为 `MM:SS` 字符串。
    fn format_duration(duration: Duration) -> String {
        // 获取总秒数
        let total_seconds = duration.as_secs();
        // 计算分钟数
        let minutes = total_seconds / 60;
        // 计算秒数
        let seconds = total_seconds % 60;
        // 格式化为 MM:SS，不足两位的用 0 填充
        format!("{:0>2}:{:0>2}", minutes, seconds)
    }

    /// 设置当前播放进度。
    pub(crate) fn set_progress(&mut self, progress: Duration) {
        self.progress = progress;
    }

    /// 设置总时长。
    pub(crate) fn set_duration(&mut self, duration: Duration) {
        self.duration = duration;
    }
}
