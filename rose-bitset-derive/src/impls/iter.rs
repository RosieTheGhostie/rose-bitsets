use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::{Ident, Type};

pub fn generate_code(ident: &Ident, uint: &Type, int: &Type, suffix: &str) -> TokenStream {
    let iterator = format_ident!("BitSetIter{suffix}");
    let ident_link = format!("[`{ident}`].");
    let feature_flag = format!("b{suffix}");
    quote! {
        #[doc = "An iterator over the bits of a"]
        #[doc = #ident_link]
        #[cfg_attr(docsrs, doc(cfg(feature = #feature_flag)))]
        pub struct #iterator<'a, Direction = crate::Ascending> {
            bits: #uint,
            i: #uint,
            _markers: (::core::marker::PhantomData<&'a ()>, ::core::marker::PhantomData<Direction>),
        }

        impl ::core::iter::Iterator for #iterator<'_, crate::Ascending> {
            type Item = bool;

            fn next(&mut self) -> ::core::option::Option<Self::Item> {
                (self.i < #ident::CAPACITY as #uint).then(|| {
                    self.i = unsafe { self.i.unchecked_add(1) };
                    let bit = self.bits & 1 != 0;
                    self.bits >>= 1;
                    bit
                })
            }
        }

        impl ::core::iter::Iterator for #iterator<'_, crate::Descending> {
            type Item = bool;

            fn next(&mut self) -> ::core::option::Option<Self::Item> {
                const MASK: #uint = <#int>::MIN as #uint;
                (self.i < #ident::CAPACITY as #uint).then(|| {
                    self.i = unsafe { self.i.unchecked_add(1) };
                    let bit = self.bits & MASK != 0;
                    self.bits <<= 1;
                    bit
                })
            }
        }

        impl<'a, Direction> #iterator<'a, Direction> {
            /// Creates an iterator over the bits of `set`.
            pub const fn new(set: &'a #ident) -> Self {
                Self {
                    bits: set.bits(),
                    i: 0,
                    _markers: (::core::marker::PhantomData, ::core::marker::PhantomData),
                }
            }
        }

        impl #ident {
            /// Creates an iterator over the bits of the set.
            #[must_use]
            #[cfg_attr(docsrs, doc(cfg(feature = #feature_flag)))]
            pub const fn iter_bits<Direction>(&self) -> #iterator<'_, Direction>
            where
                for<'a> #iterator<'a, Direction>: ::core::iter::Iterator<Item = bool>,
            {
                #iterator::new(self)
            }
        }
    }
}
