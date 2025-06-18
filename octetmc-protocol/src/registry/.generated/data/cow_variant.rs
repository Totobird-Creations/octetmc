impl CowVariant<'_> {

    /// Vanilla `minecraft:cold` cow variant.
    pub const COLD : CowVariant<'static> = CowVariant( SimpleVariant {
        id       : ident![cold],
        model    : Some(Cow::Borrowed("cold")),
        asset_id : ident![minecraft:entity/cow/cold_cow]
    } );

    /// Vanilla `minecraft:warm` cow variant.
    pub const WARM : CowVariant<'static> = CowVariant( SimpleVariant {
        id       : ident![warm],
        model    : Some(Cow::Borrowed("warm")),
        asset_id : ident![minecraft:entity/cow/warm_cow]
    } );

    /// Vanilla `minecraft:temperate` cow variant.
    pub const TEMPERATE : CowVariant<'static> = CowVariant( SimpleVariant {
        id       : ident![temperate],
        model    : None,
        asset_id : ident![minecraft:entity/cow/temperate_cow]
    } );

    /// All vanilla cow variants.
    pub const VANILLA_COW_VARIANTS : &'static [CowVariant<'static>] = &[
        Self::COLD,
        Self::WARM,
        Self::TEMPERATE,
    ];

}
