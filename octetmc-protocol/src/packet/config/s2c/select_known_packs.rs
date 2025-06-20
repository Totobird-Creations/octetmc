//! `0x0E` `select_known_packs`


use crate::mapping_check;
use crate::value::varint::VarInt;
use crate::value::known_pack::KnownPack;
use crate::packet::{ Packet, StateConfig };
use crate::packet::encode::{ EncodeBuf, PacketEncode, PacketPartEncode };
use crate::packet::prefix_check::prefix_check_config_s2c;
use std::borrow::Cow;


mapping_check!("net.minecraft.network.protocol.configuration.ClientboundSelectKnownPacks", "d43a4a63926e20bf331b551b432a53424ed22f00d36419fc93c10cc3b38fbfd8");


/// <https://minecraft.wiki/w/Java_Edition_protocol/Packets#Clientbound_Known_Packs>
#[derive(Debug, Clone)]
pub struct SelectKnownPacksS2CConfigPacket<'l> {

    /// The datapacks that are present on the server.
    pub known_packs : Cow<'l, [KnownPack<'l>]>

}


impl SelectKnownPacksS2CConfigPacket<'_> {

    /// Convert the inner parts of this packet to their owned counterparts, or
    ///  take ownership if they are already owned. Returns the newly created
    ///  `SelectKnownPacksS2CConfigPacket<'static>`.
    #[inline]
    pub fn into_static_owned(self) -> SelectKnownPacksS2CConfigPacket<'static> {
        SelectKnownPacksS2CConfigPacket { known_packs : Cow::Owned(match (self.known_packs) {
            Cow::Borrowed(s) => s.iter().map(|r| r.to_static_owned()).collect::<Vec<_>>(),
            Cow::Owned(s)    => s.into_iter().map(|r| r.into_static_owned()).collect::<Vec<_>>(),
        }) }
    }

    /// Convert the inner parts of this packet to their owned counterparts.
    ///  Returns the newly created `SelectKnownPacksS2CConfigPacket<'static>`.
    #[inline]
    pub fn to_static_owned(&self) -> SelectKnownPacksS2CConfigPacket<'static> {
        SelectKnownPacksS2CConfigPacket { known_packs : Cow::Owned(
            self.known_packs.iter().map(|r| r.to_static_owned()).collect::<Vec<_>>()
        ) }
    }

}


impl crate::Sealed for SelectKnownPacksS2CConfigPacket<'_> { }

impl Packet for SelectKnownPacksS2CConfigPacket<'_> { }


impl PacketEncode for SelectKnownPacksS2CConfigPacket<'_> {
    type State = StateConfig;

    const PREFIX : u8 = prefix_check_config_s2c!(select_known_packs, 0x0E);

    fn predict_size(&self) -> usize {
        VarInt::<u32>::MAX_BYTES
        + self.known_packs.iter().map(|known_pack|
            known_pack.namespace.predict_size()
            + known_pack.id.predict_size()
            + known_pack.version.predict_size()
        ).sum::<usize>()
    }

    fn encode(&self, buf : &mut EncodeBuf) {
        buf.encode_write(VarInt::<u32>::from(self.known_packs.len() as u32));
        for known_pack in &*self.known_packs {
            buf.encode_write(&known_pack.namespace);
            buf.encode_write(&known_pack.id);
            buf.encode_write(&known_pack.version);
        }
    }
}
