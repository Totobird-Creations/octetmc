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

impl<'l> PlayerProfile<'l> {

    /// Convert the inner parts of this `PlayerProfile` to their owned counterparts, or
    ///  take ownership if they are already owned. Returns the newly created
    ///  `PlayerProfile<'static>`.
    #[inline]
    pub fn into_static_owned(self) -> PlayerProfile<'static> {
        PlayerProfile {
            uuid : self.uuid,
            name : Cow::Owned(self.name.into_owned()),
            skin : self.skin.map(|skin| skin.into_static_owned())
        }
    }

    /// Convert the inner parts of this `PlayerProfile` to their owned counterparts.
    ///  Returns the newly created `PlayerProfile<'static>`.
    #[inline]
    pub fn to_static_owned(&self) -> PlayerProfile<'static> {
        PlayerProfile {
            uuid : self.uuid,
            name : Cow::Owned((&*self.name).to_owned()),
            skin : self.skin.as_ref().map(|skin| skin.to_static_owned())
        }
    }

}

impl PlayerProfileSkin<'_> {

    /// Convert the inner parts of this `PlayerProfileSkin` to their owned counterparts, or
    ///  take ownership if they are already owned. Returns the newly created
    ///  `PlayerProfileSkin<'static>`.
    #[inline]
    pub fn into_static_owned(self) -> PlayerProfileSkin<'static> {
        PlayerProfileSkin { sig : self.sig.map(|sig| Cow::Owned(sig.into_owned())), value : Cow::Owned(self.value.into_owned()) }
    }

    /// Convert the inner parts of this `PlayerProfileSkin` to their owned counterparts.
    ///  Returns the newly created `PlayerProfileSkin<'static>`.
    #[inline]
    pub fn to_static_owned(&self) -> PlayerProfileSkin<'static> {
        PlayerProfileSkin { sig : self.sig.as_ref().map(|sig| Cow::Owned((&**sig).to_owned())), value : Cow::Owned((&*self.value).to_owned()) }
    }

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
        buf.encode_write(self.uuid);
        buf.encode_write(&self.name);
        if let Some(skin) = &self.skin {
            buf.encode_write(VarInt::<u32>::from(1));
            buf.encode_write(SKIN_KEY);
            buf.encode_write(&skin.value);
            buf.encode_write(skin.sig.as_ref());
        } else {
            buf.encode_write(VarInt::<u32>::from(0));
        }
    }

}
