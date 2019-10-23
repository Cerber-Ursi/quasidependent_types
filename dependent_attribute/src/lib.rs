extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use proc_macro2::TokenStream as pm2_ts;

#[proc_macro_attribute]
pub fn dependent_out(input: TokenStream, _: TokenStream) -> TokenStream {
    let input = pm2_ts::from(input);
    let output = quote! {
        dependent_out_inner! { #input }
    };
    output.into()
}

#[proc_macro]
pub fn dependent_out_inner(input: TokenStream) -> TokenStream {
    input
}

#[proc_macro_attribute]
pub fn dependent_check(input: TokenStream, _: TokenStream) -> TokenStream {
    input
}
