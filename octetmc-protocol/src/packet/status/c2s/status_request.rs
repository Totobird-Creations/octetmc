//! `0x00` `status_request`


use crate::mapping_check;
use crate::packet::{ Packet, StateStatus };
use crate::packet::decode::{ DecodeBufHead, DecodeBuf, PacketDecode, IncompleteData };
use crate::packet::prefix_check::prefix_check_status_c2s;


mapping_check!("net.minecraft.network.protocol.status.ServerboundStatusRequestPacket", "59eb192139a253cc99a4271c9264d4990ee8c7e5502f58a43ec847e67f586dc8");


/// <https://minecraft.wiki/w/Java_Edition_protocol/Packets#Status_Request>
#[derive(Debug, Clone, Copy)]
pub struct StatusRequestC2SStatusPacket;


impl crate::Sealed for StatusRequestC2SStatusPacket { }

impl Packet for StatusRequestC2SStatusPacket { }


impl PacketDecode for StatusRequestC2SStatusPacket {
    type State = StateStatus;

    const PREFIX : u8 = prefix_check_status_c2s!(status_request, 0x00);
    type Output<'l> = StatusRequestC2SStatusPacket;
    type Error<'l>  = IncompleteData;

    #[inline(always)]
    fn decode<'l>(_buf : DecodeBuf<'l>, _head : &mut DecodeBufHead)
        -> Result<Self::Output<'l>, Self::Error<'l>>
    { Ok(Self) }
}
