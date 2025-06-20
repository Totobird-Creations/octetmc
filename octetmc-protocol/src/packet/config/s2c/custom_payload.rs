//! `0x02` `custom_payload`


use crate::mapping_check;
use crate::value::ident::Ident;
use crate::value::channel_data::ChannelData;
use crate::packet::{ Packet, StateConfig };
use crate::packet::encode::{ EncodeBuf, PacketEncode, PacketPartEncode };
use crate::packet::prefix_check::prefix_check_config_s2c;


mapping_check!("net.minecraft.network.protocol.common.ClientboundCustomPayloadPacket", "0cd4da0bfe187406b3a57bd5d4354d9085ca3ae5ff7b7c6b1174dd25a0f95eab");


/// <https://minecraft.wiki/w/Java_Edition_protocol/Packets#Clientbound_Plugin_Message_(configuration)>
#[derive(Debug, Clone)]
pub struct CustomPayloadS2CConfigPacket<'l> {

    /// The plugin channel data.
    pub data : ChannelData<'l>

}


impl CustomPayloadS2CConfigPacket<'_> {

    /// Convert the inner parts of this packet to their owned counterparts, or
    ///  take ownership if they are already owned. Returns the newly created
    ///  `CustomPayloadS2CConfigPacket<'static>`.
    #[inline]
    pub fn into_static_owned(self) -> CustomPayloadS2CConfigPacket<'static> {
        CustomPayloadS2CConfigPacket { data : self.data.into_static_owned() }
    }

    /// Convert the inner parts of this packet to their owned counterparts.
    ///  Returns the newly created `CustomPayloadS2CConfigPacket<'static>`.
    #[inline]
    pub fn to_static_owned(&self) -> CustomPayloadS2CConfigPacket<'static> {
        CustomPayloadS2CConfigPacket { data : self.data.to_static_owned() }
    }

}


impl crate::Sealed for CustomPayloadS2CConfigPacket<'_> { }

impl Packet for CustomPayloadS2CConfigPacket<'_> { }


impl PacketEncode for CustomPayloadS2CConfigPacket<'_> {
    type State = StateConfig;

    const PREFIX : u8 = prefix_check_config_s2c!(custom_payload, 0x01);

    fn predict_size(&self) -> usize { match (&self.data) {
        ChannelData::Brand { brand } => {
            Ident::new_str("minecraft", "brand").predict_size()
            + brand.len()
        },
        ChannelData::Custom { channel, data } => {
            channel.predict_size()
            + data.len()
        },
    } }

    fn encode(&self, buf : &mut EncodeBuf) { match (&self.data) {
        ChannelData::Brand { brand } => {
            buf.encode_write(Ident::new_str("minecraft", "brand"));
            buf.encode_write(brand);
        },
        ChannelData::Custom { channel, data } => {
            buf.encode_write(channel);
            buf.write_n(data);
        }
    } }
}
