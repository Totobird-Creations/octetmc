//! Colour datastructures.


use serde::{
    Serialize as Ser,
    Serializer as Serer
};


/// A 24-bit RGB colour.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct Rgb {
    /// Red lane.
    pub r : u8,
    /// Green lane.
    pub g : u8,
    /// Blue lane.
    pub b : u8
}

impl Rgb {

    /// Decodes an integer value to an `Rgb` colour.
    pub const fn from_u32(v : u32) -> Self { Self {
        r : ((v >> 16) & 0xff) as u8,
        g : ((v >> 8) & 0xff) as u8,
        b : (v & 0xff) as u8
    } }

    /// Encodes an `Rgb` colour to an integer value.
    pub const fn to_u32(&self) -> u32 {
        ((self.r as u32) << 16)
        | ((self.g as u32) << 8)
        | (self.b as u32)
    }

}


/// A 32-bit ARGB colour.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct Argb {
    /// Alpha lane.
    pub a : u8,
    /// Red lane.
    pub r : u8,
    /// Green lane.
    pub g : u8,
    /// Blue lane.
    pub b : u8
}

impl Ser for Argb {
    fn serialize<S>(&self, serer : S) -> Result<<S as Serer>::Ok, <S as Serer>::Error>
    where
        S : Serer
    {
        (((self.a as u32) << 24)
        | ((self.r as u32) << 16)
        | ((self.g as u32) << 8)
        | (self.b as u32)).serialize(serer)
    }
}
