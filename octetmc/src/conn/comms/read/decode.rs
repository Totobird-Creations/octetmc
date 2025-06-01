use super::{ ConnPeerComms, ReadPacketBoxed };
use crate::conn::{ ConnPeerResult, ConnPeerError };
use octetmc_protocol::packet::decode::{ PacketPrefixedDecode, DecodeBufHead, DecodeBuf, UnknownPrefix };
use core::mem::{ self, ManuallyDrop };
use core::pin::Pin;
use core::ptr;


impl ConnPeerComms {

    pub(super) fn decode_packet<P>(bytes : &[u8]) -> ConnPeerResult<P::Output<'_>>
    where
        P : PacketPrefixedDecode
    {
        let mut head = DecodeBufHead::default();
        let packet = P::decode_prefixed(DecodeBuf::from(bytes), &mut head).map_err(|e| match (e) {
            UnknownPrefix::UnknownPrefix(p) => ConnPeerError::UnknownPacketPrefix(p),
            UnknownPrefix::Error(e)         => ConnPeerError::BadPacket(e.into()),
        })?;
        if (head.consumed() < bytes.len()) { return Err(ConnPeerError::NoPacketEnd); }
        Ok(packet)
    }


    pub(super) fn decode_packet_boxed<P>(bytes : &[u8]) -> ConnPeerResult<ReadPacketBoxed<P>>
    where
        P : PacketPrefixedDecode
    {
        let mut bytes_boxed = Box::<[u8]>::new_uninit_slice(bytes.len());
        // SAFETY: `bytes` and `bytes_boxed` have the same length.
        unsafe { ptr::copy_nonoverlapping(bytes.as_ptr(), bytes_boxed.as_mut_ptr() as _, bytes.len()); }
        // SAFETY: All elements were initialised in the line above.
        let bytes_boxed = Pin::new(unsafe { bytes_boxed.assume_init() });

        let mut head        = DecodeBufHead::default();
        // SAFETY: `bytes_boxed` is kept alive by `ReadPacketBoxed<P>`.
        let packet = P::decode_prefixed(DecodeBuf::from(unsafe { mem::transmute::<&[u8], &[u8]>(&*bytes_boxed) }), &mut head).map_err(|e| match (e) {
            UnknownPrefix::UnknownPrefix(p) => ConnPeerError::UnknownPacketPrefix(p),
            UnknownPrefix::Error(e)         => ConnPeerError::BadPacket(e.into()),
        })?;
        if (head.consumed() < bytes.len()) { return Err(ConnPeerError::NoPacketEnd); }
        Ok(ReadPacketBoxed {
            raw    : ManuallyDrop::new(bytes_boxed),
            packet : ManuallyDrop::new(packet)
        })
    }

}
