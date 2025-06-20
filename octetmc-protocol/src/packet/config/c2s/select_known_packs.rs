//! `0x07` `select_known_packs`


use crate::mapping_check;
use crate::value::varint::VarInt;
use crate::value::known_pack::KnownPack;
use crate::packet::{ Packet, StateConfig };
use crate::packet::decode::{ DecodeBufHead, DecodeBuf, PacketDecode };
use crate::packet::decode::str::StringDecodeError;
use crate::packet::prefix_check::prefix_check_config_c2s;
use std::borrow::Cow;


mapping_check!("net.minecraft.network.protocol.configuration.ServerboundSelectKnownPacks", "803e456b4f4e13edbda209cfa36362eb7f5147d86b06ba3ad396e74b97cf5662");


/// <https://minecraft.wiki/w/Java_Edition_protocol/Packets#Serverbound_Known_Packs>
#[derive(Debug, Clone)]
pub struct SelectKnownPacksC2SConfigPacket<'l> {

    /// The datapacks that are present on the server.
    pub known_packs : Cow<'l, [KnownPack<'l>]>

}


impl SelectKnownPacksC2SConfigPacket<'_> {

    /// Convert the inner parts of this packet to their owned counterparts, or
    ///  take ownership if they are already owned. Returns the newly created
    ///  `SelectKnownPacksC2SConfigPacket<'static>`.
    #[inline]
    pub fn into_static_owned(self) -> SelectKnownPacksC2SConfigPacket<'static> {
        SelectKnownPacksC2SConfigPacket { known_packs : Cow::Owned(match (self.known_packs) {
            Cow::Borrowed(s) => s.iter().map(|r| r.to_static_owned()).collect::<Vec<_>>(),
            Cow::Owned(s)    => s.into_iter().map(|r| r.into_static_owned()).collect::<Vec<_>>(),
        }) }
    }

    /// Convert the inner parts of this packet to their owned counterparts.
    ///  Returns the newly created `SelectKnownPacksC2SConfigPacket<'static>`.
    #[inline]
    pub fn to_static_owned(&self) -> SelectKnownPacksC2SConfigPacket<'static> {
        SelectKnownPacksC2SConfigPacket { known_packs : Cow::Owned(
            self.known_packs.iter().map(|r| r.to_static_owned()).collect::<Vec<_>>()
        ) }
    }

}


impl crate::Sealed for SelectKnownPacksC2SConfigPacket<'_> { }

impl Packet for SelectKnownPacksC2SConfigPacket<'_> { }


impl PacketDecode for SelectKnownPacksC2SConfigPacket<'_> {
    type State = StateConfig;

    const PREFIX : u8 = prefix_check_config_c2s!(select_known_packs, 0x07);

    type Output<'l> = SelectKnownPacksC2SConfigPacket<'l>;
    type Error<'l>  = StringDecodeError;

    fn decode<'l>(buf : DecodeBuf<'l>, head : &mut DecodeBufHead)
        -> Result<Self::Output<'l>, Self::Error<'l>>
    {
        let     count       = *buf.read_decode::<VarInt<u32>>(head)? as usize;
        let mut known_packs = Vec::with_capacity(count);
        for _ in 0..count { known_packs.push(KnownPack {
            namespace : Cow::Borrowed(buf.read_decode::<&str>(head)?),
            id        : Cow::Borrowed(buf.read_decode::<&str>(head)?),
            version   : Cow::Borrowed(buf.read_decode::<&str>(head)?)
        }); }

        Ok(Self::Output { known_packs : Cow::Owned(known_packs) })
    }

}
