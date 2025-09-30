use lazy_core::structs::{BorderStyle, TitleStyle, TuiStyle};
use lazy_macro::DeriveHasTuiStyle;
use ratatui::{
    Frame,
    layout::{Constraint, Layout, Rect},
};

use crate::{
    playback::PlaybackTui,
    track::TrackTui,
    traits::{RenderTui, TuiBlock},
    volume::VolumeTui,
};

/// PlayerTui 是整体播放器 TUI 组件
#[derive(DeriveHasTuiStyle, Default)]
pub struct PlayerTui {
    title: TitleStyle,                      // 标题样式
    border: BorderStyle,                    // 边框样式
    style: TuiStyle,                        // 通用样式（颜色、对齐等）
    pub(crate) volume: VolumeTui,           // 音量组件
    pub(crate) playback_state: PlaybackTui, // 播放状态组件（播放/暂停/停止）
    pub(crate) track: TrackTui,             // 当前播放曲目组件
}

impl RenderTui for PlayerTui {
    /// 渲染整个播放器组件
    fn render(&self, frame: &mut Frame, rect: Rect) {
        // 渲染播放器边框和标题
        frame.render_widget(self.to_block(), rect);

        // 获取去掉边框后的内部区域
        let inner = self.get_inner(rect);

        // 使用水平布局将 rect 分成 3 个区域
        // 分别用于播放状态、曲目显示和音量显示
        let [playback_state_rect, track_rect, volume_rect] = Layout::horizontal([
            Constraint::Percentage(20), // 播放状态占 20%
            Constraint::Percentage(60), // 曲目显示占 60%
            Constraint::Percentage(20), // 音量占 20%
        ])
        .areas(inner); // 将原始 rect 分割成子区域

        // 渲染各个子组件
        self.volume.render(frame, volume_rect);
        self.track.render(frame, track_rect);
        self.playback_state.render(frame, playback_state_rect);
    }
}
