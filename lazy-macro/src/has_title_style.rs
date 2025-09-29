use quote::quote;
use syn::{DeriveInput, Result, Type};

use crate::utils::extract_named_field;

pub(crate) fn expand_has_title_style(ast: &DeriveInput) -> Result<proc_macro2::TokenStream> {
    // 获取结构体名字
    let struct_ident = &ast.ident;

    // 提取结构体的具名字段
    let named_field = extract_named_field(ast)?;

    let impl_title_style = named_field
        .iter()
        .map(|f| {
            let name = f.ident.as_ref().expect("结构体必须有名字");
            let ty = &f.ty;
            if let Type::Path(path) = &ty
                && let Some(segments) = path.path.segments.last()
                && segments.ident == "TitleStyle"
            {
                quote! {
                    use lazy_core::traits::{ HasTitleStyle,HasTitleStyleSetter};

                    impl HasTitleStyle for #struct_ident {
                        fn title_style(&self) -> ratatui::style::Style {
                            ratatui::style::Style::default()
                                .bg(self.#name.bg())
                                .fg(self.#name.fg())
                                .add_modifier(self.#name.modifier())
                        }

                        fn title_alignment(&self) -> ratatui::layout::Alignment {
                            self.#name.alignment()
                        }


                        fn title_text(&self) -> &str {
                            self.#name.text().as_str()
                        }
                    }
                    impl HasTitleStyleSetter for #struct_ident{
                        fn set_title_text(&mut self, text: String){
                            self.#name.set_text(text);
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

                        fn set_title_bg(&mut self, bg:ratatui::style::Color){
                            self.#name.set_bg(bg);
                        }

                    }
                }
            } else {
                quote! {}
            }
        })
        .collect::<Vec<_>>();

    Ok(quote! {
        #(#impl_title_style)*
    })
}
