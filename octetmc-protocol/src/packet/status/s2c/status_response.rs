//! `0x00` `status_response`


use crate::value::text::Text;
use crate::packet::StateStatus;
use crate::packet::encode::{ EncodeBuf, PacketEncode, PacketPartEncode };
use std::borrow::Cow;
use uuid::Uuid;
use serde::{
    Serialize as Ser,
    Serializer as Serer
};
use serde_json::to_string as to_json_string;


/// The maximum number of players that can be displayed in a `StatusResponseS2CStatusPacket`.
pub const MAX_PLAYERS : u32 = 2u32.pow(31) - 1;

/// The base64 PNG data prefix for a favicon string.
pub const FAVICON_PREFIX : &str = "data:image/png;base64,";


/// https://minecraft.wiki/w/Java_Edition_protocol/Packets#Status_Response
#[derive(Debug, Clone, Ser)]
pub struct StatusResponseS2CStatusPacket<'a, 'b, 'c, 'd, 'e> {

    /// Server version information
    pub version             : StatusVersion<'a>,

    /// Server player information.
    ///
    /// If omitted, `???` is displayed in dark grey instead.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub players             : Option<StatusPlayers<'b>>,

    /// Server MOTD.
    ///
    /// If omitted, no MOTD is shown.
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub motd                : Option<&'c Text<'d>>,

    /// Server base64 png favicon, prefixed with
    ///  `data:image/png;base64,`.
    ///
    /// If omitted, the client will use the last known favicon,
    ///  or the default icon.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub favicon             : Option<Cow<'e, str>>,

    /// Whether this server enforces 'secure chat'.
    ///
    /// octectmc-protocol does not support 'secure chat'.
    pub enforce_secure_chat : bool,

    /// Whether this server actively prevents 'secure chat'.
    ///
    /// This is not part of the vanilla specification, but is used
    ///  by mods such as No Chat Reports.
    ///
    /// octectmc-protocol does not support 'secure chat'.
    #[serde(skip_serializing_if = "is_false")]
    pub block_chat_reports  : bool

}

/// Server version information.
#[derive(Debug, Clone, Ser)]
pub struct StatusVersion<'l> {

    /// The server's version name.
    pub name     : Cow<'l, str>,

    /// The protocol version that this server supports.
    ///
    /// If the client's protocol version does not match,
    ///  `name` is displayed in red.
    pub protocol : u32

}

/// Server player information.
#[derive(Debug, Clone, Ser)]
pub struct StatusPlayers<'l> {

    /// The number of players currently online.
    pub online : u32,

    /// The maximum number of players that this server allows.
    pub max    : u32,

    /// A sample of the currently online players.
    pub sample : Cow<'l, [StatusPlayersSampleEntry<'l>]>

}


/// An entry in a sample of currently online players.
///
/// If a client has ["Allow Server Listings"](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Client_Information_.28configuration.29)
///  set to `OFF`, vanilla servers will report their name as `Anonymous Player` with a zero UUID.
#[derive(Debug, Clone, Ser)]
pub struct StatusPlayersSampleEntry<'l> {

    /// The username of the plyer.
    pub name : Cow<'l, str>,

    /// The UUID of the player.
    #[serde(serialize_with = "ser_uuid")]
    pub id   : Uuid

}


impl PacketEncode for StatusResponseS2CStatusPacket<'_, '_, '_, '_, '_> {
    type State = StateStatus;

    const PREFIX : u8 = 0x00;

    fn predict_size(&self) -> usize { 0 }

    fn encode<'l>(&self, buf : &mut EncodeBuf) {
        let json = to_json_string(self).unwrap();
        buf.reserve(json.predict_size());
        buf.encode_write(json.as_str());
    }
}


fn is_false(v : &bool) -> bool { !*v }

fn ser_uuid<S>(v : &Uuid, serer : S) -> Result<<S as Serer>::Ok, <S as Serer>::Error>
where
    S : Serer
{ v.to_string().serialize(serer) }
