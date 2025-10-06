use lazy_core::{structs::TuiStyle, traits::HasTuiStyle};
use lazy_macro::DeriveHasTuiStyle;
use ratatui::{
    Frame,
    layout::{Alignment, Rect},
    widgets::Paragraph,
};

use crate::traits::RenderTui;

/// 用于在 TUI 界面显示音量信息的组件
#[derive(DeriveHasTuiStyle)]
pub struct VolumeTui {
    /// 当前音量值，范围 0..=100
    volume: u8,
    /// TUI 样式
    style: TuiStyle,
}

impl Default for VolumeTui {
    fn default() -> Self {
        let mut style = TuiStyle::default();
        style.set_alignment(Alignment::Right);
        Self { style, volume: 50 }
    }
}

impl RenderTui for VolumeTui {
    fn render(&self, frame: &mut Frame, rect: Rect) {
        // 根据当前音量自动选择图标
        let status_icon = Self::pick_icon(self.volume, &Self::VOLUME_STATUS);
        let bar_icon = Self::pick_icon(self.volume, &Self::ICONS_BLOCK);

        let volume_status = format!("{} {} {:<3}% ", status_icon, bar_icon, self.volume);

        let volume = Paragraph::new(volume_status)
            .style(self.tui_style())
            .alignment(self.tui_alignment());
        frame.render_widget(volume, rect);
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}

impl VolumeTui {
    /// 音量条显示，从空到满（0..100）
    const ICONS_BLOCK: [&str; 6] = [
        "         ", // 0%
        "▁        ", // 1-20%
        "▁ ▃      ", // 21-40%
        "▁ ▃ ▅    ", // 41-60%
        "▁ ▃ ▅ ▇  ", // 61-80%
        "▁ ▃ ▅ ▇ █", // 81-100%
    ];

    /// 音量状态图标（静音 / 小音量 / 大音量）
    const VOLUME_STATUS: [&str; 3] = [" ", " ", " "];

    /// 根据音量值从数组中选取对应图标
    fn pick_icon<'a>(volume: u8, icons: &'a [&'a str]) -> &'a str {
        let len = icons.len();
        if volume == 0 {
            return icons[0];
        }
        // 根据音量百分比计算索引
        let idx = (volume as usize * (len - 1)).div_ceil(100).min(len - 1);
        icons[idx]
    }

    /// 直接设置音量值
    pub(crate) fn set_volume(&mut self, volume: u8) {
        self.volume = volume.min(100);
    }

    /// 调整音量，可正可负
    pub(crate) fn adjust_volume(&mut self, delta: i8) {
        let new = self.volume as i16 + delta as i16;
        self.volume = new.clamp(0, 100) as u8;
    }

    /// 获取当前音量值
    pub(crate) fn volume(&self) -> u8 {
        self.volume
    }
}
