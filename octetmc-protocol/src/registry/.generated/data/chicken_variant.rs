impl ChickenVariant<'_> {

    /// Vanilla `minecraft:temperate` chicken variant.
    pub const TEMPERATE : ChickenVariant<'static> = ChickenVariant( SimpleVariant {
        id       : ident![temperate],
        model    : None,
        asset_id : ident![minecraft:entity/chicken/temperate_chicken]
    } );

    /// Vanilla `minecraft:warm` chicken variant.
    pub const WARM : ChickenVariant<'static> = ChickenVariant( SimpleVariant {
        id       : ident![warm],
        model    : None,
        asset_id : ident![minecraft:entity/chicken/warm_chicken]
    } );

    /// Vanilla `minecraft:cold` chicken variant.
    pub const COLD : ChickenVariant<'static> = ChickenVariant( SimpleVariant {
        id       : ident![cold],
        model    : Some(Cow::Borrowed("cold")),
        asset_id : ident![minecraft:entity/chicken/cold_chicken]
    } );

    /// All vanilla chicken variants.
    pub const VANILLA_CHICKEN_VARIANTS : &'static [ChickenVariant<'static>] = &[
        Self::TEMPERATE,
        Self::WARM,
        Self::COLD,
    ];

}
