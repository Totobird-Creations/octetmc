//! `0x03` `finish_configuration`


use crate::mapping_check;
use crate::packet::{ Packet, StateConfig };
use crate::packet::decode::{ DecodeBufHead, DecodeBuf, PacketDecode, IncompleteData };
use crate::packet::prefix_check::prefix_check_config_c2s;


mapping_check!("net.minecraft.network.protocol.configuration.ServerboundFinishConfigurationPacket", "f21412a96bdd07224e0ebee32ec9f020ffd3eb4e75143a351941c9a7b577d78d");


/// <https://minecraft.wiki/w/Java_Edition_protocol/Packets#Acknowledge_Finish_Configuration>
#[derive(Debug, Clone, Copy)]
pub struct FinishConfigurationC2SConfigPacket;


impl crate::Sealed for FinishConfigurationC2SConfigPacket { }

impl Packet for FinishConfigurationC2SConfigPacket { }


impl PacketDecode for FinishConfigurationC2SConfigPacket {
    type State = StateConfig;

    const PREFIX : u8 = prefix_check_config_c2s!(finish_configuration, 0x03);
    type Output<'l> = FinishConfigurationC2SConfigPacket;
    type Error<'l>  = IncompleteData;

    #[inline(always)]
    fn decode<'l>(_buf : DecodeBuf<'l>, _head : &mut DecodeBufHead)
        -> Result<Self::Output<'l>, Self::Error<'l>>
    { Ok(Self) }
}
