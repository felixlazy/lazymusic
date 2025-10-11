// # 使用方法
//
// `delegate_to_widget!(self, WidgetType, |w: &mut WidgetType| w.some_method());`
//
// - `self`: 拥有 `widgets` 向量的实例。
// - `WidgetType`: 要查找的子组件的具体类型。
#[macro_export]
macro_rules! delegate_to_widget {
    // 宏的匹配规则
    ($self:expr, $widget_type:ty, $handler:expr) => {
        // 宏展开后的代码
        if let Some(widget) = $self.get_widget_mut::<$widget_type>() {
            ($handler)(widget);
        }
    };
}
