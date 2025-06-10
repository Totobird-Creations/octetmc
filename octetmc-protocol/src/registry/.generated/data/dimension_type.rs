impl DimensionType<'_> {

    /// Vanilla `minecraft:overworld_caves` dimension type.
    pub const OVERWORLD_CAVES : DimensionType<'static> = DimensionType {
        id              : Ident::vanilla_str("overworld_caves"),
        fixed_time      : None,
        has_skylight    : true,
        has_ceiling     : true,
        compass_stable  : true,
        min_section     : -4,
        height_sections : unsafe { NonZeroU8::new_unchecked(24) },
        effects         : DimensionEffects::Overworld,
        ambient_light   : 0.0,
        piglin_safe     : false
    };

    /// Vanilla `minecraft:the_nether` dimension type.
    pub const THE_NETHER : DimensionType<'static> = DimensionType {
        id              : Ident::vanilla_str("the_nether"),
        fixed_time      : Some(18000),
        has_skylight    : false,
        has_ceiling     : true,
        compass_stable  : false,
        min_section     : 0,
        height_sections : unsafe { NonZeroU8::new_unchecked(16) },
        effects         : DimensionEffects::Nether,
        ambient_light   : 0.1,
        piglin_safe     : true
    };

    /// Vanilla `minecraft:overworld` dimension type.
    pub const OVERWORLD : DimensionType<'static> = DimensionType {
        id              : Ident::vanilla_str("overworld"),
        fixed_time      : None,
        has_skylight    : true,
        has_ceiling     : false,
        compass_stable  : true,
        min_section     : -4,
        height_sections : unsafe { NonZeroU8::new_unchecked(24) },
        effects         : DimensionEffects::Overworld,
        ambient_light   : 0.0,
        piglin_safe     : false
    };

    /// Vanilla `minecraft:the_end` dimension type.
    pub const THE_END : DimensionType<'static> = DimensionType {
        id              : Ident::vanilla_str("the_end"),
        fixed_time      : Some(6000),
        has_skylight    : false,
        has_ceiling     : false,
        compass_stable  : false,
        min_section     : 0,
        height_sections : unsafe { NonZeroU8::new_unchecked(16) },
        effects         : DimensionEffects::End,
        ambient_light   : 0.0,
        piglin_safe     : false
    };

    /// All vanilla dimension types.
    pub const VANILLA_DIMENSION_TYPES : &'static [DimensionType<'static>] = &[
        Self::OVERWORLD_CAVES,
        Self::THE_NETHER,
        Self::OVERWORLD,
        Self::THE_END,
    ];

}
