extern crate proc_macro;

use proc_macro::TokenStream;

use syn::{ItemFn, parse_macro_input, DeriveInput, Expr};
use quote::quote;
use proc_macro2::TokenStream as TokenStream2;
use syn::AttributeArgs;

#[proc_macro_attribute]
pub fn spec(attr: TokenStream, item: TokenStream) -> TokenStream {
    let specification = parse_macro_input!(attr as Expr);
    println!("specification: \"{:?}\"", specification);
    let func = parse_macro_input!(item as ItemFn);
    println!("signature: \"{:?}\"", func.sig.inputs);
    (quote! {
        #func
    }).into()
}