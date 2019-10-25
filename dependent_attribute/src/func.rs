use proc_macro::TokenStream;
use proc_macro2::{Ident, Span, TokenStream as pm2_ts};
use quote::quote;
use syn::{Block, Error, ItemFn, ReturnType, Type};

type Extracted = (Type, Ident, pm2_ts, pm2_ts, Box<Block>);

pub fn extract_info(input: ItemFn) -> Result<Extracted, TokenStream> {
    let orig_out = match input.sig.output {
        ReturnType::Type(_, ty) => *ty,
        _ => {
            return Err(Error::new(
                Span::call_site(),
                "Function with unit return type can't be dependently-typed",
            )
            .to_compile_error()
            .into())
        }
    };
    let name = input.sig.ident;
    let (generic_pat, generic_out) = if input.sig.generics.lt_token.is_some() {
        // We need to generate generic parameters
        let pat = quote! { <$($param:path),*> };
        let out = quote! { ::<$($param)*> };
        (pat, out)
    } else {
        (quote! {}, quote! {})
    };
    let body = input.block;

    Ok((orig_out, name, generic_out, generic_pat, body))
}
