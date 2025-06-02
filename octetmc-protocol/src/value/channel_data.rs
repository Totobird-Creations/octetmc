//! Plugin channels allow mods, plugins, and custom software to communicate.


use crate::value::ident::Ident;
use std::borrow::Cow;


/// https://minecraft.wiki/w/Java_Edition_protocol/Plugin_channels
///
/// octectmc does not support the debug channels.
#[derive(Debug, Clone)]
pub enum ChannelData<'l> {

    /// Announces the server and client implementation name right after a player has logged in.
    ///
    /// https://minecraft.wiki/w/Java_Edition_protocol/Plugin_channels#Brand
    Brand {

        /// The brand of the server/client. The vanilla value is `vanilla`.
        brand : Cow<'l, str>

    },

    /// Some other channel.
    Custom {

        /// The ID of the channel.
        channel : Ident<'l>,

        /// The message data.
        data    : Cow<'l, [u8]>

    }

}

impl<'l> ChannelData<'l> {

    /// Convert the inner parts of this `ChannelData` to their owned counterparts, or
    ///  take ownership if they are already owned. Returns the newly created
    ///  `ChannelData<'static>`.
    pub fn into_static_owned(self) -> ChannelData<'static> { match (self) {
        Self::Brand { brand } => ChannelData::Brand {
            brand : Cow::Owned(brand.into_owned())
        },
        Self::Custom { channel, data } => ChannelData::Custom {
            channel : channel.into_static_owned(),
            data    : Cow::Owned(data.into_owned())
        },
    } }

    /// Convert the inner parts of this `ChannelData` to their owned counterparts.
    ///  Returns the newly created `ChannelData<'static>`.
    #[inline]
    pub fn to_static_owned(&self) -> ChannelData<'static> { match (self) {
        Self::Brand { brand } => ChannelData::Brand {
            brand : Cow::Owned((&**brand).to_owned())
        },
        Self::Custom { channel, data } => ChannelData::Custom {
            channel : channel.to_static_owned(),
            data    : Cow::Owned((&**data).to_owned())
        },
    } }

}
