//! `0x01` `key`


use crate::value::varint::VarInt;
use crate::packet::{ Packet, StateLogin };
use crate::packet::decode::{ DecodeBufHead, DecodeBuf, PacketDecode };
use crate::packet::decode::str::StringDecodeError;
use crate::packet::prefix_check::prefix_check_login_c2s;
use std::borrow::Cow;


/// <https://minecraft.wiki/w/Java_Edition_protocol/Packets#Encryption_Response>
#[derive(Debug, Clone)]
pub struct KeyC2SLoginPacket<'l> {

    /// The secret key of the cipher to use for all future communications.
    pub secret_key   : Cow<'l, [u8]>,

    /// The veryify token previously sent in `HelloS2CLoginPacket`,
    ///  encrypted with the sent public key.
    pub verify_token : Cow<'l, [u8]>

}


impl crate::Sealed for KeyC2SLoginPacket<'_> { }

impl Packet for KeyC2SLoginPacket<'_> { }


impl KeyC2SLoginPacket<'_> {

    /// Convert the inner parts of this packet to their owned counterparts, or
    ///  take ownership if they are already owned. Returns the newly created
    ///  `KeyC2SLoginPacket<'static>`.
    #[inline]
    pub fn into_static_owned(self) -> KeyC2SLoginPacket<'static> {
        KeyC2SLoginPacket { secret_key : Cow::Owned(self.secret_key.into_owned()), verify_token : Cow::Owned(self.verify_token.into_owned()) }
    }

    /// Convert the inner parts of this packet to their owned counterparts.
    ///  Returns the newly created `KeyC2SLoginPacket<'static>`.
    #[inline]
    pub fn to_static_owned(&self) -> KeyC2SLoginPacket<'static> {
        KeyC2SLoginPacket { secret_key : Cow::Owned((*self.secret_key).to_owned()), verify_token : Cow::Owned((*self.verify_token).to_owned()) }
    }

}


impl PacketDecode for KeyC2SLoginPacket<'_> {
    type State = StateLogin;

    const PREFIX : u8 = prefix_check_login_c2s!(key, 0x01);
    type Output<'l> = KeyC2SLoginPacket<'l>;
    type Error<'l>  = StringDecodeError;

    fn decode<'l>(buf : DecodeBuf<'l>, head : &mut DecodeBufHead)
        -> Result<Self::Output<'l>, Self::Error<'l>>
    {
        let secret_key_len   = *buf.read_decode::<VarInt::<u32>>(head)? as usize;
        let secret_key       = buf.read_n(head, secret_key_len)?;
        let verify_token_len = *buf.read_decode::<VarInt::<u32>>(head)? as usize;
        let verify_token     = buf.read_n(head, verify_token_len)?;
        Ok(Self::Output { secret_key : Cow::Borrowed(secret_key), verify_token : Cow::Borrowed(verify_token) })
    }
}
