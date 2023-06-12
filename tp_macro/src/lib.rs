#![feature(proc_macro_span)]
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use proc_macro::{Span, TokenStream};
use quote::{quote, ToTokens};

#[proc_macro]
pub fn jb_to_rb(input: TokenStream) -> TokenStream {
    let st = syn::parse_macro_input!(input as syn::ExprArray);
    let mut arr = quote!();
    for elem in st.elems {
        match elem {
            syn::Expr::Unary(lit) => {
                if let syn::Expr::Lit(expr_lit) = lit.expr.as_ref() {
                    if let syn::Lit::Int(v) = &expr_lit.lit {
                        let i = v.base10_parse::<i16>().unwrap();
                        let result = (0 - i) & 0xff;
                        let t = proc_macro2::Literal::i16_unsuffixed(result);
                        arr.extend(quote!(#t));
                    }
                }
            }
            syn::Expr::Lit(lit) => {
                if let syn::Lit::Int(v) = lit.lit {
                    arr.extend(v.token().into_token_stream());
                }
            }
            _ => (),
        }
        arr.extend(quote!(,));
    }
    quote!([#arr]).into()
}

#[proc_macro]
pub fn include_i32_buf(input: TokenStream) -> TokenStream {
    let st = syn::parse_macro_input!(input as syn::LitStr);
    let span = Span::call_site();
    let source = span.source_file();
    let path = source.path().parent().expect("path error").join(st.value());
    let file = File::open(path).expect("file not found");
    let mut reader = BufReader::new(file);
    let mut output = proc_macro2::TokenStream::new();
    loop {
        let mut line = String::new();
        if let Ok(amt) = reader.read_line(&mut line) {
            if amt == 0 {
                break;
            }
            let v = i64::from_str_radix(line.trim().trim_start_matches("0x"), 16)
                .expect("parse i32 error");
            let v = proc_macro2::Literal::i32_unsuffixed(v as i32);
            output.extend(quote!(#v,));
        } else {
            break;
        }
    }
    quote!(&[#output]).into()
}
