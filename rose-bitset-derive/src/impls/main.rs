use crate::type_utils::repr;
use proc_macro_error::abort;
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::{Data, DataEnum, DataStruct, DataUnion, Ident, Type, spanned::Spanned};

pub fn get_uint(data: Data) -> Type {
    let fields = match data {
        Data::Struct(DataStruct { fields, .. }) => fields,
        Data::Enum(DataEnum { enum_token, .. }) => {
            abort!(
                enum_token.span(), "unexpected keyword `enum` (expected `struct`)";
                note = "`BitSet` can only be derived for structs, not enums or unions";
            );
        }
        Data::Union(DataUnion { union_token, .. }) => {
            abort!(
                union_token.span(), "unexpected keyword `union` (expected `struct`)";
                note = "`BitSet` can only be derived for structs, not enums or unions";
            );
        }
    };
    let mut fields_iter = fields.iter();
    let field = if let (Some(field), None) = (fields_iter.next(), fields_iter.next()) {
        field
    } else {
        abort!(fields.span(), "found {} fields (expected 1)", fields.len());
    };
    if let Some(ident) = &field.ident {
        abort!(
            ident.span(), "unexpected identifier";
            note = "a bitset must be a tuple struct";
        );
    }
    field.ty.clone()
}

pub fn generate_code(ident: Ident, uint: Type) -> TokenStream {
    let uint_repr = repr(&uint);
    let uint_size = if uint_repr == "usize" {
        size_of::<usize>()
    } else {
        uint_repr
            .strip_prefix('u')
            .and_then(|uint_repr| uint_repr.parse::<usize>().ok())
            .unwrap_or_else(|| {
                abort!(
                    uint.span(), "unexpected type";
                    note = "the underlying type of a bitset must be a primitive, unsigned integer";
                );
            })
    };
    let bitset_link = format!("[`{ident}`].");
    let plural_bitset_link = format!("[`{ident}`]s.");
    let clear_i_to_size = format_ident!("clear_i_to_{uint_size}");
    let cleared_i_to_size = format_ident!("cleared_i_to_{uint_size}");
    let mask_i_to_size = format_ident!("mask_i_to_{uint_size}");
    let masked_i_to_size = format_ident!("masked_i_to_{uint_size}");
    let index_to_size = format!("`index..{uint_size}`");
    let index_to_size_with_comma = format!("{index_to_size},");
    let index_to_size_with_period = format!("{index_to_size}.\n");
    quote! {
        impl ::core::ops::BitAnd for #ident {
            type Output = Self;

            fn bitand(self, rhs: Self) -> Self::Output {
                Self::intersection(self, rhs)
            }
        }

        impl ::core::ops::BitAndAssign for #ident {
            fn bitand_assign(&mut self, rhs: Self) {
                *self = *self & rhs;
            }
        }

        impl ::core::ops::BitOr for #ident {
            type Output = Self;

            fn bitor(self, rhs: Self) -> Self::Output {
                Self::union(self, rhs)
            }
        }

        impl ::core::ops::BitOrAssign for #ident {
            fn bitor_assign(&mut self, rhs: Self) {
                *self = *self | rhs;
            }
        }

        impl ::core::ops::BitXor for #ident {
            type Output = Self;

            fn bitxor(self, rhs: Self) -> Self::Output {
                Self::symmetric_difference(self, rhs)
            }
        }

        impl ::core::ops::BitXorAssign for #ident {
            fn bitxor_assign(&mut self, rhs: Self) {
                *self = *self ^ rhs;
            }
        }

        impl ::core::convert::From<#uint> for #ident {
            fn from(value: #uint) -> Self {
                Self::from_bits(value)
            }
        }

        impl ::core::convert::From<#ident> for #uint {
            fn from(value: #ident) -> Self {
                value.bits()
            }
        }

        impl ::core::ops::Neg for #ident {
            type Output = Self;

            fn neg(self) -> Self::Output {
                self.complement()
            }
        }

        impl ::core::ops::Not for #ident {
            type Output = Self;

            fn not(self) -> Self::Output {
                self.complement()
            }
        }

        impl ::core::ops::Sub for #ident {
            type Output = Self;

            fn sub(self, rhs: Self) -> Self::Output {
                Self::difference(self, rhs)
            }
        }

        impl ::core::ops::SubAssign for #ident {
            fn sub_assign(&mut self, rhs: Self) {
                *self = *self - rhs;
            }
        }

        impl #ident {
            #[doc = "The capacity of a"]
            #[doc = #bitset_link]
            pub const CAPACITY: usize = 8 * ::core::mem::size_of::<Self>();

            const __ONE: #uint = 1;

            /// Creates an empty set.
            #[doc(alias = "empty")]
            #[must_use]
            pub const fn new() -> Self {
                Self(0)
            }

            /// Creates a set containing with only the bit at `index` set.
            ///
            /// If `index >=`[`Self::CAPACITY`], the resulting set will be empty.
            #[must_use]
            pub const fn unit(index: usize) -> Self {
                if let Some(bits) = Self::__ONE.checked_shl(index as u32) {
                    Self::from_bits(bits)
                } else {
                    Self::new()
                }
            }

            /// Creates a set with all bits set.
            #[must_use]
            pub const fn all() -> Self {
                Self(<#uint>::MAX)
            }

            /// Creates a set with the given bits.
            #[must_use]
            pub const fn from_bits(bits: #uint) -> Self {
                Self(bits)
            }

            /// Returns the underlying bits of the set.
            #[must_use]
            pub const fn bits(self) -> #uint {
                self.0
            }

            #[doc = "Creates a new set that complements `self`.\n"]
            #[doc = "This is the `const` alternative to"]
            #[doc = "[`Neg::neg`](https://doc.rust-lang.org/core/ops/trait.Neg.html#tymethod.neg)"]
            #[doc = "or"]
            #[doc = "[`Not::not`](https://doc.rust-lang.org/core/ops/trait.Not.html#tymethod.not)"]
            #[doc = "for"]
            #[doc = #plural_bitset_link]
            #[doc(alias = "inverse")]
            #[must_use]
            pub const fn complement(self) -> Self {
                Self(!self.0)
            }

            #[doc = "Creates a new set with values that are in both `self` and `rhs`.\n"]
            #[doc = "This is the `const` alternative to"]
            #[doc = "[`BitAnd::bitand`](https://doc.rust-lang.org/core/ops/trait.BitAnd.html#tymethod.bitand)"]
            #[doc = "for"]
            #[doc = #plural_bitset_link]
            #[must_use]
            pub const fn intersection(self, rhs: Self) -> Self {
                Self(self.0 & rhs.0)
            }

            #[doc = "Creates a new set with values that are in `self` or `rhs`.\n"]
            #[doc = "This is the `const` alternative to"]
            #[doc = "[`BitOr::bitor`](https://doc.rust-lang.org/core/ops/trait.BitOr.html#tymethod.bitor)"]
            #[doc = "for"]
            #[doc = #plural_bitset_link]
            #[must_use]
            pub const fn union(self, rhs: Self) -> Self {
                Self(self.0 | rhs.0)
            }

            #[doc = "Creates a new set with values that are in `self`, but not in `rhs`.\n"]
            #[doc = "This is the `const` alternative to"]
            #[doc = "[`Sub::sub`](https://doc.rust-lang.org/core/ops/trait.Sub.html#tymethod.sub)"]
            #[doc = "for"]
            #[doc = #plural_bitset_link]
            #[must_use]
            pub const fn difference(self, rhs: Self) -> Self {
                Self(self.0 & !rhs.0)
            }

            #[doc = "Creates a new set with values that are in `self` or `rhs`, but not in both.\n"]
            #[doc = "This is the `const` alternative to"]
            #[doc = "[`BitXor::bitxor`](https://doc.rust-lang.org/core/ops/trait.BitXor.html#tymethod.bitxor)"]
            #[doc = "for"]
            #[doc = #plural_bitset_link]
            #[must_use]
            pub const fn symmetric_difference(self, rhs: Self) -> Self {
                Self(self.0 ^ rhs.0)
            }

            #[doc = "Returns `true` if `self` is equal to `rhs`,"]
            #[doc = "i.e., both sets have the same exact values.\n"]
            #[doc = "This is the `const` alternative to"]
            #[doc = "[`PartialEq::eq`](https://doc.rust-lang.org/core/cmp/trait.PartialEq.html#tymethod.eq)"]
            #[doc = "for"]
            #[doc = #plural_bitset_link]
            #[must_use]
            pub const fn is(self, rhs: Self) -> bool {
                self.0 == rhs.0
            }

            #[doc = "Returns `true` if `self` is not equal to `rhs`,"]
            #[doc = "i.e., the sets do not have exactly the same values.\n"]
            #[doc = "This is the `const` alternative to"]
            #[doc = "[`PartialEq::ne`](https://doc.rust-lang.org/core/cmp/trait.PartialEq.html#tymethod.ne)"]
            #[doc = "for"]
            #[doc = #plural_bitset_link]
            #[must_use]
            pub const fn is_not(self, rhs: Self) -> bool {
                self.0 != rhs.0
            }

            /// Returns `true` if `self` has no elements in common with `rhs`. This is equivalent to
            /// checking for an empty intersection.
            #[must_use]
            pub const fn is_disjoint(self, rhs: Self) -> bool {
                self.intersection(rhs).is_empty()
            }

            /// Returns `true` if the set is a subset of another, i.e., `rhs` contains at least all
            /// the values in `self`.
            #[must_use]
            pub const fn is_subset(self, rhs: Self) -> bool {
                self.union(rhs).is(rhs)
            }

            /// Returns `true` if the set is a strict subset of another, i.e., `rhs` contains all
            /// the values in `self` **and** is larger than `self`.
            #[must_use]
            pub const fn is_strict_subset(self, rhs: Self) -> bool {
                self.is_subset(rhs) && self.is_not(rhs)
            }

            /// Returns `true` if the set is a superset of another, i.e., `self` contains at least
            /// all the values in `rhs`.
            #[must_use]
            pub const fn is_superset(self, rhs: Self) -> bool {
                rhs.is_subset(self)
            }

            /// Returns `true` if the set is a strict superset of another, i.e., `self` contains all
            /// the values in `rhs` **and** is larger than `rhs`.
            #[must_use]
            pub const fn is_strict_superset(self, rhs: Self) -> bool {
                self.is_superset(rhs) && self.is_not(rhs)
            }

            /// Returns `true` if the set contains no elements.
            #[must_use]
            pub const fn is_empty(self) -> bool {
                self.0 == 0
            }

            /// Returns `true` if the set contains all [`Self::CAPACITY`] elements.
            #[must_use]
            pub const fn is_full(self) -> bool {
                self.0 == #uint::MAX
            }

            /// Returns the number of elements in the set.
            #[must_use]
            pub const fn len(self) -> usize {
                self.0.count_ones() as usize
            }

            /// Returns `true` if the bit at `index` is set.
            #[must_use]
            pub const fn contains(self, index: usize) -> bool {
                if let Some(mask) = Self::__ONE.checked_shl(index as u32) {
                    self.0 & mask != 0
                } else {
                    false
                }
            }

            /// Gets the bit at `index`.
            ///
            /// If `index >=`[`Self::CAPACITY`], this will simply return `false`.
            #[must_use]
            pub const fn get(self, index: usize) -> bool {
                self.contains(index)
            }

            /// Returns the index of the least significant bit that is set.
            ///
            /// If no bits are set, this returns [`Self::CAPACITY`].
            #[must_use]
            pub const fn min_index(self) -> usize {
                self.0.trailing_zeros() as usize
            }

            /// Returns the index of the most significant bit that is set.
            ///
            /// # Panics
            ///
            /// Panics if no bits are set. For a non-panicking alternative, see
            /// [`max_index_checked`](Self::max_index_checked).
            #[must_use]
            pub const fn max_index(self) -> usize {
                self.0.ilog2() as usize
            }

            /// Returns the index of the most significant bit that is set, or [`None`] if no bits
            /// are set.
            #[must_use]
            pub const fn max_index_checked(self) -> ::core::option::Option<usize> {
                if let ::core::option::Option::Some(max) = self.0.checked_ilog2() {
                    ::core::option::Option::Some(max as usize)
                } else {
                    ::core::option::Option::None
                }
            }

            /// Clears the set, removing all values.
            pub const fn clear(&mut self) {
                self.0 = 0;
            }

            /// Creates a copy of this set that only has values less than `index`.
            ///
            /// # Panics
            ///
            /// Panics if `index >=`[`Self::CAPACITY`].
            #[must_use]
            pub const fn masked_0_to_i(mut self, index: usize) -> Self {
                self.mask_0_to_i(index);
                self
            }

            /// Removes any bits with indices outside the range `0..index`.
            ///
            /// # Panics
            ///
            /// Panics if `index >=`[`Self::CAPACITY`].
            pub const fn mask_0_to_i(&mut self, index: usize) {
                Self::#clear_i_to_size(self, index);
            }

            /// Creates a copy of this set that only has bits with indices greater than or equal to
            /// `index`.
            ///
            /// # Panics
            ///
            /// Panics if `index >=`[`Self::CAPACITY`].
            #[must_use]
            pub const fn #masked_i_to_size(mut self, index: usize) -> Self {
                Self::#mask_i_to_size(&mut self, index);
                self
            }

            #[doc = "Removes any bits with indices outside the range"]
            #[doc = #index_to_size_with_period]
            #[doc = "# Panics\n"]
            #[doc = "Panics if `index >=`[`Self::CAPACITY`]."]
            pub const fn #mask_i_to_size(&mut self, index: usize) {
                self.clear_0_to_i(index);
            }

            /// Creates a copy of this set without the bits with indices in the range `0..index`.
            ///
            /// # Panics
            ///
            /// Panics if `index >=`[`Self::CAPACITY`].
            #[must_use]
            pub const fn cleared_0_to_i(mut self, index: usize) -> Self {
                self.clear_0_to_i(index);
                self
            }

            #[doc = "Clears bits `0..index`, keeping bits"]
            #[doc = #index_to_size]
            #[doc = "in their original states.\n"]
            #[doc = "# Panics\n"]
            #[doc = "Panics if `index >=`[`Self::CAPACITY`]."]
            pub const fn clear_0_to_i(&mut self, index: usize) {
                self.0 &= !((Self::__ONE << index) - 1);
            }

            #[doc = "Creates a copy of this set without the bits with indices in the range"]
            #[doc = #index_to_size_with_period]
            #[doc = "# Panics\n"]
            #[doc = "Panics if `index >=`[`Self::CAPACITY`]."]
            #[must_use]
            pub const fn #cleared_i_to_size(mut self, index: usize) -> Self {
                Self::#clear_i_to_size(&mut self, index);
                self
            }

            #[doc = "Clears bits"]
            #[doc = #index_to_size_with_comma]
            #[doc = "keeping bits `0..index` in their original states.\n"]
            #[doc = "# Panics\n"]
            #[doc = "Panics if `index >=`[`Self::CAPACITY`]."]
            pub const fn #clear_i_to_size(&mut self, index: usize) {
                self.0 &= (Self::__ONE << index) - 1;
            }

            /// Sets the bit at `index` to `1`.
            ///
            /// If you would like to know if the insertion succeeded, use [`insert`](Self::insert)
            /// instead.
            pub const fn insert_quiet(&mut self, index: usize) {
                *self = self.union(Self::unit(index));
            }

            /// Sets the bit at `index` to `1`. Returns whether the bit was not already set.
            ///
            /// If the return value is not needed, use [`insert_quiet`](Self::insert_quiet) instead.
            #[must_use = "consider using the return value or calling `insert_quiet` instead"]
            pub const fn insert(&mut self, index: usize) -> bool {
                let old_set = *self;
                self.insert_quiet(index);
                old_set.is_not(*self)
            }

            /// Sets the bit at `index` to `bit`.
            ///
            /// If you would like to know the old value of the bit, use [`replace`](Self::replace)
            /// instead.
            pub const fn replace_quiet(&mut self, index: usize, bit: bool) {
                if bit {
                    self.insert_quiet(index)
                } else {
                    self.remove_quiet(index)
                }
            }

            /// Sets the bit at `index` to `bit`.
            ///
            /// If the return value is not needed, use [`replace_quiet`](Self::replace_quiet)
            /// instead.
            #[must_use = "consider using the return value or calling `replace_quiet` instead"]
            pub const fn replace(&mut self, index: usize, bit: bool) -> bool {
                let old_set = *self;
                if bit {
                    self.insert_quiet(index);
                    old_set.is(*self)
                } else {
                    self.remove_quiet(index);
                    old_set.is_not(*self)
                }
            }

            /// Sets the bit at `index` to `0`.
            ///
            /// If you would like to know if the removal succeeded, use [`remove`](Self::remove)
            /// instead.
            pub const fn remove_quiet(&mut self, index: usize) {
                *self = self.difference(Self::unit(index));
            }

            /// Sets the bit at `index` to `0`. Returns whether the bit was set.
            ///
            /// If the return value is not needed, use [`remove_quiet`](Self::remove_quiet) instead.
            #[must_use = "consider using the return value or calling `remove_quiet` instead"]
            pub const fn remove(&mut self, index: usize) -> bool {
                let old_set = *self;
                self.remove_quiet(index);
                old_set.is_not(*self)
            }
        }
    }
}
