use lazy_macro::Accessor;
use ratatui::{
    layout::Alignment,
    style::{Color, Modifier},
    widgets::Borders,
};

//////////////////////////////
/// 标题样式
//////////////////////////////
#[derive(Accessor)]
pub struct TitleStyle {
    /// 标题文本内容
    text: String,

    /// 标题对齐方式（居中/左/右），自动生成 getter/setter 并实现 Copy
    #[Accessor(Copy)]
    alignment: Alignment,

    /// 标题修饰符（如加粗、斜体），自动生成 getter/setter 并实现 Copy
    #[Accessor(Copy)]
    modifier: Modifier,

    /// 标题前景色，自动生成 getter/setter 并实现 Copy
    #[Accessor(Copy)]
    fg: Color,

    /// 标题背景色，自动生成 getter/setter 并实现 Copy
    #[Accessor(Copy)]
    bg: Color,
}

impl Default for TitleStyle {
    fn default() -> Self {
        Self {
            text: Default::default(),
            alignment: Alignment::Center,
            modifier: Modifier::ITALIC,
            fg: Color::Rgb(130, 170, 255), // 默认前景色 #82aaff
            bg: Color::Rgb(34, 36, 54),    // 默认背景色 #222436
        }
    }
}

//////////////////////////////
/// 边框样式
//////////////////////////////
#[derive(Accessor)]
pub struct BorderStyle {
    /// 边框类型（NONE/ALL/上下左右），自动生成 getter/setter 并实现 Copy
    #[Accessor(Copy)]
    border: Borders,

    /// 边框前景色，自动生成 getter/setter 并实现 Copy
    #[Accessor(Copy)]
    fg: Color,

    /// 边框背景色，自动生成 getter/setter 并实现 Copy
    #[Accessor(Copy)]
    bg: Color,
}

impl Default for BorderStyle {
    fn default() -> Self {
        Self {
            border: Borders::NONE,
            fg: Color::Rgb(130, 170, 255), // 默认前景色 #82aaff
            bg: Color::Rgb(34, 36, 54),    // 默认背景色 #222436
        }
    }
}

//////////////////////////////
/// 整体 TUI 样式（背景/前景色）
//////////////////////////////
#[derive(Accessor)]
pub struct TuiStyle {
    /// 前景色，自动生成 getter/setter 并实现 Copy
    #[Accessor(Copy)]
    fg: Color,

    /// 背景色，自动生成 getter/setter 并实现 Copy
    #[Accessor(Copy)]
    bg: Color,
}

impl Default for TuiStyle {
    fn default() -> Self {
        Self {
            fg: Color::Rgb(130, 170, 255), // 默认前景色 #82aaff
            bg: Color::Rgb(34, 36, 54),    // 默认背景色 #222436
        }
    }
}
