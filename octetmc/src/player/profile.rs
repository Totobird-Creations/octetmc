use uuid::Uuid;


/// A player's game profile.
#[derive(Debug)]
pub struct PlayerProfile {

    /// Account UUID.
    pub uuid : Uuid,

    /// Username.
    pub name : String,

    /// Skin and cape textures.
    pub skin : Option<PlayerProfileSkin>

}

/// A player's skin.
#[derive(Debug)]
pub struct PlayerProfileSkin {

    /// Skin signature.
    pub sig   : Option<String>,

    /// Base64 skin texture value.
    pub value : String

}
