//! `0x00` `client_information`


use crate::value::client_info::{ ClientInfo, ClientInfoDecodeError };
use crate::packet::StateConfig;
use crate::packet::decode::{ DecodeBufHead, DecodeBuf, PacketDecode };


/// https://minecraft.wiki/w/Java_Edition_protocol/Packets#Client_Information_(configuration)
#[derive(Debug, Clone)]
pub struct ClientInformationC2SConfigPacket<'l> {

    /// The client's settings and information.
    pub info : ClientInfo<'l>

}


impl PacketDecode for ClientInformationC2SConfigPacket<'_> {
    type State = StateConfig;

    const PREFIX : u8 = 0x00;
    type Output<'l> = ClientInformationC2SConfigPacket<'l>;
    type Error<'l>  = ClientInfoDecodeError;

    fn decode<'l>(buf : DecodeBuf<'l>, head : &mut DecodeBufHead)
        -> Result<Self::Output<'l>, Self::Error<'l>>
    {
        Ok(Self::Output { info : buf.read_decode::<ClientInfo>(head)? })
    }
}
