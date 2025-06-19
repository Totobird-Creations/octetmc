//! `0x00` `hello`


use crate::mapping_check;
use crate::packet::{ Packet, StateLogin };
use crate::packet::decode::{ DecodeBufHead, DecodeBuf, PacketDecode };
use crate::packet::decode::str::StringDecodeError;
use crate::packet::prefix_check::prefix_check_login_c2s;
use std::borrow::Cow;
use uuid::Uuid;


mapping_check!("net.minecraft.network.protocol.login.ServerboundHelloPacket", "c5b2cb77c9e1289368fbe8deba87f3e9b0ea9ef7793c978d7755a4fa9b7afb50");


/// <https://minecraft.wiki/w/Java_Edition_protocol/Packets#Login_Start>
#[derive(Debug, Clone)]
pub struct HelloC2SLoginPacket<'l> {

    /// The player's username.
    ///
    /// The client chose this value. Make sure to verify it through mojauth.
    pub username : Cow<'l, str>,

    /// The player's UUID.
    ///
    /// The client chose this value. Make sure to verify it through mojauth.
    pub uuid     : Uuid

}


impl crate::Sealed for HelloC2SLoginPacket<'_> { }

impl Packet for HelloC2SLoginPacket<'_> { }


impl HelloC2SLoginPacket<'_> {

    /// Convert the inner parts of this packet to their owned counterparts, or
    ///  take ownership if they are already owned. Returns the newly created
    ///  `HelloC2SLoginPacket<'static>`.
    #[inline]
    pub fn into_static_owned(self) -> HelloC2SLoginPacket<'static> {
        HelloC2SLoginPacket { username : Cow::Owned(self.username.into_owned()), uuid : self.uuid }
    }

    /// Convert the inner parts of this packet to their owned counterparts.
    ///  Returns the newly created `HelloC2SLoginPacket<'static>`.
    #[inline]
    pub fn to_static_owned(&self) -> HelloC2SLoginPacket<'static> {
        HelloC2SLoginPacket { username : Cow::Owned((*self.username).to_owned()), uuid : self.uuid }
    }

}


impl PacketDecode for HelloC2SLoginPacket<'_> {
    type State = StateLogin;

    const PREFIX : u8 = prefix_check_login_c2s!(hello, 0x00);
    type Output<'l> = HelloC2SLoginPacket<'l>;
    type Error<'l>  = StringDecodeError;

    fn decode<'l>(buf : DecodeBuf<'l>, head : &mut DecodeBufHead)
        -> Result<Self::Output<'l>, Self::Error<'l>>
    {
        Ok(Self::Output {
            username : Cow::Borrowed(buf.read_decode::<&str>(head)?),
            uuid     : buf.read_decode::<Uuid>(head)?
        })
    }
}
