use proc_macro::{Diagnostic, Level::*, TokenStream};
use proc_macro2::{Span, TokenStream as pm2_ts};
use quote::{quote, quote_spanned};
use syn::{
    parse2, spanned::Spanned, Error, ImplItem, ImplItemConst, ImplItemMacro, ImplItemMethod,
    ImplItemType, ItemImpl, ItemTrait, Path, Signature, TraitItemMethod, Type,
};

pub fn generate(input: ItemImpl) -> Result<(ItemTrait, ItemImpl), TokenStream> {
    // Decompose the incoming data into pieces, to more easily work with them later.
    let ItemImpl {
        attrs,
        defaultness,
        unsafety,
        impl_token,
        generics,
        trait_,
        self_ty,
        items,
        ..
    } = input;

    if let Some(defaultness) = defaultness {
        Diagnostic::spanned(
            defaultness.span().unwrap(),
            Warning,
            "Default implementation is unsupported (and makes no sence, in fact)",
        )
        .emit();
    }
    if unsafety.is_some() {
        Diagnostic::spanned(
            unsafety.span().unwrap(),
            Warning,
            "Dependent-type companion trait should not be the unsafe trait, since it shouldn't be implemented by hand anyway"
        ).emit();
    }
    let trait_name = trait_
        .ok_or(Error::new_spanned(impl_token, "No trait name in input").to_compile_error())?
        .1;

    let (consts, methods, types, macros, streams) = split_items(items)?;

    for item in consts {
        Diagnostic::spanned(
            item.span().unwrap(),
            Warning,
            "Associated constants in companion traits are not supported, skipping",
        )
        .emit();
    }
    for item in macros {
        Diagnostic::spanned(
            item.span().unwrap(),
            Warning,
            "Macro invocations inside companion trait implementation are not supported, skipping",
        )
        .emit();
    }
    for stream in streams {
        Diagnostic::spanned(
            stream.span().unwrap(),
            Warning,
            "Unexpected tokens are ignored",
        )
        .emit();
    }

    let mut inner_type: Option<Type> = None;
    for ty in types {
        if ty.ident.to_string() == "Inner" {
            if let Some(inner) = &inner_type {
                Diagnostic::spanned(
                    ty.span().unwrap(),
                    Warning,
                    "Ignoring repeated specification of inner types",
                )
                .span_note(inner.span().unwrap(), "the first declaration was here")
                .emit();
            } else {
                inner_type = Some(ty.ty);
            }
        } else {
            Diagnostic::spanned(
                ty.span().unwrap(),
                Warning,
                "Custom associated types are not supported for companion traits, skipping",
            )
            .emit();
        }
    }
    let inner_type = inner_type.ok_or_else(|| {
        Error::new(Span::call_site(), "No inner type definition found").to_compile_error()
    })?;

    let mut method_defs: Vec<TraitItemMethod> = vec![];
    let mut method_impls: Vec<ImplItemMethod> = vec![];
    for method in methods {
        let ImplItemMethod {
            attrs,
            vis,
            defaultness,
            mut sig,
            block,
        } = method;
        match vis {
            syn::Visibility::Inherited => {}
            vis => Diagnostic::spanned(
                vis.span().unwrap(),
                Warning,
                "Trait methods are always public, visibility modifier is ignored",
            )
            .emit(),
        }
        if let Some(defaultness) = defaultness {
            Diagnostic::spanned(
                defaultness.span().unwrap(),
                Warning,
                "Default implementation is unsupported, modifier is ignored",
            )
            .emit();
        }
        insert_trait_bound(&mut sig, &trait_name);
        method_defs.push(
            parse2(quote! {
                #sig;
            })
            .expect("Internal error generating trait method"),
        );
        insert_inner(&mut sig, &inner_type);
        method_impls.push(
            parse2(quote! {
                #(#attrs)*
                #sig
                #block
            })
            .expect("Internal error generating trait method implementation"),
        );
    }

    Ok((
        parse2(quote! {
            trait #trait_name<Inner: DependentInnerOperate>: Sized {
                #(#method_defs)*
            }
        })
        .expect("Internal codegen error"),
        parse2(quote! {
            #(#attrs)*
            impl#generics #trait_name<#inner_type> for #self_ty {
                #(#method_impls)*
            }
        })
        .expect("Internal codegen error"),
    ))
}

fn split_items(
    items: Vec<ImplItem>,
) -> Result<
    (
        Vec<ImplItemConst>,
        Vec<ImplItemMethod>,
        Vec<ImplItemType>,
        Vec<ImplItemMacro>,
        Vec<pm2_ts>,
    ),
    TokenStream,
> {
    let (mut consts, mut methods, mut types, mut macros, mut streams) =
        (vec![], vec![], vec![], vec![], vec![]);
    for item in items {
        use ImplItem::*;
        match item {
            Const(item) => consts.push(item),
            Method(item) => methods.push(item),
            Type(item) => types.push(item),
            Macro(item) => macros.push(item),
            Verbatim(stream) => streams.push(stream),
            item => {
                return Err(Error::new_spanned(item, "Unexpected item in impl block")
                    .to_compile_error()
                    .into())
            }
        };
    }
    Ok((consts, methods, types, macros, streams))
}

fn insert_trait_bound(sig: &mut Signature, trait_name: &Path) {
    // This function works only with generics. We don't expect the `Trait` keyword anywhere else.
    let generics = &mut sig.generics;

    use syn::{
        GenericParam::Type as gType, Ident, TypeParamBound::Trait, WherePredicate::Type as wType,
    };

    let process_type_bound = |bound: &mut _| {
        let placeholder = Ident::new("Trait", Span::call_site());
        if let Trait(bound) = bound {
            if bound.path.is_ident(&placeholder) {
                bound.path = parse2(quote_spanned! { bound.path.span() => #trait_name<Inner> })
                    .expect("Internal error substituting trait bound");
            }
        }
    };

    generics.params.iter_mut().for_each(|param| match param {
        gType(param) => param.bounds.iter_mut().for_each(process_type_bound),
        _ => {}
    });
    generics.where_clause.iter_mut().for_each(|clause| {
        clause.predicates.iter_mut().for_each(|pred| match pred {
            wType(pred) => pred.bounds.iter_mut().for_each(process_type_bound),
            _ => {}
        })
    });
}

fn insert_inner(sig: &mut Signature, inner: &Type) {}
