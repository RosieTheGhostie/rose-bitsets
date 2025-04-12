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

            // Assuming the implementation of `all` is correct, this will produce a 128-bit mask
            // that can be used to truncate large numbers.
            const MASK: u128 = (-1 & #ident::all().bits() as u128 as i128) as _;

            const BITS_A: u128 = 0x745cdc820c3e3e59739c6f3022c6abbd;
            const BITS_B: u128 = 0x446c4e86f71986f4b77f9320423feb4a;
            const BITS_C: u128 = 0x6d5c852727edd96417f66ce7a17376ee;
            const BITS_D: u128 = 0x89ba968fb8d945091704eb6e5c121aa2;

            const SET_A: #ident = #ident::from_bits((BITS_A & MASK) as _);
            const SET_B: #ident = #ident::from_bits((BITS_B & MASK) as _);
            const SET_C: #ident = #ident::from_bits((BITS_C & MASK) as _);
            const SET_D: #ident = #ident::from_bits((BITS_D & MASK) as _);

            #[test]
            fn new_set_is_empty() {
                assert!(#ident::new().is_empty());
            }

            #[test]
            fn new_set_is_not_full() {
                assert!(!#ident::new().is_full());
            }

            #[test]
            fn capacity() {
                assert_eq!(#ident::CAPACITY, ::core::mem::size_of::<#ident>() * 8);
            }

            #[test]
            fn unit_0() {
                assert_eq!(#ident::unit(0).bits(), 0b1);
            }

            #[test]
            fn unit_5() {
                assert_eq!(#ident::unit(5).bits(), 0b100000);
            }

            #[test]
            fn unit_capacity_minus_1() {
                assert!(!#ident::unit(#ident::CAPACITY - 1).is_empty());
            }

            #[test]
            fn unit_capacity_is_empty() {
                assert!(#ident::unit(#ident::CAPACITY).is_empty());
            }

            #[test]
            fn unit_1_million_is_empty() {
                assert!(#ident::unit(1_000_000).is_empty());
            }

            #[test]
            fn all_is_full() {
                assert!(#ident::all().is_full());
            }

            #[test]
            fn all_is_not_empty() {
                assert!(!#ident::all().is_empty());
            }

            #[test]
            fn complement_of_new_set_is_full() {
                assert!(#ident::new().complement().is_full());
            }

            #[test]
            fn complement_of_all_is_empty() {
                assert!(#ident::all().complement().is_empty());
            }

            #[test]
            fn len_of_units_complement() {
                assert_eq!(#ident::unit(4).complement().len(), #ident::CAPACITY - 1);
            }

            #[test]
            fn intersection_a_a() {
                let expected = (BITS_A & MASK) as _;
                assert_eq!(SET_A.intersection(SET_A).bits(), expected);
            }

            #[test]
            fn intersection_a_b() {
                let expected = ((BITS_A & BITS_B) & MASK) as _;
                assert_eq!(SET_A.intersection(SET_B).bits(), expected);
                assert_eq!(SET_B.intersection(SET_A).bits(), expected);
            }

            #[test]
            fn intersection_a_c() {
                let expected = ((BITS_A & BITS_C) & MASK) as _;
                assert_eq!(SET_A.intersection(SET_C).bits(), expected);
                assert_eq!(SET_C.intersection(SET_A).bits(), expected);
            }

            #[test]
            fn intersection_a_d() {
                let expected = ((BITS_A & BITS_D) & MASK) as _;
                assert_eq!(SET_A.intersection(SET_D).bits(), expected);
                assert_eq!(SET_D.intersection(SET_A).bits(), expected);
            }

            #[test]
            fn intersection_b_b() {
                let expected = (BITS_B & MASK) as _;
                assert_eq!(SET_B.intersection(SET_B).bits(), expected);
            }

            #[test]
            fn intersection_b_c() {
                let expected = ((BITS_B & BITS_C) & MASK) as _;
                assert_eq!(SET_B.intersection(SET_C).bits(), expected);
                assert_eq!(SET_C.intersection(SET_B).bits(), expected);
            }

            #[test]
            fn intersection_b_d() {
                let expected = ((BITS_B & BITS_D) & MASK) as _;
                assert_eq!(SET_B.intersection(SET_D).bits(), expected);
                assert_eq!(SET_D.intersection(SET_B).bits(), expected);
            }

            #[test]
            fn intersection_c_c() {
                let expected = (BITS_C & MASK) as _;
                assert_eq!(SET_C.intersection(SET_C).bits(), expected);
            }

            #[test]
            fn intersection_c_d() {
                let expected = ((BITS_C & BITS_D) & MASK) as _;
                assert_eq!(SET_C.intersection(SET_D).bits(), expected);
                assert_eq!(SET_D.intersection(SET_C).bits(), expected);
            }

            #[test]
            fn intersection_d_d() {
                let expected = (BITS_D & MASK) as _;
                assert_eq!(SET_D.intersection(SET_D).bits(), expected);
            }

            #[test]
            fn union_a_a() {
                let expected = (BITS_A & MASK) as _;
                assert_eq!(SET_A.union(SET_A).bits(), expected);
            }

            #[test]
            fn union_a_b() {
                let expected = ((BITS_A | BITS_B) & MASK) as _;
                assert_eq!(SET_A.union(SET_B).bits(), expected);
                assert_eq!(SET_B.union(SET_A).bits(), expected);
            }

            #[test]
            fn union_a_c() {
                let expected = ((BITS_A | BITS_C) & MASK) as _;
                assert_eq!(SET_A.union(SET_C).bits(), expected);
                assert_eq!(SET_C.union(SET_A).bits(), expected);
            }

            #[test]
            fn union_a_d() {
                let expected = ((BITS_A | BITS_D) & MASK) as _;
                assert_eq!(SET_A.union(SET_D).bits(), expected);
                assert_eq!(SET_D.union(SET_A).bits(), expected);
            }

            #[test]
            fn union_b_b() {
                let expected = (BITS_B & MASK) as _;
                assert_eq!(SET_B.union(SET_B).bits(), expected);
            }

            #[test]
            fn union_b_c() {
                let expected = ((BITS_B | BITS_C) & MASK) as _;
                assert_eq!(SET_B.union(SET_C).bits(), expected);
                assert_eq!(SET_C.union(SET_B).bits(), expected);
            }

            #[test]
            fn union_b_d() {
                let expected = ((BITS_B | BITS_D) & MASK) as _;
                assert_eq!(SET_B.union(SET_D).bits(), expected);
                assert_eq!(SET_D.union(SET_B).bits(), expected);
            }

            #[test]
            fn union_c_c() {
                let expected = (BITS_C & MASK) as _;
                assert_eq!(SET_C.union(SET_C).bits(), expected);
            }

            #[test]
            fn union_c_d() {
                let expected = ((BITS_C | BITS_D) & MASK) as _;
                assert_eq!(SET_C.union(SET_D).bits(), expected);
                assert_eq!(SET_D.union(SET_C).bits(), expected);
            }

            #[test]
            fn union_d_d() {
                let expected = (BITS_D & MASK) as _;
                assert_eq!(SET_D.union(SET_D).bits(), expected);
            }

            #extra_tests
        }
    }
}

fn generate_extra_tests(ident: &Ident, debug: bool, _indices: bool, _iter: bool) -> TokenStream {
    let mut tests = TokenStream::new();
    if debug {
        tests.extend(generate_debug_tests(ident));
    }
    tests
}

fn generate_debug_tests(_ident: &Ident) -> TokenStream {
    quote! {}
}
