use ratatui::style::Color;

pub struct TuiTheme {
    pub bg: Color,           // 背景主色
    pub bg_dark: Color,      // 背景深色
    pub bg_dark1: Color,     // 背景更深色
    pub bg_highlight: Color, // 背景高亮区域

    pub fg: Color,        // 前景文字色
    pub fg_dark: Color,   // 前景较暗
    pub fg_gutter: Color, // Gutter 区域（侧边栏）

    pub comment: Color, // 注释文字色

    pub blue: Color, // 主蓝色
    pub blue0: Color,
    pub blue1: Color,
    pub blue2: Color,
    pub blue5: Color,
    pub blue6: Color,
    pub blue7: Color,

    pub cyan: Color,  // 青色
    pub green: Color, // 绿色
    pub green1: Color,
    pub green2: Color,

    pub magenta: Color, // 品红
    pub magenta2: Color,
    pub orange: Color, // 橙色
    pub purple: Color, // 紫色

    pub red: Color, // 红色
    pub red1: Color,

    pub teal: Color,   // 蓝绿色
    pub yellow: Color, // 黄色

    pub dark3: Color, // 深色变体
    pub dark5: Color,
    pub terminal_black: Color, // 模拟终端黑色
}

impl Default for TuiTheme {
    fn default() -> Self {
        Self {
            bg: Color::Rgb(34, 36, 54),           // #222436
            bg_dark: Color::Rgb(30, 32, 48),      // #1e2030
            bg_dark1: Color::Rgb(25, 27, 41),     // #191B29
            bg_highlight: Color::Rgb(47, 51, 77), // #2f334d

            fg: Color::Rgb(200, 211, 245),      // #c8d3f5
            fg_dark: Color::Rgb(130, 139, 184), // #828bb8
            fg_gutter: Color::Rgb(59, 66, 97),  // #3b4261

            comment: Color::Rgb(99, 109, 166), // #636da6

            blue: Color::Rgb(130, 170, 255),  // #82aaff
            blue0: Color::Rgb(62, 104, 215),  // #3e68d7
            blue1: Color::Rgb(101, 188, 255), // #65bcff
            blue2: Color::Rgb(13, 185, 215),  // #0db9d7
            blue5: Color::Rgb(137, 221, 255), // #89ddff
            blue6: Color::Rgb(180, 249, 248), // #b4f9f8
            blue7: Color::Rgb(57, 75, 112),   // #394b70

            cyan: Color::Rgb(134, 225, 252),  // #86e1fc
            green: Color::Rgb(195, 232, 141), // #c3e88d
            green1: Color::Rgb(79, 214, 190), // #4fd6be
            green2: Color::Rgb(65, 166, 181), // #41a6b5

            magenta: Color::Rgb(192, 153, 255), // #c099ff
            magenta2: Color::Rgb(255, 0, 124),  // #ff007c
            orange: Color::Rgb(255, 150, 108),  // #ff966c
            purple: Color::Rgb(252, 167, 234),  // #fca7ea

            red: Color::Rgb(255, 117, 127), // #ff757f
            red1: Color::Rgb(197, 59, 83),  // #c53b53

            teal: Color::Rgb(79, 214, 190),    // #4fd6be
            yellow: Color::Rgb(255, 199, 119), // #ffc777

            dark3: Color::Rgb(84, 92, 126),          // #545c7e
            dark5: Color::Rgb(115, 122, 162),        // #737aa2
            terminal_black: Color::Rgb(68, 74, 115), // #444a73
        }
    }
}
