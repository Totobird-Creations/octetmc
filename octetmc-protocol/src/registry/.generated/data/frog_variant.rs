impl FrogVariant<'_> {

    /// Vanilla `minecraft:temperate` frog variant.
    pub const TEMPERATE : FrogVariant<'static> = FrogVariant( SimpleVariant {
        id       : ident![temperate],
        model    : None,
        asset_id : ident![minecraft:entity/frog/temperate_frog]
    } );

    /// Vanilla `minecraft:warm` frog variant.
    pub const WARM : FrogVariant<'static> = FrogVariant( SimpleVariant {
        id       : ident![warm],
        model    : None,
        asset_id : ident![minecraft:entity/frog/warm_frog]
    } );

    /// Vanilla `minecraft:cold` frog variant.
    pub const COLD : FrogVariant<'static> = FrogVariant( SimpleVariant {
        id       : ident![cold],
        model    : None,
        asset_id : ident![minecraft:entity/frog/cold_frog]
    } );

    /// All vanilla frog variants.
    pub const VANILLA_FROG_VARIANTS : &'static [FrogVariant<'static>] = &[
        Self::TEMPERATE,
        Self::WARM,
        Self::COLD,
    ];

}
