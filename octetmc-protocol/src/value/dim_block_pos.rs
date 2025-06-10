//! Positions of blocks in a specific world.


use crate::value::ident::Ident;
use crate::value::block_pos::BlockPos;
use crate::packet::encode::{ PacketPartEncode, EncodeBuf };


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
