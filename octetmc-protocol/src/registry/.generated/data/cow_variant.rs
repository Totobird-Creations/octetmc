impl CowVariant<'_> {

    /// Vanilla `minecraft:temperate` cow variant.
    pub const TEMPERATE : CowVariant<'static> = CowVariant( SimpleVariant {
        id       : Ident::vanilla_str("temperate"),
        model    : None,
        asset_id : Ident::parse_str("minecraft:entity/cow/temperate_cow")
    } );

    /// Vanilla `minecraft:cold` cow variant.
    pub const COLD : CowVariant<'static> = CowVariant( SimpleVariant {
        id       : Ident::vanilla_str("cold"),
        model    : Some(Cow::Borrowed("cold")),
        asset_id : Ident::parse_str("minecraft:entity/cow/cold_cow")
    } );

    /// Vanilla `minecraft:warm` cow variant.
    pub const WARM : CowVariant<'static> = CowVariant( SimpleVariant {
        id       : Ident::vanilla_str("warm"),
        model    : Some(Cow::Borrowed("warm")),
        asset_id : Ident::parse_str("minecraft:entity/cow/warm_cow")
    } );

    /// All vanilla cow variants.
    pub const VANILLA_COW_VARIANTS : &'static [CowVariant<'static>] = &[
        Self::TEMPERATE,
        Self::COLD,
        Self::WARM,
    ];

}
