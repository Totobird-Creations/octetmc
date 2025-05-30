use crate::value::text::Text;
use crate::packet::StateStatus;
use crate::packet::encode::{ EncodeBuf, PacketEncode };
use std::borrow::Cow;
use uuid::Uuid;
use serde::{
    Serialize as Ser,
    Serializer as Serer
};
use serde_json::to_string as json_to_string;


#[derive(Debug, Clone, Ser)]
pub struct StatusResponseS2CStatusPacket<'l> {
    pub version             : StatusVersion<'l>,
    pub players             : Option<StatusPlayers<'l>>,
    pub motd                : Text<'l>,
    pub favicon             : Cow<'l, str>,
    pub enforce_secure_chat : bool,
    #[serde(skip_serializing_if = "is_false")]
    pub block_chat_reports  : bool
}

#[derive(Debug, Clone, Ser)]
pub struct StatusVersion<'l> {
    pub name     : Cow<'l, str>,
    pub protocol : u32
}

#[derive(Debug, Clone, Ser)]
pub struct StatusPlayers<'l> {
    pub online : u32,
    pub max    : u32,
    pub sample : Cow<'l, [StatusPlayersSampleEntry<'l>]>
}


#[derive(Debug, Clone, Ser)]
pub struct StatusPlayersSampleEntry<'l> {
    pub name : Cow<'l, str>,
    #[serde(serialize_with = "ser_uuid")]
    pub id   : Uuid
}


impl PacketEncode for StatusResponseS2CStatusPacket<'_> {
    type State = StateStatus;

    const PREFIX : u8 = 0x00;

    fn encode<'l>(&self, buf : &mut EncodeBuf) {
        buf.encode_write(json_to_string(self).unwrap().as_str());
    }
}


fn is_false(v : &bool) -> bool { !*v }

fn ser_uuid<S>(v : &Uuid, serer : S) -> Result<<S as Serer>::Ok, <S as Serer>::Error>
where
    S : Serer
{ v.to_string().serialize(serer) }
