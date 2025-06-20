impl CatVariant<'_> {

    /// Vanilla `minecraft:black` cat variant.
    pub const BLACK : CatVariant<'static> = CatVariant( SimpleVariant {
        id       : ident![black],
        model    : None,
        asset_id : ident![minecraft:entity/cat/black]
    } );

    /// Vanilla `minecraft:tabby` cat variant.
    pub const TABBY : CatVariant<'static> = CatVariant( SimpleVariant {
        id       : ident![tabby],
        model    : None,
        asset_id : ident![minecraft:entity/cat/tabby]
    } );

    /// Vanilla `minecraft:ragdoll` cat variant.
    pub const RAGDOLL : CatVariant<'static> = CatVariant( SimpleVariant {
        id       : ident![ragdoll],
        model    : None,
        asset_id : ident![minecraft:entity/cat/ragdoll]
    } );

    /// Vanilla `minecraft:jellie` cat variant.
    pub const JELLIE : CatVariant<'static> = CatVariant( SimpleVariant {
        id       : ident![jellie],
        model    : None,
        asset_id : ident![minecraft:entity/cat/jellie]
    } );

    /// Vanilla `minecraft:persian` cat variant.
    pub const PERSIAN : CatVariant<'static> = CatVariant( SimpleVariant {
        id       : ident![persian],
        model    : None,
        asset_id : ident![minecraft:entity/cat/persian]
    } );

    /// Vanilla `minecraft:white` cat variant.
    pub const WHITE : CatVariant<'static> = CatVariant( SimpleVariant {
        id       : ident![white],
        model    : None,
        asset_id : ident![minecraft:entity/cat/white]
    } );

    /// Vanilla `minecraft:siamese` cat variant.
    pub const SIAMESE : CatVariant<'static> = CatVariant( SimpleVariant {
        id       : ident![siamese],
        model    : None,
        asset_id : ident![minecraft:entity/cat/siamese]
    } );

    /// Vanilla `minecraft:british_shorthair` cat variant.
    pub const BRITISH_SHORTHAIR : CatVariant<'static> = CatVariant( SimpleVariant {
        id       : ident![british_shorthair],
        model    : None,
        asset_id : ident![minecraft:entity/cat/british_shorthair]
    } );

    /// Vanilla `minecraft:red` cat variant.
    pub const RED : CatVariant<'static> = CatVariant( SimpleVariant {
        id       : ident![red],
        model    : None,
        asset_id : ident![minecraft:entity/cat/red]
    } );

    /// Vanilla `minecraft:calico` cat variant.
    pub const CALICO : CatVariant<'static> = CatVariant( SimpleVariant {
        id       : ident![calico],
        model    : None,
        asset_id : ident![minecraft:entity/cat/calico]
    } );

    /// Vanilla `minecraft:all_black` cat variant.
    pub const ALL_BLACK : CatVariant<'static> = CatVariant( SimpleVariant {
        id       : ident![all_black],
        model    : None,
        asset_id : ident![minecraft:entity/cat/all_black]
    } );

    /// All vanilla cat variants.
    pub const VANILLA_CAT_VARIANTS : &'static [CatVariant<'static>] = &[
        Self::BLACK,
        Self::TABBY,
        Self::RAGDOLL,
        Self::JELLIE,
        Self::PERSIAN,
        Self::WHITE,
        Self::SIAMESE,
        Self::BRITISH_SHORTHAIR,
        Self::RED,
        Self::CALICO,
        Self::ALL_BLACK,
    ];

}
