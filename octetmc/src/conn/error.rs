//! Client connection error types.


use octetmc_protocol::packet::PacketState;
use octetmc_protocol::value::text::Text;
use core::fmt;
use std::borrow::Cow;
use std::io;


/// A [`Result`] with a [`ConnPeerError`] error.
pub type ConnPeerResult<T = ()> = Result<T, ConnPeerError>;

/// An error raised by a connected peer.
pub enum ConnPeerError {

    /// Declaring intention or logging in took too long, or a keepalive was not responded to in time.
    TimedOut,

    /// The client's protocol version does not match the server's.
    ProtocolMismatch {
        /// The protocol version that was sent by the client.
        client : u32,
        /// The expected protocol version.
        server : u32
    },

    /// The client sent too much data in too short of a time span.
    ReadQueueOverflow,

    /// An invalid packet length was received.
    InvalidPacketLength,

    /// A received packet is too long.
    PacketTooLong,

    /// A received packet has an unknown or unexpected ID.
    UnknownPacketPrefix {

        /// The state the connection is currently in.
        state  : PacketState,

        /// The packet prefix that was read.
        prefix : u8

    },

    /// A received packet could not be decoded.
    BadPacket(Cow<'static, str>),

    /// A received packet was longer than the decoder expected.
    NoPacketEnd,

    /// A received packet could not be decompressed.
    BadPacketZlib,

    /// The username that the client sent is too long.
    UsernameTooLong,

    /// Failed to exchange pkey and establish shared secret cipher.
    KeyExchangeFailed,

    /// The mojauth servers are currently unreachable.
    AuthServerUnreachable,

    /// The mojauth servers returned unrecognised data.
    BadAuthServer,

    /// The client's profile could not be verified.
    AuthFailed,

    /// The player is already logged in.
    AlreadyLoggedIn,

    /// The client closed the connection.
    PeerClosed,

    /// The server kicked the client.
    Kicked(Text<'static>),

    /// Some other IO error occured.
    Io(io::Error)

}

impl From<io::Error> for ConnPeerError {
    fn from(value : io::Error) -> Self { Self::Io(value) }
}


impl fmt::Debug for ConnPeerError {
    fn fmt(&self, f : &mut fmt::Formatter<'_>) -> fmt::Result { match (self) {
        ConnPeerError::TimedOut                            => write!(f, "TimedOut"),
        ConnPeerError::ProtocolMismatch { client, server } => f.debug_struct("ProtocolMismatch").field("client", client).field("server", server).finish(),
        ConnPeerError::ReadQueueOverflow                   => write!(f, "ReadQueueOverflow"),
        ConnPeerError::InvalidPacketLength                 => write!(f, "InvalidPacketLength"),
        ConnPeerError::PacketTooLong                       => write!(f, "PacketTooLong"),
        ConnPeerError::UnknownPacketPrefix {state, prefix} => f.debug_struct("UnknownPacketPrefix").field("state", state).field("prefix", &DebugByteHex(*prefix)).finish(),
        ConnPeerError::BadPacket(err)                      => f.debug_tuple("BadPacket").field(err).finish(),
        ConnPeerError::NoPacketEnd                         => write!(f, "NoPacketEnd"),
        ConnPeerError::BadPacketZlib                       => write!(f, "BadPacketZlib"),
        ConnPeerError::UsernameTooLong                     => write!(f, "UsernameTooLong"),
        ConnPeerError::KeyExchangeFailed                   => write!(f, "KeyExchangeFailed"),
        ConnPeerError::AuthServerUnreachable               => write!(f, "AuthServerUnreachable"),
        ConnPeerError::BadAuthServer                       => write!(f, "BadAuthServer"),
        ConnPeerError::AuthFailed                          => write!(f, "AuthFailed"),
        ConnPeerError::AlreadyLoggedIn                     => write!(f, "AlreadyLoggedIn"),
        ConnPeerError::PeerClosed                          => write!(f, "PeerClosed"),
        ConnPeerError::Kicked(text)                        => f.debug_tuple("Kicked").field(&DebugText(text)).finish(),
        ConnPeerError::Io(err)                             => f.debug_tuple("Io").field(err).finish(),
    } }
}


struct DebugByteHex(u8);
impl fmt::Debug for DebugByteHex {
    fn fmt(&self, f : &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut buf = [0u8; 2];
        _ = hex::encode_to_slice(&[self.0], &mut buf);
        write!(f, "0x{}{}", buf[0] as char, buf[1] as char)
    }
}

struct DebugText<'l, 'k>(&'l Text<'k>);
impl fmt::Debug for DebugText<'_, '_> {
    fn fmt(&self, f : &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "\"")?;
        self.0.str_debug_display(f)?;
        write!(f, "\"")
    }
}
