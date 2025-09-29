use crate::utils::{extract_named_field, get_field_attribute_args};
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::{DeriveInput, Meta, Result};

/// 为结构体生成 getter 和 setter 方法
/// 支持字段级别的 #[Accessor(Copy)] 属性
pub(crate) fn expand_accessor(ast: &DeriveInput) -> Result<TokenStream> {
    // 获取结构体名字
    let struct_ident = &ast.ident;

    // 提取结构体的具名字段
    let named_field = extract_named_field(ast)?;

    // 遍历每个字段，生成对应的 getter 和 setter
    let accessor = named_field
        .iter()
        .map(|f| {
            let name = f.ident.as_ref().expect("结构体必须有名字");
            let ty = &f.ty;

            // 获取字段上的 #[Accessor(...)] 属性参数
            let metas = get_field_attribute_args(f, "Accessor").unwrap_or_default();

            // 判断字段是否有 Copy 标记
            let is_copy = metas
                .iter()
                .any(|meta| matches!(meta, Meta::Path(p) if p.is_ident("Copy")));

            let getter_name = format_ident!("{}", name);
            let setter_name = format_ident!("set_{}", name);

            // 根据 Copy 标记生成 getter
            let getter_fn = if is_copy {
                quote! {
                    pub fn #getter_name(&self) -> #ty {
                        self.#name
                    }
                }
            } else {
                quote! {
                    pub fn #getter_name(&self) -> &#ty {
                        &self.#name
                    }
                }
            };

            // 生成 setter
            let setter_fn = quote! {
                pub fn #setter_name(&mut self, value: #ty) {
                    self.#name = value;
                }
            };

            quote! {
                #getter_fn

                #setter_fn
            }
        })
        .collect::<Vec<_>>();

    // 将所有字段生成的方法包装到 impl 块中
    // 方法之间已有空行，使 impl 块清晰可读
    Ok(quote! {
        impl #struct_ident {

            #(#accessor)*

        }
    })
}
