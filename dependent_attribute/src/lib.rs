#![feature(proc_macro_diagnostic)]

extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::*;

mod func;

struct OutParams {
    trait_path: TypePath,
    struct_path: TypePath,
}

impl parse::Parse for OutParams {
    fn parse(input: parse::ParseStream) -> Result<Self> {
        let trait_path = input.parse()?;
        input.parse::<Token![,]>()?;
        let struct_path = input.parse()?;
        Ok(Self {
            trait_path,
            struct_path,
        })
    }
}

#[proc_macro_attribute]
pub fn dependent_out(attr: TokenStream, input: TokenStream) -> TokenStream {
    let mut input = parse_macro_input!(input as ItemFn);
    let OutParams {
        trait_path,
        struct_path,
    } = parse_macro_input!(attr as OutParams);

    let (orig_out, name, generic_out, generic_pat, body) = match func::extract_info(input.clone()) {
        Ok(info) => info,
        Err(error) => return error,
    };

    input.sig.output = parse2(quote! { -> impl #trait_path }).unwrap();
    input.block =
        parse2(quote! {{ let out: #orig_out = #body; let out: #struct_path = out.into(); out }})
            .unwrap();

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

