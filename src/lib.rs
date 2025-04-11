#![doc = include_str!("../README.md")]
use rose_bitset_derive::BitSet;

/// A set of 8 bits.
#[cfg(feature = "b8")]
#[derive(BitSet, Clone, Copy, Default, Eq, Hash, PartialEq)]
#[bitset(debug, indices, iter, tests)]
pub struct BitSet8(u8);

/// A set of 16 bits.
#[cfg(feature = "b16")]
#[derive(BitSet, Clone, Copy, Default, Eq, Hash, PartialEq)]
#[bitset(debug, indices, iter, tests)]
pub struct BitSet16(u16);

/// A set of 32 bits.
#[cfg(feature = "b32")]
#[derive(BitSet, Clone, Copy, Default, Eq, Hash, PartialEq)]
#[bitset(debug, indices, iter, tests)]
pub struct BitSet32(u32);

/// A set of 64 bits.
#[cfg(feature = "b64")]
#[derive(BitSet, Clone, Copy, Default, Eq, Hash, PartialEq)]
#[bitset(debug, indices, iter, tests)]
pub struct BitSet64(u64);

/// A set of 128 bits.
#[cfg(feature = "b128")]
#[derive(BitSet, Clone, Copy, Default, Eq, Hash, PartialEq)]
#[bitset(debug, indices, iter, tests)]
pub struct BitSet128(u128);

/// A bitset the length of a pointer.
///
/// I doubt there's a good use for a type like this, but it's here for the sake of completeness
/// (though it is locked behind a feature flag that's disabled by default).
#[cfg(feature = "bsize")]
#[derive(BitSet, Clone, Copy, Default, Eq, Hash, PartialEq)]
#[bitset(debug, indices, iter, tests)]
pub struct BitSetSize(usize);

/// An iteration order that starts with the smallest end/items and ends with the largest.
pub struct Ascending;

/// An iteration order that starts with the largest end/items and ends with the smallest.
pub struct Descending;
