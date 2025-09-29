use quote::quote;
use syn::{DeriveInput, Result, Type};

use crate::utils::extract_named_field;

/// 为结构体生成 `HasTitleStyle` 和 `HasTitleStyleSetter` trait 的实现
///
/// 宏会扫描结构体中类型为 `TitleStyle` 的字段，并生成对应的 getter/setter
pub(crate) fn expand_has_title_style(ast: &DeriveInput) -> Result<proc_macro2::TokenStream> {
    // 获取结构体名字，例如 `RootTui`
    let struct_ident = &ast.ident;

    // 提取结构体的具名字段（struct {...} 中的有名字字段）
    let named_field = extract_named_field(ast)?;

    // 遍历字段，查找类型为 `TitleStyle` 的字段
    let impl_title_style = named_field
        .iter()
        .map(|f| {
            // 获取字段名
            let name = f.ident.as_ref().expect("结构体必须有名字");
            let ty = &f.ty;

            // 判断字段类型是否为 TitleStyle
            if let Type::Path(path) = &ty
                && let Some(segments) = path.path.segments.last()
                && segments.ident == "TitleStyle"
            {
                // 生成 trait impl
                quote! {
                    // 为结构体实现只读 trait
                    impl lazy_core::traits::HasTitleStyle for #struct_ident {
                        fn title_style(&self) -> ratatui::style::Style {
                            ratatui::style::Style::default()
                                .bg(self.#name.bg())          // 使用字段自身 getter
                                .fg(self.#name.fg())
                                .add_modifier(self.#name.modifier())
                        }

                        fn title_alignment(&self) -> ratatui::layout::Alignment {
                            self.#name.alignment()
                        }

                        fn title_text(&self) -> &str {
                            self.#name.text().as_str()     // 返回字段文本
                        }
                    }

                    // 为结构体实现可写 trait
                    impl lazy_core::traits::HasTitleStyleSetter for #struct_ident{
                        fn set_title_text(&mut self, text: String){
                            self.#name.set_text(text);    // 调用字段的 setter
                        }

                        fn set_title_alignment(&mut self, alignment: ratatui::layout::Alignment){
                            self.#name.set_alignment(alignment)
                        }

                        fn set_title_modifier(&mut self, modifier: ratatui::style::Modifier){
                            self.#name.set_modifier(modifier)
                        }

                        fn set_title_fg(&mut self, fg: ratatui::style::Color){
                            self.#name.set_fg(fg);
                        }

                        fn set_title_bg(&mut self, bg: ratatui::style::Color){
                            self.#name.set_bg(bg);
                        }
                    }
                }
            } else {
                // 如果不是 TitleStyle 字段，则不生成任何代码
                quote! {}
            }
        })
        .collect::<Vec<_>>();

    // 拼接所有生成的 impl 并返回
    Ok(quote! {
        #(#impl_title_style)*
    })
}
