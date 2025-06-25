//! The types of characters.


use crate::value::ident::Ident;


/// The type of a character.
#[derive(Clone, Copy, Debug)]
pub struct EntityType {
    id   : u32,
    name : &'static str
}

include!(".generated/entity_type.rs");


impl EntityType {

    /// Returns the protocol ID of the character type.
    pub fn id(&self) -> u32 { self.id }

    /// Returns the name identifier of the character type.
    pub fn ident(&self) -> Ident<'static> {
        unsafe { Ident::vanilla_str_unchecked(self.name) }
    }

}
