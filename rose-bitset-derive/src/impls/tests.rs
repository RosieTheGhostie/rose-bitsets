use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::Ident;

// i'm too lazy to write the rest of the tests right now lol

pub fn generate_code(
    ident: &Ident,
    suffix: &str,
    debug: bool,
    indices: bool,
    iter: bool,
) -> TokenStream {
    let mod_name = format_ident!("bitset_{suffix}_tests");
    let extra_tests = generate_extra_tests(ident, debug, indices, iter);
    quote! {
        #[cfg(test)]
        mod #mod_name {
            use super::*;

            #[test]
            fn new_set_is_empty() {
                let set = #ident::new();
                assert!(set.is_empty());
            }

            #[test]
            fn capacity() {
                assert_eq!(#ident::CAPACITY, ::core::mem::size_of::<#ident>() * 8);
            }

            #[test]
            fn unit_5() {
                assert_eq!(#ident::unit(5), #ident::from_bits(0b100000));
            }

            #[test]
            fn unit_1_million() {
                assert_eq!(#ident::unit(1_000_000), #ident::new());
            }

            #extra_tests
        }
    }
}

fn generate_extra_tests(ident: &Ident, debug: bool, indices: bool, iter: bool) -> TokenStream {
    let mut tests = TokenStream::new();
    if debug {
        tests.extend(generate_debug_tests(ident));
    }
    if indices {
        tests.extend(generate_indices_tests(ident));
    }
    if iter {
        tests.extend(generate_iter_tests(ident));
    }
    tests
}

fn generate_debug_tests(_ident: &Ident) -> TokenStream {
    quote! {}
}

fn generate_indices_tests(_ident: &Ident) -> TokenStream {
    quote! {}
}

fn generate_iter_tests(_ident: &Ident) -> TokenStream {
    quote! {}
}
