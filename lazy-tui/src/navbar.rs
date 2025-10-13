use std::borrow::Cow;

use lazy_core::structs::{BorderStyle, TitleStyle, TuiStyle};
use lazy_core::traits::{HasBorderStyleSetter, HasTuiStyle};
use lazy_macro::DeriveHasTuiStyle;
use ratatui::{
    Frame,
    layout::Rect,
    style::{Color, Modifier, Style, Stylize},
    text::{Line, Span},
    widgets::Paragraph,
};

use crate::traits::TuiEventHandle;
use crate::types::TuiEnent;
use crate::{
    traits::{RenderTui, TuiBlock},
    types::Direction,
};

/// `NavbarItem` 枚举定义了导航栏中所有可能的项目。
///
/// 这个枚举派生了 `Default`, `Clone`, `Copy`, `PartialEq`, `Eq`, 和 `Debug` trait，
/// 以便进行默认值设置、复制、比较和调试打印。
#[derive(Default, Clone, Copy, PartialEq, Eq, Debug)]
pub enum NavbarItem {
    /// 播放队列页
    #[default]
    Queue,
    /// 日志页
    Logs,
    /// 目录页
    Directories,
    /// 艺术家页
    Artists,
    /// 专辑艺术家页
    AlbumArtists,
    /// 专辑页
    Albums,
    /// 播放列表页
    Playlists,
    /// 搜索页
    Search,
}

impl NavbarItem {
    /// 切换到下一个导航项。
    fn next(self) -> Self {
        let variants = Self::VARIANTS;
        let current_index = variants.iter().position(|&item| item == self).unwrap_or(0);
        let next_index = (current_index + 1) % variants.len();
        variants[next_index]
    }

    /// 切换到上一个导航项。
    ///
    /// 这个方法实现了一个循环切换逻辑，当到达第一个导航项时会重新回到最后一个。
    fn prev(self) -> Self {
        let variants = Self::VARIANTS;
        let current_index = variants.iter().position(|&item| item == self).unwrap_or(0);
        let prev_index = (current_index + variants.len() - 1) % variants.len();
        variants[prev_index]
    }

    /// `VARIANTS` 是一个包含所有 `NavbarItem` 枚举变体的静态切片。
    pub(crate) const VARIANTS: &'static [NavbarItem] = &[
        NavbarItem::Queue,
        NavbarItem::Logs,
        NavbarItem::Directories,
        NavbarItem::Artists,
        NavbarItem::AlbumArtists,
        NavbarItem::Albums,
        NavbarItem::Playlists,
        NavbarItem::Search,
    ];
}

/// `Navbar` 结构体代表了 TUI 中的导航栏组件。
#[derive(DeriveHasTuiStyle)]
pub struct NavbarTui {
    /// 导航栏标题样式
    title: TitleStyle,
    /// 导航栏边框样式
    border: BorderStyle,
    /// 导航栏通用样式
    style: TuiStyle,
    /// 选中项的样式
    selected: Style,
    /// 未选中项的样式
    not_selected: Style,
    /// 当前选中的导航项
    selected_item: NavbarItem,
    /// 选中项的图标
    selected_icon: String,
    /// 未选中项的图标
    not_selected_icon: String,
}

impl Default for NavbarTui {
    /// 创建一个默认的 `Navbar` 实例。
    ///
    /// 默认实现中，为选中和未选中的导航项设置了不同的背景色、前景色和文本修饰符，
    /// 以在视觉上区分它们。
    fn default() -> Self {
        Self {
            title: Default::default(),
            border: Default::default(),
            style: Default::default(),
            selected: Style::default()
                .bg(Color::Rgb(130, 170, 255))
                .fg(Color::Rgb(47, 51, 77))
                .add_modifier(Modifier::ITALIC),
            not_selected: Style::default()
                .bg(Color::Rgb(47, 51, 77))
                .fg(Color::Rgb(130, 170, 255))
                .add_modifier(Modifier::ITALIC),
            selected_item: Default::default(),
            selected_icon: "".to_string(),
            not_selected_icon: "".to_string(),
        }
    }
}

impl RenderTui for NavbarTui {
    /// 在给定的 `Frame` 和 `Rect` 中渲染导航栏。
    ///
    /// 这个方法首先获取组件的内部区域，然后渲染边框和标题。
    /// 接着，它构建并渲染一个包含所有导航项的 `Paragraph` 组件。
    fn render(&self, frame: &mut Frame, rect: Rect) {
        // 获取去掉边框的内部区域
        let inner = self.get_inner(rect);
        // 渲染根组件边框和标题
        frame.render_widget(self.to_block().bg(self.border.bg()), rect);

        let widget = Paragraph::new(self.build_navbar_line(inner.width))
            .alignment(self.tui_alignment())
            .add_modifier(self.title.modifier());

        frame.render_widget(widget, inner);
    }

    /// 将 `Navbar` 实例转换为 `&dyn Any`，以便在运行时进行类型转换。
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    /// 将 `Navbar` 实例转换为 `&mut dyn Any`，以便在运行时进行可变类型转换。
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }

    fn as_event(&self) -> Option<&dyn TuiEventHandle> {
        Some(self)
    }

    fn as_event_mut(&mut self) -> Option<&mut dyn TuiEventHandle> {
        Some(self)
    }

    fn as_border_mut(&mut self) -> Option<&mut dyn HasBorderStyleSetter> {
        Some(self)
    }
}

impl TuiEventHandle for NavbarTui {
    /// 处理 TUI 事件，并将其分发给对应的子组件。
    ///
    /// 此方法作为事件处理的中央分发器。它使用 `delegate_to_widget!` 宏
    /// 来匹配不同的事件，并将它们高效地路由到正确的子组件进行处理。
    fn event_handle(&mut self, event: TuiEnent) {
        match event {
            // 当接收到 `Playback` 事件时...
            TuiEnent::Navbar(direction) => {
                self.toggle_navbar(direction);
            }
            TuiEnent::NavbarIcon(selected_icon, not_selected_icon) => {
                self.set_icon(selected_icon, not_selected_icon);
            }
            _ => (),
        }
    }
}
impl NavbarTui {
    /// 构建用于显示在导航栏中的 `Line`。
    ///
    /// 这个方法会遍历所有的 `NavbarItem`，并为每个项目创建一个 `Span`。
    /// 选中的项目会使用 `selected` 样式，而未选中的项目会使用 `not_selected` 样式。
    /// 为了美观，项目之间会用特殊字符 "" 分隔，选中的项目左侧会显示 ""。
    ///
    /// # Arguments
    ///
    /// * `width` - 导航栏可用的总宽度。
    ///
    /// # Returns
    ///
    /// 一个 `Line` 实例，包含了所有格式化后的导航项。
    fn build_navbar_line(&self, width: u16) -> Line<'_> {
        let variants = NavbarItem::VARIANTS;
        if width < variants.len() as u16 {
            return Line::from(vec![]);
        }
        // 为每个项目的文本计算可用宽度: item width = (total_width / num_items) - separator - padding
        let item_width = (width as usize / variants.len()).saturating_sub(3);
        let app_bg = self.style.bg();

        let mut spans = variants
            .iter()
            // `scan` 用于在迭代时跟踪前一个项目的选中状态
            .scan(None, |prev_selected_state, &current_item| {
                let is_selected = current_item == self.selected_item;
                let result = Some((*prev_selected_state, is_selected, current_item));
                *prev_selected_state = Some(is_selected);
                result
            })
            .flat_map(|(prev_was_selected, is_selected, current_item)| {
                let text = format!(
                    " {:^width$} ",
                    format!("{:?}", current_item),
                    width = item_width
                );

                // 根据前一个项目的状态决定左分隔符的样式
                let left_sep = match prev_was_selected {
                    // 第一个项目，连接到应用背景
                    None => {
                        let style = if is_selected {
                            self.selected
                        } else {
                            self.not_selected
                        };
                        Span::styled(
                            &self.selected_icon,
                            Style::default().bg(style.bg.unwrap_or(app_bg)).fg(app_bg),
                        )
                    }
                    // 后续项目，根据与前一项的状态转换决定分隔符
                    Some(prev_was_selected_val) => match (prev_was_selected_val, is_selected) {
                        // not-selected -> selected
                        (false, true) => Span::styled(
                            &self.selected_icon,
                            Style::default()
                                .fg(self.not_selected.bg.unwrap_or_default())
                                .bg(self.selected.bg.unwrap_or_default()),
                        ),
                        // selected -> not-selected
                        (true, false) => Span::styled(
                            &self.selected_icon,
                            Style::default()
                                .fg(self.selected.bg.unwrap_or_default())
                                .bg(self.not_selected.bg.unwrap_or_default()),
                        ),
                        (false, false) => Span::styled(&self.not_selected_icon, self.not_selected),
                        (true, true) => Span::styled(&self.not_selected_icon, self.selected),
                    },
                };

                let text_span = Span::styled(
                    text,
                    if is_selected {
                        self.selected
                    } else {
                        self.not_selected
                    },
                );

                // 为每个项目生成 [分隔符, 文本]
                [left_sep, text_span]
            })
            .collect::<Vec<_>>();

        // --- 最后的右分隔符 ---
        // 最后一个项目需要一个渐变到应用背景的分隔符
        if let Some(last_item) = variants.last() {
            let last_is_selected = *last_item == self.selected_item;
            let last_style = if last_is_selected {
                self.selected
            } else {
                self.not_selected
            };
            let final_sep_style = Style::default()
                .fg(last_style.bg.unwrap_or(app_bg))
                .bg(app_bg);
            spans.push(Span::styled(&self.selected_icon, final_sep_style));
        }

        Line::from(spans)
    }

    /// 根据给定的方向（左或右）切换导航栏的选中项。
    ///
    /// # Arguments
    ///
    /// * `direction` - `Direction::Left` 表示切换到上一个导航项，`Direction::Right` 表示切换到下一个。
    pub(crate) fn toggle_navbar(&mut self, direction: Direction) {
        match direction {
            Direction::Left => self.selected_item = self.selected_item.prev(),
            Direction::Right => self.selected_item = self.selected_item.next(),
        };
    }

    /// 设置导航栏中用于表示选中和未选中状态的图标。
    ///
    /// 此方法允许自定义在导航栏项旁边显示的图标。
    /// 它接受可以转换为 `Cow<'a, str>` 的参数，这意味着您可以传入字符串字面量 (`&'static str`) 或 `String`。
    /// 只有在图标发生变化时，才会进行新的字符串分配。
    ///
    /// # Arguments
    ///
    /// * `selected_icon` - 用于表示当前选中项的图标。
    /// * `not_selected_icon` - 用于在未选中项之间或作为分隔符的图标。
    ///
    /// # Example
    ///
    /// ```
    /// # use lazy_tui::navbar::Navbar;
    /// let mut navbar = Navbar::default();
    /// navbar.set_icon(">>", "•");
    /// ```
    pub fn set_icon<'a, T: Into<Cow<'a, str>>>(&mut self, selected_icon: T, not_selected_icon: T) {
        let icon: Cow<'a, str> = selected_icon.into();
        if self.selected_icon != icon {
            self.selected_icon = icon.into_owned();
        }
        let not_icon: Cow<'a, str> = not_selected_icon.into();
        if self.not_selected_icon != not_icon {
            self.not_selected_icon = not_icon.into_owned();
        }
    }
}
