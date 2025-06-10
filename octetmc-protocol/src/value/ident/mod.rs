//! Resource location identifiers.


use core::fmt::{ self, Debug, Display };
use core::hash::{ self, Hash };
use core::hint::unreachable_unchecked;
use std::borrow::Cow;


mod validate;
pub use validate::*;

mod constructor;

mod serde_endec;
pub use serde_endec::*;

mod static_owned;

pub use octetmc_macros::ident;


/// <https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Identifier>
#[derive(Clone)]
pub struct Ident<'l> {
    raw : RawIdent<'l>
}

#[derive(Clone)]
enum RawIdent<'l> {

    /// Prefer `Joined` over `Split`.
    Joined {
        string  : Cow<'l, str>,
        sep_idx : usize
    },

    Split {
        nspace : Cow<'l, str>,
        path   : Cow<'l, str>
    }

}


impl<'l> Ident<'l> {

    /// Gets this `Ident` as a joined `&str`, merging it if split.
    ///
    /// If split, the merged string will be cached to avoid allocations in the future.
    pub fn as_str(&mut self) -> &str {
        match (&self.raw) {
            RawIdent::Joined { .. }           => { },
            RawIdent::Split  { nspace, path } => {
                let string     = format!("{}:{}", nspace, path);
                let nspace_len = nspace.len();
                self.raw = RawIdent::Joined {
                    string  : Cow::Owned(string),
                    sep_idx : nspace_len
                };
            },
        }
        let RawIdent::Joined { string, .. } = &self.raw
            else { unsafe { unreachable_unchecked(); } };
        string
    }

    /// Get this `Ident` as a joined `Cow<'_, str>`, merging it if split.
    ///
    /// Prefer [`Ident::as_str`] if possible, as it will cache the work done.
    pub fn to_str(&self) -> Cow<'_, str> {
        match (&self.raw) {
            RawIdent::Joined { string, .. }   => Cow::Borrowed(string),
            RawIdent::Split  { nspace, path } => {
                Cow::Owned(format!("{}:{}", nspace, path))
            },
        }
    }

    /// Returns a reference to the namespace part of this `Ident`.
    #[inline]
    pub fn nspace(&self) -> &str {
        match (&self.raw) {
            RawIdent::Joined { string, sep_idx } => unsafe { string.get_unchecked(0..(*sep_idx)) },
            RawIdent::Split  { nspace, .. }      => nspace
        }
    }

    /// Returns a reference to the path part of this `Ident`.
    #[inline]
    pub fn path(&self) -> &str {
        match (&self.raw) {
            RawIdent::Joined { string, sep_idx } => unsafe { string.get_unchecked((sep_idx + 1)..) },
            RawIdent::Split  { path, .. }        => path
        }
    }

    /// Returns references to the namespace and path of this `Ident`.
    ///
    /// Returned as `(namespace, path,)`.
    pub fn as_inner(&self) -> (&str, &str,) {
        match (&self.raw) {
            RawIdent::Joined { string, sep_idx } => {
                let nspace = unsafe { string.get_unchecked(..(*sep_idx)) };
                let path   = unsafe { string.get_unchecked((sep_idx + 1)..) };
                (nspace, path,)
            },
            RawIdent::Split { nspace, path } => (nspace, path,)
        }
    }

    /// Takes ownership of `self`, returning a joined `Cow<'_, str>`.
    ///
    /// If split, it will need to be joined.
    pub fn into_joined_inner(self) -> Cow<'l, str> {
        match (self.raw) {
            RawIdent::Joined { string, .. }   => string,
            RawIdent::Split  { nspace, path } => Cow::Owned(format!("{nspace}:{path}")),
        }
    }

    /// Takes ownership of `self`, returning the inner namespace and path.
    pub fn into_split_inner(self) -> (Cow<'l, str>, Cow<'l, str>,) {
        match (self.raw) {
            RawIdent::Joined { string, sep_idx } => { match (string) {
                Cow::Borrowed(string) => {
                    let nspace = unsafe { string.get_unchecked(..sep_idx) };
                    let path   = unsafe { string.get_unchecked((sep_idx + 1)..) };
                    (Cow::Borrowed(nspace), Cow::Borrowed(path),)
                },
                Cow::Owned(mut string) => {
                    let path = string.split_off(sep_idx + 1);
                    let Some(_) = string.pop()
                        else { unsafe { unreachable_unchecked() } };
                    (Cow::Owned(string), Cow::Owned(path),)
                },
            } },
            RawIdent::Split { nspace, path } => (nspace, path,),
        }
    }

}


impl Display for Ident<'_> {
    fn fmt(&self, f : &mut fmt::Formatter<'_>) -> fmt::Result {
        match (&self.raw) {
            RawIdent::Joined { string, .. }   => write!(f, "{string}"),
            RawIdent::Split  { nspace, path } => write!(f, "{nspace}:{path}")
        }
    }
}

impl Debug for Ident<'_> {
    #[inline]
    fn fmt(&self, f : &mut fmt::Formatter<'_>) -> fmt::Result { write!(f, "\"{self}\"") }
}


impl PartialEq for Ident<'_> {
    #[inline]
    fn eq(&self, other : &Self) -> bool {
        (self.nspace() == other.nspace()) && (self.path() == other.path())
    }
}

impl Eq for Ident<'_> { }


impl Hash for Ident<'_> {
    #[inline]
    fn hash<H : hash::Hasher>(&self, state : &mut H) {
        self.nspace().hash(state);
        self.path().hash(state);
    }
}
