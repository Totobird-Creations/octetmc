//! `0x03` `finish_configuration`


use crate::value::ident::{ Ident, IdentDecodeError };
use crate::value::channel_data::ChannelData;
use crate::packet::StateConfig;
use crate::packet::decode::{ DecodeBufHead, DecodeBuf, PacketDecode };


/// https://minecraft.wiki/w/Java_Edition_protocol/Packets#Serverbound_Plugin_Message_(configuration)
#[derive(Debug, Clone)]
pub struct CustomPayloadC2SConfigPacket<'l> {

    /// The plugin channel data.
    pub data : ChannelData<'l>

}


impl PacketDecode for CustomPayloadC2SConfigPacket<'_> {
    type State = StateConfig;

    const PREFIX : u8 = 0x02;
    type Output<'l> = CustomPayloadC2SConfigPacket<'l>;
    type Error<'l>  = IdentDecodeError;

    fn decode<'l>(buf : DecodeBuf<'l>, head : &mut DecodeBufHead)
        -> Result<Self::Output<'l>, Self::Error<'l>>
    {
        let channel = buf.read_decode::<Ident<'_>>(head)?;

        Ok(Self::Output { data : (if (channel.nspace() == "minecraft" && channel.path() == "brand") {
            ChannelData::Brand { brand : buf.read_decode::<&str>(head)? }
        } else {
            ChannelData::Custom { channel, data : buf.read_n(head, buf.remaining(head))? }
        }) })
    }
}
