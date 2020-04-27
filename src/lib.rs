#![feature(external_doc)]
// #![deny(missing_docs)]
#![doc(include = "../README.md")]

extern crate proc_macro;
use lazy_static::lazy_static;
use proc_macro::TokenStream;
use std::sync::Mutex;

mod contract;
mod method;

lazy_static! {
    pub(crate) static ref METHODS: Mutex<Vec<(String, bool)>> = Mutex::new(vec![]);
}

#[proc_macro_attribute]
pub fn contract(attr: TokenStream, item: TokenStream) -> TokenStream {
    contract::contract(attr, item)
}
