use crate::packet::StateStatus;
use crate::packet::encode::{ EncodeBuf, PacketEncode };


#[derive(Debug, Clone)]
pub struct PingResponseS2CStatusPacket {
    pub timestamp : u64
}


impl PacketEncode for PingResponseS2CStatusPacket {
    type State = StateStatus;

    const PREFIX : u8 = 0x01;

    fn encode(&self, buf : &mut EncodeBuf) {
        buf.encode_write(&self.timestamp);
    }
}
