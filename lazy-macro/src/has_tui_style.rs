use quote::quote;
use syn::{DeriveInput, Result, Type};

use crate::utils::extract_named_field;

/// 为结构体生成 HasTuiStyle 和 HasTuiStyleSetter trait impl
///
/// 宏会自动扫描结构体中类型为 `TuiStyle` 的字段，并生成对应的 getter/setter
pub(crate) fn expand_has_tui_style(ast: &DeriveInput) -> Result<proc_macro2::TokenStream> {
    // 获取结构体名字
    let struct_ident = &ast.ident;

    // 提取结构体的具名字段（即 struct { ... } 中带名字的字段）
    let named_field = extract_named_field(ast)?;

    // 遍历所有字段，查找类型为 TuiStyle 的字段
    let impl_title_style = named_field
        .iter()
        .map(|f| {
            // 获取字段名
            let name = f.ident.as_ref().expect("结构体必须有名字");
            let ty = &f.ty;

            // 检查字段类型是否为 TuiStyle
            if let Type::Path(path) = &ty
                && let Some(segments) = path.path.segments.last()
                && segments.ident == "TuiStyle"
            {
                // 生成 trait impl 代码
                quote! {
                    // 为结构体实现只读 trait
                    impl lazy_core::traits::HasTuiStyle for #struct_ident {
                        fn bg(&self) -> ratatui::style::Color {
                            self.#name.bg() // 调用字段的 getter
                        }

                        fn fg(&self) -> ratatui::style::Color {
                            self.#name.fg() // 调用字段的 getter
                        }
                    }

                    // 为结构体实现可写 trait
                    impl lazy_core::traits::HasTuiStyleSetter for #struct_ident {
                        fn set_tui_bg(&mut self, bg: ratatui::style::Color) {
                            self.#name.set_bg(bg); // 调用字段的 setter
                        }

                        fn set_tui_fg(&mut self, fg: ratatui::style::Color) {
                            self.#name.set_fg(fg); // 调用字段的 setter
                        }
                    }
                }
            } else {
                // 如果不是 TuiStyle 字段，则不生成代码
                quote! {}
            }
        })
        .collect::<Vec<_>>();

    // 拼接所有生成的 impl 代码
    Ok(quote! {
        #(#impl_title_style)*
    })
}
