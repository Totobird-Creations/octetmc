/// The state of a client's connection.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub(super) enum ConnPeerState {
    Handshake,
    Status,
    Login,
    ConfigPlay(ConfigPlay)
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub(super) enum ConfigPlay {
    Config { active_ticks : u8 },
    Play
}
