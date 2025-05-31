//! Player game profiles


use crate::value::varint::VarInt;
use crate::packet::encode::{ PacketPartEncode, EncodeBuf };
use uuid::Uuid;
use std::borrow::Cow;


const SKIN_KEY : &str = "textures";


/// A player's game profile.
#[derive(Debug, Clone)]
pub struct PlayerProfile<'l> {

    /// Account UUID.
    pub uuid : Uuid,

    /// Username.
    pub name : Cow<'l, str>,

    /// Skin and cape textures.
    pub skin : Option<PlayerProfileSkin<'l>>

}

/// A player's skin.
#[derive(Debug, Clone)]
pub struct PlayerProfileSkin<'l> {

    /// Skin signature.
    pub sig   : Option<Cow<'l, str>>,

    /// Base64 skin texture value.
    pub value : Cow<'l, str>

}


impl PacketPartEncode for PlayerProfile<'_> {

    fn predict_size(&self) -> usize {
        self.uuid.predict_size()
        + self.name.predict_size()
        + (if let Some(skin) = &self.skin {
            // prefixed array len
            VarInt::<u32>::MAX_BYTES
            // name
            + VarInt::<u32>::MAX_BYTES
            + SKIN_KEY.len()
            // value
            + VarInt::<u32>::MAX_BYTES
            + skin.value.len()
            // signature
            + 1
            + (if let Some(sig) = &skin.sig {
                VarInt::<u32>::MAX_BYTES
                + sig.len()
            } else { 0 })
        } else { 0 })
    }

    fn encode(&self, buf : &mut EncodeBuf) {
        buf.encode_write(&self.uuid);
        buf.encode_write(&*self.name);
        if let Some(skin) = &self.skin {
            buf.encode_write(&VarInt::<u32>::from(1));
            buf.encode_write(SKIN_KEY);
            buf.encode_write(&*skin.value);
            buf.encode_write(skin.sig.as_ref());

        } else {
            buf.encode_write(&VarInt::<u32>::from(0));
        }
    }

}
