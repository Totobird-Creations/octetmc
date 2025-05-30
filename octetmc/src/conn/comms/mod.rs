use super::ConnPeerState;
use core::net::SocketAddr;
use std::collections::VecDeque;
use smol::net::TcpStream;


mod read;

mod write;


pub(super) struct ConnPeerComms {
    stream             : TcpStream,
    addr               : SocketAddr,
    read_queue         : VecDeque<u8>,
    compress_threshold : Option<usize>,
    state              : ConnPeerState
}


impl ConnPeerComms {

    #[inline]
    pub(super) fn new(stream : TcpStream, addr : SocketAddr) -> Self {
        Self { stream, addr,
            read_queue         : VecDeque::new(),
            compress_threshold : None,
            state              : ConnPeerState::Handshake
        }
    }

    #[inline]
    pub(super) fn set_state(&mut self, state : ConnPeerState) {
        self.state = state;
    }

    #[inline]
    pub(super) fn set_compress_threshold(&mut self, threshold : u32) {
        self.compress_threshold = Some(threshold as usize);
    }

}
