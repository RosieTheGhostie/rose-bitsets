use proc_macro2::TokenStream;
use quote::quote;
use syn::Ident;

pub fn generate_code(ident: &Ident) -> TokenStream {
    quote! {
        impl ::core::fmt::Debug for #ident {
            fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                f.debug_set()
                    .entries(self.iter_indices::<crate::Ascending>())
                    .finish()
            }
        }
    }
}
