use lazy_macro::DeriveHasTuiStyle;
use ratatui::{Frame, layout::Rect};

use lazy_core::{
    structs::{BorderStyle, TitleStyle, TuiStyle},
    traits::HasBorderStyleSetter,
};

use crate::{
    player::PlayerTui,
    traits::{HasWidgets, RenderTui, TuiBlock}, // RenderTui 用于渲染，TuiBlock 用于生成边框块
};

/// 根 TUI 组件，作为整个播放器界面的容器
#[derive(DeriveHasTuiStyle)]
pub struct RootTui {
    title: TitleStyle,   // 根组件标题样式
    border: BorderStyle, // 根组件边框样式
    style: TuiStyle,     // 根组件通用样式（颜色、对齐等）
    widgets: Vec<Box<dyn RenderTui>>,
}

impl Default for RootTui {
    fn default() -> Self {
        Self {
            title: Default::default(),
            border: Default::default(),
            style: Default::default(),
            widgets: vec![Box::new(PlayerTui::default())],
        }
    }
}

impl HasWidgets for RootTui {
    fn get_widgets_mut(&mut self) -> &mut Vec<Box<dyn RenderTui>> {
        &mut self.widgets
    }
}

impl RootTui {
    /// 调整音量，代理到子组件 PlayerTui
    pub fn adjust_volume(&mut self, delta: i8) {
        if let Some(player) = self.get_widget_mut::<PlayerTui>() {
            player.adjust_volume(delta);
        }
    }

    /// 切换播放状态（播放/暂停），代理到 PlayerTui
    pub fn toggle_state(&mut self) {
        if let Some(player) = self.get_widget_mut::<PlayerTui>() {
            player.toggle_state();
        }
    }

    /// 设置当前播放曲目，代理到 PlayerTui
    pub fn set_track(&mut self, track: String) {
        if let Some(player) = self.get_widget_mut::<PlayerTui>() {
            player.set_track(track);
        }
    }

    /// 切换当前组件及其子组件的边框显示状态
    pub fn toggle_all_border(&mut self) {
        self.toggle_border();
        if let Some(player) = self.get_widget_mut::<PlayerTui>() {
            player.toggle_border();
        }
    }
}

impl RenderTui for RootTui {
    /// 渲染整个根组件
    fn render(&self, frame: &mut Frame, rect: Rect) {
        let inner = self.get_inner(rect); // 获取去掉边框的内部区域
        frame.render_widget(self.to_block(), rect); // 渲染根组件边框和标题

        // 渲染子组件
        if let Some(widget) = self.widgets.get(0) {
            widget.render(frame, inner);
        }
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}
