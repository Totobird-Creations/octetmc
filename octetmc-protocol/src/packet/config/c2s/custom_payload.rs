//! `0x02` `custom_payload`


use crate::mapping_check;
use crate::value::ident::{ Ident, IdentDecodeError };
use crate::value::channel_data::ChannelData;
use crate::packet::{ Packet, StateConfig };
use crate::packet::decode::{ DecodeBufHead, DecodeBuf, PacketDecode };
use crate::packet::prefix_check::prefix_check_config_c2s;
use std::borrow::Cow;


mapping_check!("net.minecraft.network.protocol.common.ServerboundCustomPayloadPacket", "4ab924bbc9e06dab98926e8b6b6bf1bceb26100bfb00dfa79d3256bb27291a1e");


/// <https://minecraft.wiki/w/Java_Edition_protocol/Packets#Serverbound_Plugin_Message_(configuration)>
#[derive(Debug, Clone)]
pub struct CustomPayloadC2SConfigPacket<'l> {

    /// The plugin channel data.
    pub data : ChannelData<'l>

}


impl CustomPayloadC2SConfigPacket<'_> {

    /// Convert the inner parts of this packet to their owned counterparts, or
    ///  take ownership if they are already owned. Returns the newly created
    ///  `CustomPayloadC2SConfigPacket<'static>`.
    #[inline]
    pub fn into_static_owned(self) -> CustomPayloadC2SConfigPacket<'static> {
        CustomPayloadC2SConfigPacket { data : self.data.into_static_owned() }
    }

    /// Convert the inner parts of this packet to their owned counterparts.
    ///  Returns the newly created `CustomPayloadC2SConfigPacket<'static>`.
    #[inline]
    pub fn to_static_owned(&self) -> CustomPayloadC2SConfigPacket<'static> {
        CustomPayloadC2SConfigPacket { data : self.data.to_static_owned() }
    }

}


impl crate::Sealed for CustomPayloadC2SConfigPacket<'_> { }

impl Packet for CustomPayloadC2SConfigPacket<'_> { }


impl PacketDecode for CustomPayloadC2SConfigPacket<'_> {
    type State = StateConfig;

    const PREFIX : u8 = prefix_check_config_c2s!(custom_payload, 0x02);
    type Output<'l> = CustomPayloadC2SConfigPacket<'l>;
    type Error<'l>  = IdentDecodeError;

    fn decode<'l>(buf : DecodeBuf<'l>, head : &mut DecodeBufHead)
        -> Result<Self::Output<'l>, Self::Error<'l>>
    {
        let channel = buf.read_decode::<Ident<'_>>(head)?;

        Ok(Self::Output { data : (if (channel.nspace() == "minecraft" && channel.path() == "brand") {
            ChannelData::Brand { brand : Cow::Borrowed(buf.read_decode::<&str>(head)?) }
        } else {
            ChannelData::Custom { channel, data : Cow::Borrowed(buf.read_n(head, buf.remaining(head))?) }
        }) })
    }
}
