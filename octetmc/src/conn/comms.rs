use super::{ ConnPeerResult, ConnPeerError };
use crate::util::future::timeout;
use octetmc_protocol::packet::{ BoundC2S, PacketBoundState };
use octetmc_protocol::packet::decode::PacketDecode;
use core::net::SocketAddr;
use core::time::Duration;
use smol::net::TcpStream;



pub(super) struct ConnPeerComms {
    stream : TcpStream,
    addr   : SocketAddr
}

impl ConnPeerComms {
    pub(super) fn new(stream : TcpStream, addr : SocketAddr) -> Self {
        Self { stream, addr }
    }
}

impl ConnPeerComms {

    pub(super) async fn read_packet<P>(&mut self) -> ConnPeerResult<P>
    where
        P                                      : PacketDecode<Bound = BoundC2S>,
        (BoundC2S, <P as PacketDecode>::State) : PacketBoundState
    { todo!(); }

    pub(super) async fn read_packet_timeout<P>(&mut self, dur : Duration) -> ConnPeerResult<P>
    where
        P                                      : PacketDecode<Bound = BoundC2S>,
        (BoundC2S, <P as PacketDecode>::State) : PacketBoundState
    { match (timeout(dur, self.read_packet::<P>()).await) {
        Ok(Ok(out))  => Ok(out),
        Ok(Err(err)) => Err(err),
        Err(_)       => Err(ConnPeerError::TimedOut)
    } }

}
