//! `0x58` `set_chunk_cache_radius`


use crate::mapping_check;
use crate::value::varint::VarInt;
use crate::packet::{ Packet, StateConfig };
use crate::packet::encode::{ EncodeBuf, PacketEncode };
use crate::packet::prefix_check::prefix_check_play_s2c;
use core::num::NonZeroU8;


mapping_check!("net.minecraft.network.protocol.game.ClientboundSetChunkCacheRadiusPacket", "50760e7d60de56606b2480b4193a0bd4cddddb32b2a4486c9788b1cb477bda4d");


/// <https://minecraft.wiki/w/Java_Edition_protocol/Packets#Set_Render_Distance>
#[derive(Debug, Clone, Copy)]
pub struct SetChunkCacheRadiusS2CPlayPacket {

    /// The view distance.
    pub view_dist : NonZeroU8

}


impl crate::Sealed for SetChunkCacheRadiusS2CPlayPacket { }

impl Packet for SetChunkCacheRadiusS2CPlayPacket { }


impl PacketEncode for SetChunkCacheRadiusS2CPlayPacket {
    type State = StateConfig;

    const PREFIX : u8 = prefix_check_play_s2c!(set_chunk_cache_radius, 0x58);

    #[inline(always)]
    fn predict_size(&self) -> usize {
        VarInt::<u32>::MAX_BYTES
    }

    #[inline(always)]
    fn encode(&self, buf : &mut EncodeBuf) {
        buf.encode_write(VarInt::<u32>::from(self.view_dist.get() as u32));
    }
}
