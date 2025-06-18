//! `0x03` `finish_configuration`


use crate::packet::{ Packet, StateConfig };
use crate::packet::encode::{ EncodeBuf, PacketEncode };
use crate::packet::prefix_check::prefix_check_config_s2c;


/// <https://minecraft.wiki/w/Java_Edition_protocol/Packets#Finish_Configuration>
#[derive(Debug, Clone, Copy)]
pub struct FinishConfigurationS2CConfigPacket;


impl crate::Sealed for FinishConfigurationS2CConfigPacket { }

impl Packet for FinishConfigurationS2CConfigPacket { }


impl PacketEncode for FinishConfigurationS2CConfigPacket {
    type State = StateConfig;

    const PREFIX : u8 = prefix_check_config_s2c!(finish_configuration, 0x03);

    #[inline(always)]
    fn predict_size(&self) -> usize { 0 }

    #[inline(always)]
    fn encode(&self, _buf : &mut EncodeBuf) { }
}
