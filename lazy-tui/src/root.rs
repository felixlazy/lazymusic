//! 根 TUI 组件模块，定义了整个 TUI 的根容器。

// 导入宏
use lazy_macro::DeriveHasTuiStyle;
// 从 ratatui 中导入所需的组件和布局
use ratatui::{Frame, layout::Rect};

// 从 lazy_core 中导入所需的结构体和 traits
use lazy_core::{
    structs::{BorderStyle, TitleStyle, TuiStyle},
    traits::HasBorderStyleSetter,
};

// 从当前 crate 中导入所需的组件和 traits
use crate::{
    player::PlayerTui,
    traits::{HasWidgets, RenderTui, TuiBlock}, // RenderTui 用于渲染，TuiBlock 用于生成边框块
};

/// `RootTui` 是根 TUI 组件，作为整个播放器界面的容器。
///
/// 它包含了 `PlayerTui`，并负责渲染整个界面。
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
            // 初始化时，将 `PlayerTui` 作为子组件
            widgets: vec![Box::new(PlayerTui::default())],
        }
    }
}

impl HasWidgets for RootTui {
    /// 获取对 `widgets` 向量的可变引用。
    fn get_widgets_mut(&mut self) -> &mut Vec<Box<dyn RenderTui>> {
        &mut self.widgets
    }
}

impl RootTui {
    /// 调整音量，此方法将调用 `PlayerTui` 的 `adjust_volume` 方法。
    ///
    /// # Arguments
    ///
    /// * `delta` - 音量变化的增量。
    pub fn adjust_volume(&mut self, delta: i8) {
        if let Some(player) = self.get_widget_mut::<PlayerTui>() {
            player.adjust_volume(delta);
        }
    }

    /// 切换播放状态（播放/暂停），此方法将调用 `PlayerTui` 的 `toggle_state` 方法。
    pub fn toggle_state(&mut self) {
        if let Some(player) = self.get_widget_mut::<PlayerTui>() {
            player.toggle_state();
        }
    }

    /// 设置当前播放曲目，此方法将调用 `PlayerTui` 的 `set_track` 方法。
    ///
    /// # Arguments
    ///
    /// * `track` - 曲目名称。
    pub fn set_track(&mut self, track: String) {
        if let Some(player) = self.get_widget_mut::<PlayerTui>() {
            player.set_track(track);
        }
    }

    /// 切换当前组件及其子组件的边框显示状态。
    pub fn toggle_all_border(&mut self) {
        self.toggle_border();
        if let Some(player) = self.get_widget_mut::<PlayerTui>() {
            player.toggle_border();
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

        // 遍历并渲染所有子组件
        self.widgets.iter().for_each(|f| {
            f.render(frame, inner);
        });
    }

    /// 将 `self` 转换为 `&dyn Any`。
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    /// 将 `self` 转换为 `&mut dyn Any`。
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}

