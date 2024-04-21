#![feature(stmt_expr_attributes)]

//! [The Little Book of Rust Macros](https://veykril.github.io/tlborm/)
//! [Macros by Example](https://doc.rust-lang.org/reference/macros.html)

use proc_macro::TokenStream;

mod visibility;

#[proc_macro_attribute]
pub fn vis_make(attrs: TokenStream, input: TokenStream) -> TokenStream {
    visibility::make(attrs, input)
}
