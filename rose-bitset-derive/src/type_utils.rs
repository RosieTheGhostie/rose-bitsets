use proc_macro2::Span;
use std::{fmt::Write, ops::Deref};
use syn::{Ident, Path, PathArguments, PathSegment, Token, Type, TypePath, punctuated::Punctuated};

pub fn signed_counterpart(uint: &Type) -> Option<Type> {
    let uint_repr = repr(uint);
    let int_repr = match uint_repr.as_str() {
        "u8" => "i8",
        "u16" => "i16",
        "u32" => "i32",
        "u64" => "i64",
        "u128" => "i128",
        "usize" => "isize",
        _ => return None,
    };
    let mut path_segments: Punctuated<PathSegment, Token![::]> = Punctuated::new();
    path_segments.push_value(PathSegment {
        ident: Ident::new(int_repr, Span::call_site()),
        arguments: PathArguments::None,
    });
    Some(Type::Path(TypePath {
        qself: None,
        path: Path {
            leading_colon: None,
            segments: path_segments,
        },
    }))
}

pub fn repr(r#type: &Type) -> String {
    match r#type {
        Type::Group(group) => repr(group.elem.deref()),
        Type::Infer(_) => "_".into(),
        Type::Never(_) => "!".into(),
        Type::Paren(t) => format!("({})", repr(t.elem.deref())),
        Type::Path(path) => repr_path(&path.path.segments),
        _ => unimplemented!(),
    }
}

fn repr_path(segments: &Punctuated<PathSegment, Token![::]>) -> String {
    let mut out = String::with_capacity(2 * segments.len());
    for (segment, sep) in segments.pairs().map(|pair| pair.into_tuple()) {
        // we're ignoring `segment.arguments` for simplicity
        if sep.is_some() {
            write!(out, "{}::", segment.ident)
        } else {
            write!(out, "{}", segment.ident)
        }
        .unwrap();
    }
    out
}
