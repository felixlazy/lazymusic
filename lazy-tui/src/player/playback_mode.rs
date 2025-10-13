//! `PlaybackModeTui` 模块，用于在 TUI 中显示和管理播放模式。

use crate::traits::RenderTui;
// 从 lazy_core 中导入 TuiStyle 结构体和 HasTuiStyle trait
use lazy_core::{structs::TuiStyle, traits::HasTuiStyle};
use lazy_macro::DeriveHasTuiStyle;
use ratatui::{
    Frame,
    layout::{Alignment, Rect},
    style::{Color, Style, Stylize},
    text::{Line, Span},
    widgets::Paragraph,
};

/// 定义了不同的播放模式。
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum PlaybackMode {
    /// **列表循环**: 播放完列表最后一首后，从第一首开始继续播放。
    #[default]
    Repeat,
    /// **随机播放**: 随机播放列表中的曲目。
    Random,
    /// **消费模式**: 播放过的曲目将从列表中移除（或标记为不再播放）。
    Consume,
    /// **单曲循环**: 单独重复播放当前曲目。
    Single,
}

impl PlaybackMode {
    /// 包含所有播放模式的常量数组，用于迭代。
    pub(crate) const VARIANTS: &'static [PlaybackMode] = &[
        PlaybackMode::Repeat,
        PlaybackMode::Random,
        PlaybackMode::Consume,
        PlaybackMode::Single,
    ];

    /// 返回一个包含所有播放模式的静态切片，方便在 UI 中显示。
    pub(crate) fn variants() -> &'static [PlaybackMode] {
        Self::VARIANTS
    }

    /// 切换到下一个播放模式。
    /// 这是一个循环切换，例如 `Repeat` -> `Random` -> `Consume` -> `Single` -> `Repeat`。
    fn next(self) -> Self {
        match self {
            PlaybackMode::Repeat => PlaybackMode::Random,
            PlaybackMode::Random => PlaybackMode::Consume,
            PlaybackMode::Consume => PlaybackMode::Single,
            PlaybackMode::Single => PlaybackMode::Repeat,
        }
    }
}

/// `PlaybackModeTui` 是一个 TUI 组件，用于渲染播放模式列表。
///
/// 它会高亮显示当前的播放模式。
#[derive(DeriveHasTuiStyle)]
pub struct PlaybackModeTui {
    /// 当前激活的播放模式。
    mode: PlaybackMode,
    /// 组件的 TUI 样式。
    style: TuiStyle,
}

impl Default for PlaybackModeTui {
    /// 创建一个默认的 `PlaybackModeTui` 实例。
    fn default() -> Self {
        // 初始化默认样式
        let mut style = TuiStyle::default();
        // 默认文本右对齐
        style.set_alignment(Alignment::Right);
        Self {
            mode: Default::default(),
            style,
        }
    }
}

impl RenderTui for PlaybackModeTui {
    /// 在指定的 `rect` 区域内渲染播放模式列表。
    fn render(&self, frame: &mut Frame, rect: Rect) {
        // 调用辅助函数构建带样式的行
        let line = self.build_mode_line();

        // 创建 Paragraph 小部件并应用对齐方式
        let widget = Paragraph::new(line).alignment(self.tui_alignment());

        frame.render_widget(widget, rect);
    }
}

impl PlaybackModeTui {
    /// 根据当前播放模式，构建一个高亮显示当前模式的 `Line`。
    fn build_mode_line(&self) -> Line<'_> {
        let variants = PlaybackMode::variants();
        let mut spans = Vec::new();
        for (i, mode) in variants.iter().enumerate() {
            // 判断是否是当前激活的模式
            let style = if *mode == self.mode {
                // 激活模式使用默认前景色（通常更亮）
                self.tui_style()
            } else {
                // 非激活模式使用灰色，以示区别
                Style::default().fg(Color::Gray)
            };

            spans.push(Span::styled(format!("{:?}", mode), style));

            // 在模式之间添加分隔符，除了最后一个
            if i < variants.len() - 1 {
                spans.push(Span::raw(" | ").fg(Color::White));
            }
        }
        Line::from(spans)
    }

    /// 切换到下一个播放模式。
    pub(crate) fn toggle_mode(&mut self) {
        self.mode = self.mode.next();
    }

    /// 设置指定的播放模式。
    pub(crate) fn set_mode(&mut self, mode: PlaybackMode) {
        self.mode = mode;
    }

    /// 获取当前的播放模式。
    pub(crate) fn mode(&self) -> PlaybackMode {
        self.mode
    }
}
