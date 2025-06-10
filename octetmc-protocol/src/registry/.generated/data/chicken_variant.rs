impl ChickenVariant<'_> {

    /// Vanilla `minecraft:temperate` chicken variant.
    pub const TEMPERATE : ChickenVariant<'static> = ChickenVariant( SimpleVariant {
        id       : Ident::vanilla_str("temperate"),
        model    : None,
        asset_id : Ident::parse_str("minecraft:entity/chicken/temperate_chicken")
    } );

    /// Vanilla `minecraft:cold` chicken variant.
    pub const COLD : ChickenVariant<'static> = ChickenVariant( SimpleVariant {
        id       : Ident::vanilla_str("cold"),
        model    : Some(Cow::Borrowed("cold")),
        asset_id : Ident::parse_str("minecraft:entity/chicken/cold_chicken")
    } );

    /// Vanilla `minecraft:warm` chicken variant.
    pub const WARM : ChickenVariant<'static> = ChickenVariant( SimpleVariant {
        id       : Ident::vanilla_str("warm"),
        model    : None,
        asset_id : Ident::parse_str("minecraft:entity/chicken/warm_chicken")
    } );

    /// All vanilla chicken variants.
    pub const VANILLA_CHICKEN_VARIANTS : &'static [ChickenVariant<'static>] = &[
        Self::TEMPERATE,
        Self::COLD,
        Self::WARM,
    ];

}
