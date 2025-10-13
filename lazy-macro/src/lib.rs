//! `lazy-macro` crate 提供了 `lazymusic` 项目中使用的所有过程宏。
//!
//! 主要包含以下宏：
//! - `#[derive(Accessor)]`: 为结构体字段自动生成 getter 和 setter 方法。
//! - `#[derive(DeriveHasTuiStyle)]`: 为 UI 组件自动实现样式相关的 Trait。

mod accessor;
mod event_mappings;
mod has_tui_style;
mod utils;
use crate::{
    accessor::expand_accessor, event_mappings::expand_event_mappings,
    has_tui_style::expand_has_tui_style,
};

use proc_macro::token_stream;
use syn::DeriveInput;

/// 为结构体字段自动生成 getter 和/或 setter 方法。
///
/// # 使用方法
///
/// 在结构体上添加 `#[derive(Accessor)]`，然后在需要生成访问器的字段上
/// 添加 `#[Accessor(...)]` 辅助属性。
///
/// ## 辅助属性
///
/// - `getter`: 为该字段生成一个公共的 getter 方法。
/// - `setter`: 为该字段生成一个公共的 setter 方法。
///
/// # Example
///
/// ```rust
/// use lazy_macro::Accessor;
///
/// #[derive(Accessor, Default)]
/// struct Foo {
///     // 为 `bar` 字段生成 pub 的 getter `bar()` 和 setter `set_bar()`
///     #[Accessor(getter, setter)]
///     bar: String,
///
///     // 只为 `val` 字段生成 pub 的 getter `val()`
///     #[Accessor(getter)]
///     val: i32,
/// }
///
/// let mut foo = Foo::default();
/// foo.set_bar("hello".to_string());
/// assert_eq!(foo.bar(), "hello");
/// assert_eq!(foo.val(), &0);
/// ```
#[proc_macro_derive(Accessor, attributes(Accessor))]
pub fn derive_acessor(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ast = syn::parse_macro_input!(input as DeriveInput);
    match expand_accessor(&ast) {
        Ok(token_stream) => token_stream.into(),
        Err(e) => e.into_compile_error().into(),
    }
}

/// 为包含样式结构体（如 `TuiStyle`, `TitleStyle`）的 UI 组件自动实现样式相关的 Trait。
///
/// 这个宏会自动为组件实现 `HasTuiStyle`、`HasTitleStyle`、`HasBorderStyle`
/// 以及它们对应的 `Setter` Trait，通过“委托模式”将调用转发给内部的样式字段。
///
/// # 辅助属性
///
/// - `skip`: 如果一个字段带有 `#[DeriveHasTuiStyle(skip)]`，宏将跳过该字段。
///
/// # Example
///
/// ```rust
/// use lazy_macro::DeriveHasTuiStyle;
/// use lazy_core::structs::{TuiStyle, TitleStyle};
/// use lazy_core::traits::{HasTuiStyle, HasTitleStyle, HasTuiStyleSetter};
///
/// #[derive(DeriveHasTuiStyle)]
/// struct MyWidget {
///     // 宏会自动为 MyWidget 实现 HasTuiStyle 和 HasTuiStyleSetter
///     style: TuiStyle,
///
///     // 宏会自动为 MyWidget 实现 HasTitleStyle 和 HasTitleStyleSetter
///     title: TitleStyle,
///
///     // 使用 `skip` 属性，这个字段将被宏忽略
///     #[DeriveHasTuiStyle(skip)]
///     other_style: TuiStyle,
/// }
///
/// let mut widget = MyWidget { style: TuiStyle::default(), title: TitleStyle::default(), other_style: TuiStyle::default() };
/// // 我们可以直接在 widget 实例上调用 Trait 方法
/// widget.set_tui_bg(ratatui::style::Color::Red);
/// assert_eq!(widget.tui_style().bg, Some(ratatui::style::Color::Red));
/// ```
#[proc_macro_derive(DeriveHasTuiStyle, attributes(DeriveHasTuiStyle))]
pub fn derive_has_tui_style(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ast = syn::parse_macro_input!(input as DeriveInput);
    match expand_has_tui_style(&ast) {
        Ok(token_stream) => token_stream.into(),
        Err(e) => e.into_compile_error().into(),
    }
}

#[proc_macro_attribute]
pub fn auto_delegate_events(
    attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    match expand_event_mappings(attr, item) {
        Ok(token_stream) => token_stream.into(),
        Err(e) => e.into_compile_error().into(),
    }
}
