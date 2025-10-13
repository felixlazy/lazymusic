//!
//! 本模块实现了 `#[auto_delegate_events]` 属性宏。
//!
//! 该宏旨在简化 TUI 事件处理。它会自动为一个实现了 `TuiEventHandle` Trait 的结构体
//! 生成 `event_handle` 方法的实现。通过解析宏属性中定义的事件到组件和方法的映射，
//! 宏能够生成一个 `match` 语句，将不同的 `TuiEnent` 变体分派给相应的子组件
//! (widget) 的方法进行处理。
//!
//! # 使用方法
//!
//! 在一个 `impl TuiEventHandle for YourStruct {}` 块上添加 `#[auto_delegate_events(...)]` 属性。
//!
//! ## 宏属性格式
//!
//! 宏属性接受一系列逗号分隔的映射规则，每个规则的格式如下：
//!
//! `TuiEnent::Variant(args) => (WidgetType, method_call(args); another_call(args))`
//!
//! - `TuiEnent::Variant(args)`: `TuiEnent` 的一个枚举变体，可以包含参数。
//! - `WidgetType`: 目标子组件的类型。
//! - `method_call(args)`: 在匹配到该事件时，在 `WidgetType` 实例上调用的方法。
//!   可以链式调用多个方法，用分号 `;` 分隔。
//!
//! # Example
//!
//! ```rust,ignore
//! #[auto_delegate_events(
//!     TuiEnent::Playback => (PlaybackTui, toggle_state()),
//!     TuiEnent::Volume(delta) => (VolumeTui, adjust_volume(delta)),
//!     TuiEnent::Track(track) => (TrackTui, set_track(track)),
//!     TuiEnent::PlaybackProgress(duration, progress) => (PlaybackProgressTui, set_progress(progress); set_duration(duration))
//! )]
//! impl TuiEventHandle for PlayerTui {}
//! ```
//!
//! 上述代码会为 `PlayerTui` 生成如下 `event_handle` 方法：
//!
//! ```rust,ignore
//! fn event_handle(&mut self, event: TuiEnent) {
//!     match event {
//!         TuiEnent::Playback => {
//!             if let Some(w) = self.get_widget_mut::<PlaybackTui>() {
//!                 w.toggle_state();
//!             }
//!         },
//!         TuiEnent::Volume(delta) => {
//!             if let Some(w) = self.get_widget_mut::<VolumeTui>() {
//!                 w.adjust_volume(delta);
//!             }
//!         },
//!         // ... 其他事件臂
//!         _ => (),
//!     }
//! }
//! ```
//!

use quote::quote;
use syn::{
    Expr, ItemImpl, Result, Token, Type,
    parse::{Parse, ParseStream},
    parse_quote,
    punctuated::Punctuated,
};

/// 代表一个事件到组件方法调用的映射规则。
///
/// 例如: `TuiEnent::Playback => (PlaybackTui, toggle_state())`
struct EventMapping {
    event: Expr,        // 事件的表达式，如 `TuiEnent::Playback`
    ty: Type,           // 目标组件的类型，如 `PlaybackTui`
    methods: Vec<Expr>, // 要调用的方法表达式列表，如 `toggle_state()`
}

/// `EventMapping` 的集合，代表宏属性中定义的所有映射规则。
struct EventMappings {
    mappings: Punctuated<EventMapping, Token![,]>,
}

impl Parse for EventMapping {
    /// 解析单个事件映射规则。
    ///
    /// 语法: `event_expr => (WidgetType, method1(); method2())`
    fn parse(input: ParseStream) -> Result<Self> {
        // 1. 解析事件表达式 (event =>)
        let event: Expr = input.parse()?;
        input.parse::<Token![=>]>()?;

        // 2. 解析括号内的内容 `(WidgetType, method1(); method2())`
        let content;
        syn::parenthesized!(content in input);

        // 3. 解析组件类型 `WidgetType`
        let ty = content.parse::<Type>()?;
        content.parse::<Token![,]>()?;

        // 4. 解析方法调用
        let mut methods = Vec::new();
        // 至少要有一个方法
        let first_method: Expr = content.parse()?;
        methods.push(first_method);

        // 如果有分号，说明有更多的方法调用
        while content.peek(Token![;]) {
            content.parse::<Token![;]>()?;
            let next_method: Expr = content.parse()?;
            methods.push(next_method);
        }

        Ok(EventMapping { event, ty, methods })
    }
}

impl Parse for EventMappings {
    /// 解析逗号分隔的多个 `EventMapping`。
    fn parse(input: ParseStream) -> Result<Self> {
        let mappings = Punctuated::<EventMapping, Token![,]>::parse_terminated(input)?;
        Ok(EventMappings { mappings })
    }
}

/// `#[auto_delegate_events]` 宏的核心实现函数。
pub fn expand_event_mappings(
    attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> Result<proc_macro2::TokenStream> {
    // 1. 解析宏属性，得到所有的事件映射规则
    let mappings = syn::parse::<EventMappings>(attr)?;
    // 2. 解析应用宏的 `impl` 块
    let mut impls = syn::parse::<ItemImpl>(item)?;

    // 3. 遍历每个映射规则，生成对应的 `match` 臂
    let mapping_tokens = mappings
        .mappings
        .iter()
        .map(|f| {
            let event = &f.event;
            let ty = &f.ty;
            let methods = &f.methods;
            quote! {
                // 生成 `TuiEnent::Variant => { ... }`
                #event => {
                    // `self.get_widget_mut` 是 `HasWidgets` Trait 的方法
                    if let Some(w) = self.get_widget_mut::<#ty>() {
                        // 在获取到的 widget 上执行所有指定的方法
                        #(w.#methods);*
                    }
                }
            }
        })
        .collect::<Vec<_>>();

    // 4. 构建完整的 `event_handle` 方法
    let fn_event_handle = parse_quote! {
        fn event_handle(&mut self, event: TuiEnent) {
            match event {
                // 插入所有生成的 match 臂
                #(#mapping_tokens),*
                // 默认臂，对于未处理的事件不执行任何操作
                _ => (),
            }
        }
    };

    impls.items.retain(|item| {
        !matches!(
            item,
            syn::ImplItem::Fn(func) if func.sig.ident == "event_handle"
        )
    });

    // 5. 将生成的 `event_handle` 方法添加到 `impl` 块中
    impls.items.push(fn_event_handle);

    // 6. 返回修改后的 `impl` 块
    Ok(quote! {
        #impls
    })
}
