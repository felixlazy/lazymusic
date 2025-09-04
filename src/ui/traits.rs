use ratatui::{
    layout,
    style::{Color, Style, Stylize},
    widgets::{Block, Borders},
};

/// 用于控制 UI 组件标题的 trait
pub trait TitleControl {
    /// 设置组件的标题文本
    fn set_title(&mut self, title: impl Into<String>);

    /// 开启或关闭标题显示
    fn set_title_enabled(&mut self, enabled: bool);

    /// 获取当前标题文本
    fn title(&self) -> &str;

    /// 检查标题是否已开启
    fn is_title_enabled(&self) -> bool;
}

/// 用于控制 UI 组件边框的 trait
pub trait BorderControl {
    /// 开启或关闭边框显示
    fn set_border_enabled(&mut self, enabled: bool);

    /// 检查边框是否已开启
    fn is_border_enabled(&self) -> bool;
}

/// 提供 UI 主题颜色的 trait
pub trait UiTheme {
    /// 获取前景色
    fn fg(&self) -> Color;

    /// 获取背景色
    fn bg(&self) -> Color;

    /// 获取蓝色（用于边框或标题）
    fn blue(&self) -> Color;
}

/// 组合 trait，集成标题、边框控制以及主题支持
pub trait UiAppearance: TitleControl + BorderControl + UiTheme {}

/// 可渲染的 UI 组件 trait
///
/// 默认实现会根据 `UiAppearance` 状态渲染边框和标题，
/// — 并调用 `render_content` 渲染内部内容。
pub trait Renderable: UiAppearance {
    /// 渲染整个组件到 frame 上
    ///
    /// 默认实现会：
    /// 1️⃣ 渲染边框和标题
    /// 2️⃣ 调用 `render_content` 渲染内部内容
    ///
    /// # 参数
    /// - `frame`：用于渲染的 `ratatui::Frame`
    fn render(&mut self, frame: &mut ratatui::Frame) {
        let area = frame.area();

        // 创建组件的 Block，包含边框和标题
        let block = Block::default()
            .style(Style::default().bg(self.bg()))
            .borders(if self.is_border_enabled() {
                Borders::ALL
            } else {
                Borders::NONE
            })
            .border_style(Style::default().fg(self.blue()))
            .title(if self.is_title_enabled() {
                self.title()
            } else {
                ""
            })
            .title_style(Style::default().italic().fg(self.blue()))
            .title_alignment(layout::Alignment::Center);

        // 绘制边框和标题
        frame.render_widget(block.clone(), area);

        // 计算内部区域，用于绘制组件内部内容
        let inner_area = if self.is_border_enabled() {
            block.inner(area)
        } else {
            area
        };

        // 调用 render_content 绘制内部内容
        self.render_content(frame, inner_area);
    }

    /// 渲染组件内部内容
    ///
    /// 默认实现为空，子类型可重写来渲染文本或子组件。
    ///
    /// # 参数
    /// - `frame`：用于渲染的 `ratatui::Frame`
    /// - `area`：Block 内部可用区域
    fn render_content(&mut self, frame: &mut ratatui::Frame, area: layout::Rect) {}
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Default)]
    struct TestUi {
        border_enabled: bool,
        title_enabled: bool,
        title: String,
    }

    impl TitleControl for TestUi {
        fn set_title(&mut self, title: impl Into<String>) {
            self.title = title.into();
        }
        fn set_title_enabled(&mut self, enabled: bool) {
            self.title_enabled = enabled;
        }
        fn title(&self) -> &str {
            &self.title
        }
        fn is_title_enabled(&self) -> bool {
            self.title_enabled
        }
    }

    impl BorderControl for TestUi {
        fn set_border_enabled(&mut self, enabled: bool) {
            self.border_enabled = enabled;
        }
        fn is_border_enabled(&self) -> bool {
            self.border_enabled
        }
    }

    impl UiTheme for TestUi {
        fn fg(&self) -> Color {
            Color::White
        }
        fn bg(&self) -> Color {
            Color::Black
        }
        fn blue(&self) -> Color {
            Color::Blue
        }
    }

    impl UiAppearance for TestUi {}
    impl Renderable for TestUi {}

    #[test]
    fn test_title_border_toggle() {
        let mut ui = TestUi::default();

        ui.set_title("Hello");
        ui.set_title_enabled(true);
        ui.set_border_enabled(true);

        assert_eq!(ui.title(), "Hello");
        assert!(ui.is_title_enabled());
        assert!(ui.is_border_enabled());

        ui.set_title_enabled(false);
        ui.set_border_enabled(false);
        assert!(!ui.is_title_enabled());
        assert!(!ui.is_border_enabled());
    }
    use ratatui::{
        Terminal,
        backend::TestBackend,
        layout::{Position, Rect},
        style::Color,
        widgets::{Block, Borders},
    };
    #[test]
    fn test_render_block_with_title() {
        let backend = TestBackend::new(10, 3);
        let mut terminal = Terminal::new(backend).unwrap();

        let block = Block::default().borders(Borders::ALL).title("Test");

        terminal
            .draw(|frame| {
                let area = Rect::new(0, 0, 10, 3);
                frame.render_widget(block, area);
            })
            .unwrap();

        let buffer = terminal.backend().buffer();

        // 使用索引操作符访问特定位置的字符
        let symbol = buffer[Position::new(0, 0)].symbol();
        assert_eq!(symbol, "┌");
    }
}
