//! `0x57` `set_chunk_cache_center`


use crate::mapping_check;
use crate::packet::{ Packet, StatePlay };
use crate::packet::encode::{ EncodeBuf, PacketEncode };
use crate::packet::prefix_check::prefix_check_play_s2c;


mapping_check!("net.minecraft.network.protocol.game.ClientboundBundleDelimiterPacket", "4f4d4ea0d50fd83ee5ae1fe73d7aaf1abf1ae208e89bfa3405d54e786d894621");


/// TODO: Address
#[derive(Debug, Clone, Copy)]
pub struct BundleDelimiterS2CPlayPacket;


impl crate::Sealed for BundleDelimiterS2CPlayPacket { }

impl Packet for BundleDelimiterS2CPlayPacket { }


impl PacketEncode for BundleDelimiterS2CPlayPacket {
    type State = StatePlay;

    const PREFIX : u8 = prefix_check_play_s2c!(bundle_delimiter, 0x00);

    #[inline(always)]
    fn predict_size(&self) -> usize { 0 }

    #[inline(always)]
    fn encode(&self, _buf : &mut EncodeBuf) { }
}
