use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(ScalarMap)]
pub fn derive_scalar_map(input: TokenStream) -> TokenStream {
    let input: DeriveInput = parse_macro_input!(input);
    let ident = &input.ident;
    let generics = &input.generics;
    let output = quote! {
        impl #generics ScalarMapExt for #ident #generics {}
    };
    output.into()
}
