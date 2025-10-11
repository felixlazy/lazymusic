//! 播放器模块，包含播放器 TUI 的主要组件。

// 导入子模块
mod artist;
mod playback;
mod playback_mode;
mod playback_progress;
mod track;
mod volume;

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
use crate::{
    delegate_to_widget,
    player::{
        artist::ArtistTui, playback::PlaybackTui, playback_mode::PlaybackModeTui,
        playback_progress::PlaybackProgressTui, track::TrackTui, volume::VolumeTui,
    },
    traits::TuiEnentHandle,
    types::TuiEnent,
};
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
            widgets: vec![
                // 第一行
                Box::new(PlaybackTui::default()),
                Box::new(TrackTui::default()),
                Box::new(VolumeTui::default()),
                // 第二行
                Box::new(PlaybackProgressTui::default()),
                Box::new(ArtistTui::default()),
                Box::new(PlaybackModeTui::default()),
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

        // 创建一个两行的垂直布局
        let rows =
            Layout::vertical([Constraint::Percentage(50), Constraint::Percentage(50)]).split(inner);

        // 为第一行创建一个三列的水平布局
        // | PlaybackTui | TrackTui | VolumeTui |
        let row1_chunks = Layout::horizontal([
            Constraint::Percentage(30), // 播放状态
            Constraint::Min(40),        // 歌名
            Constraint::Min(30),        // 音量
        ])
        .split(rows[0]);

        // 为第二行创建一个三列的水平布局
        // | PlaybackProgressTui | ArtistTui | PlaybackModeTui |
        let row2_chunks = Layout::horizontal([
            Constraint::Percentage(30), // 播放进度
            Constraint::Min(40),        // 歌手
            Constraint::Min(30),        // 播放模式
        ])
        .split(rows[1]);

        let areas_iter = row1_chunks.iter().chain(row2_chunks.iter());

        // 遍历 widgets 和渲染区域迭代器，并进行渲染
        self.widgets
            .iter()
            .zip(areas_iter)
            .for_each(|(widget, &area)| {
                widget.render(frame, area);
            });
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

    fn get_widgets(&self) -> &Vec<Box<dyn RenderTui>> {
        &self.widgets
    }
}

impl TuiEnentHandle for PlayerTui {
    /// 处理 TUI 事件，并将其分发给对应的子组件。
    ///
    /// 此方法作为事件处理的中央分发器。它使用 `delegate_to_widget!` 宏
    /// 来匹配不同的事件，并将它们高效地路由到正确的子组件进行处理。
    fn enent_handle(&mut self, event: TuiEnent) {
        match event {
            // 当接收到 `Playback` 事件时...
            TuiEnent::Playback => {
                // ...将处理逻辑委托给 `PlaybackTui` 子组件。
                // 闭包 `|w: &mut PlaybackTui| w.toggle_state()` 会在找到组件后执行。
                delegate_to_widget!(self, PlaybackTui, |w: &mut PlaybackTui| w.toggle_state());
            }
            TuiEnent::Volumei(delta) => {
                delegate_to_widget!(self, VolumeTui, |w: &mut VolumeTui| w.adjust_volume(delta));
            }
            TuiEnent::PlaybackProgress(duration, progress) => {
                delegate_to_widget!(self, PlaybackProgressTui, |w: &mut PlaybackProgressTui| {
                    w.set_progress(progress);
                    w.set_duration(duration);
                });
            }
            TuiEnent::PlaybackMode => {
                delegate_to_widget!(self, PlaybackModeTui, |w: &mut PlaybackModeTui| w
                    .toggle_mode());
            }
            TuiEnent::Artist(artist) => {
                delegate_to_widget!(self, ArtistTui, |w: &mut ArtistTui| w.set_artist(artist));
            }
            TuiEnent::Track(track) => {
                delegate_to_widget!(self, TrackTui, |w: &mut TrackTui| w.set_track(track));
            }
        }
    }
}
