mod accessor;
mod has_tui_style;
mod utils;
use crate::{accessor::expand_accessor, has_tui_style::expand_has_tui_style};

use syn::DeriveInput;
#[proc_macro_derive(Accessor, attributes(Accessor))]
pub fn derive_acessor(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ast = syn::parse_macro_input!(input as DeriveInput);
    match expand_accessor(&ast) {
        Ok(token_stream) => token_stream.into(),
        Err(e) => e.into_compile_error().into(),
    }
}

#[proc_macro_derive(DeriveHasTuiStyle, attributes(DeriveHasTuiStyle))]
pub fn derive_has_tui_style(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ast = syn::parse_macro_input!(input as DeriveInput);
    match expand_has_tui_style(&ast) {
        Ok(token_stream) => token_stream.into(),
        Err(e) => e.into_compile_error().into(),
    }
}
