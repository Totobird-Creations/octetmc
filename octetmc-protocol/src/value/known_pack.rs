//! Datapacks present on the server to alert the client about.


use std::borrow::Cow;


/// A datapack present on the server.
#[derive(Debug, Clone)]
pub struct KnownPack<'l> {

    /// Namespace of the pack.
    pub namespace : Cow<'l, str>,

    /// ID of the pack.
    pub id        : Cow<'l, str>,

    /// Version of the pack.
    pub version   : Cow<'l, str>

}

impl KnownPack<'_> {

    /// Convert the inner parts of this `KnownPack` to their owned counterparts, or
    ///  take ownership if they are already owned. Returns the newly created
    ///  `KnownPack<'static>`.
    #[inline]
    pub fn into_static_owned(self) -> KnownPack<'static> {
        KnownPack {
            namespace : Cow::Owned(self.namespace.into_owned()),
            id        : Cow::Owned(self.id.into_owned()),
            version   : Cow::Owned(self.version.into_owned())
        }
    }

    /// Convert the inner parts of this `KnownPack` to their owned counterparts.
    ///  Returns the newly created `KnownPack<'static>`.
    #[inline]
    pub fn to_static_owned(&self) -> KnownPack<'static> {
        KnownPack {
            namespace : Cow::Owned((*self.namespace).to_owned()),
            id        : Cow::Owned((*self.id).to_owned()),
            version   : Cow::Owned((*self.version).to_owned())
        }
    }

}
