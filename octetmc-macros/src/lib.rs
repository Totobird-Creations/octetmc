use proc_macro::TokenStream;


mod ident_inner;


#[proc_macro]
pub fn ident(input : TokenStream) -> TokenStream { ident_inner::ident(input) }
