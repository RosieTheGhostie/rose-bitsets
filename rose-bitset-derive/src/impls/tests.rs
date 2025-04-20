use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::Ident;

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
            const COMP_A: u128 = 0x8ba3237df3c1c1a68c6390cfdd395442;
            const BITS_B: u128 = 0x446c4e86f71986f4b77f9320423feb4a;
            const COMP_B: u128 = 0xbb93b17908e6790b48806cdfbdc014b5;
            const BITS_C: u128 = 0x6d5c852727edd96417f66ce7a17376e9;
            const COMP_C: u128 = 0x92a37ad8d812269be80993185e8c8916;
            const BITS_D: u128 = 0x89ba968fb8d945091704eb6e5c121aa6;
            const COMP_D: u128 = 0x764569704726baf6e8fb1491a3ede559;

            const SET_A: #ident = #ident::from_bits((BITS_A & MASK) as _);
            const SET_B: #ident = #ident::from_bits((BITS_B & MASK) as _);
            const SET_C: #ident = #ident::from_bits((BITS_C & MASK) as _);
            const SET_D: #ident = #ident::from_bits((BITS_D & MASK) as _);

            // fn new() -> Self

            #[test]
            fn new_set_is_empty() {
                assert!(#ident::new().is_empty());
            }

            #[test]
            fn new_set_is_not_full() {
                assert!(!#ident::new().is_full());
            }

            // const CAPACITY: usize

            #[test]
            fn capacity() {
                assert_eq!(#ident::CAPACITY, ::core::mem::size_of::<#ident>() * 8);
            }

            // fn unit(usize) -> Self

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

            // fn all() -> Self

            #[test]
            fn all_is_full() {
                assert!(#ident::all().is_full());
            }

            #[test]
            fn all_is_not_empty() {
                assert!(!#ident::all().is_empty());
            }

            // fn complement(Self) -> Self

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
            fn complement_a() {
                let expected = (COMP_A & MASK) as _;
                assert_eq!(SET_A.complement().bits(), expected);
            }

            #[test]
            fn complement_b() {
                let expected = (COMP_B & MASK) as _;
                assert_eq!(SET_B.complement().bits(), expected);
            }

            #[test]
            fn complement_c() {
                let expected = (COMP_C & MASK) as _;
                assert_eq!(SET_C.complement().bits(), expected);
            }

            #[test]
            fn complement_d() {
                let expected = (COMP_D & MASK) as _;
                assert_eq!(SET_D.complement().bits(), expected);
            }

            // fn intersection(Self, Self) -> Self

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

            // fn union(Self, Self) -> Self

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

            // fn difference(Self, Self) -> Self
            // (there are much more difference tests btw because it's not a commutative operation)

            #[test]
            fn difference_a_a() {
                assert!(SET_A.difference(SET_A).is_empty());
            }

            #[test]
            fn difference_a_b() {
                let expected = ((BITS_A & COMP_B) & MASK) as _;
                assert_eq!(SET_A.difference(SET_B).bits(), expected);
            }

            #[test]
            fn difference_a_c() {
                let expected = ((BITS_A & COMP_C) & MASK) as _;
                assert_eq!(SET_A.difference(SET_C).bits(), expected);
            }

            #[test]
            fn difference_a_d() {
                let expected = ((BITS_A & COMP_D) & MASK) as _;
                assert_eq!(SET_A.difference(SET_D).bits(), expected);
            }

            #[test]
            fn difference_b_a() {
                let expected = ((BITS_B & COMP_A) & MASK) as _;
                assert_eq!(SET_B.difference(SET_A).bits(), expected);
            }

            #[test]
            fn difference_b_b() {
                assert!(SET_B.difference(SET_B).is_empty());
            }

            #[test]
            fn difference_b_c() {
                let expected = ((BITS_B & COMP_C) & MASK) as _;
                assert_eq!(SET_B.difference(SET_C).bits(), expected);
            }

            #[test]
            fn difference_b_d() {
                let expected = ((BITS_B & COMP_D) & MASK) as _;
                assert_eq!(SET_B.difference(SET_D).bits(), expected);
            }

            #[test]
            fn difference_c_a() {
                let expected = ((BITS_C & COMP_A) & MASK) as _;
                assert_eq!(SET_C.difference(SET_A).bits(), expected);
            }

            #[test]
            fn difference_c_b() {
                let expected = ((BITS_C & COMP_B) & MASK) as _;
                assert_eq!(SET_C.difference(SET_B).bits(), expected);
            }

            #[test]
            fn difference_c_c() {
                assert!(SET_C.difference(SET_C).is_empty());
            }

            #[test]
            fn difference_c_d() {
                let expected = ((BITS_C & COMP_D) & MASK) as _;
                assert_eq!(SET_C.difference(SET_D).bits(), expected);
            }

            #[test]
            fn difference_d_a() {
                let expected = ((BITS_D & COMP_A) & MASK) as _;
                assert_eq!(SET_D.difference(SET_A).bits(), expected);
            }

            #[test]
            fn difference_d_b() {
                let expected = ((BITS_D & COMP_B) & MASK) as _;
                assert_eq!(SET_D.difference(SET_B).bits(), expected);
            }

            #[test]
            fn difference_d_c() {
                let expected = ((BITS_D & COMP_C) & MASK) as _;
                assert_eq!(SET_D.difference(SET_C).bits(), expected);
            }

            #[test]
            fn difference_d_d() {
                assert!(SET_D.difference(SET_D).is_empty());
            }

            // fn symmetric_difference(Self, Self) -> Self

            #[test]
            fn symmetric_difference_a_a() {
                assert!(SET_A.symmetric_difference(SET_A).is_empty());
            }

            #[test]
            fn symmetric_difference_a_b() {
                let expected = ((BITS_A ^ BITS_B) & MASK) as _;
                assert_eq!(SET_A.symmetric_difference(SET_B).bits(), expected);
                assert_eq!(SET_B.symmetric_difference(SET_A).bits(), expected);
            }

            #[test]
            fn symmetric_difference_a_c() {
                let expected = ((BITS_A ^ BITS_C) & MASK) as _;
                assert_eq!(SET_A.symmetric_difference(SET_C).bits(), expected);
                assert_eq!(SET_C.symmetric_difference(SET_A).bits(), expected);
            }

            #[test]
            fn symmetric_difference_a_d() {
                let expected = ((BITS_A ^ BITS_D) & MASK) as _;
                assert_eq!(SET_A.symmetric_difference(SET_D).bits(), expected);
                assert_eq!(SET_D.symmetric_difference(SET_A).bits(), expected);
            }

            #[test]
            fn symmetric_difference_b_b() {
                assert!(SET_B.symmetric_difference(SET_B).is_empty());
            }

            #[test]
            fn symmetric_difference_b_c() {
                let expected = ((BITS_B ^ BITS_C) & MASK) as _;
                assert_eq!(SET_B.symmetric_difference(SET_C).bits(), expected);
                assert_eq!(SET_C.symmetric_difference(SET_B).bits(), expected);
            }

            #[test]
            fn symmetric_difference_b_d() {
                let expected = ((BITS_B ^ BITS_D) & MASK) as _;
                assert_eq!(SET_B.symmetric_difference(SET_D).bits(), expected);
                assert_eq!(SET_D.symmetric_difference(SET_B).bits(), expected);
            }

            #[test]
            fn symmetric_difference_c_c() {
                assert!(SET_C.symmetric_difference(SET_C).is_empty());
            }

            #[test]
            fn symmetric_difference_c_d() {
                let expected = ((BITS_C ^ BITS_D) & MASK) as _;
                assert_eq!(SET_C.symmetric_difference(SET_D).bits(), expected);
                assert_eq!(SET_D.symmetric_difference(SET_C).bits(), expected);
            }

            #[test]
            fn symmetric_difference_d_d() {
                assert!(SET_D.symmetric_difference(SET_D).is_empty());
            }

            // fn is(Self, Self) -> bool

            #[test]
            fn a_is_a() {
                assert!(SET_A.is(SET_A));
            }

            #[test]
            fn b_is_b() {
                assert!(SET_B.is(SET_B));
            }

            #[test]
            fn c_is_c() {
                assert!(SET_C.is(SET_C));
            }

            #[test]
            fn d_is_d() {
                assert!(SET_D.is(SET_D));
            }

            // fn is_not(Self, Self) -> bool

            #[test]
            fn a_is_not_b() {
                assert!(SET_A.is_not(SET_B));
                assert!(SET_B.is_not(SET_A));
            }

            #[test]
            fn a_is_not_c() {
                assert!(SET_A.is_not(SET_C));
                assert!(SET_C.is_not(SET_A));
            }

            #[test]
            fn a_is_not_d() {
                assert!(SET_A.is_not(SET_D));
                assert!(SET_D.is_not(SET_A));
            }

            #[test]
            fn b_is_not_c() {
                assert!(SET_B.is_not(SET_C));
                assert!(SET_C.is_not(SET_B));
            }

            #[test]
            fn b_is_not_d() {
                assert!(SET_B.is_not(SET_D));
                assert!(SET_D.is_not(SET_B));
            }

            #[test]
            fn c_is_not_d() {
                assert!(SET_C.is_not(SET_D));
                assert!(SET_D.is_not(SET_C));
            }

            // fn is_disjoint(Self, Self) -> bool

            #[test]
            fn a_is_disjoint_from_complement() {
                assert!(SET_A.is_disjoint(SET_A.complement()));
            }

            #[test]
            fn a_isnt_disjoint_from_a() {
                assert!(!SET_A.is_disjoint(SET_A));
            }

            #[test]
            fn a_isnt_disjoint_from_b() {
                assert!(!SET_A.is_disjoint(SET_B));
                assert!(!SET_B.is_disjoint(SET_A));
            }

            #[test]
            fn a_isnt_disjoint_from_c() {
                assert!(!SET_A.is_disjoint(SET_C));
                assert!(!SET_C.is_disjoint(SET_A));
            }

            #[test]
            fn a_isnt_disjoint_from_d() {
                assert!(!SET_A.is_disjoint(SET_D));
                assert!(!SET_D.is_disjoint(SET_A));
            }

            #[test]
            fn b_is_disjoint_from_complement() {
                assert!(SET_B.is_disjoint(SET_B.complement()));
            }

            #[test]
            fn b_isnt_disjoint_from_b() {
                assert!(!SET_B.is_disjoint(SET_B));
            }

            #[test]
            fn b_isnt_disjoint_from_c() {
                assert!(!SET_B.is_disjoint(SET_C));
                assert!(!SET_C.is_disjoint(SET_B));
            }

            #[test]
            fn b_isnt_disjoint_from_d() {
                assert!(!SET_B.is_disjoint(SET_D));
                assert!(!SET_D.is_disjoint(SET_B));
            }

            #[test]
            fn c_is_disjoint_from_complement() {
                assert!(SET_C.is_disjoint(SET_C.complement()));
            }

            #[test]
            fn c_isnt_disjoint_from_c() {
                assert!(!SET_C.is_disjoint(SET_C));
            }

            #[test]
            fn c_isnt_disjoint_from_d() {
                assert!(!SET_C.is_disjoint(SET_D));
                assert!(!SET_D.is_disjoint(SET_C));
            }

            #[test]
            fn d_is_disjoint_from_complement() {
                assert!(SET_D.is_disjoint(SET_D.complement()));
            }

            #[test]
            fn d_isnt_disjoint_from_d() {
                assert!(!SET_D.is_disjoint(SET_D));
            }

            // fn is_subset(Self, Self) -> bool

            #[test]
            fn empty_is_subset_of_everything() {
                let empty = #ident::new();
                assert!(empty.is_subset(empty));
                assert!(empty.is_subset(#ident::all()));
                assert!(empty.is_subset(SET_A));
                assert!(empty.is_subset(SET_B));
                assert!(empty.is_subset(SET_C));
                assert!(empty.is_subset(SET_D));
            }

            #[test]
            fn all_isnt_subset_of_anything_but_itself() {
                let all = #ident::all();
                assert!(!all.is_subset(#ident::new()));
                assert!(all.is_subset(all));
                assert!(!all.is_subset(SET_A));
                assert!(!all.is_subset(SET_B));
                assert!(!all.is_subset(SET_C));
                assert!(!all.is_subset(SET_D));
            }

            #[test]
            fn a_isnt_subset_of_complement() {
                assert!(!SET_A.is_subset(SET_A.complement()));
            }

            #[test]
            fn a_is_subset_of_a() {
                assert!(SET_A.is_subset(SET_A));
            }

            #[test]
            fn a_isnt_subset_of_b() {
                assert!(!SET_A.is_subset(SET_B));
            }

            #[test]
            fn a_isnt_subset_of_c() {
                assert!(!SET_A.is_subset(SET_C));
            }

            #[test]
            fn a_isnt_subset_of_d() {
                assert!(!SET_A.is_subset(SET_D));
            }

            #[test]
            fn b_isnt_subset_of_complement() {
                assert!(!SET_B.is_subset(SET_B.complement()));
            }

            #[test]
            fn b_isnt_subset_of_a() {
                assert!(!SET_B.is_subset(SET_A));
            }

            #[test]
            fn b_is_subset_of_b() {
                assert!(SET_B.is_subset(SET_B));
            }

            #[test]
            fn b_isnt_subset_of_c() {
                assert!(!SET_B.is_subset(SET_C));
            }

            #[test]
            fn b_isnt_subset_of_d() {
                assert!(!SET_B.is_subset(SET_D));
            }

            #[test]
            fn c_isnt_subset_of_complement() {
                assert!(!SET_C.is_subset(SET_C.complement()));
            }

            #[test]
            fn c_isnt_subset_of_a() {
                assert!(!SET_C.is_subset(SET_A));
            }

            #[test]
            fn c_isnt_subset_of_b() {
                assert!(!SET_C.is_subset(SET_B));
            }

            #[test]
            fn c_is_subset_of_c() {
                assert!(SET_C.is_subset(SET_C));
            }

            #[test]
            fn c_isnt_subset_of_d() {
                assert!(!SET_C.is_subset(SET_D));
            }

            #[test]
            fn d_isnt_subset_of_complement() {
                assert!(!SET_D.is_subset(SET_D.complement()));
            }

            #[test]
            fn d_isnt_subset_of_a() {
                assert!(!SET_D.is_subset(SET_A));
            }

            #[test]
            fn d_isnt_subset_of_b() {
                assert!(!SET_D.is_subset(SET_B));
            }

            #[test]
            fn d_isnt_subset_of_c() {
                assert!(!SET_D.is_subset(SET_C));
            }

            #[test]
            fn d_is_subset_of_d() {
                assert!(SET_D.is_subset(SET_D));
            }

            // todo!("test fn is_strict_subset(Self, Self) -> bool")
            // todo!("test fn is_superset(Self, Self) -> bool")
            // todo!("test fn is_strict_superset(Self, Self) -> bool")
            // todo!("test fn len(Self) -> usize")
            // todo!("test fn contains(Self, usize) -> bool")
            // todo!("test fn get(Self, usize) -> bool")
            // todo!("test fn min_index(Self) -> usize")
            // todo!("test fn max_index(Self) -> usize")
            // todo!("test fn max_index_checked(Self) -> Option<usize>")
            // todo!("test fn shifted_up_by(Self, u32) -> Self")
            // todo!("test fn shift_up_by(&mut Self, u32)")
            // todo!("test fn shifted_up_by_signed(Self, i32) -> Self")
            // todo!("test fn shift_up_by_signed(&mut Self, i32)")
            // todo!("test fn shifted_down_by(Self, u32) -> Self")
            // todo!("test fn shift_down_by(&mut Self, u32)")
            // todo!("test fn shifted_down_by_signed(Self, i32) -> Self")
            // todo!("test fn shift_down_by_signed(&mut Self, i32)")
            // todo!("test fn clear(&mut Self) -> ()")
            // todo!("test fn masked_0_to_i(Self, usize) -> Self")
            // todo!("test fn mask_0_to_i(&mut Self, usize)")
            // todo!("test fn masked_i_to_N(Self, usize) -> Self")
            // todo!("test fn mask_i_to_N(&mut Self, usize)")
            // todo!("test fn cleared_0_to_i(Self, usize) -> Self")
            // todo!("test fn clear_0_to_i(&mut Self, usize)")
            // todo!("test fn cleared_i_to_N(Self, usize) -> Self")
            // todo!("test fn clear_i_to_N(&mut Self, usize)")
            // todo!("test fn insert_quiet(&mut Self, usize)")
            // todo!("test fn insert(&mut Self, usize) -> bool")
            // todo!("test fn replace_quiet(&mut Self, usize, bool)")
            // todo!("test fn replace(&mut Self, usize, bool) -> bool")
            // todo!("test fn remove_quiet(&mut Self, usize)")
            // todo!("test fn remove(&mut Self, usize) -> bool")

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

fn generate_debug_tests(ident: &Ident) -> TokenStream {
    quote! {
        struct _NotAHashSet(::std::vec::Vec<usize>);

        impl<I> ::core::convert::From<I> for _NotAHashSet
        where
            I: ::core::iter::IntoIterator<Item = usize>,
        {
            fn from(iterable: I) -> Self {
                Self(
                    iterable
                        .into_iter()
                        .take_while(|&index| index < #ident::CAPACITY)
                        .collect(),
                )
            }
        }

        impl ::core::fmt::Debug for _NotAHashSet {
            fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                f.debug_set()
                    .entries(self.0.iter())
                    .finish()
            }
        }

        #[test]
        fn debug_a() {
            let indices = _NotAHashSet::from([
                0, 2, 3, 4, 5, 7, 8, 9, 11, 13, 15, 17, 18, 22, 23, 25, 29, 36, 37, 40, 41, 42, 43,
                45, 46, 50, 51, 52, 55, 56, 57, 60, 61, 62, 64, 67, 68, 70, 73, 74, 75, 76, 77, 81,
                82, 83, 84, 85, 90, 91, 97, 103, 106, 107, 108, 110, 111, 114, 115, 116, 118, 122,
                124, 125, 126,
            ]);
            assert_eq!(format!("{SET_A:?}"), format!("{indices:?}"));
            assert_eq!(format!("{SET_A:#?}"), format!("{indices:#?}"));
        }

        #[test]
        fn debug_b() {
            let indices = _NotAHashSet::from([
                1, 3, 6, 8, 9, 11, 13, 14, 15, 16, 17, 18, 19, 20, 21, 25, 30, 37, 40, 41, 44, 47,
                48, 49, 50, 51, 52, 53, 54, 56, 57, 58, 60, 61, 63, 66, 68, 69, 70, 71, 73, 74, 79,
                80, 83, 84, 88, 89, 90, 92, 93, 94, 95, 97, 98, 103, 105, 106, 107, 110, 114, 115,
                117, 118, 122, 126,
            ]);
            assert_eq!(format!("{SET_B:?}"), format!("{indices:?}"));
            assert_eq!(format!("{SET_B:#?}"), format!("{indices:#?}"));
        }

        #[test]
        fn debug_c() {
            let indices = _NotAHashSet::from([
                0, 3, 5, 6, 7, 9, 10, 12, 13, 14, 16, 17, 20, 21, 22, 24, 29, 31, 32, 33, 34, 37,
                38, 39, 42, 43, 45, 46, 49, 50, 52, 53, 54, 55, 56, 57, 58, 60, 66, 69, 70, 72, 75,
                76, 78, 79, 80, 82, 83, 85, 86, 87, 88, 89, 90, 93, 96, 97, 98, 101, 104, 106, 111,
                114, 115, 116, 118, 120, 122, 123, 125, 126,
            ]);
            assert_eq!(format!("{SET_C:?}"), format!("{indices:?}"));
            assert_eq!(format!("{SET_C:#?}"), format!("{indices:#?}"));
        }

        #[test]
        fn debug_d() {
            let indices = _NotAHashSet::from([
                1, 2, 5, 7, 9, 11, 12, 17, 20, 26, 27, 28, 30, 33, 34, 35, 37, 38, 40, 41, 43, 45,
                46, 47, 50, 56, 57, 58, 60, 64, 67, 72, 74, 78, 80, 83, 84, 86, 87, 91, 92, 93, 95,
                96, 97, 98, 99, 103, 105, 106, 108, 111, 113, 115, 116, 117, 119, 120, 123, 127,
            ]);
            assert_eq!(format!("{SET_D:?}"), format!("{indices:?}"));
            assert_eq!(format!("{SET_D:#?}"), format!("{indices:#?}"));
        }
    }
}
