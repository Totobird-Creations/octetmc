//! Sound events.


use crate::value::ident::Ident;


/// A sound event.
#[derive(Debug, Clone)]
pub struct SoundEvent<'l> {

    /// The ID resource path of the sound.
    pub sound       : Ident<'l>,

    /// The maximum range of the sound.
    pub fixed_range : Option<f32>

}
