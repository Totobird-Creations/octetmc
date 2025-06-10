use core::fmt;
use proc_macro::{ TokenStream, Span };
use syn::{ Ident, LitInt, LitFloat, LitBool, Token, parse_macro_input };
use syn::parse::{ Parse, ParseStream };
use proc_macro_crate::{ crate_name, FoundCrate };
use quote::{ quote, ToTokens };


#[proc_macro]
pub fn ident(input : TokenStream) -> TokenStream {
    let root = match (crate_name("octetmc-protocol").unwrap()) {
        FoundCrate::Itself     => quote!{ crate },
        FoundCrate::Name(name) => {
            let ident = Ident::new(&name, Span::call_site().into());
            ident.into_token_stream()
        },
    };

    let     input     = parse_macro_input!(input as IdentTokens);
    let mut ident_str = input.to_string();
    if (! ident_str.contains(':')) {
        ident_str.insert_str(0, "minecraft:");
    }

    quote!{
        #root::value::ident::Ident::parse_str(#ident_str)
    }.into()
}


struct IdentTokens(Vec<IdentToken>);

impl fmt::Display for IdentTokens {
    fn fmt(&self, f : &mut fmt::Formatter<'_>) -> fmt::Result {
        for tt in &self.0 { tt.fmt(f)? }
        Ok(())
    }
}

impl Parse for IdentTokens {
    fn parse(input : ParseStream) -> syn::Result<Self> {
        let mut out = Vec::new();
        loop {
            let lookahead = input.lookahead1();
            if (lookahead.peek(Ident)
                || lookahead.peek(LitInt)
                || lookahead.peek(LitFloat)
                || lookahead.peek(LitBool)
                || lookahead.peek(Token![.])
                || lookahead.peek(Token![-])
                || lookahead.peek(Token![_])
                || lookahead.peek(Token![/])
                || lookahead.peek(Token![:])
            ) {
                out.push(input.parse()?);
            } else { break; }
        }
        Ok(Self(out))
    }
}


enum IdentToken {
    Ident(Ident),
    Int(LitInt),
    Float(LitFloat),
    Bool(LitBool),
    #[expect(dead_code)]
    Dot(Token![.]),
    #[expect(dead_code)]
    Hyphen(Token![-]),
    #[expect(dead_code)]
    Under(Token![_]),
    #[expect(dead_code)]
    Slash(Token![/]),
    #[expect(dead_code)]
    Colon(Token![:]),
}

impl fmt::Display for IdentToken {
    fn fmt(&self, f : &mut fmt::Formatter<'_>) -> fmt::Result { match (self) {
        Self::Ident(ident) => write!(f, "{}", ident.to_string()),
        Self::Int(int)     => write!(f, "{}{}", int.base10_digits(), int.suffix()),
        Self::Float(float) => write!(f, "{}{}", float.base10_digits(), float.suffix()),
        Self::Bool(bool)   => write!(f, "{}", bool.value),
        Self::Dot(_)       => write!(f, "."),
        Self::Hyphen(_)    => write!(f, "-"),
        Self::Under(_)     => write!(f, "_"),
        Self::Slash(_)     => write!(f, "/"),
        Self::Colon(_)     => write!(f, ":")
    } }
}

impl Parse for IdentToken {
    fn parse(input : ParseStream) -> syn::Result<Self> {
        let lookahead = input.lookahead1();
        if (lookahead.peek(Ident)) {
            Ok(Self::Ident(input.parse()?))
        } else if (lookahead.peek(LitInt)) {
            Ok(Self::Int(input.parse()?))
        } else if (lookahead.peek(LitFloat)) {
            Ok(Self::Float(input.parse()?))
        } else if (lookahead.peek(LitBool)) {
            Ok(Self::Bool(input.parse()?))
        } else if (lookahead.peek(Token![.])) {
            Ok(Self::Dot(input.parse()?))
        } else if (lookahead.peek(Token![-])) {
            Ok(Self::Hyphen(input.parse()?))
        } else if (lookahead.peek(Token![_])) {
            Ok(Self::Under(input.parse()?))
        } else if (lookahead.peek(Token![/])) {
            Ok(Self::Slash(input.parse()?))
        } else if (lookahead.peek(Token![:])) {
            Ok(Self::Colon(input.parse()?))
        } else {
            Err(lookahead.error())
        }
    }
}
