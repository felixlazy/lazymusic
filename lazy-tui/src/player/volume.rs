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

        let volume_status = format!("{} {}     {:<3}% ", status_icon, bar_icon, self.volume);

        let volume = Paragraph::new(volume_status)
            .style(self.tui_style())
            .alignment(self.tui_alignment());
        frame.render_widget(volume, rect);
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

    const MAX_VOLUME: u8 = 100;

    /// 根据音量值从数组中选取对应图标
    fn pick_icon<'a>(volume: u8, icons: &'a [&'a str]) -> &'a str {
        let len = icons.len();
        // 根据音量百分比计算索引
        let idx = (volume as usize * (len - 1))
            .div_ceil(Self::MAX_VOLUME as usize)
            .min(len - 1);
        icons[idx]
    }

    /// 直接设置音量值
    pub(crate) fn set_volume(&mut self, volume: u8) {
        self.volume = volume.min(Self::MAX_VOLUME);
    }

    /// 调整音量，可正可负
    pub(crate) fn adjust_volume(&mut self, delta: i8) {
        let new = self.volume as i16 + delta as i16;
        self.volume = new.clamp(0, Self::MAX_VOLUME as i16) as u8;
    }

    /// 获取当前音量值
    pub(crate) fn volume(&self) -> u8 {
        self.volume
    }
}
#[cfg(test)]
mod test {
    use super::*;
    use ratatui::{Terminal, backend::TestBackend};

    #[test]
    fn test_volume_tui() {
        let volume = VolumeTui::default();

        let backend = TestBackend::new(100, 30);
        let mut terminal = Terminal::new(backend).unwrap();

        terminal
            .draw(|f| {
                volume.render(f, f.area());
            })
            .unwrap();
    }
    #[test]
    fn test_volume_tui_adjust_volume() {
        let mut volume = VolumeTui::default(); // 默认音量是 50

        // 1. 测试增加音量
        volume.set_volume(50); // 重置为已知状态
        volume.adjust_volume(10);
        assert_eq!(volume.volume(), 60, "测试音量增加");

        // 2. 测试减少音量
        volume.set_volume(50); // 重置为已知状态
        volume.adjust_volume(-20);
        assert_eq!(volume.volume(), 30, "测试音量减少");

        // 3. 测试在上限处钳位 (Clamping)
        volume.set_volume(95);
        volume.adjust_volume(10);
        assert_eq!(volume.volume(), 100, "音量增加时应在100处被截断");

        // 4. 测试在下限处钳位 (Clamping)
        volume.set_volume(5);
        volume.adjust_volume(-10);
        assert_eq!(volume.volume(), 0, "音量减少时应在0处被截断");
    }

    #[test]
    fn test_volume_tui_get_volume() {
        let mut volume = VolumeTui::default();
        volume.set_volume(50);
        assert_eq!(volume.volume(), 50);
    }

    #[test]
    fn test_volume_tui_set_volume() {
        let mut volume = VolumeTui::default();
        volume.set_volume(80);
        assert_eq!(volume.volume(), 80);
    }

    #[test]
    fn test_pick_icon_logic() {
        // 测试 ICONS_BLOCK (6个图标) 的边界情况
        // 格式: (音量, 预期索引)
        let block_cases = [
            (0, 0), // 0% -> index 0
            (1, 1), // 1% -> index 1
            (20, 1),
            (21, 2), // 21% -> index 2
            (40, 2),
            (41, 3),
            (60, 3),
            (61, 4),
            (80, 4),
            (81, 5), // 81% -> index 5
            (100, 5),
        ];

        for (volume, expected_idx) in block_cases {
            let icon = VolumeTui::pick_icon(volume, &VolumeTui::ICONS_BLOCK);
            assert_eq!(
                icon,
                VolumeTui::ICONS_BLOCK[expected_idx],
                "ICONS_BLOCK: 音量 {} 应该对应索引 {}",
                volume,
                expected_idx
            );
        }

        // 测试 VOLUME_STATUS (3个图标) 的边界情况
        // 逻辑是 (vol * 2).div_ceil(100)，所以边界在 50
        let status_cases = [
            (0, 0), // 0% -> index 0 (静音)
            (1, 1), // 1% -> index 1 (低音量)
            (50, 1),
            (51, 2), // 51% -> index 2 (高音量)
            (100, 2),
        ];

        for (volume, expected_idx) in status_cases {
            let icon = VolumeTui::pick_icon(volume, &VolumeTui::VOLUME_STATUS);
            assert_eq!(
                icon,
                VolumeTui::VOLUME_STATUS[expected_idx],
                "VOLUME_STATUS: 音量 {} 应该对应索引 {}",
                volume,
                expected_idx
            );
        }
    }
}
