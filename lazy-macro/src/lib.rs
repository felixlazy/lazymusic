mod accessor;
mod utils;
use crate::accessor::expand_accessor;
use syn::DeriveInput;
#[proc_macro_derive(Accessor, attributes(Accessor))]
pub fn derive_acessor(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ast = syn::parse_macro_input!(input as DeriveInput);
    match expand_accessor(&ast) {
        Ok(token_stream) => token_stream.into(),
        Err(e) => e.into_compile_error().into(),
    }
}
