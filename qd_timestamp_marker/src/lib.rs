extern crate proc_macro;

use proc_macro::TokenStream;
use quote::{format_ident, quote, ToTokens};
use std::time::{SystemTime, UNIX_EPOCH};
use syn::*;

#[proc_macro_attribute]
pub fn marker(attr: TokenStream, input: TokenStream) -> TokenStream {
    let mut input = parse_macro_input!(input as ItemMod);

    let replaced = parse_macro_input!(attr as Ident);
    let replacement = format_ident!(
        "{}{}",
        replaced,
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_millis()
    );

    let content = input
        .content
        .as_mut()
        .expect("Only the inline modules can be labeled with timestamp");
    content.1.iter_mut().for_each(|item| {
        *item = parse2(update(item.to_token_stream(), &replaced, &replacement)).unwrap()
    });

    let output = quote! { #input };
    output.into()
}

fn update(item: proc_macro2::TokenStream, from: &Ident, to: &Ident) -> proc_macro2::TokenStream {
    use proc_macro2::*;
    item.into_iter()
        .map(|tok| match tok {
            TokenTree::Group(group) => {
                let mut new_group = Group::new(group.delimiter(), update(group.stream(), from, to));
                new_group.set_span(group.span());
                TokenTree::Group(new_group)
            }
            TokenTree::Ident(ident) if &ident == from => {
                let mut to = to.clone();
                to.set_span(ident.span());
                TokenTree::Ident(to)
            }
            tok => tok,
        })
        .collect()
}
