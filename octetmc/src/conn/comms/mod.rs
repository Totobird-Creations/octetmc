use super::error::{ ConnPeerResult, ConnPeerError };
use super::state::{ ConnPeerState, ConfigPlay };
use super::out_message::ConnPeerOutMessage;
use super::in_message::ConnPeerInMessage;
use octetmc_protocol::value::text::{ Text, TextComponent, TextContent, TextStyle, TextInteract };
use core::net::SocketAddr;
use core::hint::unreachable_unchecked;
use std::collections::VecDeque;
use std::borrow::Cow;
use smol::net::TcpStream;
use smol::channel;
use openssl::symm::Crypter;


mod read;

mod write;


const MAX_READ_QUEUE_SIZE : usize = 4096;

const GENERIC_KICK_MESSAGE : Text<'_, '_> = Text { components : Cow::Borrowed(&[
    TextComponent {
        content  : TextContent::Translate {
            key      : Cow::Borrowed("multiplayer.disconnect.duplicate_login"),
            fallback : None,
            with     : Cow::Borrowed(&[])
        },
        style    : TextStyle::NONE,
        interact : TextInteract::NONE,
        extra    : Cow::Borrowed(&[])
    }
]) };


pub(super) struct ConnPeerComms {
    stream             : TcpStream,
    #[expect(dead_code)]
    addr               : SocketAddr,
    read_queue         : VecDeque<u8>,
    buf0               : Vec<u8>, // For reading and encoding.
    buf1               : Vec<u8>, // For decompression and compression.
    buf2               : Vec<u8>, // For encryption.
    compress_threshold : Option<usize>,
    crypters           : Option<ConnPeerCrypters>,
    state              : ConnPeerState,
    conn_out_sender    : Option<channel::Sender<ConnPeerOutMessage>>,
    conn_out_receiver  : channel::Receiver<ConnPeerOutMessage>,
    conn_in_sender     : channel::Sender<ConnPeerInMessage>,
    conn_in_receiver   : Option<channel::Receiver<ConnPeerInMessage>>
}

pub(super) struct ConnPeerCrypters {
    encrypter  : Crypter,
    decrypter  : Crypter,
    block_size : usize
}


impl ConnPeerComms {

    #[inline]
    pub(super) fn new(stream : TcpStream, addr : SocketAddr) -> Self {
        let (conn_out_sender, conn_out_receiver,) = channel::unbounded();
        let (conn_in_sender,  conn_in_receiver,)  = channel::unbounded();
        stream.set_nodelay(true).unwrap();
        Self { stream, addr,
            read_queue         : VecDeque::with_capacity(MAX_READ_QUEUE_SIZE),
            buf0               : Vec::new(),
            buf1               : Vec::new(),
            buf2               : Vec::new(),
            compress_threshold : None,
            crypters           : None,
            state              : ConnPeerState::Handshake,
            conn_out_sender    : Some(conn_out_sender),
            conn_out_receiver,
            conn_in_sender,
            conn_in_receiver   : Some(conn_in_receiver)
        }
    }

    #[inline]
    pub(super) unsafe fn state_assume_config_play(&mut self) -> &mut ConfigPlay {
        let ConnPeerState::ConfigPlay(state) = &mut self.state
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
    pub(super) fn set_crypters(&mut self, encrypter : Crypter, decrypter : Crypter, block_size : usize) {
        assert!(block_size <= 64);
        self.crypters = Some(ConnPeerCrypters { encrypter, decrypter, block_size });
    }

    #[inline]
    pub(super) unsafe fn take_mainloop_conn_channels_unchecked(&mut self)
        -> (channel::Sender<ConnPeerOutMessage>, channel::Receiver<ConnPeerInMessage>,)
    { unsafe { (
        self.conn_out_sender.take().unwrap_unchecked(),
        self.conn_in_receiver.take().unwrap_unchecked()
    ) } }

    #[inline]
    pub(super) fn try_read_out_message(&self) -> ConnPeerResult<Option<ConnPeerOutMessage>> {
        match (self.conn_out_receiver.try_recv()) {
            Ok(event)                          => Ok(Some(event)),
            Err(channel::TryRecvError::Empty)  => Ok(None),
            Err(channel::TryRecvError::Closed) => Err(ConnPeerError::Kicked(GENERIC_KICK_MESSAGE))
        }
    }

    #[inline]
    pub(super) async fn send_in_message(&self, message : ConnPeerInMessage)
        -> Result<(), channel::SendError<ConnPeerInMessage>>
    { self.conn_in_sender.send(message).await }

}
