#![feature(external_doc)]
// #![deny(missing_docs)]
#![doc(include = "../README.md")]

extern crate proc_macro;
use lazy_static::lazy_static;
use proc_macro::TokenStream;
use std::sync::Mutex;

mod contract;
mod method;
use quote::quote;

lazy_static! {
    pub(crate) static ref METHODS: Mutex<Vec<(String, bool)>> = Mutex::new(vec![]);
}

#[proc_macro_attribute]
pub fn contract(attr: TokenStream, item: TokenStream) -> TokenStream {
    contract::contract(attr, item)
}

fn parse_hex(digit: u8) -> u8 {
    match digit {
        b'0'..=b'9' => digit - b'0',
        b'a'..=b'f' => digit - b'a' + 10,
        b'A'..=b'F' => digit - b'A' + 10,
        _ => panic!("invalid char {}", digit),
    }
}

#[proc_macro]
pub fn address(item: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(item as syn::LitStr);
    let s = input.value();
    let s = s.as_bytes();
    let mut bytes = [0u8; 32];

    for i in 0..32 {
        bytes[i] = (parse_hex(s[(i << 1)]) << 4) | (parse_hex(s[(i << 1) + 1]));
    }

    (quote! {
        [#(#bytes,)*]
    })
    .into()
}
