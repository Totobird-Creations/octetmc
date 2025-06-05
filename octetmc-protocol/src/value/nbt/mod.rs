//! Named binary tree format.


use crate::packet::encode::{ PacketPartEncode, EncodeBuf };
use std::borrow::Cow;


mod compound;
pub use compound::*;

mod element;
pub use element::*;

mod tag;
pub use tag::NbtTag;


/// A root NBT wrapper.
#[derive(Clone, Debug)]
pub struct Nbt<'l> {

    /// The name of the root node.
    ///
    /// Can be left empty.
    pub name : Cow<'l, str>,

    /// The root compound.
    pub root : NbtCompound<'l>

}


impl Nbt<'_> {

    /// Convert the inner parts of this `Nbt` to their owned counterparts, or
    ///  take ownership if they are already owned. Returns the newly created
    ///  `Nbt<'static>`.
    #[inline]
    pub fn into_static_owned(self) -> Nbt<'static> {
        Nbt {
            name : Cow::Owned(self.name.into_owned()),
            root : self.root.into_static_owned()
        }
    }

    /// Convert the inner parts of this `Nbt` to their owned counterparts.
    ///  Returns the newly created `Nbt<'static>`.
    #[inline]
    pub fn to_static_owned(&self) -> Nbt<'static> {
        Nbt {
            name : Cow::Owned((*self.name).to_owned()),
            root : self.root.to_static_owned()
        }
    }

}


impl<'l> From<NbtCompound<'l>> for Nbt<'l> {
    fn from(value : NbtCompound<'l>) -> Self { Self {
        name : Cow::Borrowed(""),
        root : value
    } }
}


impl PacketPartEncode for Nbt<'_> {

    #[inline]
    fn predict_size(&self) -> usize {
        1 + self.root.predict_size()
    }

    #[inline]
    fn encode(&self, buf : &mut EncodeBuf) {
        buf.write(NbtTag::Compound as u8);
        buf.encode_write(&self.root);
    }

}
