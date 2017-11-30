extern crate syn;
#[macro_use]
extern crate quote;
extern crate proc_macro;


fn sig_to_quote(ident: &syn::Ident, sig: &syn::MethodSig) -> quote::Tokens {
    assert!(!sig.decl.variadic, "Impls for variadic functions is not supported");
    let ref unsafety = sig.unsafety;
    let abi = sig.abi.as_ref().map_or(quote!{}, |x| quote!{ #x });
    let ref inputs = sig.decl.inputs;
    let ref generics = sig.generics;

    let ref _where_clause = generics.where_clause;
    let ref _lifetimes = generics.lifetimes;
    let ref _ty_params = generics.ty_params;

    let outputs = match sig.decl.output {
        syn::FunctionRetTy::Default => quote!{},
        syn::FunctionRetTy::Ty(ref ty) => quote!{ -> #ty }
    };
    
    quote!{
        #unsafety #abi fn #ident (#inputs) #outputs;
    }
}

fn parse_methods(items: &Vec<syn::ImplItem>) -> (quote::Tokens, quote::Tokens) {
    let _results: Vec<quote::Tokens> = items
        .iter()
        .map(|x| {
            match x.node {
                syn::ImplItemKind::Method(ref sig, ref decl) => {
                    let qsig = sig_to_quote(&x.ident, sig);
                    quote!{ #qsig }
                },   
                _ => quote!{ }
            }
        })
        .collect();
    
    (quote!{ }, quote!{ })
}

pub fn generate_trait(item: &syn::Item) -> quote::Tokens {
    match item.node {
        syn::ItemKind::Impl(_unsafety,
                            _impl_pol,
                            ref _generics,
                            ref _path,
                            ref ty,
                            ref items) => {
            let name = concat_idents("Trait", quote!{ #ty }.as_str());
            let (decls, impls) = parse_methods(items);
            quote! {
                trait #name {
                    #decls
                }

                impl #name for #ty {
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
