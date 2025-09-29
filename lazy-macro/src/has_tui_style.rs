use quote::quote;
use syn::{DeriveInput, Result, Type};

use crate::utils::extract_named_field;

pub(crate) fn expand_has_tui_style(ast: &DeriveInput) -> Result<proc_macro2::TokenStream> {
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
                && segments.ident == "TuiStyle"
            {
                quote! {
                    impl HasTuiStyle for #struct_ident {
                    fn bg(&self) -> ratatui::style::Color {
                        self.#name.bg()
                    }

                    fn fg(&self) -> ratatui::style::Color {
                        self.#name.fg()
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
