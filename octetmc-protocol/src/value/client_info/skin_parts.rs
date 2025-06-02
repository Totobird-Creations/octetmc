use crate::packet::decode::{ PacketPartDecode, DecodeBuf, DecodeBufHead, IncompleteData };
use core::fmt;


const CAPE_BIT            : u8 = 0b00000001;
const JACKET_BIT          : u8 = 0b00000010;
const LEFT_SLEEVE_BIT     : u8 = 0b00000100;
const RIGHT_SLEEVE_BIT    : u8 = 0b00001000;
const LEFT_PANTS_LEG_BIT  : u8 = 0b00010000;
const RIGHT_PANTS_LEG_BIT : u8 = 0b00100000;
const HAT_BIT             : u8 = 0b01000000;


/// Parts of the player's skin which are set to visible.
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct PlayerSkinParts(u8);


impl PlayerSkinParts {

    /// All skin layers are visible.
    pub const ALL_VISIBLE : Self = Self(0b01111111);

    /// All skin layers are hidden.
    pub const ALL_HIDDEN : Self = Self(0b00000000);

}


impl PlayerSkinParts {

    #[inline]
    fn get(&self, mask : u8) -> bool {
        (self.0 & mask) != 0
    }

    #[inline]
    fn set(&mut self, mask : u8, v : bool) -> &mut Self {
        if (v) { self.0 |= mask; } else { self.0 &= ! mask; }
        self
    }

    #[inline]
    fn with(mut self, mask : u8, v : bool) -> Self {
        self.set(mask, v);
        self
    }


    /// Get the cape flag.
    #[inline(always)]
    pub fn cape(&self) -> bool { self.get(CAPE_BIT) }

    /// Set the cape flag.
    #[inline(always)]
    pub fn set_cape(&mut self, v : bool) -> &mut Self { self.set(CAPE_BIT, v) }

    /// Set the cape flag.
    #[inline(always)]
    pub fn with_cape(self, v : bool) -> Self { self.with(CAPE_BIT, v) }


    /// Get the jacket flag.
    #[inline(always)]
    pub fn jacket(&self) -> bool { self.get(JACKET_BIT) }

    /// Set the jacket flag.
    #[inline(always)]
    pub fn set_jacket(&mut self, v : bool) -> &mut Self { self.set(JACKET_BIT, v) }

    /// Set the jacket flag.
    #[inline(always)]
    pub fn with_jacket(self, v : bool) -> Self { self.with(JACKET_BIT, v) }


    /// Get the left sleeve flag.
    #[inline(always)]
    pub fn left_sleeve(&self) -> bool { self.get(LEFT_SLEEVE_BIT) }

    /// Set the left sleeve flag.
    #[inline(always)]
    pub fn set_left_sleeve(&mut self, v : bool) -> &mut Self { self.set(LEFT_SLEEVE_BIT, v) }

    /// Set the left sleeve flag.
    #[inline(always)]
    pub fn with_left_sleeve(self, v : bool) -> Self { self.with(LEFT_SLEEVE_BIT, v) }


    /// Get the right sleeve flag.
    #[inline(always)]
    pub fn right_sleeve(&self) -> bool { self.get(RIGHT_SLEEVE_BIT) }

    /// Set the right sleeve flag.
    #[inline(always)]
    pub fn set_right_sleeve(&mut self, v : bool) -> &mut Self { self.set(RIGHT_SLEEVE_BIT, v) }

    /// Set the right sleeve flag.
    #[inline(always)]
    pub fn with_right_sleeve(self, v : bool) -> Self { self.with(RIGHT_SLEEVE_BIT, v) }


    /// Get the left pants leg flag.
    #[inline(always)]
    pub fn left_pants_leg(&self) -> bool { self.get(LEFT_PANTS_LEG_BIT) }

    /// Set the left pants leg flag.
    #[inline(always)]
    pub fn set_left_pants_leg(&mut self, v : bool) -> &mut Self { self.set(LEFT_PANTS_LEG_BIT, v) }

    /// Set the left pants leg flag.
    #[inline(always)]
    pub fn with_left_pants_leg(self, v : bool) -> Self { self.with(LEFT_PANTS_LEG_BIT, v) }


    /// Get the right pants leg flag.
    #[inline(always)]
    pub fn right_pants_leg(&self) -> bool { self.get(RIGHT_PANTS_LEG_BIT) }

    /// Set the right pants leg flag.
    #[inline(always)]
    pub fn set_right_pants_leg(&mut self, v : bool) -> &mut Self { self.set(RIGHT_PANTS_LEG_BIT, v) }

    /// Set the right pants leg flag.
    #[inline(always)]
    pub fn with_right_pants_leg(self, v : bool) -> Self { self.with(RIGHT_PANTS_LEG_BIT, v) }


    /// Get the hat flag.
    #[inline(always)]
    pub fn hat(&self) -> bool { self.get(HAT_BIT) }

    /// Set the hat flag.
    #[inline(always)]
    pub fn set_hat(&mut self, v : bool) -> &mut Self { self.set(HAT_BIT, v) }

    /// Set the hat flag.
    #[inline(always)]
    pub fn with_hat(self, v : bool) -> Self { self.with(HAT_BIT, v) }

}


impl fmt::Debug for PlayerSkinParts {
    fn fmt(&self, f : &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("PlayerSkinParts")
            .field("cape", &self.cape())
            .field("jacket", &self.jacket())
            .field("left_sleeve", &self.left_sleeve())
            .field("right_sleeve", &self.right_sleeve())
            .field("left_pants_leg", &self.left_pants_leg())
            .field("right_pants_leg", &self.right_pants_leg())
            .field("hat", &self.hat())
            .finish()
    }
}


impl PacketPartDecode for PlayerSkinParts {

    type Output<'l> = PlayerSkinParts;
    type Error<'l>  = IncompleteData;

    fn decode<'l>(buf : DecodeBuf<'l>, head : &mut DecodeBufHead)
        -> Result<Self::Output<'l>, Self::Error<'l>>
    { Ok(Self(buf.read(head)?)) }

}
