//! `0x00` `client_information`


use crate::mapping_check;
use crate::value::client_info::{ ClientInfo, ClientInfoDecodeError };
use crate::packet::{ Packet, StateConfig };
use crate::packet::decode::{ DecodeBufHead, DecodeBuf, PacketDecode };
use crate::packet::prefix_check::prefix_check_config_c2s;


mapping_check!("net.minecraft.network.protocol.common.ServerboundClientInformationPacket", "4ba94ef99476245e6c0c187c60a920f8e12036fa640f08fc5473f0e8bf34ffc9");


/// <https://minecraft.wiki/w/Java_Edition_protocol/Packets#Client_Information_(configuration)>
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


impl crate::Sealed for ClientInformationC2SConfigPacket<'_> { }

impl Packet for ClientInformationC2SConfigPacket<'_> { }


impl PacketDecode for ClientInformationC2SConfigPacket<'_> {
    type State = StateConfig;

    const PREFIX : u8 = prefix_check_config_c2s!(client_information, 0x00);
    type Output<'l> = ClientInformationC2SConfigPacket<'l>;
    type Error<'l>  = ClientInfoDecodeError;

    #[inline]
    fn decode<'l>(buf : DecodeBuf<'l>, head : &mut DecodeBufHead)
        -> Result<Self::Output<'l>, Self::Error<'l>>
    { Ok(Self::Output { info : buf.read_decode::<ClientInfo>(head)? }) }
}
