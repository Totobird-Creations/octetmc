//! Sound events.


use crate::mapping_check;
use crate::value::ident::Ident;


mapping_check!("net.minecraft.sounds.SoundEvent", "5ac5b6fe4a3ec5562b00d480fc09830347d1fa888394dc596c40d84f6503b71e");


/// A sound event.
#[derive(Debug, Clone)]
pub struct SoundEvent<'l> {

    /// The ID resource path of the sound.
    pub sound       : Ident<'l>,

    /// The maximum range of the sound.
    pub fixed_range : Option<f32>

}
