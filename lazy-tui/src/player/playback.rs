use lazy_core::{structs::TuiStyle, traits::HasTuiStyle};
use lazy_macro::DeriveHasTuiStyle;
use ratatui::{
    Frame,
    layout::{Alignment, Rect},
    widgets::Paragraph,
};

use crate::traits::RenderTui;

/// 播放状态枚举
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
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

#[cfg(test)]
mod tests {
    use super::*;
    use ratatui::{Terminal, backend::TestBackend};

    #[test]
    fn test_playback_state_default() {
        assert_eq!(PlaybackState::default(), PlaybackState::Stopped);
    }

    #[test]
    fn test_playback_tui_default() {
        let pbt_tui = PlaybackTui::default();
        assert_eq!(pbt_tui.state(), PlaybackState::Stopped);
        assert_eq!(pbt_tui.tui_alignment(), Alignment::Left);
    }

    #[test]
    fn test_playback_tui_set_state() {
        let mut pbt_tui = PlaybackTui::default();
        pbt_tui.set_playback_state(PlaybackState::Playing);
        assert_eq!(pbt_tui.state(), PlaybackState::Playing);

        pbt_tui.set_playback_state(PlaybackState::Paused);
        assert_eq!(pbt_tui.state(), PlaybackState::Paused);
    }

    #[test]
    fn test_playback_tui_get_playback_icon() {
        let mut pbt_tui = PlaybackTui::default();

        pbt_tui.set_playback_state(PlaybackState::Playing);
        assert_eq!(pbt_tui.get_playback_icon(), PlaybackTui::PLAYBACK_ICON[0]);

        pbt_tui.set_playback_state(PlaybackState::Paused);
        assert_eq!(pbt_tui.get_playback_icon(), PlaybackTui::PLAYBACK_ICON[1]);

        pbt_tui.set_playback_state(PlaybackState::Stopped);
        assert_eq!(pbt_tui.get_playback_icon(), PlaybackTui::PLAYBACK_ICON[2]);
    }

    #[test]
    fn test_playback_tui_toggle_state() {
        let mut pbt_tui = PlaybackTui::default(); // Starts at Stopped

        pbt_tui.toggle_state();
        assert_eq!(pbt_tui.state(), PlaybackState::Playing);

        pbt_tui.toggle_state();
        assert_eq!(pbt_tui.state(), PlaybackState::Paused);

        pbt_tui.toggle_state();
        assert_eq!(pbt_tui.state(), PlaybackState::Playing);
    }

    #[test]
    fn test_playback_tui_render_smoke_test() {
        let backend = TestBackend::new(100, 30);
        let mut terminal = Terminal::new(backend).unwrap();
        let pbt_tui = PlaybackTui::default();

        terminal
            .draw(|f| {
                pbt_tui.render(f, f.area());
            })
            .unwrap();
    }
}

