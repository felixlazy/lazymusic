use ratatui::{
    Frame,
    layout::{Alignment, Rect},
    style::{Color, Style},
    widgets::{Block, Borders},
};

/// Trait: 可绘制的 TUI 组件
/// 所有可在终端渲染的组件都应该实现这个 trait
pub trait DrawTui: TuiBorder + TuiTitle {
    /// 绘制组件到指定的 frame 区域
    /// - `frame`：ratatui 的 Frame，用于渲染组件
    /// - `rect`：组件占用的区域
    fn draw(&mut self, frame: &mut Frame, rect: Rect) {
        let block = Block::default()
            .borders(self.borders())
            .border_style(self.border_style())
            .title(self.title().unwrap_or(""))
            .title_style(self.title_style())
            .title_alignment(self.title_alignment());

        frame.render_widget(block, rect);
    }
}

/// Trait: 可设置边框的组件
pub trait TuiBorder {
    /// 启用或禁用边框显示
    fn set_border(&mut self, enable: bool);

    /// 判断边框是否启用
    fn has_border(&self) -> bool;

    /// 设置边框颜色主题
    fn set_border_theme(&mut self, theme: Color);

    /// 获取当前边框颜色主题
    fn border_theme(&self) -> Color;

    /// 获取实际用于渲染的 Borders（ALL / NONE 等）
    fn borders(&self) -> Borders;

    /// 获取用于边框的 Style（例如颜色、粗细等样式）
    fn border_style(&self) -> Style;
}

/// Trait: 可设置标题的组件
pub trait TuiTitle {
    /// 设置标题文本
    fn set_title(&mut self, title: String);

    /// 启用或禁用标题显示
    fn enable_title(&mut self, enable: bool);

    /// 判断标题是否启用
    fn has_title(&self) -> bool;

    /// 设置标题前景色（文字颜色）
    fn set_title_foreground(&mut self, fg: Color);

    /// 获取标题前景色
    fn title_foreground(&self) -> Color;

    /// 设置标题背景色
    fn set_title_background(&mut self, bg: Color);

    /// 获取标题背景色
    fn title_background(&self) -> Color;

    /// 设置标题是否斜体
    fn set_title_italic(&mut self, enable: bool);

    /// 获取标题是否斜体
    fn title_italic(&self) -> bool;

    /// 获取标题文本，如果标题未启用返回 None
    fn title(&self) -> Option<&str>;

    /// 设置标题对齐方式（左对齐、居中、右对齐）
    fn set_title_alignment(&mut self, alignment: Alignment);

    /// 获取标题对齐方式
    fn title_alignment(&self) -> Alignment;

    /// 获取标题样式（Style），包含颜色、背景色、斜体等
    fn title_style(&self) -> Style;
}
