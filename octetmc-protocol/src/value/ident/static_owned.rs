use super::{ Ident, RawIdent };
use core::{ iter, ptr };
use std::borrow::Cow;


impl Ident<'_> {

    /// Convert the inner parts of this `Ident` to their owned counterparts, or
    ///  take ownership if they are already owned. Returns the newly created
    ///  `Ident<'static>`.
    pub fn into_static_owned(self) -> Ident<'static> {
        Ident { raw : match (self.raw) {

            RawIdent::Joined { string, sep_idx } => {
                RawIdent::Joined { string : Cow::Owned(string.into_owned()), sep_idx }
            },

            RawIdent::Split { nspace, path } => { match ((nspace, path,)) {

                (Cow::Owned(nspace), Cow::Owned(path),) => {
                    RawIdent::Split { nspace : Cow::Owned(nspace), path : Cow::Owned(path) }
                },

                (Cow::Owned(mut string), Cow::Borrowed(path),) => {
                    let sep_idx = string.len();
                    string.reserve_exact(1 + path.len());
                    string.push(':');
                    string.push_str(path);
                    RawIdent::Joined { string : Cow::Owned(string), sep_idx }
                },

                (Cow::Borrowed(nspace), Cow::Owned(mut string),) => {
                    unsafe {
                        let string_mut = string.as_mut_vec();
                        string_mut.splice(0..0, iter::repeat_n(b':', 1 + nspace.len()));
                        ptr::copy_nonoverlapping(nspace.as_ptr(), string_mut.as_mut_ptr(), nspace.len());
                    }
                    RawIdent::Joined { string : Cow::Owned(string), sep_idx : nspace.len() }
                },

                (Cow::Borrowed(nspace), Cow::Borrowed(path),) => {
                    RawIdent::Joined { string : Cow::Owned(format!("{nspace}:{path}")), sep_idx : nspace.len() }
                }

            } }
        } }
    }

    /// Convert the inner parts of this `Ident` to their owned counterparts.
    ///  Returns the newly created `Ident<'static>`.
    pub fn to_static_owned(&self) -> Ident<'static> {
        Ident { raw : match (&self.raw) {

            RawIdent::Joined { string, sep_idx } => {
                RawIdent::Joined { string : Cow::Owned((**string).to_owned()), sep_idx : *sep_idx }
            },

            RawIdent::Split { nspace, path } => {
                RawIdent::Joined { string : Cow::Owned(format!("{nspace}:{path}")), sep_idx : nspace.len() }
            }

        } }
    }

    /// Convert the inner parts of this `Ident` to their borrowed counterparts.
    ///  Returns the newly created `Ident<'_>`.
    pub fn as_ref(&self) -> Ident<'_> {
        Ident { raw : match (&self.raw) {

            RawIdent::Joined { string, sep_idx } => {
                RawIdent::Joined { string : Cow::Borrowed(string), sep_idx : *sep_idx }
            },

            RawIdent::Split { nspace, path } => {
                RawIdent::Split { nspace : Cow::Borrowed(nspace), path : Cow::Borrowed(path) }
            }

        } }
    }

}
