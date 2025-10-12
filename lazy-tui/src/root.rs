//! 根 TUI 组件模块，定义了整个 TUI 的根容器。

// 导入宏
use lazy_macro::DeriveHasTuiStyle;
// 从 ratatui 中导入所需的组件和布局
use ratatui::{
    Frame,
    layout::{Constraint, Layout, Rect},
};

// 从 lazy_core 中导入所需的结构体和 traits
use lazy_core::{
    structs::{BorderStyle, TitleStyle, TuiStyle},
    traits::HasBorderStyleSetter,
};

// 从当前 crate 中导入所需的组件和 traits
use crate::{
    delegate_to_widget,
    navbar::NavbarTui,
    player::PlayerTui,
    progress::ProgressTui,
    traits::{HasWidgets, RenderTui, TuiBlock, TuiEnentHandle},
    types::TuiEnent, // RenderTui 用于渲染，TuiBlock 用于生成边框块
};

/// `RootTui` 是根 TUI 组件，作为整个播放器界面的容器。
///
/// 它包含了 `PlayerTui` 和 `ProgressTui`，并负责渲染整个界面。
#[derive(DeriveHasTuiStyle)]
pub struct RootTui {
    title: TitleStyle,                // 根组件标题样式
    border: BorderStyle,              // 根组件边框样式
    style: TuiStyle,                  // 根组件通用样式（颜色、对齐等）
    widgets: Vec<Box<dyn RenderTui>>, // 包含的子组件
}

impl Default for RootTui {
    /// 创建一个默认的 `RootTui` 实例。
    fn default() -> Self {
        Self {
            title: Default::default(),
            border: Default::default(),
            style: Default::default(),
            // 初始化时，将 `PlayerTui` 和 `ProgressTui` 作为子组件
            widgets: vec![
                Box::new(PlayerTui::default()),
                Box::new(NavbarTui::default()),
                Box::new(ProgressTui::default()),
            ],
        }
    }
}

impl HasWidgets for RootTui {
    /// 获取对 `widgets` 向量的可变引用。
    fn get_widgets_mut(&mut self) -> &mut Vec<Box<dyn RenderTui>> {
        &mut self.widgets
    }

    /// 获取对 `widgets` 向量的不可变引用。
    fn get_widgets(&self) -> &Vec<Box<dyn RenderTui>> {
        &self.widgets
    }
}

impl RootTui {
    /// 切换当前组件及其子组件的边框显示状态。
    pub fn toggle_all_border(&mut self) {
        self.toggle_border();

        // 宏，用于切换指定类型子组件的边框
        macro_rules! toggle_widget_border {
            ($widget_type:ty) => {
                if let Some(widget) = self.get_widget_mut::<$widget_type>() {
                    widget.toggle_border();
                }
            };
        }

        // 切换 PlayerTui 和 ProgressTui 的边框
        toggle_widget_border!(PlayerTui);
        toggle_widget_border!(ProgressTui);
    }

    /// 检查指定类型的子组件是否具有边框。
    ///
    /// # 类型参数
    ///
    /// * `T`: 需要检查的子组件类型，必须实现 `TuiBlock` trait。
    fn has_widgets_border<T>(&self) -> bool
    where
        T: TuiBlock + 'static,
    {
        if let Some(widget) = self.get_widget::<T>() {
            widget.has_border()
        } else {
            false
        }
    }

    /// 更新进度条组件的进度。
    ///
    /// # Arguments
    ///
    /// * `progress`: 进度值，范围从 0.0 到 1.0。
    pub fn update_progress(&mut self, progress: f64) {
        if let Some(player) = self.get_widget_mut::<ProgressTui>() {
            player.set_ratio(progress);
        }
    }
}

impl RenderTui for RootTui {
    /// 渲染整个根组件。
    ///
    /// # Arguments
    ///
    /// * `frame` - `ratatui` 的 `Frame`，用于绘制。
    /// * `rect` - 要渲染的区域。
    fn render(&self, frame: &mut Frame, rect: Rect) {
        // 获取去掉边框的内部区域
        let inner = self.get_inner(rect);
        // 渲染根组件边框和标题
        frame.render_widget(self.to_block(), rect);

        // 定义垂直布局
        let chunks = Layout::vertical([
            Constraint::Min(4), // 播放器最小高度
            Constraint::Min(3),
            Constraint::Fill(20), // 填充剩余空间
            // 进度条高度，根据是否有边框动态调整
            Constraint::Max(1 + 2 * u16::from(self.has_widgets_border::<ProgressTui>())),
        ])
        .split(inner);

        // 遍历并渲染所有子组件
        self.widgets.iter().enumerate().for_each(|(i, f)| {
            f.render(frame, chunks[i]);
        });
    }

    /// 将 `self` 转换为 `&dyn Any`，用于类型转换。
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    /// 将 `self` 转换为 `&mut dyn Any`，用于可变类型转换。
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}

impl TuiEnentHandle for RootTui {
    /// 处理 TUI 事件。
    ///
    /// 根据事件类型，将事件委托给相应的子组件处理。
    ///
    /// # Arguments
    ///
    /// * `event`: TUI 事件。
    fn enent_handle(&mut self, event: TuiEnent) {
        match event {
            // 匹配与播放器相关的事件
            TuiEnent::Playback
            | TuiEnent::Volumei(_)
            | TuiEnent::PlaybackProgress(_, _)
            | TuiEnent::PlaybackMode
            | TuiEnent::Artist(_)
            | TuiEnent::Track(_) => {
                // 将事件委托给 PlayerTui 组件处理
                delegate_to_widget!(self, PlayerTui, |w: &mut PlayerTui| w.enent_handle(event));
            }
            TuiEnent::Navber(_) | TuiEnent::NavberIcon(_, _) => {
                // 将事件委托给 Navbar 组件处理
                delegate_to_widget!(self, NavbarTui, |w: &mut NavbarTui| w.enent_handle(event));
            }
        }
    }
}
