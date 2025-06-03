use super::{ ConnPeerComms, ConnPeerCrypters, ConnPeerResult, ConnPeerError, MAX_READ_QUEUE_SIZE };
use crate::util::future::timeout;
use octetmc_protocol::packet::decode::{ PacketPrefixedDecode };
use core::time::Duration;
use smol::io::AsyncReadExt;


mod len;

mod decode;

mod content;

mod boxed;
use boxed::ReadPacketBoxed;


const MAX_UNCOMPRESSED_PACKET_LEN : usize = 4096;


impl ConnPeerComms {

    #[inline]
    pub(crate) fn try_read_packet<P>(&mut self) -> ConnPeerResult<Option<P::Output<'_>>>
    where
        P : PacketPrefixedDecode
    { self.try_read_packet_inner(|bytes| Self::decode_packet::<P>(bytes)) }

    #[expect(unused)]
    #[inline]
    pub(crate) fn try_read_packet_boxed<P>(&mut self) -> ConnPeerResult<Option<ReadPacketBoxed<P>>>
    where
        P : PacketPrefixedDecode
    { self.try_read_packet_inner(|bytes| Self::decode_packet_boxed::<P>(bytes)) }

    fn try_read_packet_inner<'l, F, T>(&'l mut self, f : F) -> ConnPeerResult<Option<T>>
    where
        F : FnOnce(&'l [u8]) -> ConnPeerResult<T>
    {
        let Some((total_len, _,)) = self.try_read_packet_len()?
            else { return Ok(None); };
        if (self.read_queue.len() < total_len) { // <NOTE 1>
            return Ok(None);
        }
        f(unsafe { self.read_packet_compressed_content(total_len)? }).map(Some)
    }


    #[inline]
    pub(crate) async fn read_packet<P>(&mut self) -> ConnPeerResult<P::Output<'_>>
    where
        P : PacketPrefixedDecode
    { self.read_packet_inner(|bytes| Self::decode_packet::<P>(bytes)).await }

    #[inline]
    pub(crate) async fn read_packet_boxed<P>(&mut self) -> ConnPeerResult<ReadPacketBoxed<P>>
    where
        P : PacketPrefixedDecode
    { self.read_packet_inner(|bytes| Self::decode_packet_boxed::<P>(bytes)).await }

    async fn read_packet_inner<'l, F, T>(&'l mut self, f : F) -> ConnPeerResult<T>
    where
        F : FnOnce(&'l [u8]) -> ConnPeerResult<T>
    {
        let (total_len, _,) = self.read_packet_len().await?;
        self.wait_for_bytes(total_len).await?; // <NOTE 2>
        f(unsafe { self.read_packet_compressed_content(total_len)? })
    }


    pub(crate) async fn read_packet_timeout<P>(&mut self, dur : Duration) -> ConnPeerResult<P::Output<'_>>
    where
        P : PacketPrefixedDecode
    { Self::read_packet_timeout_inner(dur, self.read_packet::<P>()).await }

    #[inline]
    pub(crate) async fn read_packet_boxed_timeout<P>(&mut self, dur : Duration) -> ConnPeerResult<ReadPacketBoxed<P>>
    where
        P : PacketPrefixedDecode
    { Self::read_packet_timeout_inner(dur, self.read_packet_boxed::<P>()).await }

    async fn read_packet_timeout_inner<F, T>(dur : Duration, f : F) -> ConnPeerResult<T>
    where
        F : Future<Output = ConnPeerResult<T>>
    { match (timeout(dur, f).await) {
        Ok(Ok(out))  => Ok(out),
        Ok(Err(err)) => Err(err),
        Err(_)       => Err(ConnPeerError::TimedOut)
    } }


    async fn wait_for_bytes(&mut self, n : usize) -> ConnPeerResult {
        while (self.read_queue.len() < n) {
            self.accept_more_bytes().await?;
        }
        Ok(())
    }

    async fn accept_more_bytes(&mut self) -> ConnPeerResult {
        let mut buf = [0u8; 64];
        match (self.stream.read(&mut buf).await?) {
            0     => Err(ConnPeerError::PeerClosed),
            count => {
                if ((self.read_queue.len() + count) > MAX_READ_QUEUE_SIZE) {
                    return Err(ConnPeerError::ReadQueueOverflow);
                }
                // Gives space for up to `block_size` 64.
                let mut decrypted_buf = [0u8; 128];
                let decrypted_buf = if let Some(ConnPeerCrypters { decrypter, .. }) = &mut self.crypters {
                    // SAFETY: `block_size` can not be greater than 64 (see `ConnPeerComs::set_crypters`)
                    let count = unsafe { decrypter.update_unchecked(
                        // SAFETY: count can never be greater than `buf.len()`.
                        buf.get_unchecked(0..count),
                        decrypted_buf.get_unchecked_mut(0..count)
                    ) }.unwrap();
                    // SAFETY: `count` can not be greater than `buf.len() + block_size`.
                    unsafe { decrypted_buf.get_unchecked(0..count) }
                } else { &buf };
                // SAFETY: count can never be greater than `buf.len()`.
                self.read_queue.extend(unsafe { decrypted_buf.get_unchecked(0..count) }.iter());
                Ok(())
            }
        }
    }

}
