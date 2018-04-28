#![feature(proc_macro)]
#![feature(proc_macro_non_items)]

#![doc(html_root_url = "https://docs.rs/maud_macros/0.17.3")]

// TokenStream values are reference counted, and the mental overhead of tracking
// lifetimes outweighs the marginal gains from explicit borrowing
#![cfg_attr(feature = "cargo-clippy", allow(needless_pass_by_value))]

extern crate literalext;
extern crate maud_htmlescape;
extern crate proc_macro;

mod parse;
mod build;

use proc_macro::{Literal, Span, Term, TokenStream, TokenTree};
use proc_macro::quote;

type ParseResult<T> = Result<T, String>;

#[proc_macro]
pub fn html(input: TokenStream) -> TokenStream {
    expand(input)
}

#[proc_macro]
pub fn html_debug(input: TokenStream) -> TokenStream {
    let expr = expand(input);
    println!("expansion:\n{}", expr);
    expr
}

fn expand(input: TokenStream) -> TokenStream {
    let output_ident = TokenTree::Term(Term::new("__maud_output", Span::def_site()));
    // Heuristic: the size of the resulting markup tends to correlate with the
    // code size of the template itself
    let size_hint = input.to_string().len();
    let size_hint = TokenTree::Literal(Literal::u64_unsuffixed(size_hint as u64));
    let stmts = match parse::parse(input, output_ident.clone()) {
        Ok(stmts) => stmts,
        Err(e) => panic!(e),
    };
    quote!({
        extern crate maud;
        let mut $output_ident = String::with_capacity($size_hint);
        $stmts
        maud::PreEscaped($output_ident)
    })
}
