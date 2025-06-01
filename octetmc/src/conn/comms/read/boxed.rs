use octetmc_protocol::packet::decode::{ PacketPrefixedDecode };
use core::mem::{ self, ManuallyDrop };
use core::pin::Pin;


#[must_use]
pub(crate) struct ReadPacketBoxed<P>
where
    P : PacketPrefixedDecode
{
    pub(super) raw    : ManuallyDrop<Pin<Box<[u8]>>>,
    pub(super) packet : ManuallyDrop<P::Output<'static>>
}

impl<P> ReadPacketBoxed<P>
where
    P : PacketPrefixedDecode
{
    #[inline(always)]
    pub(crate) fn get<'l>(&'l self) -> &'l P::Output<'l> {
        // SAFETY: This just changes the lifetime so that it can not be used after `self` is dropped.
        unsafe { mem::transmute::<&P::Output<'static>, &P::Output<'l>>(&self.packet) }
    }
}

// Forces `self.raw` to live until after `self.packet` is dropped.
impl<P> Drop for ReadPacketBoxed< P>
where
    P : PacketPrefixedDecode
{
    fn drop(&mut self) {
        // SAFETY: `self.packet` still hasn't been dropped.
        unsafe { ManuallyDrop::drop(&mut self.packet); }
        // SAFETY: `self.raw` still hasn't been dropped.
        unsafe { ManuallyDrop::drop(&mut self.raw); }
    }
}
