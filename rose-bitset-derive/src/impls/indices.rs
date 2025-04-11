use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::{Ident, Type};

pub fn generate_code(ident: &Ident, uint: &Type, suffix: &str) -> TokenStream {
    let iterator = format_ident!("BitSetIndices{suffix}");
    let bitset_link = format!("[`{ident}`].");
    quote! {
        #[doc = "An iterator over the indices of the bits that are set in a"]
        #[doc = #bitset_link]
        pub struct #iterator<'a, Direction = crate::Ascending> {
            bits: #uint,
            shift: #uint,
            _markers: (::core::marker::PhantomData<&'a ()>, ::core::marker::PhantomData<Direction>),
        }

        impl ::core::iter::Iterator for #iterator<'_, crate::Ascending> {
            type Item = usize;

            fn next(&mut self) -> ::core::option::Option<Self::Item> {
                let i = self.bits.trailing_zeros() as usize;
                (i < #ident::CAPACITY).then(|| {
                    let relative_shift = unsafe { i.unchecked_add(1) } as #uint;
                    self.bits >>= relative_shift;
                    self.shift += relative_shift;
                    i
                })
            }
        }

        impl ::core::iter::Iterator for #iterator<'_, crate::Descending> {
            type Item = usize;

            fn next(&mut self) -> ::core::option::Option<Self::Item> {
                let relative_shift = {
                    let leading_zeros = self.bits.leading_zeros() as #uint;
                    if leading_zeros == #ident::CAPACITY as #uint {
                        return ::core::option::Option::None;
                    }
                    unsafe { leading_zeros.unchecked_add(1) }
                };
                self.bits <<= relative_shift;
                self.shift += relative_shift;
                ::core::option::Option::Some(unsafe {
                    #ident::CAPACITY.unchecked_sub(self.shift as usize)
                })
            }
        }

        impl<'a, Direction> #iterator<'a, Direction> {
            /// Creates an iterator over the indices of the bits that are set in `set`.
            pub const fn new(set: &'a #ident) -> Self {
                Self {
                    bits: set.bits(),
                    shift: 0,
                    _markers: (::core::marker::PhantomData, ::core::marker::PhantomData),
                }
            }
        }

        impl #ident {
            /// Creates an iterator over the indices of the bits that are set in the set.
            #[must_use]
            pub const fn iter_indices<Direction>(&self) -> #iterator<'_, Direction>
            where
                for<'a> #iterator<'a, Direction>: ::core::iter::Iterator<Item = usize>,
            {
                #iterator::new(self)
            }
        }
    }
}
