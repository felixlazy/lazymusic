//! 播放器模块，包含播放器 TUI 的主要组件。

// 导入子模块
mod artist;
mod playback;
mod track;
mod volume;

use std::borrow::Cow;

// 从 lazy_core 中导入结构体
use lazy_core::structs::{BorderStyle, TitleStyle, TuiStyle};
// 导入宏
use lazy_macro::DeriveHasTuiStyle;
// 从 ratatui 中导入所需的组件和布局
use ratatui::{
    Frame,
    layout::{Constraint, Layout, Rect},
};

// 从当前 crate 的子模块中导入 TUI 组件
use crate::player::{artist::ArtistTui, playback::PlaybackTui, track::TrackTui, volume::VolumeTui};
// 从当前 crate 中导入 traits
use crate::traits::{HasWidgets, RenderTui, TuiBlock};

/// `PlayerTui` 是整体播放器 TUI 组件。
///
/// 它包含了播放器的所有子组件，并负责将它们渲染到屏幕上。
#[derive(DeriveHasTuiStyle)]
pub struct PlayerTui {
    title: TitleStyle,                // 标题样式
    border: BorderStyle,              // 边框样式
    style: TuiStyle,                  // 通用样式（颜色、对齐等）
    widgets: Vec<Box<dyn RenderTui>>, // 所有实现了 `RenderTui` 的子组件
}

impl Default for PlayerTui {
    /// 创建一个默认的 `PlayerTui` 实例。
    fn default() -> Self {
        Self {
            title: Default::default(),
            border: Default::default(),
            style: Default::default(),
            // 初始化时，将所有子组件添加到 `widgets` 向量中
            widgets: vec![
                Box::new(PlaybackTui::default()),
                Box::new(TrackTui::default()),
                Box::new(ArtistTui::default()),
                Box::new(VolumeTui::default()),
            ],
        }
    }
}

impl RenderTui for PlayerTui {
    /// 渲染整个播放器组件。
    ///
    /// # Arguments
    ///
    /// * `frame` - `ratatui` 的 `Frame`，用于绘制。
    /// * `rect` - 要渲染的区域。
    fn render(&self, frame: &mut Frame, rect: Rect) {
        // 渲染播放器边框和标题
        frame.render_widget(self.to_block(), rect);

        // 获取去掉边框后的内部区域
        let inner = self.get_inner(rect);

        // 使用水平布局将 `inner` 分成 4 个区域
        // 注意：这里的布局和 `widgets` 向量中的组件顺序是对应的
        let chunks = Layout::horizontal([
            Constraint::Percentage(20), // 对应 `PlaybackTui`
            Constraint::Percentage(40), // 对应 `TrackTui`
            Constraint::Percentage(20), // 对应 `ArtistTui`
            Constraint::Percentage(20), // 对应 `VolumeTui`
        ])
        .split(inner); // 将 `inner` 分割成子区域

        // 遍历并渲染各个子组件
        for (i, widget) in self.widgets.iter().enumerate() {
            if let Some(chunk) = chunks.get(i) {
                widget.render(frame, *chunk);
            }
        }
    }

    /// 将 `self` 转换为 `&dyn Any`。
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    /// 将 `self` 转换为 `&mut dyn Any`。
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}

impl HasWidgets for PlayerTui {
    /// 获取对 `widgets` 向量的可变引用。
    fn get_widgets_mut(&mut self) -> &mut Vec<Box<dyn RenderTui>> {
        &mut self.widgets
    }
}

impl PlayerTui {
    /// 调整音量。
    ///
    /// # Arguments
    ///
    /// * `delta` - 音量变化的增量（正数表示增加，负数表示减少）。
    pub fn adjust_volume(&mut self, delta: i8) {
        if let Some(volume_tui) = self.get_widget_mut::<VolumeTui>() {
            volume_tui.adjust_volume(delta);
        }
    }

    /// 切换播放状态（播放/暂停）。
    pub fn toggle_state(&mut self) {
        if let Some(playback_tui) = self.get_widget_mut::<PlaybackTui>() {
            playback_tui.toggle_state();
        }
    }

    /// 设置当前播放的曲目。
    ///
    /// # Arguments
    ///
    /// * `track` - 曲目名称。
    pub fn set_track<'a>(&mut self, track: impl Into<Cow<'a, str>>) {
        if let Some(track_tui) = self.get_widget_mut::<TrackTui>() {
            track_tui.set_track(track);
        }
    }

    /// 设置当前播放的歌手。
    ///
    /// # Arguments
    ///
    /// * `artist` - 歌手名称。
    pub fn set_artist<'a>(&mut self, artist: impl Into<Cow<'a, str>>) {
        if let Some(artist_tui) = self.get_widget_mut::<ArtistTui>() {
            artist_tui.set_artist(artist);
        }
    }
}
