#![feature(proc_macro_span)]

use node_object::get_basetypes_from_str;
use proc_macro::{TokenStream, TokenTree};
use proc_macro2::{Ident, Span};
use quote::quote;

mod node_object;

#[proc_macro]
pub fn treesitter_consts(input: TokenStream) -> TokenStream {
    let tokens: Vec<_> = input.into_iter().collect();

    let json_data = match tokens.as_slice() {
        [TokenTree::Literal(lit)] => unwrap_string_literal(lit),
        _ => panic!("This macro only accepts a single, non-empty string argument"),
    };

    json_to_mod(json_data)
}

fn unwrap_string_literal(lit: &proc_macro::Literal) -> String {
    let mut repr = lit.to_string();
    if !repr.starts_with('"') || !repr.ends_with('"') {
        panic!("This macro only accepts a single, non-empty string argument")
    }

    repr.remove(0);
    repr.pop();

    repr
}

fn json_to_mod(file: String) -> TokenStream {
    let span = proc_macro::Span::call_site();
    let pa = span.source_file().path();
    let pa = pa
        .parent()
        .unwrap_or_else(|| panic!("{} do not have parent dir", pa.display()))
        .canonicalize()
        .expect("Cannot canonicalize the path");

    let real_pa = pa.join(&file);

    let context = std::fs::read_to_string(&real_pa)
        .unwrap_or_else(|_| panic!("Unreachable file, {}", real_pa.display()));
    let data = get_basetypes_from_str(&context).unwrap_or_else(|_| { panic!("{}", "Unreadable data".to_string()) });

    let mut tokens = Vec::new();
    let mut consts_tokens = Vec::new();
    for da in data {
        if da.contains_unique() {
            continue;
        }
        let prename = format!("SYS_{}", da.get_type().to_uppercase());
        let name = Ident::new(&prename, Span::call_site());
        let type_ = da.get_type();
        tokens.push(quote! {
            pub const #name: &str = #type_;
        });
        consts_tokens.push(quote! {
            #type_
        });
    }
    let fnq = quote! {
        pub mod node_consts {
            #(#tokens) *
            pub const NODE_TYPES: &[&str] = &[ #(#consts_tokens),* ];
        }
    };
    TokenStream::from(fnq)
}
