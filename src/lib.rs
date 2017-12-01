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

#![feature(proc_macro)]
#![recursion_limit = "256"]

extern crate syn;
extern crate quote;
extern crate proc_macro;
extern crate trait_derive_core;

use proc_macro::TokenStream;
use std::str::FromStr;
use trait_derive_core::generate_trait;

static ATTR_ERROR_MSG: &str = "Invalid trait name. Please enter another name. Names must be in the pattern #[make_trait(YOUR_NAME)]";

#[proc_macro_attribute]
pub fn make_trait(attr_ts: TokenStream, body_ts: TokenStream) -> TokenStream {
    let attr_s = attr_ts.to_string();
    let len = attr_s.len();
    let name = if len == 0 {
        None
    } else {
        // String opening '(' and closing ')'
        assert!(attr_s.starts_with('('), ATTR_ERROR_MSG);
        assert!(attr_s.ends_with(')'), ATTR_ERROR_MSG);
        let slice = &attr_s[1..len-1];
        Some(String::from_str(slice).expect(ATTR_ERROR_MSG))
    };
    
    let raw_item = syn::parse_item(&body_ts.to_string()).unwrap();
    let final_output = generate_trait(&raw_item, name);
    TokenStream::from_str(final_output.as_str()).unwrap()
}
