extern crate syn;
#[macro_use]
extern crate quote;
extern crate proc_macro;

mod fnargs;

use fnargs::parse_args;

fn generics_to_quote(generics: &syn::Generics) -> quote::Tokens {
    let mut generic_tokens = quote!{ < };

    for life_ty in &generics.lifetimes {
        generic_tokens.append(quote! {  #life_ty, });
    }

    for generic in &generics.ty_params {
        generic_tokens.append(quote! { #generic, });
    }

    generic_tokens.append(quote!{ > });
    generic_tokens
}

fn sig_to_quote(ident: &syn::Ident, sig: &syn::MethodSig) -> quote::Tokens {
    assert!(!sig.decl.variadic, "Impls for variadic functions is not supported");
    let ref unsafety = sig.unsafety;
    let abi = sig.abi.as_ref().map_or(quote!{}, |x| quote!{ #x });
    let ref inputs = sig.decl.inputs;
    let ref generics = sig.generics;

    let ref where_clause = generics.where_clause;
    let qwhere_clause = quote!{ #where_clause };
    let qgenerics = generics_to_quote(generics);

    let outputs = match sig.decl.output {
        syn::FunctionRetTy::Default => quote!{},
        syn::FunctionRetTy::Ty(ref ty) => quote!{ -> #ty }
    };

    let args = parse_args(&inputs);
    let ref args_with_types = args.args_with_types;
    quote!{
        #unsafety #abi fn #ident #qgenerics (#args_with_types) #outputs #qwhere_clause
    }
}

fn parse_methods(items: &Vec<syn::ImplItem>) -> (quote::Tokens, quote::Tokens) {
    let mut decls = quote!{};
    let mut impls = quote!{};
    for x in items.iter() {
        let (decl, imp) = match x.node {
            syn::ImplItemKind::Method(ref sig, ref body) => {
                let qsig = sig_to_quote(&x.ident, sig);
                let qimpl = quote! {
                    #qsig #body
                };

                (quote!{ #qsig; }, qimpl)
            },
            _ => panic!()
        };

        decls.append(decl);
        impls.append(imp);
    }
    
    (decls, impls)
}

pub fn generate_trait(item: &syn::Item) -> quote::Tokens {
    match item.node {
        syn::ItemKind::Impl(_unsafety,
                            _impl_pol,
                            ref generics,
                            ref _path,
                            ref ty,
                            ref items) => {
            let name = concat_idents("Trait", quote!{ #ty }.as_str());
            let (decls, impls) = parse_methods(items);

            let ref where_clause = generics.where_clause;
            let qwhere_clause = quote!{ #where_clause };
            let qgenerics = generics_to_quote(generics);
            quote! {
                trait #name {
                    #decls
                }

                impl #qgenerics #name for #ty #qwhere_clause {
                    #impls
                }
            }
        }
        _ => { panic!() }

    }
}

fn concat_idents(lhs: &str, rhs: &str) -> syn::Ident {
    syn::Ident::new(format!("{}{}", lhs, rhs))
}
