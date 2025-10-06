mod playback;
mod track;
mod volume;
use lazy_core::structs::{BorderStyle, TitleStyle, TuiStyle};
use lazy_macro::DeriveHasTuiStyle;
use ratatui::{
    Frame,
    layout::{Constraint, Layout, Rect},
};

use crate::player::{playback::PlaybackTui, track::TrackTui, volume::VolumeTui};
use crate::traits::{HasWidgets, RenderTui, TuiBlock};

/// PlayerTui 是整体播放器 TUI 组件
#[derive(DeriveHasTuiStyle)]
pub struct PlayerTui {
    title: TitleStyle,                // 标题样式
    border: BorderStyle,              // 边框样式
    style: TuiStyle,                  // 通用样式（颜色、对齐等）
    widgets: Vec<Box<dyn RenderTui>>, // 所有实现了 RenderTui 的组件
}

impl Default for PlayerTui {
    fn default() -> Self {
        Self {
            title: Default::default(),
            border: Default::default(),
            style: Default::default(),
            widgets: vec![
                Box::new(PlaybackTui::default()),
                Box::new(TrackTui::default()),
                Box::new(VolumeTui::default()),
            ],
        }
    }
}

impl RenderTui for PlayerTui {
    /// 渲染整个播放器组件
    fn render(&self, frame: &mut Frame, rect: Rect) {
        // 渲染播放器边框和标题
        frame.render_widget(self.to_block(), rect);

        // 获取去掉边框后的内部区域
        let inner = self.get_inner(rect);

        // 使用水平布局将 rect 分成 3 个区域
        // 注意：这里的布局和 widgets vector 中的组件顺序是对应的
        let chunks = Layout::horizontal([
            Constraint::Percentage(20), // 对应 PlaybackTui
            Constraint::Percentage(60), // 对应 TrackTui
            Constraint::Percentage(20), // 对应 VolumeTui
        ])
        .split(inner); // 将原始 rect 分割成子区域

        // 渲染各个子组件
        for (i, widget) in self.widgets.iter().enumerate() {
            if let Some(chunk) = chunks.get(i) {
                widget.render(frame, *chunk);
            }
        }
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}

impl HasWidgets for PlayerTui {
    fn get_widgets_mut(&mut self) -> &mut Vec<Box<dyn RenderTui>> {
        &mut self.widgets
    }
}

impl PlayerTui {
    pub fn adjust_volume(&mut self, delta: i8) {
        if let Some(volume_tui) = self.get_widget_mut::<VolumeTui>() {
            volume_tui.adjust_volume(delta);
        }
    }

    pub fn toggle_state(&mut self) {
        if let Some(playback_tui) = self.get_widget_mut::<PlaybackTui>() {
            playback_tui.toggle_state();
        }
    }

    pub fn set_track(&mut self, track: String) {
        if let Some(track_tui) = self.get_widget_mut::<TrackTui>() {
            track_tui.set_track(track);
        }
    }
}
