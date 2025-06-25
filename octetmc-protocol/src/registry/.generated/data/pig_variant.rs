impl PigVariant<'_> {

    /// Vanilla `minecraft:warm` pig variant.
    pub const WARM : PigVariant<'static> = PigVariant( SimpleVariant {
        id       : ident![warm],
        model    : None,
        asset_id : ident![minecraft:entity/pig/warm_pig]
    } );

    /// Vanilla `minecraft:cold` pig variant.
    pub const COLD : PigVariant<'static> = PigVariant( SimpleVariant {
        id       : ident![cold],
        model    : Some(Cow::Borrowed("cold")),
        asset_id : ident![minecraft:entity/pig/cold_pig]
    } );

    /// Vanilla `minecraft:temperate` pig variant.
    pub const TEMPERATE : PigVariant<'static> = PigVariant( SimpleVariant {
        id       : ident![temperate],
        model    : None,
        asset_id : ident![minecraft:entity/pig/temperate_pig]
    } );

    /// All vanilla pig variants.
    pub const VANILLA_PIG_VARIANTS : &'static [PigVariant<'static>] = &[
        Self::WARM,
        Self::COLD,
        Self::TEMPERATE,
    ];

}
