use ratatui::{
    layout::Alignment,
    style::{Color, Modifier, Style},
    widgets::Borders,
};

/// 提供标题样式信息
pub trait HasTitleStyle {
    /// 获取标题的整体样式（颜色、修饰符等）
    fn title_style(&self) -> Style;

    /// 获取标题的对齐方式
    fn title_alignment(&self) -> Alignment;

    /// 获取标题文本
    fn title_text(&self) -> &str;
}

/// 提供边框样式信息
pub trait HasBorderStyle {
    /// 获取边框样式（颜色）
    fn border_style(&self) -> Style;

    /// 获取边框类型（哪些边可见）
    fn borders(&self) -> Borders;

    /// 是否显示边框
    fn has_border(&self) -> bool;
}

/// 提供整体 TUI 样式信息（背景/前景色）
pub trait HasTuiStyle {
    /// 获取组件背景色
    fn bg(&self) -> Color;

    /// 获取组件前景色
    fn fg(&self) -> Color;
}

/// 修改标题样式
pub trait HasTitleStyleSetter {
    /// 设置标题文本
    fn set_title_text(&mut self, text: String);

    /// 设置标题对齐方式
    fn set_title_alignment(&mut self, alignment: Alignment);

    /// 设置标题修饰符（加粗、斜体等）
    fn set_title_modifier(&mut self, modifier: Modifier);

    /// 设置标题前景色
    fn set_title_fg(&mut self, fg: Color);

    /// 设置标题背景色
    fn set_title_bg(&mut self, bg: Color);
}

/// 修改边框样式
pub trait HasBorderStyleSetter {
    /// 设置边框类型
    fn set_border(&mut self, border: Borders);

    /// 设置边框前景色
    fn set_border_fg(&mut self, fg: Color);

    /// 设置边框背景色
    fn set_border_bg(&mut self, bg: Color);
}

/// 修改整体 TUI 样式
pub trait HasTuiStyleSetter {
    /// 设置组件背景色
    fn set_tui_bg(&mut self, color: Color);

    /// 设置组件前景色
    fn set_tui_fg(&mut self, color: Color);
}
