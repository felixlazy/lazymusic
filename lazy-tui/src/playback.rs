use lazy_core::{structs::TuiStyle, traits::HasTuiStyle};
use lazy_macro::DeriveHasTuiStyle;
use ratatui::{
    Frame,
    layout::{Alignment, Rect},
    widgets::Paragraph,
};

use crate::traits::RenderTui;

/// 播放状态枚举
#[derive(Clone, Copy, Debug, Default)]
pub enum PlaybackState {
    Playing,
    Paused,
    #[default]
    Stopped,
}

/// 播放状态 TUI 组件
#[derive(DeriveHasTuiStyle)]
pub struct PlaybackTui {
    style: TuiStyle,
    state: PlaybackState,
}
impl Default for PlaybackTui {
    fn default() -> Self {
        let mut style = TuiStyle::default();
        style.set_alignment(Alignment::Left);
        Self {
            style,
            state: Default::default(),
        }
    }
}

impl RenderTui for PlaybackTui {
    fn render(&self, frame: &mut Frame, rect: Rect) {
        let text = self.get_playback_icon();
        let widget = Paragraph::new(text)
            .style(self.tui_style())
            .alignment(self.tui_alignment());
        frame.render_widget(widget, rect);
    }
}

impl PlaybackTui {
    /// 播放状态对应图标数组
    const PLAYBACK_ICON: [&str; 3] = ["   Playing", "   Paused", "   Stopped"];

    /// 设置播放状态
    pub(crate) fn set_playback_state(&mut self, state: PlaybackState) {
        self.state = state;
    }

    /// 获取当前播放状态
    pub(crate) fn state(&self) -> PlaybackState {
        self.state
    }

    /// 自动根据状态枚举索引返回对应图标
    fn get_playback_icon(&self) -> &str {
        Self::PLAYBACK_ICON[self.state as usize]
    }

    /// 切换状态（循环切换）
    pub(crate) fn toggle_state(&mut self) {
        self.state = match self.state {
            PlaybackState::Playing => PlaybackState::Paused, // 播放中 → 暂停
            PlaybackState::Paused | PlaybackState::Stopped => PlaybackState::Playing, // 暂停或停止 → 播放
        };
    }
}
