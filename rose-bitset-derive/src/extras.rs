use crate::{impls, type_utils::signed_counterpart};
use proc_macro_error::{Diagnostic, DiagnosticExt, Level, SpanRange, abort};
use proc_macro2::{Span, TokenStream};
use syn::{
    Attribute, Error, Ident, Token, Type, parse::Parse, punctuated::Punctuated, spanned::Spanned,
};

#[derive(Clone, Copy, Default)]
pub struct Extras {
    debug: bool,
    indices: bool,
    iter: bool,
    tests: bool,
}

impl Parse for Extras {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let mut extras = Self::default();
        let mut debug_span: Option<Span> = None;
        for ident in Punctuated::<Ident, Token![,]>::parse_terminated(input)? {
            let ident_as_string = ident.to_string();
            match ident_as_string.as_str() {
                "debug" => {
                    if !extras.debug {
                        extras.debug = true;
                        debug_span = Some(ident.span());
                    } else {
                        return Err(syn::Error::new_spanned(
                            ident,
                            "duplicate of `debug` specifier",
                        ));
                    }
                }
                "indices" => {
                    if !extras.indices {
                        extras.indices = true;
                    } else {
                        return Err(syn::Error::new_spanned(
                            ident,
                            "duplicate of `indices` specifier",
                        ));
                    }
                }
                "iter" => {
                    if !extras.iter {
                        extras.iter = true;
                    } else {
                        return Err(syn::Error::new_spanned(
                            ident,
                            "duplicate of `iter` specifier",
                        ));
                    }
                }
                "tests" => {
                    if !extras.tests {
                        extras.tests = true;
                    } else {
                        return Err(syn::Error::new_spanned(
                            ident,
                            "duplicate of `tests` specifier",
                        ));
                    }
                }
                _ => return Err(syn::Error::new_spanned(ident, "unknown specifier")),
            }
        }
        if extras.debug && !extras.indices {
            return Err(syn::Error::new(
                debug_span.unwrap(),
                "cannot implement `Debug` without the `iter_indices` method",
            ));
        }
        Ok(extras)
    }
}

impl Extras {
    pub fn generate_code_from_attribute(
        bitset: &Attribute,
        ident: &Ident,
        uint: &Type,
    ) -> TokenStream {
        match bitset.meta.require_list() {
            Ok(meta_list) => meta_list
                .parse_args::<Extras>()
                .unwrap_or_else(|error| {
                    diagnostic_from_error(error).abort();
                })
                .generate_code(ident, uint),
            Err(_) => abort!(bitset.meta.span(), "expected list"),
        }
    }

    pub fn generate_code(&self, ident: &Ident, uint: &Type) -> TokenStream {
        if self.nothing_to_implement() {
            return TokenStream::new();
        }

        let ident_as_string = ident.to_string();
        let suffix = ident_as_string.strip_prefix("BitSet").unwrap_or_else(|| {
            abort!(
                ident, "bad name";
                help = "bitsets should be named `BitSet{N}`, where N is the size in bits";
            );
        });

        let mut code = TokenStream::new();
        if self.debug {
            code.extend(impls::debug::generate_code(ident));
        }
        if self.indices {
            code.extend(impls::indices::generate_code(ident, uint, suffix));
        }
        if self.iter {
            let int = signed_counterpart(uint)
                .unwrap_or_else(|| abort!(uint, "not a primitive unsigned integer"));
            code.extend(impls::iter::generate_code(ident, uint, &int, suffix));
        }
        if self.tests {
            code.extend(impls::tests::generate_code(
                ident,
                suffix,
                self.debug,
                self.indices,
                self.iter,
            ));
        }

        code
    }

    const fn nothing_to_implement(&self) -> bool {
        !(self.debug || self.indices || self.iter || self.tests)
    }
}

fn diagnostic_from_error(error: Error) -> Diagnostic {
    use proc_macro2::{Delimiter, TokenTree};

    fn gut_error(ts: &mut impl Iterator<Item = TokenTree>) -> Option<(SpanRange, String)> {
        let first = ts.next()?.span();
        ts.next().unwrap(); // !

        let lit = if let TokenTree::Group(group) = ts.next().unwrap() {
            // Currently `syn` builds `compile_error!` invocations
            // exclusively in `ident{"..."}` (braced) form which is not
            // followed by `;` (semicolon).
            //
            // But if it changes to `ident("...");` (parenthesized)
            // or `ident["..."];` (bracketed) form,
            // we will need to skip the `;` as well.
            // Highly unlikely, but better safe than sorry.
            if matches!(
                group.delimiter(),
                Delimiter::Parenthesis | Delimiter::Bracket
            ) {
                ts.next().unwrap(); // ;
            }

            match group.stream().into_iter().next().unwrap() {
                TokenTree::Literal(lit) => lit,
                _ => return None,
            }
        } else {
            return None;
        };

        let last = lit.span();
        let mut msg = lit.to_string();

        // "abc" => abc
        msg.pop();
        msg.remove(0);

        Some((SpanRange { first, last }, msg))
    }

    let mut tokens = error.to_compile_error().into_iter();

    let (span_range, msg) = match gut_error(&mut tokens) {
        Some(x) => x,
        None => return Diagnostic::new(Level::Error, "failed to gut error".to_string()),
    };
    let mut res = Diagnostic::spanned_range(span_range, Level::Error, msg);

    while let Some((span_range, msg)) = gut_error(&mut tokens) {
        res = res.span_range_error(span_range, msg);
    }

    res
}
