use super::{ ConnPeerComms, ConnPeerResult, ConnPeerError, MAX_UNCOMPRESSED_PACKET_LEN };
use octetmc_protocol::value::varint::{ VarInt, VarIntDecodeError };
use octetmc_protocol::packet::decode::{ PacketPartDecode, DecodeBuf, DecodeBufHead };


impl ConnPeerComms {

    /// Returns (length value, consumed byte count,).
    pub(super) async fn read_packet_len(&mut self) -> ConnPeerResult<(usize, usize,)> {
        let mut buf   = [0u8; VarInt::<u32>::MAX_BYTES];
        let mut index = 0;
        loop {
            match (self.read_queue.get(index)) {
                None => { self.accept_more_bytes().await?; },
                Some(&b) => {
                    // SAFETY: index can never be greater than `VarInt::<u32>::MAX_BYTES`.
                    match (unsafe { self.try_decode_varintu32(&mut buf, &mut index, b) }) {
                        Ok(Some(out)) => { return Ok(out); },
                        Ok(None)      => { },
                        Err(err)      => { return Err(err); }
                    }
                }
            }
        }
    }

    /// Returns (length value, consumed byte count,).
    pub(super) fn try_read_packet_len(&mut self) -> ConnPeerResult<Option<(usize, usize,)>> {
        let mut buf   = [0u8; VarInt::<u32>::MAX_BYTES];
        let mut index = 0;
        loop { match (self.read_queue.get(index)) {
            None => { return Ok(None); },
            Some(&b) => {
                // SAFETY: index can never be greater than `VarInt::<u32>::MAX_BYTES`.
                match (unsafe { self.try_decode_varintu32(&mut buf, &mut index, b) }) {
                    Ok(Some(out)) => { return Ok(Some(out)); },
                    Ok(None)      => { },
                    Err(err)      => { return Err(err); }
                }
            }
        } }
    }


    unsafe fn try_decode_varintu32(&mut self,
        buf   : &mut [u8; VarInt::<u32>::MAX_BYTES],
        index : &mut usize,
        b     : u8
    ) -> ConnPeerResult<Option<(usize, usize,)>> {
        buf[*index] = b;
        *index += 1;
        match (VarInt::<u32>::decode(DecodeBuf::from(unsafe { buf.get_unchecked(0..(*index)) }), &mut DecodeBufHead::default())) {
            Err(VarIntDecodeError::IncompleteData) => {
                if (*index >= VarInt::<u32>::MAX_BYTES) {
                    Err(ConnPeerError::InvalidPacketLength)
                } else { Ok(None) }
            },
            Err(VarIntDecodeError::TooLong) => {
                Err(ConnPeerError::InvalidPacketLength)
            },
            Ok(value) => {
                let _ = self.read_queue.drain(0..(*index));
                let value = *value as usize;
                if (value > MAX_UNCOMPRESSED_PACKET_LEN) {
                    Err(ConnPeerError::PacketTooLong)
                } else { Ok(Some((value, *index,))) }
            }
        }
    }

}
