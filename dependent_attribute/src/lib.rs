extern crate proc_macro;

use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;
use syn::*;

#[proc_macro_attribute]
pub fn dependent_out(attr: TokenStream, input: TokenStream) -> TokenStream {
    let mut input = parse_macro_input!(input as ItemFn);
    let Params { trait_path, struct_path } = parse_macro_input!(attr as Params);

    let orig_out = match input.sig.output {
        ReturnType::Type(_, ty) => *ty.clone(),
        _ => {
            return Error::new(
                Span::call_site(),
                "Function with unit return type can't be dependently-typed",
            )
            .to_compile_error()
            .into()
        }
    };
    input.sig.output = parse2(quote! { -> impl #trait_path }).unwrap();

    let name = input.sig.ident.clone();

    let params = input.sig.generics.clone();
    let (generic_pat, generic_out) = if params.lt_token.is_some() {
        // We need to generate generic parameters
        let pat = quote! { [$($param:tt)*] };
        let out = quote! { ::<$($param)*> };
        (pat, out)
    } else {
        (quote! {}, quote! {})
    };

    let body = input.block.clone();
    input.block = parse2(quote! {{ let out: #orig_out = #body; let out: #struct_path = out.into(); out }}).unwrap();

    let output = quote! {
        macro_rules! #name {
            (#generic_pat $($input:tt)*) => {{
                #input
                #name#generic_out($($input)*)
            }};
            // This pattern will be matched only if the generics are required but not provided.
            // If they are not required, the first pattern will always match.
            ($($input:tt)*) => { compile_error!("Function with dependent output cannot infer its generic parameters") };
        }
    };
    output.into()
}

struct Params {
    trait_path: TypePath,
    struct_path: TypePath,
}

impl parse::Parse for Params {
    fn parse(input: parse::ParseStream) -> Result<Self> {
        let trait_path = input.parse()?;
        input.parse::<Token![,]>()?;
        let struct_path = input.parse()?;
        Ok(Self { trait_path, struct_path })
    }
}
