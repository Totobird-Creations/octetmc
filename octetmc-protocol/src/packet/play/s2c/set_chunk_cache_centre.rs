//! `0x57` `set_chunk_cache_center`


use crate::value::varint::VarInt;
use crate::value::chunk_pos::ChunkPos;
use crate::packet::{ Packet, StateConfig };
use crate::packet::encode::{ EncodeBuf, PacketEncode };


/// <https://minecraft.wiki/w/Java_Edition_protocol/Packets#Set_Center_Chunk>
#[derive(Debug, Clone, Copy)]
pub struct SetChunkCacheCentreS2CPlayPacket {

    /// The centre chunk.
    pub chunk : ChunkPos

}


impl crate::Sealed for SetChunkCacheCentreS2CPlayPacket { }

impl Packet for SetChunkCacheCentreS2CPlayPacket { }


impl PacketEncode for SetChunkCacheCentreS2CPlayPacket {
    type State = StateConfig;

    const PREFIX : u8 = 0x57;

    #[inline(always)]
    fn predict_size(&self) -> usize {
        VarInt::<u32>::MAX_BYTES * 2
    }

    #[inline(always)]
    fn encode(&self, buf : &mut EncodeBuf) {
        buf.encode_write(VarInt::<i32>::from(self.chunk.x));
        buf.encode_write(VarInt::<i32>::from(self.chunk.z));
    }
}
