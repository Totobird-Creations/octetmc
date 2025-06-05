//! `0x07` `registry_data`


use crate::value::varint::VarInt;
use crate::value::ident::Ident;
use crate::value::nbt::Nbt;
use crate::packet::StateConfig;
use crate::packet::encode::{ EncodeBuf, PacketEncode, PacketPartEncode };
use std::borrow::Cow;


/// <https://minecraft.wiki/w/Java_Edition_protocol/Packets#Registry_Data_2>
#[derive(Debug, Clone)]
pub struct RegistryDataS2CConfigPacket<'l> {

    /// The ID of the registry to overwrite.
    pub id      : Ident<'l>,

    /// Entries in the registry.
    pub entries : Cow<'l, [RegistryEntry<'l>]>

}


impl RegistryDataS2CConfigPacket<'_> {

    /// Convert the inner parts of this packet to their owned counterparts, or
    ///  take ownership if they are already owned. Returns the newly created
    ///  `RegistryDataS2CConfigPacket<'static>`.
    #[inline]
    pub fn into_static_owned(self) -> RegistryDataS2CConfigPacket<'static> {
        RegistryDataS2CConfigPacket {
            id      : self.id.into_static_owned(),
            entries : Cow::Owned(match (self.entries) {
                Cow::Borrowed(entries) => entries.iter().map(|entry| entry.to_static_owned()).collect(),
                Cow::Owned(entries)    => entries.into_iter().map(|entry| entry.into_static_owned()).collect(),
            })
        }
    }

    /// Convert the inner parts of this packet to their owned counterparts.
    ///  Returns the newly created `RegistryDataS2CConfigPacket<'static>`.
    #[inline]
    pub fn to_static_owned(&self) -> RegistryDataS2CConfigPacket<'static> {
        RegistryDataS2CConfigPacket {
            id      : self.id.to_static_owned(),
            entries : self.entries.iter().map(|entry| entry.to_static_owned()).collect()
        }
    }

}


impl PacketEncode for RegistryDataS2CConfigPacket<'_> {
    type State = StateConfig;

    const PREFIX : u8 = 0x07;

    fn predict_size(&self) -> usize {
        self.id.predict_size()
        + VarInt::<u32>::MAX_BYTES
        + self.entries.iter().map(|entry| entry.predict_size()).sum::<usize>()
    }

    fn encode(&self, buf : &mut EncodeBuf) {
        buf.encode_write(&self.id);
        buf.encode_write(VarInt::<u32>::from(self.entries.len() as u32));
        for entry in &*self.entries {
            buf.encode_write(entry);
        }
    }
}


/// An entry in a registry.
#[derive(Debug, Clone)]
pub struct RegistryEntry<'l> {

    /// The ID of the entry.
    pub id   : Ident<'l>,

    /// Entry data.
    pub data : Option<Nbt<'l>>

}


impl RegistryEntry<'_> {

    /// Convert the inner parts of this `RegistryEntry` to their owned counterparts, or
    ///  take ownership if they are already owned. Returns the newly created
    ///  `RegistryEntry<'static>`.
    #[inline]
    pub fn into_static_owned(self) -> RegistryEntry<'static> {
        RegistryEntry {
            id   : self.id.into_static_owned(),
            data : self.data.map(|data| data.into_static_owned())
        }
    }

    /// Convert the inner parts of this `RegistryEntry` to their owned counterparts.
    ///  Returns the newly created `RegistryEntry<'static>`.
    #[inline]
    pub fn to_static_owned(&self) -> RegistryEntry<'static> {
        RegistryEntry {
            id   : self.id.to_static_owned(),
            data : self.data.as_ref().map(|data| data.to_static_owned())
        }
    }

}


impl PacketPartEncode for RegistryEntry<'_> {

    #[inline]
    fn predict_size(&self) -> usize {
        self.id.predict_size()
        + self.data.predict_size()
    }

    fn encode(&self, buf : &mut EncodeBuf) {
        buf.encode_write(&self.id);
        buf.encode_write(&self.data);
    }

}
