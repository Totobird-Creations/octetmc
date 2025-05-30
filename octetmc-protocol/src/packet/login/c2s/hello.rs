//! `0x00` `hello`


use crate::packet::StateLogin;
use crate::packet::decode::{ DecodeBufHead, DecodeBuf, PacketDecode };
use crate::packet::decode::str::StringDecodeError;
use uuid::Uuid;


/// https://minecraft.wiki/w/Java_Edition_protocol/Packets#Login_Start
#[derive(Debug, Clone)]
pub struct HelloC2SLoginPacket<'l> {

    /// The player's username.
    ///
    /// The client chose this value. Make sure to verify it through mojauth.
    pub username : &'l str,

    /// The player's UUID.
    ///
    /// The client chose this value. Make sure to verify it through mojauth.
    pub uuid     : Uuid

}


impl PacketDecode for HelloC2SLoginPacket<'_> {
    type State = StateLogin;

    const PREFIX : u8 = 0x00;
    type Output<'l> = HelloC2SLoginPacket<'l>;
    type Error<'l>  = StringDecodeError;

    fn decode<'l>(buf : DecodeBuf<'l>, head : &mut DecodeBufHead)
        -> Result<Self::Output<'l>, Self::Error<'l>>
    {
        Ok(Self::Output {
            username : buf.read_decode::<&str>(head)?,
            uuid     : buf.read_decode::<Uuid>(head)?
        })
    }
}
