//! `0x01` `key`


use crate::value::varint::VarInt;
use crate::packet::StateLogin;
use crate::packet::decode::{ DecodeBufHead, DecodeBuf, PacketDecode };
use crate::packet::decode::str::StringDecodeError;


/// https://minecraft.wiki/w/Java_Edition_protocol/Packets#Encryption_Response
#[derive(Debug, Clone)]
pub struct KeyC2SLoginPacket<'l> {

    /// The secret key of the cipher to use for all future communications.
    pub secret_key   : &'l [u8],

    /// The veryify token previously sent in `HelloS2CLoginPacket`,
    ///  encrypted with the sent public key.
    pub verify_token : &'l [u8]

}


impl PacketDecode for KeyC2SLoginPacket<'_> {
    type State = StateLogin;

    const PREFIX : u8 = 0x01;
    type Output<'l> = KeyC2SLoginPacket<'l>;
    type Error<'l>  = StringDecodeError;

    fn decode<'l>(buf : DecodeBuf<'l>, head : &mut DecodeBufHead)
        -> Result<Self::Output<'l>, Self::Error<'l>>
    {
        let secret_key_len   = *buf.read_decode::<VarInt::<u32>>(head)? as usize;
        let secret_key       = buf.read_n(head, secret_key_len)?;
        let verify_token_len = *buf.read_decode::<VarInt::<u32>>(head)? as usize;
        let verify_token     = buf.read_n(head, verify_token_len)?;
        Ok(Self::Output { secret_key, verify_token })
    }
}
