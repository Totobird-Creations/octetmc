impl WolfVariant<'_> {

    /// Vanilla `minecraft:black` wolf variant.
    pub const BLACK : WolfVariant<'static> = WolfVariant {
        id             : ident![black],
        wild_asset_id  : ident![minecraft:entity/wolf/wolf_black],
        tame_asset_id  : ident![minecraft:entity/wolf/wolf_black_tame],
        angry_asset_id : ident![minecraft:entity/wolf/wolf_black_angry]
    };

    /// Vanilla `minecraft:pale` wolf variant.
    pub const PALE : WolfVariant<'static> = WolfVariant {
        id             : ident![pale],
        wild_asset_id  : ident![minecraft:entity/wolf/wolf],
        tame_asset_id  : ident![minecraft:entity/wolf/wolf_tame],
        angry_asset_id : ident![minecraft:entity/wolf/wolf_angry]
    };

    /// Vanilla `minecraft:chestnut` wolf variant.
    pub const CHESTNUT : WolfVariant<'static> = WolfVariant {
        id             : ident![chestnut],
        wild_asset_id  : ident![minecraft:entity/wolf/wolf_chestnut],
        tame_asset_id  : ident![minecraft:entity/wolf/wolf_chestnut_tame],
        angry_asset_id : ident![minecraft:entity/wolf/wolf_chestnut_angry]
    };

    /// Vanilla `minecraft:ashen` wolf variant.
    pub const ASHEN : WolfVariant<'static> = WolfVariant {
        id             : ident![ashen],
        wild_asset_id  : ident![minecraft:entity/wolf/wolf_ashen],
        tame_asset_id  : ident![minecraft:entity/wolf/wolf_ashen_tame],
        angry_asset_id : ident![minecraft:entity/wolf/wolf_ashen_angry]
    };

    /// Vanilla `minecraft:striped` wolf variant.
    pub const STRIPED : WolfVariant<'static> = WolfVariant {
        id             : ident![striped],
        wild_asset_id  : ident![minecraft:entity/wolf/wolf_striped],
        tame_asset_id  : ident![minecraft:entity/wolf/wolf_striped_tame],
        angry_asset_id : ident![minecraft:entity/wolf/wolf_striped_angry]
    };

    /// Vanilla `minecraft:rusty` wolf variant.
    pub const RUSTY : WolfVariant<'static> = WolfVariant {
        id             : ident![rusty],
        wild_asset_id  : ident![minecraft:entity/wolf/wolf_rusty],
        tame_asset_id  : ident![minecraft:entity/wolf/wolf_rusty_tame],
        angry_asset_id : ident![minecraft:entity/wolf/wolf_rusty_angry]
    };

    /// Vanilla `minecraft:spotted` wolf variant.
    pub const SPOTTED : WolfVariant<'static> = WolfVariant {
        id             : ident![spotted],
        wild_asset_id  : ident![minecraft:entity/wolf/wolf_spotted],
        tame_asset_id  : ident![minecraft:entity/wolf/wolf_spotted_tame],
        angry_asset_id : ident![minecraft:entity/wolf/wolf_spotted_angry]
    };

    /// Vanilla `minecraft:snowy` wolf variant.
    pub const SNOWY : WolfVariant<'static> = WolfVariant {
        id             : ident![snowy],
        wild_asset_id  : ident![minecraft:entity/wolf/wolf_snowy],
        tame_asset_id  : ident![minecraft:entity/wolf/wolf_snowy_tame],
        angry_asset_id : ident![minecraft:entity/wolf/wolf_snowy_angry]
    };

    /// Vanilla `minecraft:woods` wolf variant.
    pub const WOODS : WolfVariant<'static> = WolfVariant {
        id             : ident![woods],
        wild_asset_id  : ident![minecraft:entity/wolf/wolf_woods],
        tame_asset_id  : ident![minecraft:entity/wolf/wolf_woods_tame],
        angry_asset_id : ident![minecraft:entity/wolf/wolf_woods_angry]
    };

    /// All vanilla wolf variants.
    pub const VANILLA_WOLF_VARIANTS : &'static [WolfVariant<'static>] = &[
        Self::BLACK,
        Self::PALE,
        Self::CHESTNUT,
        Self::ASHEN,
        Self::STRIPED,
        Self::RUSTY,
        Self::SPOTTED,
        Self::SNOWY,
        Self::WOODS,
    ];

}
