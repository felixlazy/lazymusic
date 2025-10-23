//! `ArtistTui` 模块，用于在 TUI 中显示当前播放的歌手信息。

// 从 lazy_core 中导入 TuiStyle 结构体和 HasTuiStyle trait
use lazy_core::{structs::TuiStyle, traits::HasTuiStyle};
// 导入宏，用于自动派生 trait
use lazy_macro::DeriveHasTuiStyle;
use std::borrow::Cow;
// 从 ratatui 中导入所需的组件和布局
use ratatui::{
    Frame,
    layout::{Alignment, Rect},
    widgets::Paragraph,
};

// 从当前 crate 中导入 RenderTui trait
use crate::traits::RenderTui;

/// `ArtistTui` 用于在 TUI 中显示当前播放歌手。
#[derive(DeriveHasTuiStyle)] // 自动派生 HasTuiStyle trait，实现 tui_style() 和 tui_alignment() 等方法
pub struct ArtistTui {
    artist: String,  // 当前歌手名称
    style: TuiStyle, // TUI 样式（颜色、对齐方式等）
}

impl Default for ArtistTui {
    /// 创建一个默认的 `ArtistTui` 实例。
    fn default() -> Self {
        // 初始化默认样式
        let mut style = TuiStyle::default();
        // 默认文本居中显示
        style.set_alignment(Alignment::Center);
        Self {
            // 默认歌手名称
            artist: "Not Artist".to_string(),
            style,
        }
    }
}

impl RenderTui for ArtistTui {
    /// 渲染函数，将 `ArtistTui` 显示在指定的矩形区域。
    ///
    /// # Arguments
    ///
    /// * `frame` - `ratatui` 的 `Frame`，用于绘制。
    /// * `rect` - 要渲染的区域。
    fn render(&self, frame: &mut Frame, rect: Rect) {
        // 创建 Paragraph 小部件，用于显示歌手名称
        let widget = Paragraph::new(self.artist())
            // 应用样式（颜色、粗体等）
            .style(self.tui_style())
            // 应用对齐方式
            .alignment(self.tui_alignment());
        // 在 frame 的指定 rect 区域渲染 widget
        frame.render_widget(widget, rect);
    }
}

impl ArtistTui {
    /// 获取当前歌手名称的引用。
    pub(crate) fn artist(&self) -> &str {
        &self.artist
    }

    /// 设置歌手名称。
    ///
    /// 使用 `impl Into<Cow<'a, str>>` 来接受任何可以转换为 `Cow` 的类型，
    /// 例如 `&str` 或 `String`。
    /// 这种方法可以避免在 artist 未更改时不必要的内存分配。
    pub(crate) fn set_artist<'a>(&mut self, artist: impl Into<Cow<'a, str>>) {
        let artist: Cow<str> = artist.into();
        if self.artist != artist {
            self.artist = artist.into_owned();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ratatui::{Terminal, backend::TestBackend};

    #[test]
    fn test_artist_tui_default() {
        let artist_tui = ArtistTui::default();
        assert_eq!(artist_tui.artist(), "Not Artist");
        assert_eq!(artist_tui.tui_alignment(), Alignment::Center);
    }

    #[test]
    fn test_artist_tui_set_artist() {
        let mut artist_tui = ArtistTui::default();

        // Test setting with &str
        artist_tui.set_artist("New Artist");
        assert_eq!(artist_tui.artist(), "New Artist");

        // Test setting with String
        artist_tui.set_artist("Another Artist".to_string());
        assert_eq!(artist_tui.artist(), "Another Artist");

        // Test setting with the same value (should not reallocate if Cow is optimized)
        let initial_ptr = artist_tui.artist().as_ptr();
        artist_tui.set_artist("Another Artist");
        let new_ptr = artist_tui.artist().as_ptr();
        assert_eq!(
            initial_ptr, new_ptr,
            "Setting same artist should not reallocate"
        );
        assert_eq!(artist_tui.artist(), "Another Artist");
    }

    #[test]
    fn test_artist_tui_artist_getter() {
        let mut artist_tui = ArtistTui::default();
        artist_tui.set_artist("Test Getter");
        assert_eq!(artist_tui.artist(), "Test Getter");
    }

    #[test]
    fn test_artist_tui_render_smoke_test() {
        let backend = TestBackend::new(100, 30);
        let mut terminal = Terminal::new(backend).unwrap();
        let artist_tui = ArtistTui::default();

        terminal
            .draw(|f| {
                artist_tui.render(f, f.area());
            })
            .unwrap();
    }
}

