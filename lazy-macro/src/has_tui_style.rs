//!
//! 本模块实现了 `#[derive(DeriveHasTuiStyle)]` 过程宏。
//!
//! 该宏为结构体自动实现 `lazy-core` 中定义的各种样式相关的 Trait
//! (例如 `HasTuiStyle`, `HasTitleStyle` 等)。
//! 它通过检查结构体中的字段类型 (如 `TuiStyle`, `TitleStyle`)，
//! 并为父结构体生成相应的 Trait 实现，将调用委托给这些字段。
//!

use crate::utils::{extract_named_field, get_field_attribute_args, has_field_ty};
use quote::quote;
use syn::{DeriveInput, Meta, Result};

/// 为字段类型是 TuiStyle 的字段生成 trait impl
fn gen_tui_style_impl(
    struct_ident: &syn::Ident,
    field_name: &syn::Ident,
) -> proc_macro2::TokenStream {
    quote! {
        // 以下代码为父结构体 `#struct_ident` 生成 Trait 实现。
        // 所有方法的实现都委托 (delegate) 给内部的 `#field_name` 字段。

        impl lazy_core::traits::HasTuiStyle for #struct_ident {
            fn tui_style(&self) -> ratatui::style::Style {
                ratatui::style::Style::default()
                    .bg(self.#field_name.bg())
                    .fg(self.#field_name.fg())
                    .add_modifier(self.#field_name.modifier())
            }
            fn tui_alignment(&self) -> ratatui::layout::Alignment {
                self.#field_name.alignment()
            }
        }

        impl lazy_core::traits::HasTuiStyleSetter for #struct_ident {
            fn set_tui_alignment(&mut self, alignment: ratatui::layout::Alignment) {
                self.#field_name.set_alignment(alignment);
            }
            fn set_tui_modifier(&mut self, modifier: ratatui::style::Modifier) {
                self.#field_name.set_modifier(modifier);
            }
            fn set_tui_bg(&mut self, bg: ratatui::style::Color) {
                self.#field_name.set_bg(bg);
            }
            fn set_tui_fg(&mut self, fg: ratatui::style::Color) {
                self.#field_name.set_fg(fg);
            }
        }
    }
}

/// 为字段类型是 TitleStyle 的字段生成 trait impl
fn gen_title_style_impl(
    struct_ident: &syn::Ident,
    field_name: &syn::Ident,
) -> proc_macro2::TokenStream {
    quote! {
        // 以下代码为父结构体 `#struct_ident` 生成 Trait 实现。
        // 所有方法的实现都委托 (delegate) 给内部的 `#field_name` 字段。

        impl lazy_core::traits::HasTitleStyle for #struct_ident {
            fn title_style(&self) -> ratatui::style::Style {
                ratatui::style::Style::default()
                    .bg(self.#field_name.bg())
                    .fg(self.#field_name.fg())
                    .add_modifier(self.#field_name.modifier())
            }
            fn title_alignment(&self) -> ratatui::layout::Alignment {
                self.#field_name.alignment()
            }
            fn title_text(&self) -> &str {
                self.#field_name.text().as_str()
            }
        }

        impl lazy_core::traits::HasTitleStyleSetter for #struct_ident {
            fn set_title_text(&mut self, text: String) {
                self.#field_name.set_text(text);
            }
            fn set_title_alignment(&mut self, alignment: ratatui::layout::Alignment) {
                self.#field_name.set_alignment(alignment);
            }
            fn set_title_modifier(&mut self, modifier: ratatui::style::Modifier) {
                self.#field_name.set_modifier(modifier);
            }
            fn set_title_fg(&mut self, fg: ratatui::style::Color) {
                self.#field_name.set_fg(fg);
            }
            fn set_title_bg(&mut self, bg: ratatui::style::Color) {
                self.#field_name.set_bg(bg);
            }
        }
    }
}

/// 为字段类型是 BorderStyle 的字段生成 trait impl
fn gen_border_style_impl(
    struct_ident: &syn::Ident,
    field_name: &syn::Ident,
) -> proc_macro2::TokenStream {
    quote! {
        // 以下代码为父结构体 `#struct_ident` 生成 Trait 实现。
        // 所有方法的实现都委托 (delegate) 给内部的 `#field_name` 字段。

        impl lazy_core::traits::HasBorderStyle for #struct_ident {
            fn border_style(&self) -> ratatui::style::Style {
                ratatui::style::Style::default()
                    .bg(self.#field_name.bg())
                    .fg(self.#field_name.fg())
            }
            fn has_border(&self) -> bool {
                self.#field_name.border() != ratatui::widgets::Borders::NONE
            }
            fn borders(&self) -> ratatui::widgets::Borders {
                self.#field_name.border()
            }
        }

        impl lazy_core::traits::HasBorderStyleSetter for #struct_ident {
            fn set_border(&mut self, border: ratatui::widgets::Borders) {
                self.#field_name.set_border(border);
            }
            fn set_border_fg(&mut self, fg: ratatui::style::Color) {
                self.#field_name.set_fg(fg);
            }
            fn set_border_bg(&mut self, bg: ratatui::style::Color) {
                self.#field_name.set_bg(bg);
            }
            fn toggle_border(&mut self) {
                let new_border = if self.#field_name.border() == ratatui::widgets::Borders::NONE {
                    ratatui::widgets::Borders::ALL
                } else {
                    ratatui::widgets::Borders::NONE
                };
                self.#field_name.set_border(new_border);
            }
        }
    }
}

/// 主宏函数
pub(crate) fn expand_has_tui_style(ast: &DeriveInput) -> Result<proc_macro2::TokenStream> {
    let struct_ident = &ast.ident;
    // 提取所有命名字段
    let named_field = extract_named_field(ast)?;

    // 遍历每个字段，根据其类型生成对应的 Trait 实现
    let impl_tokens = named_field
        .iter()
        .map(|f| {
            let name = f.ident.as_ref().expect("字段必须有名字");
            let ty = &f.ty;

            // 检查字段是否带有 `#[DeriveHasTuiStyle(Skip)]` 辅助属性。
            // 如果有，则跳过该字段，不为它生成任何代码。
            let has_none_attr = get_field_attribute_args(f, "DeriveHasTuiStyle")
                .unwrap_or_default()
                .iter()
                .any(|meta| matches!(meta, Meta::Path(p) if p.is_ident("Skip")));

            if has_none_attr {
                return quote! {}; // 跳过，生成空的代码
            }

            // 根据字段的类型，调用相应的代码生成函数
            if has_field_ty(ty, &["TuiStyle"]) {
                gen_tui_style_impl(struct_ident, name)
            } else if has_field_ty(ty, &["TitleStyle"]) {
                gen_title_style_impl(struct_ident, name)
            } else if has_field_ty(ty, &["BorderStyle"]) {
                gen_border_style_impl(struct_ident, name)
            } else {
                // 如果类型不匹配，则不生成任何代码
                quote! {}
            }
        })
        .collect::<Vec<_>>();

    // 将所有生成的 impl 代码块组合在一起并返回
    Ok(quote! { #(#impl_tokens)* })
}
