impl CatVariant<'_> {

    /// Vanilla `minecraft:all_black` cat variant.
    pub const ALL_BLACK : CatVariant<'static> = CatVariant( SimpleVariant {
        id       : Ident::vanilla_str("all_black"),
        model    : None,
        asset_id : Ident::parse_str("minecraft:entity/cat/all_black")
    } );

    /// Vanilla `minecraft:ragdoll` cat variant.
    pub const RAGDOLL : CatVariant<'static> = CatVariant( SimpleVariant {
        id       : Ident::vanilla_str("ragdoll"),
        model    : None,
        asset_id : Ident::parse_str("minecraft:entity/cat/ragdoll")
    } );

    /// Vanilla `minecraft:white` cat variant.
    pub const WHITE : CatVariant<'static> = CatVariant( SimpleVariant {
        id       : Ident::vanilla_str("white"),
        model    : None,
        asset_id : Ident::parse_str("minecraft:entity/cat/white")
    } );

    /// Vanilla `minecraft:black` cat variant.
    pub const BLACK : CatVariant<'static> = CatVariant( SimpleVariant {
        id       : Ident::vanilla_str("black"),
        model    : None,
        asset_id : Ident::parse_str("minecraft:entity/cat/black")
    } );

    /// Vanilla `minecraft:calico` cat variant.
    pub const CALICO : CatVariant<'static> = CatVariant( SimpleVariant {
        id       : Ident::vanilla_str("calico"),
        model    : None,
        asset_id : Ident::parse_str("minecraft:entity/cat/calico")
    } );

    /// Vanilla `minecraft:persian` cat variant.
    pub const PERSIAN : CatVariant<'static> = CatVariant( SimpleVariant {
        id       : Ident::vanilla_str("persian"),
        model    : None,
        asset_id : Ident::parse_str("minecraft:entity/cat/persian")
    } );

    /// Vanilla `minecraft:british_shorthair` cat variant.
    pub const BRITISH_SHORTHAIR : CatVariant<'static> = CatVariant( SimpleVariant {
        id       : Ident::vanilla_str("british_shorthair"),
        model    : None,
        asset_id : Ident::parse_str("minecraft:entity/cat/british_shorthair")
    } );

    /// Vanilla `minecraft:red` cat variant.
    pub const RED : CatVariant<'static> = CatVariant( SimpleVariant {
        id       : Ident::vanilla_str("red"),
        model    : None,
        asset_id : Ident::parse_str("minecraft:entity/cat/red")
    } );

    /// Vanilla `minecraft:jellie` cat variant.
    pub const JELLIE : CatVariant<'static> = CatVariant( SimpleVariant {
        id       : Ident::vanilla_str("jellie"),
        model    : None,
        asset_id : Ident::parse_str("minecraft:entity/cat/jellie")
    } );

    /// Vanilla `minecraft:tabby` cat variant.
    pub const TABBY : CatVariant<'static> = CatVariant( SimpleVariant {
        id       : Ident::vanilla_str("tabby"),
        model    : None,
        asset_id : Ident::parse_str("minecraft:entity/cat/tabby")
    } );

    /// Vanilla `minecraft:siamese` cat variant.
    pub const SIAMESE : CatVariant<'static> = CatVariant( SimpleVariant {
        id       : Ident::vanilla_str("siamese"),
        model    : None,
        asset_id : Ident::parse_str("minecraft:entity/cat/siamese")
    } );

    /// All vanilla cat variants.
    pub const VANILLA_CAT_VARIANTS : &'static [CatVariant<'static>] = &[
        Self::ALL_BLACK,
        Self::RAGDOLL,
        Self::WHITE,
        Self::BLACK,
        Self::CALICO,
        Self::PERSIAN,
        Self::BRITISH_SHORTHAIR,
        Self::RED,
        Self::JELLIE,
        Self::TABBY,
        Self::SIAMESE,
    ];

}
