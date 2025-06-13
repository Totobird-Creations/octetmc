//! Server information.


use crate::util::macros::deref_single;
use octetmc_protocol::value::text::Text;
use std::borrow::Cow;
use bevy_ecs::resource::Resource;


mod favicon;
pub use favicon::*;


deref_single!{
    /// The server's brand.
    ///
    /// This is displayed in the top left of the F3 debug menu,
    ///  and in server lists if the server version doesnt match the client's.
    #[derive(Resource)]
    pub struct ServerBrand(Cow<'static, str>);
    Dirtyable;
    From;
}

deref_single!{
    /// The server 'message of the day' text.
    ///
    /// The 'message of the day' is displayed in the server list.
    #[derive(Resource)]
    pub struct ServerMotd(Text<'static, 'static>);
    Dirtyable;
    From;
}


/// Common imports.
pub mod prelude {
    pub use super::{ ServerMotd, ServerFavicon };
}
