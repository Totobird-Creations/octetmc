//! `0x00` `status_response`


use crate::mapping_check;
use crate::value::text::Text;
use crate::packet::{ Packet, StateStatus };
use crate::packet::encode::{ EncodeBuf, PacketEncode, PacketPartEncode };
use crate::packet::prefix_check::prefix_check_status_s2c;
use std::borrow::Cow;
use uuid::Uuid;
use serde::{
    Serialize as Ser,
    Serializer as Serer
};
use serde_json::to_string as to_json_string;


mapping_check!("net.minecraft.network.protocol.status.ClientboundStatusResponsePacket", "2f22ebec17cc520ec4755254fee2e07d39b7b8084ac221c8954130fe2504de26");
mapping_check!("net.minecraft.network.protocol.status.ServerStatus", "76405cc2aaa343f60b9ebc1bad387f1fe69b84640707c269f2331be4f45c2f37");
mapping_check!("net.minecraft.network.protocol.status.ServerStatus$Favicon", "f4b731311ecee171dfaf3c214cd26c46ac853f0d41bafe9e14e2dc3e2d3c6e29");
mapping_check!("net.minecraft.network.protocol.status.ServerStatus$Players", "dc636d8091930b3ed510d4e9ccff840312f261ea089f7753c95a0ee602bd016f");
mapping_check!("net.minecraft.network.protocol.status.ServerStatus$Version", "8b609110e54927307355b2c3ab50adaedb90ee4508ed044e1d32d184a36dcd69");


/// The maximum number of players that can be displayed in a `StatusResponseS2CStatusPacket`.
pub const MAX_PLAYERS : u32 = 2u32.pow(31) - 1;

/// The base64 PNG data prefix for a favicon string.
pub const FAVICON_PREFIX : &str = "data:image/png;base64,";


/// <https://minecraft.wiki/w/Java_Edition_protocol/Packets#Status_Response>
#[derive(Debug, Clone, Ser)]
pub struct StatusResponseS2CStatusPacket<'l, 'k> {

    /// Server version information
    pub version             : StatusVersion<'l>,

    /// Server player information.
    ///
    /// If omitted, `???` is displayed in dark grey instead.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub players             : Option<StatusPlayers<'l>>,

    /// Server MOTD.
    ///
    /// If omitted, no MOTD is shown.
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub motd                : Option<Text<'l, 'k>>,

    /// Server base64 png favicon, prefixed with
    ///  `data:image/png;base64,`.
    ///
    /// If omitted, the client will use the last known favicon,
    ///  or the default icon.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub favicon             : Option<Cow<'l, str>>,

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


impl StatusResponseS2CStatusPacket<'_, '_> {

    /// Convert the inner parts of this packet to their owned counterparts, or
    ///  take ownership if they are already owned. Returns the newly created
    ///  `StatusResponseS2CStatusPacket<'static>`.
    #[inline]
    pub fn into_static_owned(self) -> StatusResponseS2CStatusPacket<'static, 'static> {
        StatusResponseS2CStatusPacket {
            version             : self.version.into_static_owned(),
            players             : self.players.map(|players| players.into_static_owned()),
            motd                : self.motd.map(|motd| motd.into_static_owned()),
            favicon             : self.favicon.map(|favicon| Cow::Owned(favicon.into_owned())),
            enforce_secure_chat : self.enforce_secure_chat,
            block_chat_reports  : self.block_chat_reports
        }
    }
    /// Convert the inner parts of this packet to their owned counterparts.
    ///  Returns the newly created `StatusResponseS2CStatusPacket<'static>`.
    #[inline]
    pub fn to_static_owned(&self) -> StatusResponseS2CStatusPacket<'static, 'static> {
        StatusResponseS2CStatusPacket {
            version             : self.version.to_static_owned(),
            players             : self.players.as_ref().map(|players| players.to_static_owned()),
            motd                : self.motd.as_ref().map(|motd| motd.to_static_owned()),
            favicon             : self.favicon.as_ref().map(|favicon| Cow::Owned((**favicon).to_owned())),
            enforce_secure_chat : self.enforce_secure_chat,
            block_chat_reports  : self.block_chat_reports
        }
    }

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


impl<'l> StatusVersion<'l> {

    /// Convert the inner parts of this `StatusVersion` to their owned counterparts, or
    ///  take ownership if they are already owned. Returns the newly created
    ///  `StatusVersion<'static>`.
    #[inline]
    pub fn into_static_owned(self) -> StatusVersion<'static> {
        StatusVersion { name : Cow::Owned(self.name.into_owned()), protocol : self.protocol }
    }

    /// Convert the inner parts of this `StatusVersion` to their owned counterparts.
    ///  Returns the newly created `StatusVersion<'static>`.
    #[inline]
    pub fn to_static_owned(&self) -> StatusVersion<'static> {
        StatusVersion { name : Cow::Owned((*self.name).to_owned()), protocol : self.protocol }
    }

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


impl<'l> StatusPlayers<'l> {

    /// Convert the inner parts of this `StatusPlayers` to their owned counterparts, or
    ///  take ownership if they are already owned. Returns the newly created
    ///  `StatusPlayers<'static>`.
    #[inline]
    pub fn into_static_owned(self) -> StatusPlayers<'static> {
        StatusPlayers {
            online : self.online,
            max    : self.max,
            sample : Cow::Owned(match (self.sample) {
                Cow::Borrowed(sample) => sample.iter().map(|entry| entry.to_static_owned()).collect(),
                Cow::Owned(sample)    => sample.into_iter().map(|entry| entry.into_static_owned()).collect(),
            })
        }
    }

    /// Convert the inner parts of this `StatusPlayers` to their owned counterparts.
    ///  Returns the newly created `StatusPlayers<'static>`.
    #[inline]
    pub fn to_static_owned(&self) -> StatusPlayers<'static> {
        StatusPlayers {
            online : self.online,
            max    : self.max,
            sample : Cow::Owned(self.sample.iter().map(|entry| entry.to_static_owned()).collect())
        }
    }

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


impl<'l> StatusPlayersSampleEntry<'l> {

    /// Convert the inner parts of this `StatusPlayersSampleEntry` to their owned counterparts, or
    ///  take ownership if they are already owned. Returns the newly created
    ///  `StatusPlayersSampleEntry<'static>`.
    #[inline]
    pub fn into_static_owned(self) -> StatusPlayersSampleEntry<'static> {
        StatusPlayersSampleEntry { name : Cow::Owned(self.name.into_owned()), id : self.id }
    }

    /// Convert the inner parts of this `StatusPlayersSampleEntry` to their owned counterparts.
    ///  Returns the newly created `StatusPlayersSampleEntry<'static>`.
    #[inline]
    pub fn to_static_owned(&self) -> StatusPlayersSampleEntry<'static> {
        StatusPlayersSampleEntry { name : Cow::Owned((*self.name).to_owned()), id : self.id }
    }

}


impl crate::Sealed for StatusResponseS2CStatusPacket<'_, '_> { }

impl Packet for StatusResponseS2CStatusPacket<'_, '_> { }


impl PacketEncode for StatusResponseS2CStatusPacket<'_, '_> {
    type State = StateStatus;

    const PREFIX : u8 = prefix_check_status_s2c!(status_response, 0x00);

    fn predict_size(&self) -> usize { 0 }

    fn encode<'l>(&self, buf : &mut EncodeBuf) {
        let json = to_json_string(self).unwrap();
        buf.reserve(json.predict_size());
        buf.encode_write(json);
    }
}


fn is_false(v : &bool) -> bool { !*v }

fn ser_uuid<S>(v : &Uuid, serer : S) -> Result<<S as Serer>::Ok, <S as Serer>::Error>
where
    S : Serer
{ v.to_string().serialize(serer) }
