//! `0x01` `ping_request`


use crate::mapping_check;
use crate::packet::{ Packet, StateStatus };
use crate::packet::decode::{ DecodeBufHead, DecodeBuf, PacketDecode, IncompleteData };
use crate::packet::prefix_check::prefix_check_status_c2s;


mapping_check!("net.minecraft.network.protocol.ping.ServerboundPingRequestPacket", "245c35b2bfb950eecd03c5f921c80b6aba54cd5f7ad84c459dc50f7550c71dc8");


/// <https://minecraft.wiki/w/Java_Edition_protocol/Packets#Ping_Request_(status)>
#[derive(Debug, Clone, Copy)]
pub struct PingRequestC2SStatusPacket {

    /// May be any number, but vanilla clients will use the timestamp in milliseconds.
    pub timestamp : u64

}


impl crate::Sealed for PingRequestC2SStatusPacket { }

impl Packet for PingRequestC2SStatusPacket { }


impl PacketDecode for PingRequestC2SStatusPacket {
    type State = StateStatus;

    const PREFIX : u8 = prefix_check_status_c2s!(ping_request, 0x01);
    type Output<'l> = PingRequestC2SStatusPacket;
    type Error<'l>  = IncompleteData;

    #[inline]
    fn decode<'l>(buf : DecodeBuf<'l>, head : &mut DecodeBufHead)
        -> Result<Self::Output<'l>, Self::Error<'l>>
    { Ok(Self { timestamp : buf.read_decode::<u64>(head)? }) }
}
