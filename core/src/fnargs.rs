/*
MIT License

Copyright (c) 2017 David DeSimone

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
*/
use syn;
use quote;

pub fn parse_args(decl: &Vec<syn::FnArg>) -> FnArgs {
    let mut argc = 0;
    let mut args = FnArgs::new();
    for input in decl {
        match input {
            &syn::FnArg::SelfRef(ref lifetime, ref mutability) => {
                args.args_with_types = quote! { &#lifetime #mutability self };
                args.mutable_status = mutability.clone();
                args.is_instance_method = true;
            },
            &syn::FnArg::SelfValue(ref mutability) => {
                args.args_with_types = quote!{#mutability self };
                args.mutable_status = mutability.clone();
                args.is_instance_method = true;
                args.takes_self_ownership = true;
            },
            &syn::FnArg::Captured(ref pat, ref ty) => {
                let tok = quote!{ #pat };
                if argc > 0 {
                    args.args_with_types.append(quote! {,});
                }

                if argc > 1 {
                    args.args_with_no_self_no_types.append(quote!{,});
                }

                args.args_with_types.append(quote! { #tok: #ty });
                args.args_with_no_self_no_types.append(quote! { #tok });
            },
            _ => {}
        }

        argc += 1;
    }

    args
}

pub struct FnArgs {
    pub args_with_types: quote::Tokens,
    pub args_with_no_self_no_types: quote::Tokens,
    pub mutable_status: syn::Mutability,
    pub is_instance_method: bool,
    pub takes_self_ownership: bool,
}

impl FnArgs {
    fn new() -> FnArgs {
        FnArgs {
            args_with_types: quote! { },
            args_with_no_self_no_types: quote! { },
            mutable_status: syn::Mutability::Immutable,
            is_instance_method: false,
            takes_self_ownership: false,
        }
    }
}
