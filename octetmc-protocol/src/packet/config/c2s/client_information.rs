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


impl ClientInformationC2SConfigPacket<'_> {

    /// Convert the inner parts of this packet to their owned counterparts, or
    ///  take ownership if they are already owned. Returns the newly created
    ///  `ClientInformationC2SConfigPacket<'static>`.
    #[inline]
    pub fn into_static_owned(self) -> ClientInformationC2SConfigPacket<'static> {
        ClientInformationC2SConfigPacket { info : self.info.into_static_owned() }
    }

    /// Convert the inner parts of this packet to their owned counterparts.
    ///  Returns the newly created `ClientInformationC2SConfigPacket<'static>`.
    #[inline]
    pub fn to_static_owned(&self) -> ClientInformationC2SConfigPacket<'static> {
        ClientInformationC2SConfigPacket { info : self.info.to_static_owned() }
    }

}


impl PacketDecode for ClientInformationC2SConfigPacket<'_> {
    type State = StateConfig;

    const PREFIX : u8 = 0x00;
    type Output<'l> = ClientInformationC2SConfigPacket<'l>;
    type Error<'l>  = ClientInfoDecodeError;

    #[inline]
    fn decode<'l>(buf : DecodeBuf<'l>, head : &mut DecodeBufHead)
        -> Result<Self::Output<'l>, Self::Error<'l>>
    { Ok(Self::Output { info : buf.read_decode::<ClientInfo>(head)? }) }
}
