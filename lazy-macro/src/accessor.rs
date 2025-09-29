use crate::utils::extract_named_field;
use proc_macro2::TokenStream;
use quote::quote;
use syn::{DeriveInput, Result};

pub(crate) fn expand_accessor(ast: &DeriveInput) -> Result<TokenStream> {
    let struct_ident = &ast.ident;
    let named_field = extract_named_field(ast)?;
    let accessor = named_field
        .iter()
        .map(|f| {
            let name = f.ident.as_ref().expect("结构体必须有名字");
            let ty = &f.ty;
            let setter = quote::format_ident!("set_{}", name);
            let getter = quote::format_ident!("{}", name);
            quote! {
                pub fn #setter(&mut self,#name:#ty){
                    self.#name=#name;
                }
                pub fn #getter(&self)->&#ty{
                    &self.#name
                }

            }
        })
        .collect::<Vec<_>>();
    Ok(quote! {
        impl #struct_ident{
            #(#accessor)*
        }
    })
}
