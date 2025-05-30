//! Resource location identifiers.


use std::borrow::Cow;
use serde::{
    Serialize as Ser,
    Serializer as Serer
};


/// https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Identifier
#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct Ident<'l> {
    nspace : Cow<'l, str>,
    path   : Cow<'l, str>
}

impl Ser for Ident<'_> {

    fn serialize<S>(&self, serer : S) -> Result<<S as Serer>::Ok, <S as Serer>::Error>
    where
        S : Serer
    {
        let mut s = String::with_capacity(self.nspace.len() + self.path.len());
        s += &self.nspace;
        s += &self.path;
        s.serialize(serer)
    }

}
