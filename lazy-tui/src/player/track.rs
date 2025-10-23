//! `TrackTui` 模块，用于在 TUI 中显示当前播放的曲目信息。

// 从 lazy_core 中导入 TuiStyle 结构体和 HasTuiStyle trait
use lazy_core::{structs::TuiStyle, traits::HasTuiStyle};
// 导入宏，用于自动派生 trait
use lazy_macro::DeriveHasTuiStyle;
use std::borrow::Cow;
// 从 ratatui 中导入所需的组件和布局
use ratatui::{
    Frame,
    layout::{Alignment, Rect},
    widgets::Paragraph,
};

// 从当前 crate 中导入 RenderTui trait
use crate::traits::RenderTui;

/// `TrackTui` 用于在 TUI 中显示当前播放曲目。
#[derive(DeriveHasTuiStyle)] // 自动派生 HasTuiStyle trait，实现 tui_style() 和 tui_alignment() 等方法
pub struct TrackTui {
    track: String,   // 当前曲目名称
    style: TuiStyle, // TUI 样式（颜色、对齐方式等）
}

impl Default for TrackTui {
    /// 创建一个默认的 `TrackTui` 实例。
    fn default() -> Self {
        // 初始化默认样式
        let mut style = TuiStyle::default();
        // 默认文本居中显示
        style.set_alignment(Alignment::Center);
        Self {
            // 默认曲目名称
            track: "Not Song".to_string(),
            style,
        }
    }
}

impl RenderTui for TrackTui {
    /// 渲染函数，将 `TrackTui` 显示在指定的矩形区域。
    ///
    /// # Arguments
    ///
    /// * `frame` - `ratatui` 的 `Frame`，用于绘制。
    /// * `rect` - 要渲染的区域。
    fn render(&self, frame: &mut Frame, rect: Rect) {
        // 创建 Paragraph 小部件，用于显示曲目名称
        let widget = Paragraph::new(format!("󰝚 {}", self.track()))
            // 应用样式（颜色、粗体等）
            .style(self.tui_style())
            // 应用对齐方式
            .alignment(self.tui_alignment());
        // 在 frame 的指定 rect 区域渲染 widget
        frame.render_widget(widget, rect);
    }
}

impl TrackTui {
    /// 获取当前曲目名称的引用。
    pub(crate) fn track(&self) -> &str {
        &self.track
    }

    /// 设置曲目名称。
    ///
    /// 使用 `impl Into<Cow<'a, str>>` 来接受任何可以转换为 `Cow` 的类型，
    /// 例如 `&str` 或 `String`。
    /// 这种方法可以避免在 track 未更改时不必要的内存分配。
    pub(crate) fn set_track<'a>(&mut self, track: impl Into<Cow<'a, str>>) {
        let track: Cow<str> = track.into();
        if self.track != track {
            self.track = track.into_owned();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ratatui::{Terminal, backend::TestBackend};

    #[test]
    fn test_track_tui_default() {
        let track_tui = TrackTui::default();
        assert_eq!(track_tui.track(), "Not Song");
        assert_eq!(track_tui.tui_alignment(), Alignment::Center);
    }

    #[test]
    fn test_track_tui_set_track() {
        let mut track_tui = TrackTui::default();

        // Test setting with &str
        track_tui.set_track("New Song");
        assert_eq!(track_tui.track(), "New Song");

        // Test setting with String
        track_tui.set_track("Another Song".to_string());
        assert_eq!(track_tui.track(), "Another Song");

        // Test setting with the same value (should not reallocate if Cow is optimized)
        let initial_ptr = track_tui.track().as_ptr();
        track_tui.set_track("Another Song");
        let new_ptr = track_tui.track().as_ptr();
        assert_eq!(
            initial_ptr, new_ptr,
            "Setting same track should not reallocate"
        );
        assert_eq!(track_tui.track(), "Another Song");
    }

    #[test]
    fn test_track_tui_track_getter() {
        let mut track_tui = TrackTui::default();
        track_tui.set_track("Test Getter");
        assert_eq!(track_tui.track(), "Test Getter");
    }

    #[test]
    fn test_track_tui_render_smoke_test() {
        let backend = TestBackend::new(100, 30);
        let mut terminal = Terminal::new(backend).unwrap();
        let track_tui = TrackTui::default();

        terminal
            .draw(|f| {
                track_tui.render(f, f.area());
            })
            .unwrap();
    }
}

