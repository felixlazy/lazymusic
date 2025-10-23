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
            Span::raw(" ").fg(self.style.fg()),
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

#[cfg(test)]
mod tests {
    use super::*;
    use ratatui::{Terminal, backend::TestBackend};
    use std::time::Duration;

    #[test]
    fn test_playback_progress_tui_default() {
        let pppt_tui = PlaybackProgressTui::default();
        assert_eq!(pppt_tui.progress, Duration::ZERO);
        assert_eq!(pppt_tui.duration, Duration::ZERO);
        assert_eq!(pppt_tui.tui_alignment(), Alignment::Left);
    }

    #[test]
    fn test_playback_progress_tui_set_progress() {
        let mut pppt_tui = PlaybackProgressTui::default();
        let test_duration = Duration::from_secs(123);
        pppt_tui.set_progress(test_duration);
        assert_eq!(pppt_tui.progress, test_duration);
    }

    #[test]
    fn test_playback_progress_tui_set_duration() {
        let mut pppt_tui = PlaybackProgressTui::default();
        let test_duration = Duration::from_secs(345);
        pppt_tui.set_duration(test_duration);
        assert_eq!(pppt_tui.duration, test_duration);
    }

    #[test]
    fn test_playback_progress_tui_format_duration() {
        assert_eq!(
            PlaybackProgressTui::format_duration(Duration::ZERO),
            "00:00"
        );
        assert_eq!(
            PlaybackProgressTui::format_duration(Duration::from_secs(5)),
            "00:05"
        );
        assert_eq!(
            PlaybackProgressTui::format_duration(Duration::from_secs(60)),
            "01:00"
        );
        assert_eq!(
            PlaybackProgressTui::format_duration(Duration::from_secs(123)),
            "02:03"
        );
        assert_eq!(
            PlaybackProgressTui::format_duration(Duration::from_secs(3600)),
            "60:00"
        );
        assert_eq!(
            PlaybackProgressTui::format_duration(Duration::from_secs(3661)),
            "61:01"
        );
    }

    #[test]
    fn test_playback_progress_tui_render_smoke_test() {
        let backend = TestBackend::new(100, 30);
        let mut terminal = Terminal::new(backend).unwrap();
        let pppt_tui = PlaybackProgressTui::default();

        terminal
            .draw(|f| {
                pppt_tui.render(f, f.area());
            })
            .unwrap();
    }
}

