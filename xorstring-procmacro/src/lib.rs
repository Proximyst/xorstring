#![warn(rust_2018_idioms)]

extern crate proc_macro;

use proc_macro2::Span;
use quote::quote;
use std::{
    num::Wrapping,
    time::{SystemTime, UNIX_EPOCH},
};
use syn::{parse_macro_input, Lit};

lazy_static::lazy_static! {
    static ref XORKEY: u8 = (SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs() % 255) as u8;
}

#[proc_macro]
pub fn xorstring(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    if input.is_empty() {
        // fetch the key used for ALL encryptions for use in decryption
        let key = *XORKEY;
        return quote! { (#key as u8) }.into();
    }

    let string: Lit = parse_macro_input!(input as Lit);
    let string = match string {
        Lit::Str(lit) => lit,
        _ => panic!("not string input"),
    };

    let string = string.value();

    let bytes = string.bytes().collect::<Vec<_>>();
    let mut encrypted = Vec::with_capacity(bytes.len());
    for (i, c) in bytes.iter().enumerate() {
        // XOR every character to encrypt it with the key
        encrypted.push(c ^ (Wrapping(*XORKEY as usize) + Wrapping(i)).0 as u8);
    }

    // ok boys it's encrypted, time to send it off to the caller!
    let lit = syn::LitByteStr::new(encrypted.as_slice(), Span::call_site());
    let length = string.len();
    return quote! { ((#lit as &[u8]), (#length as usize)) }.into();
}
