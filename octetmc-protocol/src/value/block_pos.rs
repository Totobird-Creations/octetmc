//! Positions of blocks in worlds.


use crate::value::ident::Ident;
use crate::packet::encode::{ PacketPartEncode, EncodeBuf };


/// A block position in a world.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct BlockPos {

    /// X coordinate.
    ///
    /// Positive is east, negative is west.
    pub x : i32,

    /// Y coordinate.
    ///
    /// Positive is up, negative is down.
    pub y : i32,

    /// Z coordinate.
    ///
    /// Positive is south, negative is north.
    pub z : i32

}

impl PacketPartEncode for BlockPos {

    #[inline(always)]
    fn predict_size(&self) -> usize { 8 }

    #[inline]
    fn encode(&self, buf : &mut EncodeBuf) {
        buf.encode_write((((self.x as u64) & 0x3FFFFFF) << 38) | (((self.z as u64) & 0x3FFFFFF) << 12) | ((self.y as u64) & 0xFFF));
    }

}


/// The position of a block in some dimension.
#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct DimBlockPos<'l> {

    /// The dimension the block is in.
    pub dimension : Ident<'l>,

    /// The block position.
    pub pos       : BlockPos

}

impl DimBlockPos<'_> {

    /// Convert the inner parts of this `DimBlockPos` to their owned counterparts, or
    ///  take ownership if they are already owned. Returns the newly created
    ///  `DimBlockPos<'static>`.
    #[inline]
    pub fn into_static_owned(self) -> DimBlockPos<'static> {
        DimBlockPos { dimension : self.dimension.into_static_owned(), pos : self.pos }
    }

    /// Convert the inner parts of this `DimBlockPos` to their owned counterparts.
    ///  Returns the newly created `DimBlockPos<'static>`.
    #[inline]
    pub fn to_static_owned(&self) -> DimBlockPos<'static> {
        DimBlockPos { dimension : self.dimension.to_static_owned(), pos : self.pos }
    }

}

impl PacketPartEncode for DimBlockPos<'_> {

    #[inline]
    fn predict_size(&self) -> usize {
        self.dimension.predict_size() + self.pos.predict_size()
    }

    #[inline]
    fn encode(&self, buf : &mut EncodeBuf) {
        buf.encode_write(&self.dimension);
        buf.encode_write(self.pos);
    }

}
