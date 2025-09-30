use crate::utils::{extract_named_field, has_field_ty};
use quote::quote;
use syn::{DeriveInput, Result};

/// 为字段类型是 TuiStyle 的字段生成 trait impl
fn gen_tui_style_impl(
    struct_ident: &syn::Ident,
    field_name: &syn::Ident,
) -> proc_macro2::TokenStream {
    quote! {
        impl lazy_core::traits::HasTuiStyle for #struct_ident {
            fn bg(&self) -> ratatui::style::Color { self.#field_name.bg() }
            fn fg(&self) -> ratatui::style::Color { self.#field_name.fg() }
        }

        impl lazy_core::traits::HasTuiStyleSetter for #struct_ident {
            fn set_tui_bg(&mut self, bg: ratatui::style::Color) { self.#field_name.set_bg(bg); }
            fn set_tui_fg(&mut self, fg: ratatui::style::Color) { self.#field_name.set_fg(fg); }
        }
    }
}

/// 为字段类型是 TitleStyle 的字段生成 trait impl
fn gen_title_style_impl(
    struct_ident: &syn::Ident,
    field_name: &syn::Ident,
) -> proc_macro2::TokenStream {
    quote! {
        impl lazy_core::traits::HasTitleStyle for #struct_ident {
            fn title_style(&self) -> ratatui::style::Style {
                ratatui::style::Style::default()
                    .bg(self.#field_name.bg())
                    .fg(self.#field_name.fg())
                    .add_modifier(self.#field_name.modifier())
            }
            fn title_alignment(&self) -> ratatui::layout::Alignment { self.#field_name.alignment() }
            fn title_text(&self) -> &str { self.#field_name.text().as_str() }
        }

        impl lazy_core::traits::HasTitleStyleSetter for #struct_ident {
            fn set_title_text(&mut self, text: String) { self.#field_name.set_text(text); }
            fn set_title_alignment(&mut self, alignment: ratatui::layout::Alignment) { self.#field_name.set_alignment(alignment); }
            fn set_title_modifier(&mut self, modifier: ratatui::style::Modifier) { self.#field_name.set_modifier(modifier); }
            fn set_title_fg(&mut self, fg: ratatui::style::Color) { self.#field_name.set_fg(fg); }
            fn set_title_bg(&mut self, bg: ratatui::style::Color) { self.#field_name.set_bg(bg); }
        }
    }
}

/// 为字段类型是 BorderStyle 的字段生成 trait impl
fn gen_border_style_impl(
    struct_ident: &syn::Ident,
    field_name: &syn::Ident,
) -> proc_macro2::TokenStream {
    quote! {
        impl lazy_core::traits::HasBorderStyle for #struct_ident {
            fn border_style(&self) -> ratatui::style::Style {
                ratatui::style::Style::default()
                    .bg(self.#field_name.bg())
                    .fg(self.#field_name.fg())
            }
            fn has_border(&self) -> bool { self.#field_name.border() != ratatui::widgets::Borders::NONE }
            fn borders(&self) -> ratatui::widgets::Borders { self.#field_name.border() }
        }

        impl lazy_core::traits::HasBorderStyleSetter for #struct_ident {
            fn set_border(&mut self, border: ratatui::widgets::Borders) { self.#field_name.set_border(border); }
            fn set_border_fg(&mut self, fg: ratatui::style::Color) { self.#field_name.set_fg(fg); }
            fn set_border_bg(&mut self, bg: ratatui::style::Color) { self.#field_name.set_bg(bg); }
        }
    }
}

/// 主宏函数
pub(crate) fn expand_has_tui_style(ast: &DeriveInput) -> Result<proc_macro2::TokenStream> {
    let struct_ident = &ast.ident;
    let named_field = extract_named_field(ast)?;

    let impl_tokens = named_field
        .iter()
        .map(|f| {
            let name = f.ident.as_ref().expect("字段必须有名字");
            let ty = &f.ty;

            if has_field_ty(ty, &["TuiStyle"]) {
                gen_tui_style_impl(struct_ident, name)
            } else if has_field_ty(ty, &["TitleStyle"]) {
                gen_title_style_impl(struct_ident, name)
            } else if has_field_ty(ty, &["BorderStyle"]) {
                gen_border_style_impl(struct_ident, name)
            } else {
                quote! {}
            }
        })
        .collect::<Vec<_>>();

    Ok(quote! { #(#impl_tokens)* })
}
