//! Plugin channels allow mods, plugins, and custom software to communicate.


use crate::value::ident::Ident;


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
        brand : &'l str

    },

    /// Some other channel.
    Custom {

        /// The ID of the channel.
        channel : Ident<'l>,

        /// The message data.
        data    : &'l [u8]

    }

}
