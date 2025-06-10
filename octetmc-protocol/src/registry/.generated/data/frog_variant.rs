impl FrogVariant<'_> {

    /// Vanilla `minecraft:cold` frog variant.
    pub const COLD : FrogVariant<'static> = FrogVariant( SimpleVariant {
        id       : Ident::vanilla_str("cold"),
        model    : None,
        asset_id : Ident::parse_str("minecraft:entity/frog/cold_frog")
    } );

    /// Vanilla `minecraft:temperate` frog variant.
    pub const TEMPERATE : FrogVariant<'static> = FrogVariant( SimpleVariant {
        id       : Ident::vanilla_str("temperate"),
        model    : None,
        asset_id : Ident::parse_str("minecraft:entity/frog/temperate_frog")
    } );

    /// Vanilla `minecraft:warm` frog variant.
    pub const WARM : FrogVariant<'static> = FrogVariant( SimpleVariant {
        id       : Ident::vanilla_str("warm"),
        model    : None,
        asset_id : Ident::parse_str("minecraft:entity/frog/warm_frog")
    } );

    /// All vanilla frog variants.
    pub const VANILLA_FROG_VARIANTS : &'static [FrogVariant<'static>] = &[
        Self::COLD,
        Self::TEMPERATE,
        Self::WARM,
    ];

}
