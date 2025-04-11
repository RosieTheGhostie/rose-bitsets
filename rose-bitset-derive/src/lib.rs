extern crate proc_macro;
extern crate proc_macro2;

mod extras;
mod impls;
mod type_utils;

use extras::Extras;
use proc_macro::TokenStream;
use proc_macro_error::{abort, proc_macro_error};
use proc_macro2::TokenStream as TokenStream2;
use syn::{DeriveInput, parse_macro_input, spanned::Spanned};

#[proc_macro_error]
#[proc_macro_derive(BitSet, attributes(bitset))]
pub fn bitset_derive(input: TokenStream) -> TokenStream {
    let DeriveInput {
        attrs,
        ident,
        data,
        generics,
        ..
    } = parse_macro_input!(input as DeriveInput);
    if !generics.params.is_empty() {
        abort!(
            generics.span(), "unexpected generics";
            note = "`BitSet` cannot be derived for generic types";
        );
    }
    let uint = impls::main::get_uint(data);

    let iter_impls = if let Some(bitset_attribute) = attrs
        .iter()
        .find(|attribute| attribute.meta.path().is_ident("bitset"))
    {
        Extras::generate_code_from_attribute(bitset_attribute, &ident, &uint)
    } else {
        TokenStream2::new()
    };
    let mut impls = impls::main::generate_code(ident, uint);
    impls.extend(iter_impls);
    impls.into()
}
