impl WolfSoundVariant<'_> {

    /// Vanilla `minecraft:puglin` wolf sound variant.
    pub const PUGLIN : WolfSoundVariant<'static> = WolfSoundVariant {
        id             : ident![puglin],
        ambient_sound  : ident![minecraft:entity.wolf_puglin.ambient],
        death_sound    : ident![minecraft:entity.wolf_puglin.death],
        growl_sound    : ident![minecraft:entity.wolf_puglin.growl],
        hurt_sound     : ident![minecraft:entity.wolf_puglin.hurt],
        pant_sound     : ident![minecraft:entity.wolf_puglin.pant],
        whine_sound    : ident![minecraft:entity.wolf_puglin.whine]
    };

    /// Vanilla `minecraft:big` wolf sound variant.
    pub const BIG : WolfSoundVariant<'static> = WolfSoundVariant {
        id             : ident![big],
        ambient_sound  : ident![minecraft:entity.wolf_big.ambient],
        death_sound    : ident![minecraft:entity.wolf_big.death],
        growl_sound    : ident![minecraft:entity.wolf_big.growl],
        hurt_sound     : ident![minecraft:entity.wolf_big.hurt],
        pant_sound     : ident![minecraft:entity.wolf_big.pant],
        whine_sound    : ident![minecraft:entity.wolf_big.whine]
    };

    /// Vanilla `minecraft:cute` wolf sound variant.
    pub const CUTE : WolfSoundVariant<'static> = WolfSoundVariant {
        id             : ident![cute],
        ambient_sound  : ident![minecraft:entity.wolf_cute.ambient],
        death_sound    : ident![minecraft:entity.wolf_cute.death],
        growl_sound    : ident![minecraft:entity.wolf_cute.growl],
        hurt_sound     : ident![minecraft:entity.wolf_cute.hurt],
        pant_sound     : ident![minecraft:entity.wolf_cute.pant],
        whine_sound    : ident![minecraft:entity.wolf_cute.whine]
    };

    /// Vanilla `minecraft:classic` wolf sound variant.
    pub const CLASSIC : WolfSoundVariant<'static> = WolfSoundVariant {
        id             : ident![classic],
        ambient_sound  : ident![minecraft:entity.wolf.ambient],
        death_sound    : ident![minecraft:entity.wolf.death],
        growl_sound    : ident![minecraft:entity.wolf.growl],
        hurt_sound     : ident![minecraft:entity.wolf.hurt],
        pant_sound     : ident![minecraft:entity.wolf.pant],
        whine_sound    : ident![minecraft:entity.wolf.whine]
    };

    /// Vanilla `minecraft:sad` wolf sound variant.
    pub const SAD : WolfSoundVariant<'static> = WolfSoundVariant {
        id             : ident![sad],
        ambient_sound  : ident![minecraft:entity.wolf_sad.ambient],
        death_sound    : ident![minecraft:entity.wolf_sad.death],
        growl_sound    : ident![minecraft:entity.wolf_sad.growl],
        hurt_sound     : ident![minecraft:entity.wolf_sad.hurt],
        pant_sound     : ident![minecraft:entity.wolf_sad.pant],
        whine_sound    : ident![minecraft:entity.wolf_sad.whine]
    };

    /// Vanilla `minecraft:angry` wolf sound variant.
    pub const ANGRY : WolfSoundVariant<'static> = WolfSoundVariant {
        id             : ident![angry],
        ambient_sound  : ident![minecraft:entity.wolf_angry.ambient],
        death_sound    : ident![minecraft:entity.wolf_angry.death],
        growl_sound    : ident![minecraft:entity.wolf_angry.growl],
        hurt_sound     : ident![minecraft:entity.wolf_angry.hurt],
        pant_sound     : ident![minecraft:entity.wolf_angry.pant],
        whine_sound    : ident![minecraft:entity.wolf_angry.whine]
    };

    /// Vanilla `minecraft:grumpy` wolf sound variant.
    pub const GRUMPY : WolfSoundVariant<'static> = WolfSoundVariant {
        id             : ident![grumpy],
        ambient_sound  : ident![minecraft:entity.wolf_grumpy.ambient],
        death_sound    : ident![minecraft:entity.wolf_grumpy.death],
        growl_sound    : ident![minecraft:entity.wolf_grumpy.growl],
        hurt_sound     : ident![minecraft:entity.wolf_grumpy.hurt],
        pant_sound     : ident![minecraft:entity.wolf_grumpy.pant],
        whine_sound    : ident![minecraft:entity.wolf_grumpy.whine]
    };

    /// All vanilla wolf sound variants.
    pub const VANILLA_WOLF_SOUND_VARIANTS : &'static [WolfSoundVariant<'static>] = &[
        Self::PUGLIN,
        Self::BIG,
        Self::CUTE,
        Self::CLASSIC,
        Self::SAD,
        Self::ANGRY,
        Self::GRUMPY,
    ];

}
