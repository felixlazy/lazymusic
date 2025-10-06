use lazy_core::{structs::TuiStyle, traits::HasTuiStyle};
use lazy_macro::DeriveHasTuiStyle;
use ratatui::{
    Frame,
    layout::{Alignment, Rect},
    widgets::Paragraph,
};

use crate::traits::RenderTui;

/// TrackTui 用于在 TUI 中显示当前播放曲目
#[derive(DeriveHasTuiStyle)] // 自动派生 HasTuiStyle trait，实现 tui_style() 和 tui_alignment() 等方法
pub struct TrackTui {
    track: String,   // 当前曲目名称
    style: TuiStyle, // TUI 样式（颜色、对齐方式等）
}

impl Default for TrackTui {
    fn default() -> Self {
        let mut style = TuiStyle::default(); // 初始化默认样式
        style.set_alignment(Alignment::Center); // 默认文本居中显示
        Self {
            track: "lazy music".to_string(), // 默认曲目名称
            style,
        }
    }
}

impl RenderTui for TrackTui {
    /// 渲染函数，将 TrackTui 显示在指定矩形区域
    fn render(&self, frame: &mut Frame, rect: Rect) {
        let widget = Paragraph::new(self.track()) // 创建 Paragraph 小部件，显示曲目名称
            .style(self.tui_style()) // 应用样式（颜色、粗体等）
            .alignment(self.tui_alignment()); // 应用对齐方式
        frame.render_widget(widget, rect); // 在 frame 的指定 rect 区域渲染
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}

impl TrackTui {
    /// 获取当前曲目名称的引用
    pub(crate) fn track(&self) -> &str {
        self.track.as_str()
    }

    /// 设置曲目名称
    pub(crate) fn set_track(&mut self, track: String) {
        self.track = track;
    }
}
