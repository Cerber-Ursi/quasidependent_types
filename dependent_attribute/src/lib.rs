extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::*;

#[proc_macro_attribute]
pub fn randomize(attr: TokenStream, input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as ItemMod);
    let name = input.ident.clone();

    let output = quote! {
        #input
        pub use self::#name::*;
    };
    output.into()
}