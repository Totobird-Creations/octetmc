use super::{ NbtElement, NbtTag };
use crate::packet::encode::{ PacketPartEncode, EncodeBuf };
use std::borrow::Cow;


/// An NBT compound/map.
#[derive(Clone, Debug)]
pub struct NbtCompound<'l> {

    /// Entries in this compound.
    pub entries : Cow<'l, [NbtCompoundEntry<'l>]>

}


impl NbtCompound<'_> {

    /// Convert the inner parts of this `NbtCompound` to their owned counterparts, or
    ///  take ownership if they are already owned. Returns the newly created
    ///  `NbtCompound<'static>`.
    pub fn into_static_owned(self) -> NbtCompound<'static> {
        NbtCompound {
            entries : Cow::Owned(match (self.entries) {
                Cow::Borrowed(entries) => entries.iter().map(|entry| entry.to_static_owned()).collect(),
                Cow::Owned(entries)    => entries.into_iter().map(|entry| entry.into_static_owned()).collect()
            })
        }
    }

    /// Convert the inner parts of this `NbtCompound` to their owned counterparts.
    ///  Returns the newly created `NbtCompound<'static>`.
    pub fn to_static_owned(&self) -> NbtCompound<'static> {
        NbtCompound {
            entries : Cow::Owned(self.entries.iter().map(|entry| entry.to_static_owned()).collect())
        }
    }

}


impl PacketPartEncode for NbtCompound<'_> {

    fn predict_size(&self) -> usize {
        self.entries.iter().map(|entry| entry.predict_size()).sum::<usize>()
        + 1
    }

    fn encode(&self, buf : &mut EncodeBuf) {
        for entry in &*self.entries {
            buf.encode_write(entry);
        }
        buf.write(NbtTag::End as u8);
    }

}



/// An entry in an NBT compound.
#[derive(Clone, Debug)]
pub struct NbtCompoundEntry<'l> {

    /// The key of the entry.
    pub key   : Cow<'l, str>,

    /// The value of the entry.
    pub value : NbtElement<'l>

}


impl NbtCompoundEntry<'_> {

    /// Convert the inner parts of this `NbtCompoundEntry` to their owned counterparts, or
    ///  take ownership if they are already owned. Returns the newly created
    ///  `NbtCompoundEntry<'static>`.
    #[inline]
    pub fn into_static_owned(self) -> NbtCompoundEntry<'static> {
        NbtCompoundEntry {
            key   : Cow::Owned(self.key.into_owned()),
            value : self.value.into_static_owned()
        }
    }

    /// Convert the inner parts of this `NbtCompoundEntry` to their owned counterparts.
    ///  Returns the newly created `NbtCompoundEntry<'static>`.
    #[inline]
    pub fn to_static_owned(&self) -> NbtCompoundEntry<'static> {
        NbtCompoundEntry {
            key   : Cow::Owned((*self.key).to_owned()),
            value : self.value.to_static_owned()
        }
    }

}


impl PacketPartEncode for NbtCompoundEntry<'_> {

    fn predict_size(&self) -> usize {
        1
        + NbtElement::String(Cow::Borrowed(&self.key)).predict_size()
        + self.value.predict_size()
    }

    fn encode(&self, buf : &mut EncodeBuf) {
        buf.write(self.value.tag() as u8);
        buf.encode_write(&NbtElement::String(Cow::Borrowed(&self.key)));
        buf.encode_write(&self.value);
    }

}
