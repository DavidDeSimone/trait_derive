#![feature(proc_macro)]
#![recursion_limit = "256"]

extern crate syn;
extern crate quote;
extern crate proc_macro;
extern crate trait_derive_core;

use proc_macro::TokenStream;
use std::str::FromStr;
use trait_derive_core::generate_trait;

#[proc_macro_attribute]
pub fn make_trait(_attr_ts: TokenStream, body_ts: TokenStream) -> TokenStream {
    let raw_item = syn::parse_item(&body_ts.to_string()).unwrap();
    let final_output = generate_trait(&raw_item);
    TokenStream::from_str(final_output.as_str()).unwrap()
}
