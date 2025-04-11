#![doc = include_str!("../README.md")]
#![cfg_attr(docsrs, feature(doc_cfg, doc_auto_cfg))]
use rose_bitset_derive::BitSet;

/// A set of 8 bits.
#[cfg(feature = "b8")]
#[cfg_attr(docsrs, doc(cfg(feature = "b8")))]
#[derive(BitSet, Clone, Copy, Default, Eq, Hash, PartialEq)]
#[bitset(debug, indices, iter, tests)]
pub struct BitSet8(u8);

/// A set of 16 bits.
#[cfg(feature = "b16")]
#[cfg_attr(docsrs, doc(cfg(feature = "b16")))]
#[derive(BitSet, Clone, Copy, Default, Eq, Hash, PartialEq)]
#[bitset(debug, indices, iter, tests)]
pub struct BitSet16(u16);

/// A set of 32 bits.
#[cfg(feature = "b32")]
#[cfg_attr(docsrs, doc(cfg(feature = "b32")))]
#[derive(BitSet, Clone, Copy, Default, Eq, Hash, PartialEq)]
#[bitset(debug, indices, iter, tests)]
pub struct BitSet32(u32);

/// A set of 64 bits.
#[cfg(feature = "b64")]
#[cfg_attr(docsrs, doc(cfg(feature = "b64")))]
#[derive(BitSet, Clone, Copy, Default, Eq, Hash, PartialEq)]
#[bitset(debug, indices, iter, tests)]
pub struct BitSet64(u64);

/// A set of 128 bits.
#[cfg(feature = "b128")]
#[cfg_attr(docsrs, doc(cfg(feature = "b128")))]
#[derive(BitSet, Clone, Copy, Default, Eq, Hash, PartialEq)]
#[bitset(debug, indices, iter, tests)]
pub struct BitSet128(u128);

/// A bitset the length of a pointer.
///
/// I doubt there's a good use for a type like this, but it's here for the sake of completeness
/// (though it is locked behind a feature flag that's disabled by default).
#[cfg(feature = "bsize")]
#[cfg_attr(docsrs, doc(cfg(feature = "bsize")))]
#[derive(BitSet, Clone, Copy, Default, Eq, Hash, PartialEq)]
#[bitset(debug, indices, iter, tests)]
pub struct BitSetSize(usize);

/// An iteration order that starts with the smallest end/items and ends with the largest.
pub struct Ascending;

/// An iteration order that starts with the largest end/items and ends with the smallest.
pub struct Descending;

#[cfg(test)]
mod iter_tests {
    #[cfg(feature = "rose-bitset-derive")]
    use crate::{Ascending, Descending};
    #[cfg(feature = "rose-bitset-derive")]
    use rstest::rstest;
    #[cfg(feature = "rose-bitset-derive")]
    use std::iter::zip;

    #[cfg(feature = "b8")]
    mod b8 {
        use super::*;
        use crate::BitSet8;

        const SET_1: BitSet8 = BitSet8::from_bits(0b00101110);
        const INDICES_1: &[usize] = &[1, 2, 3, 5];
        const BITS_1: &[bool; 8] = &[false, true, true, true, false, true, false, false];

        const SET_2: BitSet8 = BitSet8::from_bits(0b10000101);
        const INDICES_2: &[usize] = &[0, 2, 7];
        const BITS_2: &[bool; 8] = &[true, false, true, false, false, false, false, true];

        const SET_3: BitSet8 = BitSet8::from_bits(0b01100101);
        const INDICES_3: &[usize] = &[0, 2, 5, 6];
        const BITS_3: &[bool; 8] = &[true, false, true, false, false, true, true, false];

        const SET_4: BitSet8 = BitSet8::from_bits(0b01101011);
        const INDICES_4: &[usize] = &[0, 1, 3, 5, 6];
        const BITS_4: &[bool; 8] = &[true, true, false, true, false, true, true, false];

        const SET_5: BitSet8 = BitSet8::new();
        const INDICES_5: &[usize] = &[];
        const BITS_5: &[bool; 8] = &[false; 8];

        const SET_6: BitSet8 = BitSet8::all();
        const INDICES_6: &[usize] = &[0, 1, 2, 3, 4, 5, 6, 7];
        const BITS_6: &[bool; 8] = &[true; 8];

        #[rstest]
        #[case(SET_1, INDICES_1)]
        #[case(SET_2, INDICES_2)]
        #[case(SET_3, INDICES_3)]
        #[case(SET_4, INDICES_4)]
        #[case(SET_5, INDICES_5)]
        #[case(SET_6, INDICES_6)]
        fn ascending_indices(#[case] set: BitSet8, #[case] indices: &[usize]) {
            for (&lhs, rhs) in zip(indices, set.iter_indices::<Ascending>()) {
                assert_eq!(lhs, rhs);
            }
        }

        #[rstest]
        #[case(SET_1, INDICES_1)]
        #[case(SET_2, INDICES_2)]
        #[case(SET_3, INDICES_3)]
        #[case(SET_4, INDICES_4)]
        #[case(SET_5, INDICES_5)]
        #[case(SET_6, INDICES_6)]
        fn descending_indices(#[case] set: BitSet8, #[case] indices: &[usize]) {
            for (&lhs, rhs) in zip(indices.iter().rev(), set.iter_indices::<Descending>()) {
                assert_eq!(lhs, rhs);
            }
        }

        #[rstest]
        #[case(SET_1, BITS_1)]
        #[case(SET_2, BITS_2)]
        #[case(SET_3, BITS_3)]
        #[case(SET_4, BITS_4)]
        #[case(SET_5, BITS_5)]
        #[case(SET_6, BITS_6)]
        fn ascending_bits(#[case] set: BitSet8, #[case] bits: &[bool]) {
            for (&lhs, rhs) in zip(bits, set.iter_bits::<Ascending>()) {
                assert_eq!(lhs, rhs);
            }
        }

        #[rstest]
        #[case(SET_1, BITS_1)]
        #[case(SET_2, BITS_2)]
        #[case(SET_3, BITS_3)]
        #[case(SET_4, BITS_4)]
        #[case(SET_5, BITS_5)]
        #[case(SET_6, BITS_6)]
        fn descending_bits(#[case] set: BitSet8, #[case] bits: &[bool]) {
            for (&lhs, rhs) in zip(bits.iter().rev(), set.iter_bits::<Descending>()) {
                assert_eq!(lhs, rhs);
            }
        }
    }

    #[cfg(feature = "b16")]
    mod b16 {
        use super::*;
        use crate::BitSet16;

        const SET_1: BitSet16 = BitSet16::from_bits(0b1010011001000000);
        const INDICES_1: &[usize] = &[6, 9, 10, 13, 15];
        const BITS_1: &[bool; 16] = &[
            false, false, false, false, false, false, true, false, false, true, true, false, false,
            true, false, true,
        ];

        const SET_2: BitSet16 = BitSet16::from_bits(0b1100000011000010);
        const INDICES_2: &[usize] = &[1, 6, 7, 14, 15];
        const BITS_2: &[bool; 16] = &[
            false, true, false, false, false, false, true, true, false, false, false, false, false,
            false, true, true,
        ];

        const SET_3: BitSet16 = BitSet16::from_bits(0b1111111001000011);
        const INDICES_3: &[usize] = &[0, 1, 6, 9, 10, 11, 12, 13, 14, 15];
        const BITS_3: &[bool; 16] = &[
            true, true, false, false, false, false, true, false, false, true, true, true, true,
            true, true, true,
        ];

        const SET_4: BitSet16 = BitSet16::from_bits(0b1101100010101011);
        const INDICES_4: &[usize] = &[0, 1, 3, 5, 7, 11, 12, 14, 15];
        const BITS_4: &[bool; 16] = &[
            true, true, false, true, false, true, false, true, false, false, false, true, true,
            false, true, true,
        ];

        const SET_5: BitSet16 = BitSet16::new();
        const INDICES_5: &[usize] = &[];
        const BITS_5: &[bool; 16] = &[false; 16];

        const SET_6: BitSet16 = BitSet16::all();
        const INDICES_6: &[usize] = &[0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15];
        const BITS_6: &[bool; 16] = &[true; 16];

        #[rstest]
        #[case(SET_1, INDICES_1)]
        #[case(SET_2, INDICES_2)]
        #[case(SET_3, INDICES_3)]
        #[case(SET_4, INDICES_4)]
        #[case(SET_5, INDICES_5)]
        #[case(SET_6, INDICES_6)]
        fn ascending_indices(#[case] set: BitSet16, #[case] indices: &[usize]) {
            for (&lhs, rhs) in zip(indices, set.iter_indices::<Ascending>()) {
                assert_eq!(lhs, rhs);
            }
        }

        #[rstest]
        #[case(SET_1, INDICES_1)]
        #[case(SET_2, INDICES_2)]
        #[case(SET_3, INDICES_3)]
        #[case(SET_4, INDICES_4)]
        #[case(SET_5, INDICES_5)]
        #[case(SET_6, INDICES_6)]
        fn descending_indices(#[case] set: BitSet16, #[case] indices: &[usize]) {
            for (&lhs, rhs) in zip(indices.iter().rev(), set.iter_indices::<Descending>()) {
                assert_eq!(lhs, rhs);
            }
        }

        #[rstest]
        #[case(SET_1, BITS_1)]
        #[case(SET_2, BITS_2)]
        #[case(SET_3, BITS_3)]
        #[case(SET_4, BITS_4)]
        #[case(SET_5, BITS_5)]
        #[case(SET_6, BITS_6)]
        fn ascending_bits(#[case] set: BitSet16, #[case] bits: &[bool]) {
            for (&lhs, rhs) in zip(bits, set.iter_bits::<Ascending>()) {
                assert_eq!(lhs, rhs);
            }
        }

        #[rstest]
        #[case(SET_1, BITS_1)]
        #[case(SET_2, BITS_2)]
        #[case(SET_3, BITS_3)]
        #[case(SET_4, BITS_4)]
        #[case(SET_5, BITS_5)]
        #[case(SET_6, BITS_6)]
        fn descending_bits(#[case] set: BitSet16, #[case] bits: &[bool]) {
            for (&lhs, rhs) in zip(bits.iter().rev(), set.iter_bits::<Descending>()) {
                assert_eq!(lhs, rhs);
            }
        }
    }

    #[cfg(feature = "b32")]
    mod b32 {
        use super::*;
        use crate::BitSet32;

        const SET_1: BitSet32 = BitSet32::from_bits(0x9fcfc89d);
        const INDICES_1: &[usize] = &[
            0, 2, 3, 4, 7, 11, 14, 15, 16, 17, 18, 19, 22, 23, 24, 25, 26, 27, 28, 31,
        ];
        const BITS_1: &[bool; 32] = &[
            true, false, true, true, true, false, false, true, false, false, false, true, false,
            false, true, true, true, true, true, true, false, false, true, true, true, true, true,
            true, true, false, false, true,
        ];

        const SET_2: BitSet32 = BitSet32::from_bits(0xa8e4862e);
        const INDICES_2: &[usize] = &[1, 2, 3, 5, 9, 10, 15, 18, 21, 22, 23, 27, 29, 31];
        const BITS_2: &[bool; 32] = &[
            false, true, true, true, false, true, false, false, false, true, true, false, false,
            false, false, true, false, false, true, false, false, true, true, true, false, false,
            false, true, false, true, false, true,
        ];

        const SET_3: BitSet32 = BitSet32::from_bits(0x0876a181);
        const INDICES_3: &[usize] = &[0, 7, 8, 13, 15, 17, 18, 20, 21, 22, 27];
        const BITS_3: &[bool; 32] = &[
            true, false, false, false, false, false, false, true, true, false, false, false, false,
            true, false, true, false, true, true, false, true, true, true, false, false, false,
            false, true, false, false, false, false,
        ];

        const SET_4: BitSet32 = BitSet32::from_bits(0x667e1426);
        const INDICES_4: &[usize] = &[1, 2, 5, 10, 12, 17, 18, 19, 20, 21, 22, 25, 26, 29, 30];
        const BITS_4: &[bool; 32] = &[
            false, true, true, false, false, true, false, false, false, false, true, false, true,
            false, false, false, false, true, true, true, true, true, true, false, false, true,
            true, false, false, true, true, false,
        ];

        const SET_5: BitSet32 = BitSet32::new();
        const INDICES_5: &[usize] = &[];
        const BITS_5: &[bool; 32] = &[false; 32];

        const SET_6: BitSet32 = BitSet32::all();
        const INDICES_6: &[usize] = &[
            0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23,
            24, 25, 26, 27, 28, 29, 30, 31,
        ];
        const BITS_6: &[bool; 32] = &[true; 32];

        #[rstest]
        #[case(SET_1, INDICES_1)]
        #[case(SET_2, INDICES_2)]
        #[case(SET_3, INDICES_3)]
        #[case(SET_4, INDICES_4)]
        #[case(SET_5, INDICES_5)]
        #[case(SET_6, INDICES_6)]
        fn ascending_indices(#[case] set: BitSet32, #[case] indices: &[usize]) {
            for (&lhs, rhs) in zip(indices, set.iter_indices::<Ascending>()) {
                assert_eq!(lhs, rhs);
            }
        }

        #[rstest]
        #[case(SET_1, INDICES_1)]
        #[case(SET_2, INDICES_2)]
        #[case(SET_3, INDICES_3)]
        #[case(SET_4, INDICES_4)]
        #[case(SET_5, INDICES_5)]
        #[case(SET_6, INDICES_6)]
        fn descending_indices(#[case] set: BitSet32, #[case] indices: &[usize]) {
            for (&lhs, rhs) in zip(indices.iter().rev(), set.iter_indices::<Descending>()) {
                assert_eq!(lhs, rhs);
            }
        }

        #[rstest]
        #[case(SET_1, BITS_1)]
        #[case(SET_2, BITS_2)]
        #[case(SET_3, BITS_3)]
        #[case(SET_4, BITS_4)]
        #[case(SET_5, BITS_5)]
        #[case(SET_6, BITS_6)]
        fn ascending_bits(#[case] set: BitSet32, #[case] bits: &[bool]) {
            for (&lhs, rhs) in zip(bits, set.iter_bits::<Ascending>()) {
                assert_eq!(lhs, rhs);
            }
        }

        #[rstest]
        #[case(SET_1, BITS_1)]
        #[case(SET_2, BITS_2)]
        #[case(SET_3, BITS_3)]
        #[case(SET_4, BITS_4)]
        #[case(SET_5, BITS_5)]
        #[case(SET_6, BITS_6)]
        fn descending_bits(#[case] set: BitSet32, #[case] bits: &[bool]) {
            for (&lhs, rhs) in zip(bits.iter().rev(), set.iter_bits::<Descending>()) {
                assert_eq!(lhs, rhs);
            }
        }
    }

    #[cfg(feature = "b64")]
    mod b64 {
        use super::*;
        use crate::BitSet64;

        const SET_1: BitSet64 = BitSet64::from_bits(0x64dc46ef58cbc169);
        const INDICES_1: &[usize] = &[
            0, 3, 5, 6, 8, 14, 15, 16, 17, 19, 22, 23, 27, 28, 30, 32, 33, 34, 35, 37, 38, 39, 41,
            42, 46, 50, 51, 52, 54, 55, 58, 61, 62,
        ];
        const BITS_1: &[bool; 64] = &[
            true, false, false, true, false, true, true, false, true, false, false, false, false,
            false, true, true, true, true, false, true, false, false, true, true, false, false,
            false, true, true, false, true, false, true, true, true, true, false, true, true, true,
            false, true, true, false, false, false, true, false, false, false, true, true, true,
            false, true, true, false, false, true, false, false, true, true, false,
        ];

        const SET_2: BitSet64 = BitSet64::from_bits(0x5df951b7aaac5647);
        const INDICES_2: &[usize] = &[
            0, 1, 2, 6, 9, 10, 12, 14, 18, 19, 21, 23, 25, 27, 29, 31, 32, 33, 34, 36, 37, 39, 40,
            44, 46, 48, 51, 52, 53, 54, 55, 56, 58, 59, 60, 62,
        ];
        const BITS_2: &[bool; 64] = &[
            true, true, true, false, false, false, true, false, false, true, true, false, true,
            false, true, false, false, false, true, true, false, true, false, true, false, true,
            false, true, false, true, false, true, true, true, true, false, true, true, false,
            true, true, false, false, false, true, false, true, false, true, false, false, true,
            true, true, true, true, true, false, true, true, true, false, true, false,
        ];

        const SET_3: BitSet64 = BitSet64::from_bits(0x0863f744162ebcfd);
        const INDICES_3: &[usize] = &[
            0, 2, 3, 4, 5, 6, 7, 10, 11, 12, 13, 15, 17, 18, 19, 21, 25, 26, 28, 34, 38, 40, 41,
            42, 44, 45, 46, 47, 48, 49, 53, 54, 59,
        ];
        const BITS_3: &[bool; 64] = &[
            true, false, true, true, true, true, true, true, false, false, true, true, true, true,
            false, true, false, true, true, true, false, true, false, false, false, true, true,
            false, true, false, false, false, false, false, true, false, false, false, true, false,
            true, true, true, false, true, true, true, true, true, true, false, false, false, true,
            true, false, false, false, false, true, false, false, false, false,
        ];

        const SET_4: BitSet64 = BitSet64::from_bits(0x21ce32fc5754e1db);
        const INDICES_4: &[usize] = &[
            0, 1, 3, 4, 6, 7, 8, 13, 14, 15, 18, 20, 22, 24, 25, 26, 28, 30, 34, 35, 36, 37, 38,
            39, 41, 44, 45, 49, 50, 51, 54, 55, 56, 61,
        ];
        const BITS_4: &[bool; 64] = &[
            true, true, false, true, true, false, true, true, true, false, false, false, false,
            true, true, true, false, false, true, false, true, false, true, false, true, true,
            true, false, true, false, true, false, false, false, true, true, true, true, true,
            true, false, true, false, false, true, true, false, false, false, true, true, true,
            false, false, true, true, true, false, false, false, false, true, false, false,
        ];

        const SET_5: BitSet64 = BitSet64::new();
        const INDICES_5: &[usize] = &[];
        const BITS_5: &[bool; 64] = &[false; 64];

        const SET_6: BitSet64 = BitSet64::all();
        const INDICES_6: &[usize] = &[
            0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23,
            24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45,
            46, 47, 48, 49, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 60, 61, 62, 63,
        ];
        const BITS_6: &[bool; 64] = &[true; 64];

        #[rstest]
        #[case(SET_1, INDICES_1)]
        #[case(SET_2, INDICES_2)]
        #[case(SET_3, INDICES_3)]
        #[case(SET_4, INDICES_4)]
        #[case(SET_5, INDICES_5)]
        #[case(SET_6, INDICES_6)]
        fn ascending_indices(#[case] set: BitSet64, #[case] indices: &[usize]) {
            for (&lhs, rhs) in zip(indices, set.iter_indices::<Ascending>()) {
                assert_eq!(lhs, rhs);
            }
        }

        #[rstest]
        #[case(SET_1, INDICES_1)]
        #[case(SET_2, INDICES_2)]
        #[case(SET_3, INDICES_3)]
        #[case(SET_4, INDICES_4)]
        #[case(SET_5, INDICES_5)]
        #[case(SET_6, INDICES_6)]
        fn descending_indices(#[case] set: BitSet64, #[case] indices: &[usize]) {
            for (&lhs, rhs) in zip(indices.iter().rev(), set.iter_indices::<Descending>()) {
                assert_eq!(lhs, rhs);
            }
        }

        #[rstest]
        #[case(SET_1, BITS_1)]
        #[case(SET_2, BITS_2)]
        #[case(SET_3, BITS_3)]
        #[case(SET_4, BITS_4)]
        #[case(SET_5, BITS_5)]
        #[case(SET_6, BITS_6)]
        fn ascending_bits(#[case] set: BitSet64, #[case] bits: &[bool]) {
            for (&lhs, rhs) in zip(bits, set.iter_bits::<Ascending>()) {
                assert_eq!(lhs, rhs);
            }
        }

        #[rstest]
        #[case(SET_1, BITS_1)]
        #[case(SET_2, BITS_2)]
        #[case(SET_3, BITS_3)]
        #[case(SET_4, BITS_4)]
        #[case(SET_5, BITS_5)]
        #[case(SET_6, BITS_6)]
        fn descending_bits(#[case] set: BitSet64, #[case] bits: &[bool]) {
            for (&lhs, rhs) in zip(bits.iter().rev(), set.iter_bits::<Descending>()) {
                assert_eq!(lhs, rhs);
            }
        }
    }
}
