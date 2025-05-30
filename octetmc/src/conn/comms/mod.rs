use super::ConnPeerState;
use core::net::SocketAddr;
use std::collections::VecDeque;
use smol::net::TcpStream;
use openssl::symm::Crypter;


mod read;

mod write;


pub(super) struct ConnPeerComms {
    stream             : TcpStream,
    addr               : SocketAddr,
    read_queue         : VecDeque<u8>,
    compress_threshold : Option<usize>,
    encrypter          : Option<Crypter>,
    decrypter          : Option<Crypter>,
    state              : ConnPeerState
}


impl ConnPeerComms {

    #[inline]
    pub(super) fn new(stream : TcpStream, addr : SocketAddr) -> Self {
        Self { stream, addr,
            read_queue         : VecDeque::new(),
            compress_threshold : None,
            encrypter          : None,
            decrypter          : None,
            state              : ConnPeerState::Handshake
        }
    }

    #[inline]
    pub(super) fn addr(&self) -> SocketAddr { self.addr }

    #[inline]
    pub(super) fn set_state(&mut self, state : ConnPeerState) {
        self.state = state;
    }

    #[inline]
    pub(super) fn set_compress_threshold(&mut self, threshold : u32) {
        self.compress_threshold = Some(threshold as usize);
    }

    #[inline]
    pub(super) fn set_crypters(&mut self, encrypter : Crypter, decrypter : Crypter) {
        self.encrypter = Some(encrypter);
        self.decrypter = Some(decrypter);
    }

}
