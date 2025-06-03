//! `0x03` `finish_configuration`


use crate::packet::StateConfig;
use crate::packet::decode::{ DecodeBufHead, DecodeBuf, PacketDecode, IncompleteData };


/// https://minecraft.wiki/w/Java_Edition_protocol/Packets#Acknowledge_Finish_Configuration
#[derive(Debug, Clone, Copy)]
pub struct FinishConfigurationC2SConfigPacket;


impl PacketDecode for FinishConfigurationC2SConfigPacket {
    type State = StateConfig;

    const PREFIX : u8 = 0x03;
    type Output<'l> = FinishConfigurationC2SConfigPacket;
    type Error<'l>  = IncompleteData;

    #[inline(always)]
    fn decode<'l>(_buf : DecodeBuf<'l>, _head : &mut DecodeBufHead)
        -> Result<Self::Output<'l>, Self::Error<'l>>
    { Ok(Self) }
}
