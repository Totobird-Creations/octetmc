//! `0x03` `finish_configuration`


use crate::packet::StateConfig;
use crate::packet::encode::{ EncodeBuf, PacketEncode };


/// https://minecraft.wiki/w/Java_Edition_protocol/Packets#Finish_Configuration
#[derive(Debug, Clone)]
pub struct FinishConfigurationS2CConfigPacket;


impl PacketEncode for FinishConfigurationS2CConfigPacket {
    type State = StateConfig;

    const PREFIX : u8 = 0x03;

    #[inline(always)]
    fn predict_size(&self) -> usize { 0 }

    #[inline(always)]
    fn encode(&self, _buf : &mut EncodeBuf) { }
}
