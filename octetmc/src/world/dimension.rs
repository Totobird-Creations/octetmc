//! The dimension of a world.


use octetmc_protocol::registry::dimension_type::DimensionType;
use core::mem::{ self, MaybeUninit };
use std::borrow::Cow;
use bevy_ecs::component::Component;


/// The dimension of a world.
#[derive(Clone, Debug, Component)]
pub struct Dimension<'l> {

    /// The type of dimension.
    pub kind         : Cow<'l, DimensionType<'l>>,

    /// Whether the world is a [superflat world](https://minecraft.wiki/w/Superflat).
    ///
    /// Superflat worlds have different void fog, and a horizon at y=0 instead of y=63.
    pub is_superflat : bool,

    /// Used client side for biome noise.
    ///
    /// Vanilla servers use the first 8 bytes of the SHA-256 hash of the world seed.
    pub hashed_seed  : u64,

    /// The world's sea level.
    ///
    /// Default is 64, unless superflat.
    pub sea_level    : i32

}

impl Dimension<'_> {

    /// All vanilla dimensions in their superflat forms.
    pub const VANILLA_SUPERFLAT_DIMENSIONS : &'static [Dimension<'static>] = {
        const LEN : usize = DimensionType::VANILLA_DIMENSION_TYPES.len();

        let mut dims = unsafe { mem::transmute::<MaybeUninit<[Dimension; LEN]>, [MaybeUninit<Dimension>; LEN]>(MaybeUninit::uninit()) };

        let mut i = 0;
        while (i < LEN) {
            dims[i].write(Dimension {
                kind         : Cow::Borrowed(&DimensionType::VANILLA_DIMENSION_TYPES[i]),
                is_superflat : true,
                hashed_seed  : i as u64,
                sea_level    : 0
            });

            i += 1;
        }

        &unsafe { mem::transmute::<[MaybeUninit<Dimension>; LEN], [Dimension; LEN]>(dims) }

    };


}
