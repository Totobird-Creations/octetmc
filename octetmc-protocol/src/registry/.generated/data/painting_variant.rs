impl PaintingVariant<'_> {

    /// Vanilla `minecraft:creebet` painting variant.
    pub const CREEBET : PaintingVariant<'static> = PaintingVariant {
        id       : Ident::vanilla_str("creebet"),
        asset_id : Ident::parse_str("minecraft:creebet"),
        width    : unsafe { NonZeroU8::new_unchecked(2) },
        height   : unsafe { NonZeroU8::new_unchecked(1) }
    };

    /// Vanilla `minecraft:alban` painting variant.
    pub const ALBAN : PaintingVariant<'static> = PaintingVariant {
        id       : Ident::vanilla_str("alban"),
        asset_id : Ident::parse_str("minecraft:alban"),
        width    : unsafe { NonZeroU8::new_unchecked(1) },
        height   : unsafe { NonZeroU8::new_unchecked(1) }
    };

    /// Vanilla `minecraft:fire` painting variant.
    pub const FIRE : PaintingVariant<'static> = PaintingVariant {
        id       : Ident::vanilla_str("fire"),
        asset_id : Ident::parse_str("minecraft:fire"),
        width    : unsafe { NonZeroU8::new_unchecked(2) },
        height   : unsafe { NonZeroU8::new_unchecked(2) }
    };

    /// Vanilla `minecraft:fighters` painting variant.
    pub const FIGHTERS : PaintingVariant<'static> = PaintingVariant {
        id       : Ident::vanilla_str("fighters"),
        asset_id : Ident::parse_str("minecraft:fighters"),
        width    : unsafe { NonZeroU8::new_unchecked(4) },
        height   : unsafe { NonZeroU8::new_unchecked(2) }
    };

    /// Vanilla `minecraft:cotan` painting variant.
    pub const COTAN : PaintingVariant<'static> = PaintingVariant {
        id       : Ident::vanilla_str("cotan"),
        asset_id : Ident::parse_str("minecraft:cotan"),
        width    : unsafe { NonZeroU8::new_unchecked(3) },
        height   : unsafe { NonZeroU8::new_unchecked(3) }
    };

    /// Vanilla `minecraft:wanderer` painting variant.
    pub const WANDERER : PaintingVariant<'static> = PaintingVariant {
        id       : Ident::vanilla_str("wanderer"),
        asset_id : Ident::parse_str("minecraft:wanderer"),
        width    : unsafe { NonZeroU8::new_unchecked(1) },
        height   : unsafe { NonZeroU8::new_unchecked(2) }
    };

    /// Vanilla `minecraft:aztec` painting variant.
    pub const AZTEC : PaintingVariant<'static> = PaintingVariant {
        id       : Ident::vanilla_str("aztec"),
        asset_id : Ident::parse_str("minecraft:aztec"),
        width    : unsafe { NonZeroU8::new_unchecked(1) },
        height   : unsafe { NonZeroU8::new_unchecked(1) }
    };

    /// Vanilla `minecraft:humble` painting variant.
    pub const HUMBLE : PaintingVariant<'static> = PaintingVariant {
        id       : Ident::vanilla_str("humble"),
        asset_id : Ident::parse_str("minecraft:humble"),
        width    : unsafe { NonZeroU8::new_unchecked(2) },
        height   : unsafe { NonZeroU8::new_unchecked(2) }
    };

    /// Vanilla `minecraft:fern` painting variant.
    pub const FERN : PaintingVariant<'static> = PaintingVariant {
        id       : Ident::vanilla_str("fern"),
        asset_id : Ident::parse_str("minecraft:fern"),
        width    : unsafe { NonZeroU8::new_unchecked(3) },
        height   : unsafe { NonZeroU8::new_unchecked(3) }
    };

    /// Vanilla `minecraft:courbet` painting variant.
    pub const COURBET : PaintingVariant<'static> = PaintingVariant {
        id       : Ident::vanilla_str("courbet"),
        asset_id : Ident::parse_str("minecraft:courbet"),
        width    : unsafe { NonZeroU8::new_unchecked(2) },
        height   : unsafe { NonZeroU8::new_unchecked(1) }
    };

    /// Vanilla `minecraft:owlemons` painting variant.
    pub const OWLEMONS : PaintingVariant<'static> = PaintingVariant {
        id       : Ident::vanilla_str("owlemons"),
        asset_id : Ident::parse_str("minecraft:owlemons"),
        width    : unsafe { NonZeroU8::new_unchecked(3) },
        height   : unsafe { NonZeroU8::new_unchecked(3) }
    };

    /// Vanilla `minecraft:cavebird` painting variant.
    pub const CAVEBIRD : PaintingVariant<'static> = PaintingVariant {
        id       : Ident::vanilla_str("cavebird"),
        asset_id : Ident::parse_str("minecraft:cavebird"),
        width    : unsafe { NonZeroU8::new_unchecked(3) },
        height   : unsafe { NonZeroU8::new_unchecked(3) }
    };

    /// Vanilla `minecraft:pond` painting variant.
    pub const POND : PaintingVariant<'static> = PaintingVariant {
        id       : Ident::vanilla_str("pond"),
        asset_id : Ident::parse_str("minecraft:pond"),
        width    : unsafe { NonZeroU8::new_unchecked(3) },
        height   : unsafe { NonZeroU8::new_unchecked(4) }
    };

    /// Vanilla `minecraft:finding` painting variant.
    pub const FINDING : PaintingVariant<'static> = PaintingVariant {
        id       : Ident::vanilla_str("finding"),
        asset_id : Ident::parse_str("minecraft:finding"),
        width    : unsafe { NonZeroU8::new_unchecked(4) },
        height   : unsafe { NonZeroU8::new_unchecked(2) }
    };

    /// Vanilla `minecraft:sunflowers` painting variant.
    pub const SUNFLOWERS : PaintingVariant<'static> = PaintingVariant {
        id       : Ident::vanilla_str("sunflowers"),
        asset_id : Ident::parse_str("minecraft:sunflowers"),
        width    : unsafe { NonZeroU8::new_unchecked(3) },
        height   : unsafe { NonZeroU8::new_unchecked(3) }
    };

    /// Vanilla `minecraft:baroque` painting variant.
    pub const BAROQUE : PaintingVariant<'static> = PaintingVariant {
        id       : Ident::vanilla_str("baroque"),
        asset_id : Ident::parse_str("minecraft:baroque"),
        width    : unsafe { NonZeroU8::new_unchecked(2) },
        height   : unsafe { NonZeroU8::new_unchecked(2) }
    };

    /// Vanilla `minecraft:wind` painting variant.
    pub const WIND : PaintingVariant<'static> = PaintingVariant {
        id       : Ident::vanilla_str("wind"),
        asset_id : Ident::parse_str("minecraft:wind"),
        width    : unsafe { NonZeroU8::new_unchecked(2) },
        height   : unsafe { NonZeroU8::new_unchecked(2) }
    };

    /// Vanilla `minecraft:void` painting variant.
    pub const VOID : PaintingVariant<'static> = PaintingVariant {
        id       : Ident::vanilla_str("void"),
        asset_id : Ident::parse_str("minecraft:void"),
        width    : unsafe { NonZeroU8::new_unchecked(2) },
        height   : unsafe { NonZeroU8::new_unchecked(2) }
    };

    /// Vanilla `minecraft:earth` painting variant.
    pub const EARTH : PaintingVariant<'static> = PaintingVariant {
        id       : Ident::vanilla_str("earth"),
        asset_id : Ident::parse_str("minecraft:earth"),
        width    : unsafe { NonZeroU8::new_unchecked(2) },
        height   : unsafe { NonZeroU8::new_unchecked(2) }
    };

    /// Vanilla `minecraft:water` painting variant.
    pub const WATER : PaintingVariant<'static> = PaintingVariant {
        id       : Ident::vanilla_str("water"),
        asset_id : Ident::parse_str("minecraft:water"),
        width    : unsafe { NonZeroU8::new_unchecked(2) },
        height   : unsafe { NonZeroU8::new_unchecked(2) }
    };

    /// Vanilla `minecraft:kebab` painting variant.
    pub const KEBAB : PaintingVariant<'static> = PaintingVariant {
        id       : Ident::vanilla_str("kebab"),
        asset_id : Ident::parse_str("minecraft:kebab"),
        width    : unsafe { NonZeroU8::new_unchecked(1) },
        height   : unsafe { NonZeroU8::new_unchecked(1) }
    };

    /// Vanilla `minecraft:orb` painting variant.
    pub const ORB : PaintingVariant<'static> = PaintingVariant {
        id       : Ident::vanilla_str("orb"),
        asset_id : Ident::parse_str("minecraft:orb"),
        width    : unsafe { NonZeroU8::new_unchecked(4) },
        height   : unsafe { NonZeroU8::new_unchecked(4) }
    };

    /// Vanilla `minecraft:stage` painting variant.
    pub const STAGE : PaintingVariant<'static> = PaintingVariant {
        id       : Ident::vanilla_str("stage"),
        asset_id : Ident::parse_str("minecraft:stage"),
        width    : unsafe { NonZeroU8::new_unchecked(2) },
        height   : unsafe { NonZeroU8::new_unchecked(2) }
    };

    /// Vanilla `minecraft:skull_and_roses` painting variant.
    pub const SKULL_AND_ROSES : PaintingVariant<'static> = PaintingVariant {
        id       : Ident::vanilla_str("skull_and_roses"),
        asset_id : Ident::parse_str("minecraft:skull_and_roses"),
        width    : unsafe { NonZeroU8::new_unchecked(2) },
        height   : unsafe { NonZeroU8::new_unchecked(2) }
    };

    /// Vanilla `minecraft:unpacked` painting variant.
    pub const UNPACKED : PaintingVariant<'static> = PaintingVariant {
        id       : Ident::vanilla_str("unpacked"),
        asset_id : Ident::parse_str("minecraft:unpacked"),
        width    : unsafe { NonZeroU8::new_unchecked(4) },
        height   : unsafe { NonZeroU8::new_unchecked(4) }
    };

    /// Vanilla `minecraft:sunset` painting variant.
    pub const SUNSET : PaintingVariant<'static> = PaintingVariant {
        id       : Ident::vanilla_str("sunset"),
        asset_id : Ident::parse_str("minecraft:sunset"),
        width    : unsafe { NonZeroU8::new_unchecked(2) },
        height   : unsafe { NonZeroU8::new_unchecked(1) }
    };

    /// Vanilla `minecraft:plant` painting variant.
    pub const PLANT : PaintingVariant<'static> = PaintingVariant {
        id       : Ident::vanilla_str("plant"),
        asset_id : Ident::parse_str("minecraft:plant"),
        width    : unsafe { NonZeroU8::new_unchecked(1) },
        height   : unsafe { NonZeroU8::new_unchecked(1) }
    };

    /// Vanilla `minecraft:aztec2` painting variant.
    pub const AZTEC2 : PaintingVariant<'static> = PaintingVariant {
        id       : Ident::vanilla_str("aztec2"),
        asset_id : Ident::parse_str("minecraft:aztec2"),
        width    : unsafe { NonZeroU8::new_unchecked(1) },
        height   : unsafe { NonZeroU8::new_unchecked(1) }
    };

    /// Vanilla `minecraft:burning_skull` painting variant.
    pub const BURNING_SKULL : PaintingVariant<'static> = PaintingVariant {
        id       : Ident::vanilla_str("burning_skull"),
        asset_id : Ident::parse_str("minecraft:burning_skull"),
        width    : unsafe { NonZeroU8::new_unchecked(4) },
        height   : unsafe { NonZeroU8::new_unchecked(4) }
    };

    /// Vanilla `minecraft:tides` painting variant.
    pub const TIDES : PaintingVariant<'static> = PaintingVariant {
        id       : Ident::vanilla_str("tides"),
        asset_id : Ident::parse_str("minecraft:tides"),
        width    : unsafe { NonZeroU8::new_unchecked(3) },
        height   : unsafe { NonZeroU8::new_unchecked(3) }
    };

    /// Vanilla `minecraft:passage` painting variant.
    pub const PASSAGE : PaintingVariant<'static> = PaintingVariant {
        id       : Ident::vanilla_str("passage"),
        asset_id : Ident::parse_str("minecraft:passage"),
        width    : unsafe { NonZeroU8::new_unchecked(4) },
        height   : unsafe { NonZeroU8::new_unchecked(2) }
    };

    /// Vanilla `minecraft:wither` painting variant.
    pub const WITHER : PaintingVariant<'static> = PaintingVariant {
        id       : Ident::vanilla_str("wither"),
        asset_id : Ident::parse_str("minecraft:wither"),
        width    : unsafe { NonZeroU8::new_unchecked(2) },
        height   : unsafe { NonZeroU8::new_unchecked(2) }
    };

    /// Vanilla `minecraft:pigscene` painting variant.
    pub const PIGSCENE : PaintingVariant<'static> = PaintingVariant {
        id       : Ident::vanilla_str("pigscene"),
        asset_id : Ident::parse_str("minecraft:pigscene"),
        width    : unsafe { NonZeroU8::new_unchecked(4) },
        height   : unsafe { NonZeroU8::new_unchecked(4) }
    };

    /// Vanilla `minecraft:skeleton` painting variant.
    pub const SKELETON : PaintingVariant<'static> = PaintingVariant {
        id       : Ident::vanilla_str("skeleton"),
        asset_id : Ident::parse_str("minecraft:skeleton"),
        width    : unsafe { NonZeroU8::new_unchecked(4) },
        height   : unsafe { NonZeroU8::new_unchecked(3) }
    };

    /// Vanilla `minecraft:meditative` painting variant.
    pub const MEDITATIVE : PaintingVariant<'static> = PaintingVariant {
        id       : Ident::vanilla_str("meditative"),
        asset_id : Ident::parse_str("minecraft:meditative"),
        width    : unsafe { NonZeroU8::new_unchecked(1) },
        height   : unsafe { NonZeroU8::new_unchecked(1) }
    };

    /// Vanilla `minecraft:pointer` painting variant.
    pub const POINTER : PaintingVariant<'static> = PaintingVariant {
        id       : Ident::vanilla_str("pointer"),
        asset_id : Ident::parse_str("minecraft:pointer"),
        width    : unsafe { NonZeroU8::new_unchecked(4) },
        height   : unsafe { NonZeroU8::new_unchecked(4) }
    };

    /// Vanilla `minecraft:bouquet` painting variant.
    pub const BOUQUET : PaintingVariant<'static> = PaintingVariant {
        id       : Ident::vanilla_str("bouquet"),
        asset_id : Ident::parse_str("minecraft:bouquet"),
        width    : unsafe { NonZeroU8::new_unchecked(3) },
        height   : unsafe { NonZeroU8::new_unchecked(3) }
    };

    /// Vanilla `minecraft:graham` painting variant.
    pub const GRAHAM : PaintingVariant<'static> = PaintingVariant {
        id       : Ident::vanilla_str("graham"),
        asset_id : Ident::parse_str("minecraft:graham"),
        width    : unsafe { NonZeroU8::new_unchecked(1) },
        height   : unsafe { NonZeroU8::new_unchecked(2) }
    };

    /// Vanilla `minecraft:prairie_ride` painting variant.
    pub const PRAIRIE_RIDE : PaintingVariant<'static> = PaintingVariant {
        id       : Ident::vanilla_str("prairie_ride"),
        asset_id : Ident::parse_str("minecraft:prairie_ride"),
        width    : unsafe { NonZeroU8::new_unchecked(1) },
        height   : unsafe { NonZeroU8::new_unchecked(2) }
    };

    /// Vanilla `minecraft:lowmist` painting variant.
    pub const LOWMIST : PaintingVariant<'static> = PaintingVariant {
        id       : Ident::vanilla_str("lowmist"),
        asset_id : Ident::parse_str("minecraft:lowmist"),
        width    : unsafe { NonZeroU8::new_unchecked(4) },
        height   : unsafe { NonZeroU8::new_unchecked(2) }
    };

    /// Vanilla `minecraft:match` painting variant.
    pub const MATCH : PaintingVariant<'static> = PaintingVariant {
        id       : Ident::vanilla_str("match"),
        asset_id : Ident::parse_str("minecraft:match"),
        width    : unsafe { NonZeroU8::new_unchecked(2) },
        height   : unsafe { NonZeroU8::new_unchecked(2) }
    };

    /// Vanilla `minecraft:pool` painting variant.
    pub const POOL : PaintingVariant<'static> = PaintingVariant {
        id       : Ident::vanilla_str("pool"),
        asset_id : Ident::parse_str("minecraft:pool"),
        width    : unsafe { NonZeroU8::new_unchecked(2) },
        height   : unsafe { NonZeroU8::new_unchecked(1) }
    };

    /// Vanilla `minecraft:backyard` painting variant.
    pub const BACKYARD : PaintingVariant<'static> = PaintingVariant {
        id       : Ident::vanilla_str("backyard"),
        asset_id : Ident::parse_str("minecraft:backyard"),
        width    : unsafe { NonZeroU8::new_unchecked(3) },
        height   : unsafe { NonZeroU8::new_unchecked(4) }
    };

    /// Vanilla `minecraft:endboss` painting variant.
    pub const ENDBOSS : PaintingVariant<'static> = PaintingVariant {
        id       : Ident::vanilla_str("endboss"),
        asset_id : Ident::parse_str("minecraft:endboss"),
        width    : unsafe { NonZeroU8::new_unchecked(3) },
        height   : unsafe { NonZeroU8::new_unchecked(3) }
    };

    /// Vanilla `minecraft:bust` painting variant.
    pub const BUST : PaintingVariant<'static> = PaintingVariant {
        id       : Ident::vanilla_str("bust"),
        asset_id : Ident::parse_str("minecraft:bust"),
        width    : unsafe { NonZeroU8::new_unchecked(2) },
        height   : unsafe { NonZeroU8::new_unchecked(2) }
    };

    /// Vanilla `minecraft:changing` painting variant.
    pub const CHANGING : PaintingVariant<'static> = PaintingVariant {
        id       : Ident::vanilla_str("changing"),
        asset_id : Ident::parse_str("minecraft:changing"),
        width    : unsafe { NonZeroU8::new_unchecked(4) },
        height   : unsafe { NonZeroU8::new_unchecked(2) }
    };

    /// Vanilla `minecraft:donkey_kong` painting variant.
    pub const DONKEY_KONG : PaintingVariant<'static> = PaintingVariant {
        id       : Ident::vanilla_str("donkey_kong"),
        asset_id : Ident::parse_str("minecraft:donkey_kong"),
        width    : unsafe { NonZeroU8::new_unchecked(4) },
        height   : unsafe { NonZeroU8::new_unchecked(3) }
    };

    /// Vanilla `minecraft:sea` painting variant.
    pub const SEA : PaintingVariant<'static> = PaintingVariant {
        id       : Ident::vanilla_str("sea"),
        asset_id : Ident::parse_str("minecraft:sea"),
        width    : unsafe { NonZeroU8::new_unchecked(2) },
        height   : unsafe { NonZeroU8::new_unchecked(1) }
    };

    /// Vanilla `minecraft:wasteland` painting variant.
    pub const WASTELAND : PaintingVariant<'static> = PaintingVariant {
        id       : Ident::vanilla_str("wasteland"),
        asset_id : Ident::parse_str("minecraft:wasteland"),
        width    : unsafe { NonZeroU8::new_unchecked(1) },
        height   : unsafe { NonZeroU8::new_unchecked(1) }
    };

    /// Vanilla `minecraft:bomb` painting variant.
    pub const BOMB : PaintingVariant<'static> = PaintingVariant {
        id       : Ident::vanilla_str("bomb"),
        asset_id : Ident::parse_str("minecraft:bomb"),
        width    : unsafe { NonZeroU8::new_unchecked(1) },
        height   : unsafe { NonZeroU8::new_unchecked(1) }
    };

    /// All vanilla painting variants.
    pub const VANILLA_PAINTING_VARIANTS : &'static [PaintingVariant<'static>] = &[
        Self::CREEBET,
        Self::ALBAN,
        Self::FIRE,
        Self::FIGHTERS,
        Self::COTAN,
        Self::WANDERER,
        Self::AZTEC,
        Self::HUMBLE,
        Self::FERN,
        Self::COURBET,
        Self::OWLEMONS,
        Self::CAVEBIRD,
        Self::POND,
        Self::FINDING,
        Self::SUNFLOWERS,
        Self::BAROQUE,
        Self::WIND,
        Self::VOID,
        Self::EARTH,
        Self::WATER,
        Self::KEBAB,
        Self::ORB,
        Self::STAGE,
        Self::SKULL_AND_ROSES,
        Self::UNPACKED,
        Self::SUNSET,
        Self::PLANT,
        Self::AZTEC2,
        Self::BURNING_SKULL,
        Self::TIDES,
        Self::PASSAGE,
        Self::WITHER,
        Self::PIGSCENE,
        Self::SKELETON,
        Self::MEDITATIVE,
        Self::POINTER,
        Self::BOUQUET,
        Self::GRAHAM,
        Self::PRAIRIE_RIDE,
        Self::LOWMIST,
        Self::MATCH,
        Self::POOL,
        Self::BACKYARD,
        Self::ENDBOSS,
        Self::BUST,
        Self::CHANGING,
        Self::DONKEY_KONG,
        Self::SEA,
        Self::WASTELAND,
        Self::BOMB,
    ];

}
