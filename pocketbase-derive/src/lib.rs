extern crate proc_macro;

use proc_macro::TokenStream;
use syn::{DeriveInput, parse_macro_input};
use quote::quote;

#[proc_macro_derive(Changeset)]
pub fn changeset(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;

    let expanded = quote! {
        impl Changeset for #name {}
    };

    TokenStream::from(expanded)
}

#[proc_macro_derive(Recordable)]
pub fn recordable(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;

    let expanded = quote! {
        impl Recordable for #name {}
    };

    TokenStream::from(expanded)
}
