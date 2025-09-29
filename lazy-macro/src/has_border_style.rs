use quote::quote;
use syn::{DeriveInput, Result, Type};

use crate::utils::extract_named_field;

/// 为结构体生成 `HasBorderStyle` 和 `HasBorderStyleSetter` trait 的实现
///
/// 宏会扫描结构体中类型为 `BorderStyle` 的字段，并生成对应的 getter/setter 方法
pub(crate) fn expand_has_border_style(ast: &DeriveInput) -> Result<proc_macro2::TokenStream> {
    // 获取结构体名字，例如 `RootTui`
    let struct_ident = &ast.ident;

    // 提取结构体的具名字段（struct {...} 中的有名字字段）
    let named_field = extract_named_field(ast)?;

    // 遍历字段，查找类型为 `BorderStyle` 的字段
    let impl_border_style = named_field
        .iter()
        .map(|f| {
            // 获取字段名
            let name = f.ident.as_ref().expect("结构体必须有名字");
            let ty = &f.ty;

            // 判断字段类型是否为 BorderStyle
            if let Type::Path(path) = &ty
                && let Some(segments) = path.path.segments.last()
                && segments.ident == "BorderStyle"
            {
                // 生成 trait impl
                quote! {
                    // 为结构体实现只读 trait
                    impl lazy_core::traits::HasBorderStyle for #struct_ident {
                        fn border_style(&self) -> ratatui::style::Style {
                            // 使用字段自身的 fg/bg getter，而不是固定 self.border
                            ratatui::style::Style::default()
                                .bg(self.#name.bg())
                                .fg(self.#name.fg())
                        }

                        fn has_border(&self) -> bool {
                            self.#name.border() != ratatui::widgets::Borders::NONE
                        }

                        fn borders(&self) -> ratatui::widgets::Borders {
                            self.#name.border()
                        }
                    }

                    // 为结构体实现可写 trait
                    impl lazy_core::traits::HasBorderStyleSetter for #struct_ident{
                        fn set_border(&mut self, border: ratatui::widgets::Borders){
                            self.#name.set_border(border);
                        }

                        fn set_border_fg(&mut self, fg: ratatui::style::Color){
                            self.#name.set_fg(fg);
                        }

                        fn set_border_bg(&mut self, bg: ratatui::style::Color){
                            self.#name.set_bg(bg); // ⚠️ 修正：之前错误写成 set_fg
                        }
                    }
                }
            } else {
                // 如果不是 BorderStyle 字段，则不生成代码
                quote! {}
            }
        })
        .collect::<Vec<_>>();

    // 拼接所有生成的 impl 并返回
    Ok(quote! {
        #(#impl_border_style)*
    })
}
