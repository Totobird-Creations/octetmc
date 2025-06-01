use super::{ ConnPeerState, ConfigPlay, ConnPeerResult, ConnPeerError };
use super::event::ConnPeerEvent;
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

const GENERIC_KICK_MESSAGE : Text<'_> = Text { components : Cow::Borrowed(&[
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
    addr               : SocketAddr,
    read_queue         : VecDeque<u8>,
    write_buf0         : Vec<u8>, // For encoding.
    write_buf1         : Vec<u8>, // For compression.
    write_buf2         : Vec<u8>, // For encryption.
    compress_threshold : Option<usize>,
    crypters           : Option<ConnPeerCrypters>,
    state              : ConnPeerState,
    conn_sender        : Option<channel::Sender<ConnPeerEvent>>,
    conn_receiver      : channel::Receiver<ConnPeerEvent>
}

pub(super) struct ConnPeerCrypters {
    encrypter  : Crypter,
    decrypter  : Crypter,
    block_size : usize
}


impl ConnPeerComms {

    #[inline]
    pub(super) fn new(stream : TcpStream, addr : SocketAddr) -> Self {
        let (conn_sender, conn_receiver,) = channel::unbounded();
        stream.set_nodelay(true).unwrap();
        Self { stream, addr,
            read_queue         : VecDeque::with_capacity(MAX_READ_QUEUE_SIZE),
            write_buf0         : Vec::new(),
            write_buf1         : Vec::new(),
            write_buf2         : Vec::new(),
            compress_threshold : None,
            crypters           : None,
            state              : ConnPeerState::Handshake,
            conn_sender        : Some(conn_sender),
            conn_receiver
        }
    }

    #[inline]
    pub(super) fn state(&self) -> ConnPeerState { self.state }
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
    pub(super) async unsafe fn switch_state_play(&mut self) -> ConnPeerResult {
        if let ConfigPlay::Play = (unsafe { self.state_assume_config_play() }) {
            return Ok(());
        }
        todo!("switch state play");
        //self.send_packet(packet).await?;
        //self.set_state_config_play(ConfigPlay::Play);
        //Ok(())
    }
    #[inline]
    pub(super) async unsafe fn switch_state_config(&mut self) -> ConnPeerResult {
        if let ConfigPlay::Config { active_ticks } = (unsafe { self.state_assume_config_play() }) {
            *active_ticks = 0;
        } else {
            todo!("switch state config");
            //self.send_packet(packet).await?;
            //self.set_state_config_play(ConfigPlay::Config { active_ticks : 0 });
        }
        Ok(())
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
    pub(super) unsafe fn take_conn_sender_unchecked(&mut self) -> channel::Sender<ConnPeerEvent> {
        unsafe { self.conn_sender.take().unwrap_unchecked() }
    }

    #[inline]
    pub(super) fn try_read_event(&self) -> ConnPeerResult<Option<ConnPeerEvent>> {
        match (self.conn_receiver.try_recv()) {
            Ok(event)                          => Ok(Some(event)),
            Err(channel::TryRecvError::Empty)  => Ok(None),
            Err(channel::TryRecvError::Closed) => Err(ConnPeerError::Kicked(GENERIC_KICK_MESSAGE))
        }
    }
}
