use super::{ ConnPeerState, ConfigPlay };
use super::event::ConnPeerEvent;
use core::net::SocketAddr;
use core::hint::unreachable_unchecked;
use std::collections::VecDeque;
use smol::net::TcpStream;
use smol::channel;
use openssl::symm::Crypter;


mod read;

mod write;


const MAX_READ_QUEUE_SIZE : usize = 4096;


pub(super) struct ConnPeerComms {
    stream             : TcpStream,
    addr               : SocketAddr,
    read_queue         : VecDeque<u8>,
    compress_threshold : Option<usize>,
    encrypter          : Option<Crypter>,
    decrypter          : Option<Crypter>,
    state              : ConnPeerState,
    conn_sender        : Option<channel::Sender<ConnPeerEvent>>,
    conn_receiver      : channel::Receiver<ConnPeerEvent>
}


impl ConnPeerComms {

    #[inline]
    pub(super) fn new(stream : TcpStream, addr : SocketAddr) -> Self {
        let (conn_sender, conn_receiver,) = channel::unbounded();
        Self { stream, addr,
            read_queue         : VecDeque::with_capacity(MAX_READ_QUEUE_SIZE),
            compress_threshold : None,
            encrypter          : None,
            decrypter          : None,
            state              : ConnPeerState::Handshake,
            conn_sender        : Some(conn_sender),
            conn_receiver
        }
    }

    #[inline]
    pub(super) fn state(&self) -> ConnPeerState { self.state }
    #[inline]
    pub(super) unsafe fn state_assume_config_play(&self) -> ConfigPlay {
        let ConnPeerState::ConfigPlay(state) = self.state
            else { unsafe { unreachable_unchecked(); } };
        state
    }

    #[inline]
    pub(super) fn set_state(&mut self, state : ConnPeerState) {
        self.state = state;
    }
    #[inline(always)]
    pub(super) fn set_state_config_play(&mut self, state : ConfigPlay) {
        self.set_state(ConnPeerState::ConfigPlay(state));
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

    #[inline]
    pub(super) unsafe fn take_conn_sender_unchecked(&mut self) -> channel::Sender<ConnPeerEvent> {
        unsafe { self.conn_sender.take().unwrap_unchecked() }
    }

}
