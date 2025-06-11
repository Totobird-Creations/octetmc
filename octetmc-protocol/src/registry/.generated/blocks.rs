/// `minecraft:tube_coral_wall_fan` block.
pub mod tube_coral_wall_fan {
    /// `minecraft:tube_coral_wall_fan` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct TubeCoralWallFan {
        /// `waterlogged` state.
        pub waterlogged : bool,
        /// `facing` state.
        pub facing : Facing,
    }
    /// `facing` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Facing {
        /// `north` variant.
        North,
        /// `south` variant.
        South,
        /// `west` variant.
        West,
        /// `east` variant.
        East,
    }
    impl super::Block for TubeCoralWallFan {
    }
    impl From<TubeCoralWallFan> for super::BlockState {
        fn from(value : TubeCoralWallFan) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:exposed_cut_copper_stairs` block.
pub mod exposed_cut_copper_stairs {
    /// `minecraft:exposed_cut_copper_stairs` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct ExposedCutCopperStairs {
        /// `facing` state.
        pub facing : Facing,
        /// `waterlogged` state.
        pub waterlogged : bool,
        /// `shape` state.
        pub shape : Shape,
        /// `half` state.
        pub half : Half,
    }
    /// `facing` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Facing {
        /// `north` variant.
        North,
        /// `south` variant.
        South,
        /// `west` variant.
        West,
        /// `east` variant.
        East,
    }
    /// `shape` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Shape {
        /// `straight` variant.
        Straight,
        /// `inner_left` variant.
        InnerLeft,
        /// `inner_right` variant.
        InnerRight,
        /// `outer_left` variant.
        OuterLeft,
        /// `outer_right` variant.
        OuterRight,
    }
    /// `half` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Half {
        /// `top` variant.
        Top,
        /// `bottom` variant.
        Bottom,
    }
    impl super::Block for ExposedCutCopperStairs {
    }
    impl From<ExposedCutCopperStairs> for super::BlockState {
        fn from(value : ExposedCutCopperStairs) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:bubble_coral_wall_fan` block.
pub mod bubble_coral_wall_fan {
    /// `minecraft:bubble_coral_wall_fan` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct BubbleCoralWallFan {
        /// `waterlogged` state.
        pub waterlogged : bool,
        /// `facing` state.
        pub facing : Facing,
    }
    /// `facing` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Facing {
        /// `north` variant.
        North,
        /// `south` variant.
        South,
        /// `west` variant.
        West,
        /// `east` variant.
        East,
    }
    impl super::Block for BubbleCoralWallFan {
    }
    impl From<BubbleCoralWallFan> for super::BlockState {
        fn from(value : BubbleCoralWallFan) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:coal_ore` block.
pub mod coal_ore {
    /// `minecraft:coal_ore` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct CoalOre;
    impl super::Block for CoalOre {
    }
    impl From<CoalOre> for super::BlockState {
        fn from(value : CoalOre) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:iron_ore` block.
pub mod iron_ore {
    /// `minecraft:iron_ore` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct IronOre;
    impl super::Block for IronOre {
    }
    impl From<IronOre> for super::BlockState {
        fn from(value : IronOre) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:sea_lantern` block.
pub mod sea_lantern {
    /// `minecraft:sea_lantern` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct SeaLantern;
    impl super::Block for SeaLantern {
    }
    impl From<SeaLantern> for super::BlockState {
        fn from(value : SeaLantern) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:waxed_weathered_copper_grate` block.
pub mod waxed_weathered_copper_grate {
    /// `minecraft:waxed_weathered_copper_grate` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct WaxedWeatheredCopperGrate {
        /// `waterlogged` state.
        pub waterlogged : bool,
    }
    impl super::Block for WaxedWeatheredCopperGrate {
    }
    impl From<WaxedWeatheredCopperGrate> for super::BlockState {
        fn from(value : WaxedWeatheredCopperGrate) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:oak_wood` block.
pub mod oak_wood {
    /// `minecraft:oak_wood` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct OakWood {
        /// `axis` state.
        pub axis : Axis,
    }
    /// `axis` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Axis {
        /// `x` variant.
        X,
        /// `y` variant.
        Y,
        /// `z` variant.
        Z,
    }
    impl super::Block for OakWood {
    }
    impl From<OakWood> for super::BlockState {
        fn from(value : OakWood) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:light_gray_concrete_powder` block.
pub mod light_gray_concrete_powder {
    /// `minecraft:light_gray_concrete_powder` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct LightGrayConcretePowder;
    impl super::Block for LightGrayConcretePowder {
    }
    impl From<LightGrayConcretePowder> for super::BlockState {
        fn from(value : LightGrayConcretePowder) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:crimson_pressure_plate` block.
pub mod crimson_pressure_plate {
    /// `minecraft:crimson_pressure_plate` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct CrimsonPressurePlate {
        /// `powered` state.
        pub powered : bool,
    }
    impl super::Block for CrimsonPressurePlate {
    }
    impl From<CrimsonPressurePlate> for super::BlockState {
        fn from(value : CrimsonPressurePlate) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:dark_oak_slab` block.
pub mod dark_oak_slab {
    /// `minecraft:dark_oak_slab` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct DarkOakSlab {
        /// `waterlogged` state.
        pub waterlogged : bool,
        /// `type` state.
        pub kind : Kind,
    }
    /// `type` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Kind {
        /// `top` variant.
        Top,
        /// `bottom` variant.
        Bottom,
        /// `double` variant.
        Double,
    }
    impl super::Block for DarkOakSlab {
    }
    impl From<DarkOakSlab> for super::BlockState {
        fn from(value : DarkOakSlab) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:polished_blackstone_stairs` block.
pub mod polished_blackstone_stairs {
    /// `minecraft:polished_blackstone_stairs` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct PolishedBlackstoneStairs {
        /// `waterlogged` state.
        pub waterlogged : bool,
        /// `facing` state.
        pub facing : Facing,
        /// `shape` state.
        pub shape : Shape,
        /// `half` state.
        pub half : Half,
    }
    /// `facing` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Facing {
        /// `north` variant.
        North,
        /// `south` variant.
        South,
        /// `west` variant.
        West,
        /// `east` variant.
        East,
    }
    /// `shape` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Shape {
        /// `straight` variant.
        Straight,
        /// `inner_left` variant.
        InnerLeft,
        /// `inner_right` variant.
        InnerRight,
        /// `outer_left` variant.
        OuterLeft,
        /// `outer_right` variant.
        OuterRight,
    }
    /// `half` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Half {
        /// `top` variant.
        Top,
        /// `bottom` variant.
        Bottom,
    }
    impl super::Block for PolishedBlackstoneStairs {
    }
    impl From<PolishedBlackstoneStairs> for super::BlockState {
        fn from(value : PolishedBlackstoneStairs) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:quartz_block` block.
pub mod quartz_block {
    /// `minecraft:quartz_block` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct QuartzBlock;
    impl super::Block for QuartzBlock {
    }
    impl From<QuartzBlock> for super::BlockState {
        fn from(value : QuartzBlock) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:diorite` block.
pub mod diorite {
    /// `minecraft:diorite` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct Diorite;
    impl super::Block for Diorite {
    }
    impl From<Diorite> for super::BlockState {
        fn from(value : Diorite) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:magenta_wall_banner` block.
pub mod magenta_wall_banner {
    /// `minecraft:magenta_wall_banner` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct MagentaWallBanner {
        /// `facing` state.
        pub facing : Facing,
    }
    /// `facing` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Facing {
        /// `north` variant.
        North,
        /// `south` variant.
        South,
        /// `west` variant.
        West,
        /// `east` variant.
        East,
    }
    impl super::Block for MagentaWallBanner {
    }
    impl From<MagentaWallBanner> for super::BlockState {
        fn from(value : MagentaWallBanner) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:gravel` block.
pub mod gravel {
    /// `minecraft:gravel` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct Gravel;
    impl super::Block for Gravel {
    }
    impl From<Gravel> for super::BlockState {
        fn from(value : Gravel) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:decorated_pot` block.
pub mod decorated_pot {
    /// `minecraft:decorated_pot` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct DecoratedPot {
        /// `cracked` state.
        pub cracked : bool,
        /// `waterlogged` state.
        pub waterlogged : bool,
        /// `facing` state.
        pub facing : Facing,
    }
    /// `facing` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Facing {
        /// `north` variant.
        North,
        /// `south` variant.
        South,
        /// `west` variant.
        West,
        /// `east` variant.
        East,
    }
    impl super::Block for DecoratedPot {
    }
    impl From<DecoratedPot> for super::BlockState {
        fn from(value : DecoratedPot) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:cobbled_deepslate_stairs` block.
pub mod cobbled_deepslate_stairs {
    /// `minecraft:cobbled_deepslate_stairs` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct CobbledDeepslateStairs {
        /// `half` state.
        pub half : Half,
        /// `shape` state.
        pub shape : Shape,
        /// `facing` state.
        pub facing : Facing,
        /// `waterlogged` state.
        pub waterlogged : bool,
    }
    /// `half` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Half {
        /// `top` variant.
        Top,
        /// `bottom` variant.
        Bottom,
    }
    /// `shape` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Shape {
        /// `straight` variant.
        Straight,
        /// `inner_left` variant.
        InnerLeft,
        /// `inner_right` variant.
        InnerRight,
        /// `outer_left` variant.
        OuterLeft,
        /// `outer_right` variant.
        OuterRight,
    }
    /// `facing` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Facing {
        /// `north` variant.
        North,
        /// `south` variant.
        South,
        /// `west` variant.
        West,
        /// `east` variant.
        East,
    }
    impl super::Block for CobbledDeepslateStairs {
    }
    impl From<CobbledDeepslateStairs> for super::BlockState {
        fn from(value : CobbledDeepslateStairs) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:flower_pot` block.
pub mod flower_pot {
    /// `minecraft:flower_pot` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct FlowerPot;
    impl super::Block for FlowerPot {
    }
    impl From<FlowerPot> for super::BlockState {
        fn from(value : FlowerPot) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:blue_wool` block.
pub mod blue_wool {
    /// `minecraft:blue_wool` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct BlueWool;
    impl super::Block for BlueWool {
    }
    impl From<BlueWool> for super::BlockState {
        fn from(value : BlueWool) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:cave_air` block.
pub mod cave_air {
    /// `minecraft:cave_air` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct CaveAir;
    impl super::Block for CaveAir {
    }
    impl From<CaveAir> for super::BlockState {
        fn from(value : CaveAir) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:allium` block.
pub mod allium {
    /// `minecraft:allium` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct Allium;
    impl super::Block for Allium {
    }
    impl From<Allium> for super::BlockState {
        fn from(value : Allium) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:coarse_dirt` block.
pub mod coarse_dirt {
    /// `minecraft:coarse_dirt` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct CoarseDirt;
    impl super::Block for CoarseDirt {
    }
    impl From<CoarseDirt> for super::BlockState {
        fn from(value : CoarseDirt) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:cyan_concrete_powder` block.
pub mod cyan_concrete_powder {
    /// `minecraft:cyan_concrete_powder` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct CyanConcretePowder;
    impl super::Block for CyanConcretePowder {
    }
    impl From<CyanConcretePowder> for super::BlockState {
        fn from(value : CyanConcretePowder) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:deepslate_diamond_ore` block.
pub mod deepslate_diamond_ore {
    /// `minecraft:deepslate_diamond_ore` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct DeepslateDiamondOre;
    impl super::Block for DeepslateDiamondOre {
    }
    impl From<DeepslateDiamondOre> for super::BlockState {
        fn from(value : DeepslateDiamondOre) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:blue_glazed_terracotta` block.
pub mod blue_glazed_terracotta {
    /// `minecraft:blue_glazed_terracotta` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct BlueGlazedTerracotta {
        /// `facing` state.
        pub facing : Facing,
    }
    /// `facing` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Facing {
        /// `north` variant.
        North,
        /// `south` variant.
        South,
        /// `west` variant.
        West,
        /// `east` variant.
        East,
    }
    impl super::Block for BlueGlazedTerracotta {
    }
    impl From<BlueGlazedTerracotta> for super::BlockState {
        fn from(value : BlueGlazedTerracotta) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:mud_bricks` block.
pub mod mud_bricks {
    /// `minecraft:mud_bricks` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct MudBricks;
    impl super::Block for MudBricks {
    }
    impl From<MudBricks> for super::BlockState {
        fn from(value : MudBricks) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:light_blue_wall_banner` block.
pub mod light_blue_wall_banner {
    /// `minecraft:light_blue_wall_banner` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct LightBlueWallBanner {
        /// `facing` state.
        pub facing : Facing,
    }
    /// `facing` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Facing {
        /// `north` variant.
        North,
        /// `south` variant.
        South,
        /// `west` variant.
        West,
        /// `east` variant.
        East,
    }
    impl super::Block for LightBlueWallBanner {
    }
    impl From<LightBlueWallBanner> for super::BlockState {
        fn from(value : LightBlueWallBanner) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:lime_concrete_powder` block.
pub mod lime_concrete_powder {
    /// `minecraft:lime_concrete_powder` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct LimeConcretePowder;
    impl super::Block for LimeConcretePowder {
    }
    impl From<LimeConcretePowder> for super::BlockState {
        fn from(value : LimeConcretePowder) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:scaffolding` block.
pub mod scaffolding {
    /// `minecraft:scaffolding` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct Scaffolding {
        /// `waterlogged` state.
        pub waterlogged : bool,
        /// `distance` state.
        pub distance : Distance,
        /// `bottom` state.
        pub bottom : bool,
    }
    /// `distance` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Distance {
        /// `0` variant.
        N0,
        /// `1` variant.
        N1,
        /// `2` variant.
        N2,
        /// `3` variant.
        N3,
        /// `4` variant.
        N4,
        /// `5` variant.
        N5,
        /// `6` variant.
        N6,
        /// `7` variant.
        N7,
    }
    impl super::Block for Scaffolding {
    }
    impl From<Scaffolding> for super::BlockState {
        fn from(value : Scaffolding) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:trapped_chest` block.
pub mod trapped_chest {
    /// `minecraft:trapped_chest` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct TrappedChest {
        /// `type` state.
        pub kind : Kind,
        /// `waterlogged` state.
        pub waterlogged : bool,
        /// `facing` state.
        pub facing : Facing,
    }
    /// `type` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Kind {
        /// `single` variant.
        Single,
        /// `left` variant.
        Left,
        /// `right` variant.
        Right,
    }
    /// `facing` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Facing {
        /// `north` variant.
        North,
        /// `south` variant.
        South,
        /// `west` variant.
        West,
        /// `east` variant.
        East,
    }
    impl super::Block for TrappedChest {
    }
    impl From<TrappedChest> for super::BlockState {
        fn from(value : TrappedChest) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:heavy_weighted_pressure_plate` block.
pub mod heavy_weighted_pressure_plate {
    /// `minecraft:heavy_weighted_pressure_plate` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct HeavyWeightedPressurePlate {
        /// `power` state.
        pub power : Power,
    }
    /// `power` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Power {
        /// `0` variant.
        N0,
        /// `1` variant.
        N1,
        /// `2` variant.
        N2,
        /// `3` variant.
        N3,
        /// `4` variant.
        N4,
        /// `5` variant.
        N5,
        /// `6` variant.
        N6,
        /// `7` variant.
        N7,
        /// `8` variant.
        N8,
        /// `9` variant.
        N9,
        /// `10` variant.
        N10,
        /// `11` variant.
        N11,
        /// `12` variant.
        N12,
        /// `13` variant.
        N13,
        /// `14` variant.
        N14,
        /// `15` variant.
        N15,
    }
    impl super::Block for HeavyWeightedPressurePlate {
    }
    impl From<HeavyWeightedPressurePlate> for super::BlockState {
        fn from(value : HeavyWeightedPressurePlate) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:mossy_cobblestone_slab` block.
pub mod mossy_cobblestone_slab {
    /// `minecraft:mossy_cobblestone_slab` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct MossyCobblestoneSlab {
        /// `waterlogged` state.
        pub waterlogged : bool,
        /// `type` state.
        pub kind : Kind,
    }
    /// `type` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Kind {
        /// `top` variant.
        Top,
        /// `bottom` variant.
        Bottom,
        /// `double` variant.
        Double,
    }
    impl super::Block for MossyCobblestoneSlab {
    }
    impl From<MossyCobblestoneSlab> for super::BlockState {
        fn from(value : MossyCobblestoneSlab) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:bamboo_sapling` block.
pub mod bamboo_sapling {
    /// `minecraft:bamboo_sapling` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct BambooSapling;
    impl super::Block for BambooSapling {
    }
    impl From<BambooSapling> for super::BlockState {
        fn from(value : BambooSapling) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:cave_vines_plant` block.
pub mod cave_vines_plant {
    /// `minecraft:cave_vines_plant` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct CaveVinesPlant {
        /// `berries` state.
        pub berries : bool,
    }
    impl super::Block for CaveVinesPlant {
    }
    impl From<CaveVinesPlant> for super::BlockState {
        fn from(value : CaveVinesPlant) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:lapis_ore` block.
pub mod lapis_ore {
    /// `minecraft:lapis_ore` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct LapisOre;
    impl super::Block for LapisOre {
    }
    impl From<LapisOre> for super::BlockState {
        fn from(value : LapisOre) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:pale_moss_carpet` block.
pub mod pale_moss_carpet {
    /// `minecraft:pale_moss_carpet` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct PaleMossCarpet {
        /// `bottom` state.
        pub bottom : bool,
        /// `east` state.
        pub east : East,
        /// `south` state.
        pub south : South,
        /// `west` state.
        pub west : West,
        /// `north` state.
        pub north : North,
    }
    /// `east` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum East {
        /// `none` variant.
        None,
        /// `low` variant.
        Low,
        /// `tall` variant.
        Tall,
    }
    /// `south` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum South {
        /// `none` variant.
        None,
        /// `low` variant.
        Low,
        /// `tall` variant.
        Tall,
    }
    /// `west` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum West {
        /// `none` variant.
        None,
        /// `low` variant.
        Low,
        /// `tall` variant.
        Tall,
    }
    /// `north` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum North {
        /// `none` variant.
        None,
        /// `low` variant.
        Low,
        /// `tall` variant.
        Tall,
    }
    impl super::Block for PaleMossCarpet {
    }
    impl From<PaleMossCarpet> for super::BlockState {
        fn from(value : PaleMossCarpet) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:jungle_door` block.
pub mod jungle_door {
    /// `minecraft:jungle_door` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct JungleDoor {
        /// `hinge` state.
        pub hinge : Hinge,
        /// `open` state.
        pub open : bool,
        /// `facing` state.
        pub facing : Facing,
        /// `half` state.
        pub half : Half,
        /// `powered` state.
        pub powered : bool,
    }
    /// `hinge` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Hinge {
        /// `left` variant.
        Left,
        /// `right` variant.
        Right,
    }
    /// `facing` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Facing {
        /// `north` variant.
        North,
        /// `south` variant.
        South,
        /// `west` variant.
        West,
        /// `east` variant.
        East,
    }
    /// `half` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Half {
        /// `upper` variant.
        Upper,
        /// `lower` variant.
        Lower,
    }
    impl super::Block for JungleDoor {
    }
    impl From<JungleDoor> for super::BlockState {
        fn from(value : JungleDoor) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:jungle_planks` block.
pub mod jungle_planks {
    /// `minecraft:jungle_planks` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct JunglePlanks;
    impl super::Block for JunglePlanks {
    }
    impl From<JunglePlanks> for super::BlockState {
        fn from(value : JunglePlanks) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:lilac` block.
pub mod lilac {
    /// `minecraft:lilac` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct Lilac {
        /// `half` state.
        pub half : Half,
    }
    /// `half` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Half {
        /// `upper` variant.
        Upper,
        /// `lower` variant.
        Lower,
    }
    impl super::Block for Lilac {
    }
    impl From<Lilac> for super::BlockState {
        fn from(value : Lilac) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:piglin_head` block.
pub mod piglin_head {
    /// `minecraft:piglin_head` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct PiglinHead {
        /// `powered` state.
        pub powered : bool,
        /// `rotation` state.
        pub rotation : Rotation,
    }
    /// `rotation` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Rotation {
        /// `0` variant.
        N0,
        /// `1` variant.
        N1,
        /// `2` variant.
        N2,
        /// `3` variant.
        N3,
        /// `4` variant.
        N4,
        /// `5` variant.
        N5,
        /// `6` variant.
        N6,
        /// `7` variant.
        N7,
        /// `8` variant.
        N8,
        /// `9` variant.
        N9,
        /// `10` variant.
        N10,
        /// `11` variant.
        N11,
        /// `12` variant.
        N12,
        /// `13` variant.
        N13,
        /// `14` variant.
        N14,
        /// `15` variant.
        N15,
    }
    impl super::Block for PiglinHead {
    }
    impl From<PiglinHead> for super::BlockState {
        fn from(value : PiglinHead) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:pale_oak_wood` block.
pub mod pale_oak_wood {
    /// `minecraft:pale_oak_wood` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct PaleOakWood {
        /// `axis` state.
        pub axis : Axis,
    }
    /// `axis` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Axis {
        /// `x` variant.
        X,
        /// `y` variant.
        Y,
        /// `z` variant.
        Z,
    }
    impl super::Block for PaleOakWood {
    }
    impl From<PaleOakWood> for super::BlockState {
        fn from(value : PaleOakWood) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:piston_head` block.
pub mod piston_head {
    /// `minecraft:piston_head` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct PistonHead {
        /// `type` state.
        pub kind : Kind,
        /// `facing` state.
        pub facing : Facing,
        /// `short` state.
        pub short : bool,
    }
    /// `type` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Kind {
        /// `normal` variant.
        Normal,
        /// `sticky` variant.
        Sticky,
    }
    /// `facing` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Facing {
        /// `north` variant.
        North,
        /// `east` variant.
        East,
        /// `south` variant.
        South,
        /// `west` variant.
        West,
        /// `up` variant.
        Up,
        /// `down` variant.
        Down,
    }
    impl super::Block for PistonHead {
    }
    impl From<PistonHead> for super::BlockState {
        fn from(value : PistonHead) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:glass_pane` block.
pub mod glass_pane {
    /// `minecraft:glass_pane` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct GlassPane {
        /// `west` state.
        pub west : bool,
        /// `north` state.
        pub north : bool,
        /// `south` state.
        pub south : bool,
        /// `east` state.
        pub east : bool,
        /// `waterlogged` state.
        pub waterlogged : bool,
    }
    impl super::Block for GlassPane {
    }
    impl From<GlassPane> for super::BlockState {
        fn from(value : GlassPane) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:green_stained_glass_pane` block.
pub mod green_stained_glass_pane {
    /// `minecraft:green_stained_glass_pane` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct GreenStainedGlassPane {
        /// `north` state.
        pub north : bool,
        /// `waterlogged` state.
        pub waterlogged : bool,
        /// `east` state.
        pub east : bool,
        /// `south` state.
        pub south : bool,
        /// `west` state.
        pub west : bool,
    }
    impl super::Block for GreenStainedGlassPane {
    }
    impl From<GreenStainedGlassPane> for super::BlockState {
        fn from(value : GreenStainedGlassPane) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:jungle_wood` block.
pub mod jungle_wood {
    /// `minecraft:jungle_wood` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct JungleWood {
        /// `axis` state.
        pub axis : Axis,
    }
    /// `axis` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Axis {
        /// `x` variant.
        X,
        /// `y` variant.
        Y,
        /// `z` variant.
        Z,
    }
    impl super::Block for JungleWood {
    }
    impl From<JungleWood> for super::BlockState {
        fn from(value : JungleWood) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:lantern` block.
pub mod lantern {
    /// `minecraft:lantern` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct Lantern {
        /// `waterlogged` state.
        pub waterlogged : bool,
        /// `hanging` state.
        pub hanging : bool,
    }
    impl super::Block for Lantern {
    }
    impl From<Lantern> for super::BlockState {
        fn from(value : Lantern) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:potted_pale_oak_sapling` block.
pub mod potted_pale_oak_sapling {
    /// `minecraft:potted_pale_oak_sapling` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct PottedPaleOakSapling;
    impl super::Block for PottedPaleOakSapling {
    }
    impl From<PottedPaleOakSapling> for super::BlockState {
        fn from(value : PottedPaleOakSapling) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:structure_void` block.
pub mod structure_void {
    /// `minecraft:structure_void` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct StructureVoid;
    impl super::Block for StructureVoid {
    }
    impl From<StructureVoid> for super::BlockState {
        fn from(value : StructureVoid) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:chiseled_resin_bricks` block.
pub mod chiseled_resin_bricks {
    /// `minecraft:chiseled_resin_bricks` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct ChiseledResinBricks;
    impl super::Block for ChiseledResinBricks {
    }
    impl From<ChiseledResinBricks> for super::BlockState {
        fn from(value : ChiseledResinBricks) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:red_sand` block.
pub mod red_sand {
    /// `minecraft:red_sand` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct RedSand;
    impl super::Block for RedSand {
    }
    impl From<RedSand> for super::BlockState {
        fn from(value : RedSand) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:smooth_stone_slab` block.
pub mod smooth_stone_slab {
    /// `minecraft:smooth_stone_slab` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct SmoothStoneSlab {
        /// `waterlogged` state.
        pub waterlogged : bool,
        /// `type` state.
        pub kind : Kind,
    }
    /// `type` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Kind {
        /// `top` variant.
        Top,
        /// `bottom` variant.
        Bottom,
        /// `double` variant.
        Double,
    }
    impl super::Block for SmoothStoneSlab {
    }
    impl From<SmoothStoneSlab> for super::BlockState {
        fn from(value : SmoothStoneSlab) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:mud_brick_wall` block.
pub mod mud_brick_wall {
    /// `minecraft:mud_brick_wall` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct MudBrickWall {
        /// `north` state.
        pub north : North,
        /// `west` state.
        pub west : West,
        /// `up` state.
        pub up : bool,
        /// `south` state.
        pub south : South,
        /// `east` state.
        pub east : East,
        /// `waterlogged` state.
        pub waterlogged : bool,
    }
    /// `north` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum North {
        /// `none` variant.
        None,
        /// `low` variant.
        Low,
        /// `tall` variant.
        Tall,
    }
    /// `west` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum West {
        /// `none` variant.
        None,
        /// `low` variant.
        Low,
        /// `tall` variant.
        Tall,
    }
    /// `south` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum South {
        /// `none` variant.
        None,
        /// `low` variant.
        Low,
        /// `tall` variant.
        Tall,
    }
    /// `east` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum East {
        /// `none` variant.
        None,
        /// `low` variant.
        Low,
        /// `tall` variant.
        Tall,
    }
    impl super::Block for MudBrickWall {
    }
    impl From<MudBrickWall> for super::BlockState {
        fn from(value : MudBrickWall) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:quartz_slab` block.
pub mod quartz_slab {
    /// `minecraft:quartz_slab` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct QuartzSlab {
        /// `type` state.
        pub kind : Kind,
        /// `waterlogged` state.
        pub waterlogged : bool,
    }
    /// `type` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Kind {
        /// `top` variant.
        Top,
        /// `bottom` variant.
        Bottom,
        /// `double` variant.
        Double,
    }
    impl super::Block for QuartzSlab {
    }
    impl From<QuartzSlab> for super::BlockState {
        fn from(value : QuartzSlab) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:cobblestone` block.
pub mod cobblestone {
    /// `minecraft:cobblestone` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct Cobblestone;
    impl super::Block for Cobblestone {
    }
    impl From<Cobblestone> for super::BlockState {
        fn from(value : Cobblestone) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:polished_tuff` block.
pub mod polished_tuff {
    /// `minecraft:polished_tuff` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct PolishedTuff;
    impl super::Block for PolishedTuff {
    }
    impl From<PolishedTuff> for super::BlockState {
        fn from(value : PolishedTuff) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:frogspawn` block.
pub mod frogspawn {
    /// `minecraft:frogspawn` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct Frogspawn;
    impl super::Block for Frogspawn {
    }
    impl From<Frogspawn> for super::BlockState {
        fn from(value : Frogspawn) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:diorite_wall` block.
pub mod diorite_wall {
    /// `minecraft:diorite_wall` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct DioriteWall {
        /// `east` state.
        pub east : East,
        /// `up` state.
        pub up : bool,
        /// `west` state.
        pub west : West,
        /// `waterlogged` state.
        pub waterlogged : bool,
        /// `south` state.
        pub south : South,
        /// `north` state.
        pub north : North,
    }
    /// `east` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum East {
        /// `none` variant.
        None,
        /// `low` variant.
        Low,
        /// `tall` variant.
        Tall,
    }
    /// `west` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum West {
        /// `none` variant.
        None,
        /// `low` variant.
        Low,
        /// `tall` variant.
        Tall,
    }
    /// `south` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum South {
        /// `none` variant.
        None,
        /// `low` variant.
        Low,
        /// `tall` variant.
        Tall,
    }
    /// `north` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum North {
        /// `none` variant.
        None,
        /// `low` variant.
        Low,
        /// `tall` variant.
        Tall,
    }
    impl super::Block for DioriteWall {
    }
    impl From<DioriteWall> for super::BlockState {
        fn from(value : DioriteWall) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:green_banner` block.
pub mod green_banner {
    /// `minecraft:green_banner` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct GreenBanner {
        /// `rotation` state.
        pub rotation : Rotation,
    }
    /// `rotation` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Rotation {
        /// `0` variant.
        N0,
        /// `1` variant.
        N1,
        /// `2` variant.
        N2,
        /// `3` variant.
        N3,
        /// `4` variant.
        N4,
        /// `5` variant.
        N5,
        /// `6` variant.
        N6,
        /// `7` variant.
        N7,
        /// `8` variant.
        N8,
        /// `9` variant.
        N9,
        /// `10` variant.
        N10,
        /// `11` variant.
        N11,
        /// `12` variant.
        N12,
        /// `13` variant.
        N13,
        /// `14` variant.
        N14,
        /// `15` variant.
        N15,
    }
    impl super::Block for GreenBanner {
    }
    impl From<GreenBanner> for super::BlockState {
        fn from(value : GreenBanner) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:cracked_deepslate_tiles` block.
pub mod cracked_deepslate_tiles {
    /// `minecraft:cracked_deepslate_tiles` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct CrackedDeepslateTiles;
    impl super::Block for CrackedDeepslateTiles {
    }
    impl From<CrackedDeepslateTiles> for super::BlockState {
        fn from(value : CrackedDeepslateTiles) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:netherite_block` block.
pub mod netherite_block {
    /// `minecraft:netherite_block` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct NetheriteBlock;
    impl super::Block for NetheriteBlock {
    }
    impl From<NetheriteBlock> for super::BlockState {
        fn from(value : NetheriteBlock) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:light_gray_glazed_terracotta` block.
pub mod light_gray_glazed_terracotta {
    /// `minecraft:light_gray_glazed_terracotta` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct LightGrayGlazedTerracotta {
        /// `facing` state.
        pub facing : Facing,
    }
    /// `facing` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Facing {
        /// `north` variant.
        North,
        /// `south` variant.
        South,
        /// `west` variant.
        West,
        /// `east` variant.
        East,
    }
    impl super::Block for LightGrayGlazedTerracotta {
    }
    impl From<LightGrayGlazedTerracotta> for super::BlockState {
        fn from(value : LightGrayGlazedTerracotta) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:mossy_cobblestone_stairs` block.
pub mod mossy_cobblestone_stairs {
    /// `minecraft:mossy_cobblestone_stairs` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct MossyCobblestoneStairs {
        /// `half` state.
        pub half : Half,
        /// `shape` state.
        pub shape : Shape,
        /// `facing` state.
        pub facing : Facing,
        /// `waterlogged` state.
        pub waterlogged : bool,
    }
    /// `half` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Half {
        /// `top` variant.
        Top,
        /// `bottom` variant.
        Bottom,
    }
    /// `shape` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Shape {
        /// `straight` variant.
        Straight,
        /// `inner_left` variant.
        InnerLeft,
        /// `inner_right` variant.
        InnerRight,
        /// `outer_left` variant.
        OuterLeft,
        /// `outer_right` variant.
        OuterRight,
    }
    /// `facing` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Facing {
        /// `north` variant.
        North,
        /// `south` variant.
        South,
        /// `west` variant.
        West,
        /// `east` variant.
        East,
    }
    impl super::Block for MossyCobblestoneStairs {
    }
    impl From<MossyCobblestoneStairs> for super::BlockState {
        fn from(value : MossyCobblestoneStairs) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:cyan_bed` block.
pub mod cyan_bed {
    /// `minecraft:cyan_bed` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct CyanBed {
        /// `part` state.
        pub part : Part,
        /// `occupied` state.
        pub occupied : bool,
        /// `facing` state.
        pub facing : Facing,
    }
    /// `part` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Part {
        /// `head` variant.
        Head,
        /// `foot` variant.
        Foot,
    }
    /// `facing` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Facing {
        /// `north` variant.
        North,
        /// `south` variant.
        South,
        /// `west` variant.
        West,
        /// `east` variant.
        East,
    }
    impl super::Block for CyanBed {
    }
    impl From<CyanBed> for super::BlockState {
        fn from(value : CyanBed) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:prismarine_bricks` block.
pub mod prismarine_bricks {
    /// `minecraft:prismarine_bricks` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct PrismarineBricks;
    impl super::Block for PrismarineBricks {
    }
    impl From<PrismarineBricks> for super::BlockState {
        fn from(value : PrismarineBricks) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:acacia_pressure_plate` block.
pub mod acacia_pressure_plate {
    /// `minecraft:acacia_pressure_plate` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct AcaciaPressurePlate {
        /// `powered` state.
        pub powered : bool,
    }
    impl super::Block for AcaciaPressurePlate {
    }
    impl From<AcaciaPressurePlate> for super::BlockState {
        fn from(value : AcaciaPressurePlate) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:dead_brain_coral` block.
pub mod dead_brain_coral {
    /// `minecraft:dead_brain_coral` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct DeadBrainCoral {
        /// `waterlogged` state.
        pub waterlogged : bool,
    }
    impl super::Block for DeadBrainCoral {
    }
    impl From<DeadBrainCoral> for super::BlockState {
        fn from(value : DeadBrainCoral) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:bamboo_door` block.
pub mod bamboo_door {
    /// `minecraft:bamboo_door` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct BambooDoor {
        /// `half` state.
        pub half : Half,
        /// `open` state.
        pub open : bool,
        /// `facing` state.
        pub facing : Facing,
        /// `powered` state.
        pub powered : bool,
        /// `hinge` state.
        pub hinge : Hinge,
    }
    /// `half` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Half {
        /// `upper` variant.
        Upper,
        /// `lower` variant.
        Lower,
    }
    /// `facing` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Facing {
        /// `north` variant.
        North,
        /// `south` variant.
        South,
        /// `west` variant.
        West,
        /// `east` variant.
        East,
    }
    /// `hinge` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Hinge {
        /// `left` variant.
        Left,
        /// `right` variant.
        Right,
    }
    impl super::Block for BambooDoor {
    }
    impl From<BambooDoor> for super::BlockState {
        fn from(value : BambooDoor) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:black_stained_glass_pane` block.
pub mod black_stained_glass_pane {
    /// `minecraft:black_stained_glass_pane` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct BlackStainedGlassPane {
        /// `east` state.
        pub east : bool,
        /// `north` state.
        pub north : bool,
        /// `waterlogged` state.
        pub waterlogged : bool,
        /// `west` state.
        pub west : bool,
        /// `south` state.
        pub south : bool,
    }
    impl super::Block for BlackStainedGlassPane {
    }
    impl From<BlackStainedGlassPane> for super::BlockState {
        fn from(value : BlackStainedGlassPane) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:blue_stained_glass_pane` block.
pub mod blue_stained_glass_pane {
    /// `minecraft:blue_stained_glass_pane` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct BlueStainedGlassPane {
        /// `east` state.
        pub east : bool,
        /// `west` state.
        pub west : bool,
        /// `waterlogged` state.
        pub waterlogged : bool,
        /// `south` state.
        pub south : bool,
        /// `north` state.
        pub north : bool,
    }
    impl super::Block for BlueStainedGlassPane {
    }
    impl From<BlueStainedGlassPane> for super::BlockState {
        fn from(value : BlueStainedGlassPane) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:dead_horn_coral` block.
pub mod dead_horn_coral {
    /// `minecraft:dead_horn_coral` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct DeadHornCoral {
        /// `waterlogged` state.
        pub waterlogged : bool,
    }
    impl super::Block for DeadHornCoral {
    }
    impl From<DeadHornCoral> for super::BlockState {
        fn from(value : DeadHornCoral) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:nether_sprouts` block.
pub mod nether_sprouts {
    /// `minecraft:nether_sprouts` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct NetherSprouts;
    impl super::Block for NetherSprouts {
    }
    impl From<NetherSprouts> for super::BlockState {
        fn from(value : NetherSprouts) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:bamboo_fence` block.
pub mod bamboo_fence {
    /// `minecraft:bamboo_fence` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct BambooFence {
        /// `west` state.
        pub west : bool,
        /// `east` state.
        pub east : bool,
        /// `south` state.
        pub south : bool,
        /// `north` state.
        pub north : bool,
        /// `waterlogged` state.
        pub waterlogged : bool,
    }
    impl super::Block for BambooFence {
    }
    impl From<BambooFence> for super::BlockState {
        fn from(value : BambooFence) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:piston` block.
pub mod piston {
    /// `minecraft:piston` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct Piston {
        /// `extended` state.
        pub extended : bool,
        /// `facing` state.
        pub facing : Facing,
    }
    /// `facing` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Facing {
        /// `north` variant.
        North,
        /// `east` variant.
        East,
        /// `south` variant.
        South,
        /// `west` variant.
        West,
        /// `up` variant.
        Up,
        /// `down` variant.
        Down,
    }
    impl super::Block for Piston {
    }
    impl From<Piston> for super::BlockState {
        fn from(value : Piston) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:smoker` block.
pub mod smoker {
    /// `minecraft:smoker` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct Smoker {
        /// `lit` state.
        pub lit : bool,
        /// `facing` state.
        pub facing : Facing,
    }
    /// `facing` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Facing {
        /// `north` variant.
        North,
        /// `south` variant.
        South,
        /// `west` variant.
        West,
        /// `east` variant.
        East,
    }
    impl super::Block for Smoker {
    }
    impl From<Smoker> for super::BlockState {
        fn from(value : Smoker) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:smooth_red_sandstone` block.
pub mod smooth_red_sandstone {
    /// `minecraft:smooth_red_sandstone` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct SmoothRedSandstone;
    impl super::Block for SmoothRedSandstone {
    }
    impl From<SmoothRedSandstone> for super::BlockState {
        fn from(value : SmoothRedSandstone) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:white_candle_cake` block.
pub mod white_candle_cake {
    /// `minecraft:white_candle_cake` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct WhiteCandleCake {
        /// `lit` state.
        pub lit : bool,
    }
    impl super::Block for WhiteCandleCake {
    }
    impl From<WhiteCandleCake> for super::BlockState {
        fn from(value : WhiteCandleCake) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:oxidized_cut_copper_slab` block.
pub mod oxidized_cut_copper_slab {
    /// `minecraft:oxidized_cut_copper_slab` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct OxidizedCutCopperSlab {
        /// `type` state.
        pub kind : Kind,
        /// `waterlogged` state.
        pub waterlogged : bool,
    }
    /// `type` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Kind {
        /// `top` variant.
        Top,
        /// `bottom` variant.
        Bottom,
        /// `double` variant.
        Double,
    }
    impl super::Block for OxidizedCutCopperSlab {
    }
    impl From<OxidizedCutCopperSlab> for super::BlockState {
        fn from(value : OxidizedCutCopperSlab) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:pale_oak_pressure_plate` block.
pub mod pale_oak_pressure_plate {
    /// `minecraft:pale_oak_pressure_plate` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct PaleOakPressurePlate {
        /// `powered` state.
        pub powered : bool,
    }
    impl super::Block for PaleOakPressurePlate {
    }
    impl From<PaleOakPressurePlate> for super::BlockState {
        fn from(value : PaleOakPressurePlate) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:cyan_shulker_box` block.
pub mod cyan_shulker_box {
    /// `minecraft:cyan_shulker_box` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct CyanShulkerBox {
        /// `facing` state.
        pub facing : Facing,
    }
    /// `facing` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Facing {
        /// `north` variant.
        North,
        /// `east` variant.
        East,
        /// `south` variant.
        South,
        /// `west` variant.
        West,
        /// `up` variant.
        Up,
        /// `down` variant.
        Down,
    }
    impl super::Block for CyanShulkerBox {
    }
    impl From<CyanShulkerBox> for super::BlockState {
        fn from(value : CyanShulkerBox) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:honey_block` block.
pub mod honey_block {
    /// `minecraft:honey_block` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct HoneyBlock;
    impl super::Block for HoneyBlock {
    }
    impl From<HoneyBlock> for super::BlockState {
        fn from(value : HoneyBlock) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:mangrove_log` block.
pub mod mangrove_log {
    /// `minecraft:mangrove_log` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct MangroveLog {
        /// `axis` state.
        pub axis : Axis,
    }
    /// `axis` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Axis {
        /// `x` variant.
        X,
        /// `y` variant.
        Y,
        /// `z` variant.
        Z,
    }
    impl super::Block for MangroveLog {
    }
    impl From<MangroveLog> for super::BlockState {
        fn from(value : MangroveLog) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:big_dripleaf` block.
pub mod big_dripleaf {
    /// `minecraft:big_dripleaf` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct BigDripleaf {
        /// `facing` state.
        pub facing : Facing,
        /// `waterlogged` state.
        pub waterlogged : bool,
        /// `tilt` state.
        pub tilt : Tilt,
    }
    /// `facing` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Facing {
        /// `north` variant.
        North,
        /// `south` variant.
        South,
        /// `west` variant.
        West,
        /// `east` variant.
        East,
    }
    /// `tilt` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Tilt {
        /// `none` variant.
        None,
        /// `unstable` variant.
        Unstable,
        /// `partial` variant.
        Partial,
        /// `full` variant.
        Full,
    }
    impl super::Block for BigDripleaf {
    }
    impl From<BigDripleaf> for super::BlockState {
        fn from(value : BigDripleaf) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:detector_rail` block.
pub mod detector_rail {
    /// `minecraft:detector_rail` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct DetectorRail {
        /// `waterlogged` state.
        pub waterlogged : bool,
        /// `powered` state.
        pub powered : bool,
        /// `shape` state.
        pub shape : Shape,
    }
    /// `shape` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Shape {
        /// `north_south` variant.
        NorthSouth,
        /// `east_west` variant.
        EastWest,
        /// `ascending_east` variant.
        AscendingEast,
        /// `ascending_west` variant.
        AscendingWest,
        /// `ascending_north` variant.
        AscendingNorth,
        /// `ascending_south` variant.
        AscendingSouth,
    }
    impl super::Block for DetectorRail {
    }
    impl From<DetectorRail> for super::BlockState {
        fn from(value : DetectorRail) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:waxed_exposed_copper_grate` block.
pub mod waxed_exposed_copper_grate {
    /// `minecraft:waxed_exposed_copper_grate` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct WaxedExposedCopperGrate {
        /// `waterlogged` state.
        pub waterlogged : bool,
    }
    impl super::Block for WaxedExposedCopperGrate {
    }
    impl From<WaxedExposedCopperGrate> for super::BlockState {
        fn from(value : WaxedExposedCopperGrate) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:red_tulip` block.
pub mod red_tulip {
    /// `minecraft:red_tulip` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct RedTulip;
    impl super::Block for RedTulip {
    }
    impl From<RedTulip> for super::BlockState {
        fn from(value : RedTulip) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:potted_white_tulip` block.
pub mod potted_white_tulip {
    /// `minecraft:potted_white_tulip` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct PottedWhiteTulip;
    impl super::Block for PottedWhiteTulip {
    }
    impl From<PottedWhiteTulip> for super::BlockState {
        fn from(value : PottedWhiteTulip) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:potted_allium` block.
pub mod potted_allium {
    /// `minecraft:potted_allium` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct PottedAllium;
    impl super::Block for PottedAllium {
    }
    impl From<PottedAllium> for super::BlockState {
        fn from(value : PottedAllium) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:red_banner` block.
pub mod red_banner {
    /// `minecraft:red_banner` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct RedBanner {
        /// `rotation` state.
        pub rotation : Rotation,
    }
    /// `rotation` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Rotation {
        /// `0` variant.
        N0,
        /// `1` variant.
        N1,
        /// `2` variant.
        N2,
        /// `3` variant.
        N3,
        /// `4` variant.
        N4,
        /// `5` variant.
        N5,
        /// `6` variant.
        N6,
        /// `7` variant.
        N7,
        /// `8` variant.
        N8,
        /// `9` variant.
        N9,
        /// `10` variant.
        N10,
        /// `11` variant.
        N11,
        /// `12` variant.
        N12,
        /// `13` variant.
        N13,
        /// `14` variant.
        N14,
        /// `15` variant.
        N15,
    }
    impl super::Block for RedBanner {
    }
    impl From<RedBanner> for super::BlockState {
        fn from(value : RedBanner) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:black_glazed_terracotta` block.
pub mod black_glazed_terracotta {
    /// `minecraft:black_glazed_terracotta` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct BlackGlazedTerracotta {
        /// `facing` state.
        pub facing : Facing,
    }
    /// `facing` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Facing {
        /// `north` variant.
        North,
        /// `south` variant.
        South,
        /// `west` variant.
        West,
        /// `east` variant.
        East,
    }
    impl super::Block for BlackGlazedTerracotta {
    }
    impl From<BlackGlazedTerracotta> for super::BlockState {
        fn from(value : BlackGlazedTerracotta) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:blue_concrete` block.
pub mod blue_concrete {
    /// `minecraft:blue_concrete` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct BlueConcrete;
    impl super::Block for BlueConcrete {
    }
    impl From<BlueConcrete> for super::BlockState {
        fn from(value : BlueConcrete) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:blue_ice` block.
pub mod blue_ice {
    /// `minecraft:blue_ice` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct BlueIce;
    impl super::Block for BlueIce {
    }
    impl From<BlueIce> for super::BlockState {
        fn from(value : BlueIce) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:black_banner` block.
pub mod black_banner {
    /// `minecraft:black_banner` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct BlackBanner {
        /// `rotation` state.
        pub rotation : Rotation,
    }
    /// `rotation` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Rotation {
        /// `0` variant.
        N0,
        /// `1` variant.
        N1,
        /// `2` variant.
        N2,
        /// `3` variant.
        N3,
        /// `4` variant.
        N4,
        /// `5` variant.
        N5,
        /// `6` variant.
        N6,
        /// `7` variant.
        N7,
        /// `8` variant.
        N8,
        /// `9` variant.
        N9,
        /// `10` variant.
        N10,
        /// `11` variant.
        N11,
        /// `12` variant.
        N12,
        /// `13` variant.
        N13,
        /// `14` variant.
        N14,
        /// `15` variant.
        N15,
    }
    impl super::Block for BlackBanner {
    }
    impl From<BlackBanner> for super::BlockState {
        fn from(value : BlackBanner) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:weathered_copper_grate` block.
pub mod weathered_copper_grate {
    /// `minecraft:weathered_copper_grate` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct WeatheredCopperGrate {
        /// `waterlogged` state.
        pub waterlogged : bool,
    }
    impl super::Block for WeatheredCopperGrate {
    }
    impl From<WeatheredCopperGrate> for super::BlockState {
        fn from(value : WeatheredCopperGrate) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:white_wall_banner` block.
pub mod white_wall_banner {
    /// `minecraft:white_wall_banner` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct WhiteWallBanner {
        /// `facing` state.
        pub facing : Facing,
    }
    /// `facing` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Facing {
        /// `north` variant.
        North,
        /// `south` variant.
        South,
        /// `west` variant.
        West,
        /// `east` variant.
        East,
    }
    impl super::Block for WhiteWallBanner {
    }
    impl From<WhiteWallBanner> for super::BlockState {
        fn from(value : WhiteWallBanner) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:blackstone_slab` block.
pub mod blackstone_slab {
    /// `minecraft:blackstone_slab` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct BlackstoneSlab {
        /// `waterlogged` state.
        pub waterlogged : bool,
        /// `type` state.
        pub kind : Kind,
    }
    /// `type` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Kind {
        /// `top` variant.
        Top,
        /// `bottom` variant.
        Bottom,
        /// `double` variant.
        Double,
    }
    impl super::Block for BlackstoneSlab {
    }
    impl From<BlackstoneSlab> for super::BlockState {
        fn from(value : BlackstoneSlab) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:pink_stained_glass` block.
pub mod pink_stained_glass {
    /// `minecraft:pink_stained_glass` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct PinkStainedGlass;
    impl super::Block for PinkStainedGlass {
    }
    impl From<PinkStainedGlass> for super::BlockState {
        fn from(value : PinkStainedGlass) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:black_wall_banner` block.
pub mod black_wall_banner {
    /// `minecraft:black_wall_banner` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct BlackWallBanner {
        /// `facing` state.
        pub facing : Facing,
    }
    /// `facing` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Facing {
        /// `north` variant.
        North,
        /// `south` variant.
        South,
        /// `west` variant.
        West,
        /// `east` variant.
        East,
    }
    impl super::Block for BlackWallBanner {
    }
    impl From<BlackWallBanner> for super::BlockState {
        fn from(value : BlackWallBanner) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:candle_cake` block.
pub mod candle_cake {
    /// `minecraft:candle_cake` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct CandleCake {
        /// `lit` state.
        pub lit : bool,
    }
    impl super::Block for CandleCake {
    }
    impl From<CandleCake> for super::BlockState {
        fn from(value : CandleCake) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:crimson_door` block.
pub mod crimson_door {
    /// `minecraft:crimson_door` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct CrimsonDoor {
        /// `facing` state.
        pub facing : Facing,
        /// `half` state.
        pub half : Half,
        /// `hinge` state.
        pub hinge : Hinge,
        /// `open` state.
        pub open : bool,
        /// `powered` state.
        pub powered : bool,
    }
    /// `facing` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Facing {
        /// `north` variant.
        North,
        /// `south` variant.
        South,
        /// `west` variant.
        West,
        /// `east` variant.
        East,
    }
    /// `half` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Half {
        /// `upper` variant.
        Upper,
        /// `lower` variant.
        Lower,
    }
    /// `hinge` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Hinge {
        /// `left` variant.
        Left,
        /// `right` variant.
        Right,
    }
    impl super::Block for CrimsonDoor {
    }
    impl From<CrimsonDoor> for super::BlockState {
        fn from(value : CrimsonDoor) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:cut_red_sandstone` block.
pub mod cut_red_sandstone {
    /// `minecraft:cut_red_sandstone` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct CutRedSandstone;
    impl super::Block for CutRedSandstone {
    }
    impl From<CutRedSandstone> for super::BlockState {
        fn from(value : CutRedSandstone) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:dark_oak_trapdoor` block.
pub mod dark_oak_trapdoor {
    /// `minecraft:dark_oak_trapdoor` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct DarkOakTrapdoor {
        /// `half` state.
        pub half : Half,
        /// `waterlogged` state.
        pub waterlogged : bool,
        /// `open` state.
        pub open : bool,
        /// `facing` state.
        pub facing : Facing,
        /// `powered` state.
        pub powered : bool,
    }
    /// `half` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Half {
        /// `top` variant.
        Top,
        /// `bottom` variant.
        Bottom,
    }
    /// `facing` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Facing {
        /// `north` variant.
        North,
        /// `south` variant.
        South,
        /// `west` variant.
        West,
        /// `east` variant.
        East,
    }
    impl super::Block for DarkOakTrapdoor {
    }
    impl From<DarkOakTrapdoor> for super::BlockState {
        fn from(value : DarkOakTrapdoor) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:nether_wart_block` block.
pub mod nether_wart_block {
    /// `minecraft:nether_wart_block` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct NetherWartBlock;
    impl super::Block for NetherWartBlock {
    }
    impl From<NetherWartBlock> for super::BlockState {
        fn from(value : NetherWartBlock) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:birch_sapling` block.
pub mod birch_sapling {
    /// `minecraft:birch_sapling` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct BirchSapling {
        /// `stage` state.
        pub stage : Stage,
    }
    /// `stage` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Stage {
        /// `0` variant.
        N0,
        /// `1` variant.
        N1,
    }
    impl super::Block for BirchSapling {
    }
    impl From<BirchSapling> for super::BlockState {
        fn from(value : BirchSapling) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:horn_coral_block` block.
pub mod horn_coral_block {
    /// `minecraft:horn_coral_block` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct HornCoralBlock;
    impl super::Block for HornCoralBlock {
    }
    impl From<HornCoralBlock> for super::BlockState {
        fn from(value : HornCoralBlock) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:nether_portal` block.
pub mod nether_portal {
    /// `minecraft:nether_portal` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct NetherPortal {
        /// `axis` state.
        pub axis : Axis,
    }
    /// `axis` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Axis {
        /// `x` variant.
        X,
        /// `z` variant.
        Z,
    }
    impl super::Block for NetherPortal {
    }
    impl From<NetherPortal> for super::BlockState {
        fn from(value : NetherPortal) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:obsidian` block.
pub mod obsidian {
    /// `minecraft:obsidian` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct Obsidian;
    impl super::Block for Obsidian {
    }
    impl From<Obsidian> for super::BlockState {
        fn from(value : Obsidian) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:potted_cornflower` block.
pub mod potted_cornflower {
    /// `minecraft:potted_cornflower` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct PottedCornflower;
    impl super::Block for PottedCornflower {
    }
    impl From<PottedCornflower> for super::BlockState {
        fn from(value : PottedCornflower) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:waxed_chiseled_copper` block.
pub mod waxed_chiseled_copper {
    /// `minecraft:waxed_chiseled_copper` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct WaxedChiseledCopper;
    impl super::Block for WaxedChiseledCopper {
    }
    impl From<WaxedChiseledCopper> for super::BlockState {
        fn from(value : WaxedChiseledCopper) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:lime_wall_banner` block.
pub mod lime_wall_banner {
    /// `minecraft:lime_wall_banner` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct LimeWallBanner {
        /// `facing` state.
        pub facing : Facing,
    }
    /// `facing` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Facing {
        /// `north` variant.
        North,
        /// `south` variant.
        South,
        /// `west` variant.
        West,
        /// `east` variant.
        East,
    }
    impl super::Block for LimeWallBanner {
    }
    impl From<LimeWallBanner> for super::BlockState {
        fn from(value : LimeWallBanner) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:polished_deepslate_stairs` block.
pub mod polished_deepslate_stairs {
    /// `minecraft:polished_deepslate_stairs` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct PolishedDeepslateStairs {
        /// `facing` state.
        pub facing : Facing,
        /// `half` state.
        pub half : Half,
        /// `waterlogged` state.
        pub waterlogged : bool,
        /// `shape` state.
        pub shape : Shape,
    }
    /// `facing` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Facing {
        /// `north` variant.
        North,
        /// `south` variant.
        South,
        /// `west` variant.
        West,
        /// `east` variant.
        East,
    }
    /// `half` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Half {
        /// `top` variant.
        Top,
        /// `bottom` variant.
        Bottom,
    }
    /// `shape` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Shape {
        /// `straight` variant.
        Straight,
        /// `inner_left` variant.
        InnerLeft,
        /// `inner_right` variant.
        InnerRight,
        /// `outer_left` variant.
        OuterLeft,
        /// `outer_right` variant.
        OuterRight,
    }
    impl super::Block for PolishedDeepslateStairs {
    }
    impl From<PolishedDeepslateStairs> for super::BlockState {
        fn from(value : PolishedDeepslateStairs) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:sunflower` block.
pub mod sunflower {
    /// `minecraft:sunflower` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct Sunflower {
        /// `half` state.
        pub half : Half,
    }
    /// `half` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Half {
        /// `upper` variant.
        Upper,
        /// `lower` variant.
        Lower,
    }
    impl super::Block for Sunflower {
    }
    impl From<Sunflower> for super::BlockState {
        fn from(value : Sunflower) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:pearlescent_froglight` block.
pub mod pearlescent_froglight {
    /// `minecraft:pearlescent_froglight` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct PearlescentFroglight {
        /// `axis` state.
        pub axis : Axis,
    }
    /// `axis` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Axis {
        /// `x` variant.
        X,
        /// `y` variant.
        Y,
        /// `z` variant.
        Z,
    }
    impl super::Block for PearlescentFroglight {
    }
    impl From<PearlescentFroglight> for super::BlockState {
        fn from(value : PearlescentFroglight) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:orange_carpet` block.
pub mod orange_carpet {
    /// `minecraft:orange_carpet` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct OrangeCarpet;
    impl super::Block for OrangeCarpet {
    }
    impl From<OrangeCarpet> for super::BlockState {
        fn from(value : OrangeCarpet) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:medium_amethyst_bud` block.
pub mod medium_amethyst_bud {
    /// `minecraft:medium_amethyst_bud` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct MediumAmethystBud {
        /// `facing` state.
        pub facing : Facing,
        /// `waterlogged` state.
        pub waterlogged : bool,
    }
    /// `facing` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Facing {
        /// `north` variant.
        North,
        /// `east` variant.
        East,
        /// `south` variant.
        South,
        /// `west` variant.
        West,
        /// `up` variant.
        Up,
        /// `down` variant.
        Down,
    }
    impl super::Block for MediumAmethystBud {
    }
    impl From<MediumAmethystBud> for super::BlockState {
        fn from(value : MediumAmethystBud) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:stripped_crimson_stem` block.
pub mod stripped_crimson_stem {
    /// `minecraft:stripped_crimson_stem` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct StrippedCrimsonStem {
        /// `axis` state.
        pub axis : Axis,
    }
    /// `axis` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Axis {
        /// `x` variant.
        X,
        /// `y` variant.
        Y,
        /// `z` variant.
        Z,
    }
    impl super::Block for StrippedCrimsonStem {
    }
    impl From<StrippedCrimsonStem> for super::BlockState {
        fn from(value : StrippedCrimsonStem) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:brain_coral` block.
pub mod brain_coral {
    /// `minecraft:brain_coral` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct BrainCoral {
        /// `waterlogged` state.
        pub waterlogged : bool,
    }
    impl super::Block for BrainCoral {
    }
    impl From<BrainCoral> for super::BlockState {
        fn from(value : BrainCoral) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:dead_horn_coral_wall_fan` block.
pub mod dead_horn_coral_wall_fan {
    /// `minecraft:dead_horn_coral_wall_fan` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct DeadHornCoralWallFan {
        /// `facing` state.
        pub facing : Facing,
        /// `waterlogged` state.
        pub waterlogged : bool,
    }
    /// `facing` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Facing {
        /// `north` variant.
        North,
        /// `south` variant.
        South,
        /// `west` variant.
        West,
        /// `east` variant.
        East,
    }
    impl super::Block for DeadHornCoralWallFan {
    }
    impl From<DeadHornCoralWallFan> for super::BlockState {
        fn from(value : DeadHornCoralWallFan) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:terracotta` block.
pub mod terracotta {
    /// `minecraft:terracotta` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct Terracotta;
    impl super::Block for Terracotta {
    }
    impl From<Terracotta> for super::BlockState {
        fn from(value : Terracotta) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:oxidized_chiseled_copper` block.
pub mod oxidized_chiseled_copper {
    /// `minecraft:oxidized_chiseled_copper` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct OxidizedChiseledCopper;
    impl super::Block for OxidizedChiseledCopper {
    }
    impl From<OxidizedChiseledCopper> for super::BlockState {
        fn from(value : OxidizedChiseledCopper) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:stone` block.
pub mod stone {
    /// `minecraft:stone` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct Stone;
    impl super::Block for Stone {
    }
    impl From<Stone> for super::BlockState {
        fn from(value : Stone) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:spruce_wall_hanging_sign` block.
pub mod spruce_wall_hanging_sign {
    /// `minecraft:spruce_wall_hanging_sign` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct SpruceWallHangingSign {
        /// `waterlogged` state.
        pub waterlogged : bool,
        /// `facing` state.
        pub facing : Facing,
    }
    /// `facing` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Facing {
        /// `north` variant.
        North,
        /// `south` variant.
        South,
        /// `west` variant.
        West,
        /// `east` variant.
        East,
    }
    impl super::Block for SpruceWallHangingSign {
    }
    impl From<SpruceWallHangingSign> for super::BlockState {
        fn from(value : SpruceWallHangingSign) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:diamond_ore` block.
pub mod diamond_ore {
    /// `minecraft:diamond_ore` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct DiamondOre;
    impl super::Block for DiamondOre {
    }
    impl From<DiamondOre> for super::BlockState {
        fn from(value : DiamondOre) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:bricks` block.
pub mod bricks {
    /// `minecraft:bricks` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct Bricks;
    impl super::Block for Bricks {
    }
    impl From<Bricks> for super::BlockState {
        fn from(value : Bricks) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:ochre_froglight` block.
pub mod ochre_froglight {
    /// `minecraft:ochre_froglight` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct OchreFroglight {
        /// `axis` state.
        pub axis : Axis,
    }
    /// `axis` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Axis {
        /// `x` variant.
        X,
        /// `y` variant.
        Y,
        /// `z` variant.
        Z,
    }
    impl super::Block for OchreFroglight {
    }
    impl From<OchreFroglight> for super::BlockState {
        fn from(value : OchreFroglight) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:oxidized_copper_grate` block.
pub mod oxidized_copper_grate {
    /// `minecraft:oxidized_copper_grate` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct OxidizedCopperGrate {
        /// `waterlogged` state.
        pub waterlogged : bool,
    }
    impl super::Block for OxidizedCopperGrate {
    }
    impl From<OxidizedCopperGrate> for super::BlockState {
        fn from(value : OxidizedCopperGrate) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:enchanting_table` block.
pub mod enchanting_table {
    /// `minecraft:enchanting_table` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct EnchantingTable;
    impl super::Block for EnchantingTable {
    }
    impl From<EnchantingTable> for super::BlockState {
        fn from(value : EnchantingTable) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:kelp_plant` block.
pub mod kelp_plant {
    /// `minecraft:kelp_plant` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct KelpPlant;
    impl super::Block for KelpPlant {
    }
    impl From<KelpPlant> for super::BlockState {
        fn from(value : KelpPlant) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:light_blue_bed` block.
pub mod light_blue_bed {
    /// `minecraft:light_blue_bed` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct LightBlueBed {
        /// `occupied` state.
        pub occupied : bool,
        /// `facing` state.
        pub facing : Facing,
        /// `part` state.
        pub part : Part,
    }
    /// `facing` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Facing {
        /// `north` variant.
        North,
        /// `south` variant.
        South,
        /// `west` variant.
        West,
        /// `east` variant.
        East,
    }
    /// `part` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Part {
        /// `head` variant.
        Head,
        /// `foot` variant.
        Foot,
    }
    impl super::Block for LightBlueBed {
    }
    impl From<LightBlueBed> for super::BlockState {
        fn from(value : LightBlueBed) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:red_sandstone_wall` block.
pub mod red_sandstone_wall {
    /// `minecraft:red_sandstone_wall` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct RedSandstoneWall {
        /// `west` state.
        pub west : West,
        /// `up` state.
        pub up : bool,
        /// `east` state.
        pub east : East,
        /// `north` state.
        pub north : North,
        /// `south` state.
        pub south : South,
        /// `waterlogged` state.
        pub waterlogged : bool,
    }
    /// `west` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum West {
        /// `none` variant.
        None,
        /// `low` variant.
        Low,
        /// `tall` variant.
        Tall,
    }
    /// `east` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum East {
        /// `none` variant.
        None,
        /// `low` variant.
        Low,
        /// `tall` variant.
        Tall,
    }
    /// `north` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum North {
        /// `none` variant.
        None,
        /// `low` variant.
        Low,
        /// `tall` variant.
        Tall,
    }
    /// `south` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum South {
        /// `none` variant.
        None,
        /// `low` variant.
        Low,
        /// `tall` variant.
        Tall,
    }
    impl super::Block for RedSandstoneWall {
    }
    impl From<RedSandstoneWall> for super::BlockState {
        fn from(value : RedSandstoneWall) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:cyan_banner` block.
pub mod cyan_banner {
    /// `minecraft:cyan_banner` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct CyanBanner {
        /// `rotation` state.
        pub rotation : Rotation,
    }
    /// `rotation` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Rotation {
        /// `0` variant.
        N0,
        /// `1` variant.
        N1,
        /// `2` variant.
        N2,
        /// `3` variant.
        N3,
        /// `4` variant.
        N4,
        /// `5` variant.
        N5,
        /// `6` variant.
        N6,
        /// `7` variant.
        N7,
        /// `8` variant.
        N8,
        /// `9` variant.
        N9,
        /// `10` variant.
        N10,
        /// `11` variant.
        N11,
        /// `12` variant.
        N12,
        /// `13` variant.
        N13,
        /// `14` variant.
        N14,
        /// `15` variant.
        N15,
    }
    impl super::Block for CyanBanner {
    }
    impl From<CyanBanner> for super::BlockState {
        fn from(value : CyanBanner) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:chiseled_tuff` block.
pub mod chiseled_tuff {
    /// `minecraft:chiseled_tuff` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct ChiseledTuff;
    impl super::Block for ChiseledTuff {
    }
    impl From<ChiseledTuff> for super::BlockState {
        fn from(value : ChiseledTuff) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:red_mushroom_block` block.
pub mod red_mushroom_block {
    /// `minecraft:red_mushroom_block` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct RedMushroomBlock {
        /// `north` state.
        pub north : bool,
        /// `south` state.
        pub south : bool,
        /// `down` state.
        pub down : bool,
        /// `east` state.
        pub east : bool,
        /// `up` state.
        pub up : bool,
        /// `west` state.
        pub west : bool,
    }
    impl super::Block for RedMushroomBlock {
    }
    impl From<RedMushroomBlock> for super::BlockState {
        fn from(value : RedMushroomBlock) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:chiseled_polished_blackstone` block.
pub mod chiseled_polished_blackstone {
    /// `minecraft:chiseled_polished_blackstone` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct ChiseledPolishedBlackstone;
    impl super::Block for ChiseledPolishedBlackstone {
    }
    impl From<ChiseledPolishedBlackstone> for super::BlockState {
        fn from(value : ChiseledPolishedBlackstone) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:pointed_dripstone` block.
pub mod pointed_dripstone {
    /// `minecraft:pointed_dripstone` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct PointedDripstone {
        /// `vertical_direction` state.
        pub vertical_direction : VerticalDirection,
        /// `waterlogged` state.
        pub waterlogged : bool,
        /// `thickness` state.
        pub thickness : Thickness,
    }
    /// `vertical_direction` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum VerticalDirection {
        /// `up` variant.
        Up,
        /// `down` variant.
        Down,
    }
    /// `thickness` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Thickness {
        /// `tip_merge` variant.
        TipMerge,
        /// `tip` variant.
        Tip,
        /// `frustum` variant.
        Frustum,
        /// `middle` variant.
        Middle,
        /// `base` variant.
        Base,
    }
    impl super::Block for PointedDripstone {
    }
    impl From<PointedDripstone> for super::BlockState {
        fn from(value : PointedDripstone) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:pale_oak_planks` block.
pub mod pale_oak_planks {
    /// `minecraft:pale_oak_planks` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct PaleOakPlanks;
    impl super::Block for PaleOakPlanks {
    }
    impl From<PaleOakPlanks> for super::BlockState {
        fn from(value : PaleOakPlanks) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:potted_wither_rose` block.
pub mod potted_wither_rose {
    /// `minecraft:potted_wither_rose` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct PottedWitherRose;
    impl super::Block for PottedWitherRose {
    }
    impl From<PottedWitherRose> for super::BlockState {
        fn from(value : PottedWitherRose) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:warped_pressure_plate` block.
pub mod warped_pressure_plate {
    /// `minecraft:warped_pressure_plate` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct WarpedPressurePlate {
        /// `powered` state.
        pub powered : bool,
    }
    impl super::Block for WarpedPressurePlate {
    }
    impl From<WarpedPressurePlate> for super::BlockState {
        fn from(value : WarpedPressurePlate) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:end_stone_bricks` block.
pub mod end_stone_bricks {
    /// `minecraft:end_stone_bricks` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct EndStoneBricks;
    impl super::Block for EndStoneBricks {
    }
    impl From<EndStoneBricks> for super::BlockState {
        fn from(value : EndStoneBricks) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:polished_andesite_stairs` block.
pub mod polished_andesite_stairs {
    /// `minecraft:polished_andesite_stairs` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct PolishedAndesiteStairs {
        /// `half` state.
        pub half : Half,
        /// `facing` state.
        pub facing : Facing,
        /// `waterlogged` state.
        pub waterlogged : bool,
        /// `shape` state.
        pub shape : Shape,
    }
    /// `half` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Half {
        /// `top` variant.
        Top,
        /// `bottom` variant.
        Bottom,
    }
    /// `facing` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Facing {
        /// `north` variant.
        North,
        /// `south` variant.
        South,
        /// `west` variant.
        West,
        /// `east` variant.
        East,
    }
    /// `shape` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Shape {
        /// `straight` variant.
        Straight,
        /// `inner_left` variant.
        InnerLeft,
        /// `inner_right` variant.
        InnerRight,
        /// `outer_left` variant.
        OuterLeft,
        /// `outer_right` variant.
        OuterRight,
    }
    impl super::Block for PolishedAndesiteStairs {
    }
    impl From<PolishedAndesiteStairs> for super::BlockState {
        fn from(value : PolishedAndesiteStairs) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:resin_brick_stairs` block.
pub mod resin_brick_stairs {
    /// `minecraft:resin_brick_stairs` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct ResinBrickStairs {
        /// `half` state.
        pub half : Half,
        /// `waterlogged` state.
        pub waterlogged : bool,
        /// `shape` state.
        pub shape : Shape,
        /// `facing` state.
        pub facing : Facing,
    }
    /// `half` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Half {
        /// `top` variant.
        Top,
        /// `bottom` variant.
        Bottom,
    }
    /// `shape` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Shape {
        /// `straight` variant.
        Straight,
        /// `inner_left` variant.
        InnerLeft,
        /// `inner_right` variant.
        InnerRight,
        /// `outer_left` variant.
        OuterLeft,
        /// `outer_right` variant.
        OuterRight,
    }
    /// `facing` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Facing {
        /// `north` variant.
        North,
        /// `south` variant.
        South,
        /// `west` variant.
        West,
        /// `east` variant.
        East,
    }
    impl super::Block for ResinBrickStairs {
    }
    impl From<ResinBrickStairs> for super::BlockState {
        fn from(value : ResinBrickStairs) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:dark_oak_fence_gate` block.
pub mod dark_oak_fence_gate {
    /// `minecraft:dark_oak_fence_gate` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct DarkOakFenceGate {
        /// `facing` state.
        pub facing : Facing,
        /// `open` state.
        pub open : bool,
        /// `powered` state.
        pub powered : bool,
        /// `in_wall` state.
        pub in_wall : bool,
    }
    /// `facing` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Facing {
        /// `north` variant.
        North,
        /// `south` variant.
        South,
        /// `west` variant.
        West,
        /// `east` variant.
        East,
    }
    impl super::Block for DarkOakFenceGate {
    }
    impl From<DarkOakFenceGate> for super::BlockState {
        fn from(value : DarkOakFenceGate) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:large_amethyst_bud` block.
pub mod large_amethyst_bud {
    /// `minecraft:large_amethyst_bud` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct LargeAmethystBud {
        /// `waterlogged` state.
        pub waterlogged : bool,
        /// `facing` state.
        pub facing : Facing,
    }
    /// `facing` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Facing {
        /// `north` variant.
        North,
        /// `east` variant.
        East,
        /// `south` variant.
        South,
        /// `west` variant.
        West,
        /// `up` variant.
        Up,
        /// `down` variant.
        Down,
    }
    impl super::Block for LargeAmethystBud {
    }
    impl From<LargeAmethystBud> for super::BlockState {
        fn from(value : LargeAmethystBud) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:mangrove_fence` block.
pub mod mangrove_fence {
    /// `minecraft:mangrove_fence` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct MangroveFence {
        /// `west` state.
        pub west : bool,
        /// `east` state.
        pub east : bool,
        /// `north` state.
        pub north : bool,
        /// `south` state.
        pub south : bool,
        /// `waterlogged` state.
        pub waterlogged : bool,
    }
    impl super::Block for MangroveFence {
    }
    impl From<MangroveFence> for super::BlockState {
        fn from(value : MangroveFence) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:closed_eyeblossom` block.
pub mod closed_eyeblossom {
    /// `minecraft:closed_eyeblossom` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct ClosedEyeblossom;
    impl super::Block for ClosedEyeblossom {
    }
    impl From<ClosedEyeblossom> for super::BlockState {
        fn from(value : ClosedEyeblossom) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:orange_candle_cake` block.
pub mod orange_candle_cake {
    /// `minecraft:orange_candle_cake` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct OrangeCandleCake {
        /// `lit` state.
        pub lit : bool,
    }
    impl super::Block for OrangeCandleCake {
    }
    impl From<OrangeCandleCake> for super::BlockState {
        fn from(value : OrangeCandleCake) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:quartz_bricks` block.
pub mod quartz_bricks {
    /// `minecraft:quartz_bricks` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct QuartzBricks;
    impl super::Block for QuartzBricks {
    }
    impl From<QuartzBricks> for super::BlockState {
        fn from(value : QuartzBricks) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:skeleton_skull` block.
pub mod skeleton_skull {
    /// `minecraft:skeleton_skull` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct SkeletonSkull {
        /// `powered` state.
        pub powered : bool,
        /// `rotation` state.
        pub rotation : Rotation,
    }
    /// `rotation` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Rotation {
        /// `0` variant.
        N0,
        /// `1` variant.
        N1,
        /// `2` variant.
        N2,
        /// `3` variant.
        N3,
        /// `4` variant.
        N4,
        /// `5` variant.
        N5,
        /// `6` variant.
        N6,
        /// `7` variant.
        N7,
        /// `8` variant.
        N8,
        /// `9` variant.
        N9,
        /// `10` variant.
        N10,
        /// `11` variant.
        N11,
        /// `12` variant.
        N12,
        /// `13` variant.
        N13,
        /// `14` variant.
        N14,
        /// `15` variant.
        N15,
    }
    impl super::Block for SkeletonSkull {
    }
    impl From<SkeletonSkull> for super::BlockState {
        fn from(value : SkeletonSkull) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:birch_sign` block.
pub mod birch_sign {
    /// `minecraft:birch_sign` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct BirchSign {
        /// `waterlogged` state.
        pub waterlogged : bool,
        /// `rotation` state.
        pub rotation : Rotation,
    }
    /// `rotation` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Rotation {
        /// `0` variant.
        N0,
        /// `1` variant.
        N1,
        /// `2` variant.
        N2,
        /// `3` variant.
        N3,
        /// `4` variant.
        N4,
        /// `5` variant.
        N5,
        /// `6` variant.
        N6,
        /// `7` variant.
        N7,
        /// `8` variant.
        N8,
        /// `9` variant.
        N9,
        /// `10` variant.
        N10,
        /// `11` variant.
        N11,
        /// `12` variant.
        N12,
        /// `13` variant.
        N13,
        /// `14` variant.
        N14,
        /// `15` variant.
        N15,
    }
    impl super::Block for BirchSign {
    }
    impl From<BirchSign> for super::BlockState {
        fn from(value : BirchSign) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:oak_wall_sign` block.
pub mod oak_wall_sign {
    /// `minecraft:oak_wall_sign` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct OakWallSign {
        /// `waterlogged` state.
        pub waterlogged : bool,
        /// `facing` state.
        pub facing : Facing,
    }
    /// `facing` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Facing {
        /// `north` variant.
        North,
        /// `south` variant.
        South,
        /// `west` variant.
        West,
        /// `east` variant.
        East,
    }
    impl super::Block for OakWallSign {
    }
    impl From<OakWallSign> for super::BlockState {
        fn from(value : OakWallSign) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:jungle_sapling` block.
pub mod jungle_sapling {
    /// `minecraft:jungle_sapling` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct JungleSapling {
        /// `stage` state.
        pub stage : Stage,
    }
    /// `stage` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Stage {
        /// `0` variant.
        N0,
        /// `1` variant.
        N1,
    }
    impl super::Block for JungleSapling {
    }
    impl From<JungleSapling> for super::BlockState {
        fn from(value : JungleSapling) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:orange_tulip` block.
pub mod orange_tulip {
    /// `minecraft:orange_tulip` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct OrangeTulip;
    impl super::Block for OrangeTulip {
    }
    impl From<OrangeTulip> for super::BlockState {
        fn from(value : OrangeTulip) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:soul_sand` block.
pub mod soul_sand {
    /// `minecraft:soul_sand` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct SoulSand;
    impl super::Block for SoulSand {
    }
    impl From<SoulSand> for super::BlockState {
        fn from(value : SoulSand) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:cherry_wood` block.
pub mod cherry_wood {
    /// `minecraft:cherry_wood` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct CherryWood {
        /// `axis` state.
        pub axis : Axis,
    }
    /// `axis` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Axis {
        /// `x` variant.
        X,
        /// `y` variant.
        Y,
        /// `z` variant.
        Z,
    }
    impl super::Block for CherryWood {
    }
    impl From<CherryWood> for super::BlockState {
        fn from(value : CherryWood) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:white_concrete_powder` block.
pub mod white_concrete_powder {
    /// `minecraft:white_concrete_powder` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct WhiteConcretePowder;
    impl super::Block for WhiteConcretePowder {
    }
    impl From<WhiteConcretePowder> for super::BlockState {
        fn from(value : WhiteConcretePowder) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:green_concrete` block.
pub mod green_concrete {
    /// `minecraft:green_concrete` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct GreenConcrete;
    impl super::Block for GreenConcrete {
    }
    impl From<GreenConcrete> for super::BlockState {
        fn from(value : GreenConcrete) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:oak_sign` block.
pub mod oak_sign {
    /// `minecraft:oak_sign` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct OakSign {
        /// `rotation` state.
        pub rotation : Rotation,
        /// `waterlogged` state.
        pub waterlogged : bool,
    }
    /// `rotation` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Rotation {
        /// `0` variant.
        N0,
        /// `1` variant.
        N1,
        /// `2` variant.
        N2,
        /// `3` variant.
        N3,
        /// `4` variant.
        N4,
        /// `5` variant.
        N5,
        /// `6` variant.
        N6,
        /// `7` variant.
        N7,
        /// `8` variant.
        N8,
        /// `9` variant.
        N9,
        /// `10` variant.
        N10,
        /// `11` variant.
        N11,
        /// `12` variant.
        N12,
        /// `13` variant.
        N13,
        /// `14` variant.
        N14,
        /// `15` variant.
        N15,
    }
    impl super::Block for OakSign {
    }
    impl From<OakSign> for super::BlockState {
        fn from(value : OakSign) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:firefly_bush` block.
pub mod firefly_bush {
    /// `minecraft:firefly_bush` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct FireflyBush;
    impl super::Block for FireflyBush {
    }
    impl From<FireflyBush> for super::BlockState {
        fn from(value : FireflyBush) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:bamboo_slab` block.
pub mod bamboo_slab {
    /// `minecraft:bamboo_slab` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct BambooSlab {
        /// `type` state.
        pub kind : Kind,
        /// `waterlogged` state.
        pub waterlogged : bool,
    }
    /// `type` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Kind {
        /// `top` variant.
        Top,
        /// `bottom` variant.
        Bottom,
        /// `double` variant.
        Double,
    }
    impl super::Block for BambooSlab {
    }
    impl From<BambooSlab> for super::BlockState {
        fn from(value : BambooSlab) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:lapis_block` block.
pub mod lapis_block {
    /// `minecraft:lapis_block` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct LapisBlock;
    impl super::Block for LapisBlock {
    }
    impl From<LapisBlock> for super::BlockState {
        fn from(value : LapisBlock) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:brown_mushroom_block` block.
pub mod brown_mushroom_block {
    /// `minecraft:brown_mushroom_block` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct BrownMushroomBlock {
        /// `up` state.
        pub up : bool,
        /// `south` state.
        pub south : bool,
        /// `down` state.
        pub down : bool,
        /// `east` state.
        pub east : bool,
        /// `north` state.
        pub north : bool,
        /// `west` state.
        pub west : bool,
    }
    impl super::Block for BrownMushroomBlock {
    }
    impl From<BrownMushroomBlock> for super::BlockState {
        fn from(value : BrownMushroomBlock) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:polished_diorite_slab` block.
pub mod polished_diorite_slab {
    /// `minecraft:polished_diorite_slab` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct PolishedDioriteSlab {
        /// `type` state.
        pub kind : Kind,
        /// `waterlogged` state.
        pub waterlogged : bool,
    }
    /// `type` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Kind {
        /// `top` variant.
        Top,
        /// `bottom` variant.
        Bottom,
        /// `double` variant.
        Double,
    }
    impl super::Block for PolishedDioriteSlab {
    }
    impl From<PolishedDioriteSlab> for super::BlockState {
        fn from(value : PolishedDioriteSlab) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:polished_diorite_stairs` block.
pub mod polished_diorite_stairs {
    /// `minecraft:polished_diorite_stairs` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct PolishedDioriteStairs {
        /// `waterlogged` state.
        pub waterlogged : bool,
        /// `shape` state.
        pub shape : Shape,
        /// `half` state.
        pub half : Half,
        /// `facing` state.
        pub facing : Facing,
    }
    /// `shape` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Shape {
        /// `straight` variant.
        Straight,
        /// `inner_left` variant.
        InnerLeft,
        /// `inner_right` variant.
        InnerRight,
        /// `outer_left` variant.
        OuterLeft,
        /// `outer_right` variant.
        OuterRight,
    }
    /// `half` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Half {
        /// `top` variant.
        Top,
        /// `bottom` variant.
        Bottom,
    }
    /// `facing` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Facing {
        /// `north` variant.
        North,
        /// `south` variant.
        South,
        /// `west` variant.
        West,
        /// `east` variant.
        East,
    }
    impl super::Block for PolishedDioriteStairs {
    }
    impl From<PolishedDioriteStairs> for super::BlockState {
        fn from(value : PolishedDioriteStairs) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:dark_oak_door` block.
pub mod dark_oak_door {
    /// `minecraft:dark_oak_door` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct DarkOakDoor {
        /// `half` state.
        pub half : Half,
        /// `facing` state.
        pub facing : Facing,
        /// `hinge` state.
        pub hinge : Hinge,
        /// `powered` state.
        pub powered : bool,
        /// `open` state.
        pub open : bool,
    }
    /// `half` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Half {
        /// `upper` variant.
        Upper,
        /// `lower` variant.
        Lower,
    }
    /// `facing` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Facing {
        /// `north` variant.
        North,
        /// `south` variant.
        South,
        /// `west` variant.
        West,
        /// `east` variant.
        East,
    }
    /// `hinge` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Hinge {
        /// `left` variant.
        Left,
        /// `right` variant.
        Right,
    }
    impl super::Block for DarkOakDoor {
    }
    impl From<DarkOakDoor> for super::BlockState {
        fn from(value : DarkOakDoor) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:mossy_stone_brick_stairs` block.
pub mod mossy_stone_brick_stairs {
    /// `minecraft:mossy_stone_brick_stairs` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct MossyStoneBrickStairs {
        /// `half` state.
        pub half : Half,
        /// `shape` state.
        pub shape : Shape,
        /// `facing` state.
        pub facing : Facing,
        /// `waterlogged` state.
        pub waterlogged : bool,
    }
    /// `half` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Half {
        /// `top` variant.
        Top,
        /// `bottom` variant.
        Bottom,
    }
    /// `shape` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Shape {
        /// `straight` variant.
        Straight,
        /// `inner_left` variant.
        InnerLeft,
        /// `inner_right` variant.
        InnerRight,
        /// `outer_left` variant.
        OuterLeft,
        /// `outer_right` variant.
        OuterRight,
    }
    /// `facing` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Facing {
        /// `north` variant.
        North,
        /// `south` variant.
        South,
        /// `west` variant.
        West,
        /// `east` variant.
        East,
    }
    impl super::Block for MossyStoneBrickStairs {
    }
    impl From<MossyStoneBrickStairs> for super::BlockState {
        fn from(value : MossyStoneBrickStairs) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:oak_sapling` block.
pub mod oak_sapling {
    /// `minecraft:oak_sapling` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct OakSapling {
        /// `stage` state.
        pub stage : Stage,
    }
    /// `stage` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Stage {
        /// `0` variant.
        N0,
        /// `1` variant.
        N1,
    }
    impl super::Block for OakSapling {
    }
    impl From<OakSapling> for super::BlockState {
        fn from(value : OakSapling) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:magenta_candle_cake` block.
pub mod magenta_candle_cake {
    /// `minecraft:magenta_candle_cake` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct MagentaCandleCake {
        /// `lit` state.
        pub lit : bool,
    }
    impl super::Block for MagentaCandleCake {
    }
    impl From<MagentaCandleCake> for super::BlockState {
        fn from(value : MagentaCandleCake) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:black_carpet` block.
pub mod black_carpet {
    /// `minecraft:black_carpet` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct BlackCarpet;
    impl super::Block for BlackCarpet {
    }
    impl From<BlackCarpet> for super::BlockState {
        fn from(value : BlackCarpet) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:blue_banner` block.
pub mod blue_banner {
    /// `minecraft:blue_banner` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct BlueBanner {
        /// `rotation` state.
        pub rotation : Rotation,
    }
    /// `rotation` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Rotation {
        /// `0` variant.
        N0,
        /// `1` variant.
        N1,
        /// `2` variant.
        N2,
        /// `3` variant.
        N3,
        /// `4` variant.
        N4,
        /// `5` variant.
        N5,
        /// `6` variant.
        N6,
        /// `7` variant.
        N7,
        /// `8` variant.
        N8,
        /// `9` variant.
        N9,
        /// `10` variant.
        N10,
        /// `11` variant.
        N11,
        /// `12` variant.
        N12,
        /// `13` variant.
        N13,
        /// `14` variant.
        N14,
        /// `15` variant.
        N15,
    }
    impl super::Block for BlueBanner {
    }
    impl From<BlueBanner> for super::BlockState {
        fn from(value : BlueBanner) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:iron_trapdoor` block.
pub mod iron_trapdoor {
    /// `minecraft:iron_trapdoor` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct IronTrapdoor {
        /// `open` state.
        pub open : bool,
        /// `powered` state.
        pub powered : bool,
        /// `waterlogged` state.
        pub waterlogged : bool,
        /// `half` state.
        pub half : Half,
        /// `facing` state.
        pub facing : Facing,
    }
    /// `half` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Half {
        /// `top` variant.
        Top,
        /// `bottom` variant.
        Bottom,
    }
    /// `facing` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Facing {
        /// `north` variant.
        North,
        /// `south` variant.
        South,
        /// `west` variant.
        West,
        /// `east` variant.
        East,
    }
    impl super::Block for IronTrapdoor {
    }
    impl From<IronTrapdoor> for super::BlockState {
        fn from(value : IronTrapdoor) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:cut_red_sandstone_slab` block.
pub mod cut_red_sandstone_slab {
    /// `minecraft:cut_red_sandstone_slab` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct CutRedSandstoneSlab {
        /// `waterlogged` state.
        pub waterlogged : bool,
        /// `type` state.
        pub kind : Kind,
    }
    /// `type` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Kind {
        /// `top` variant.
        Top,
        /// `bottom` variant.
        Bottom,
        /// `double` variant.
        Double,
    }
    impl super::Block for CutRedSandstoneSlab {
    }
    impl From<CutRedSandstoneSlab> for super::BlockState {
        fn from(value : CutRedSandstoneSlab) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:green_concrete_powder` block.
pub mod green_concrete_powder {
    /// `minecraft:green_concrete_powder` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct GreenConcretePowder;
    impl super::Block for GreenConcretePowder {
    }
    impl From<GreenConcretePowder> for super::BlockState {
        fn from(value : GreenConcretePowder) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:dried_kelp_block` block.
pub mod dried_kelp_block {
    /// `minecraft:dried_kelp_block` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct DriedKelpBlock;
    impl super::Block for DriedKelpBlock {
    }
    impl From<DriedKelpBlock> for super::BlockState {
        fn from(value : DriedKelpBlock) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:yellow_stained_glass` block.
pub mod yellow_stained_glass {
    /// `minecraft:yellow_stained_glass` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct YellowStainedGlass;
    impl super::Block for YellowStainedGlass {
    }
    impl From<YellowStainedGlass> for super::BlockState {
        fn from(value : YellowStainedGlass) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:white_stained_glass_pane` block.
pub mod white_stained_glass_pane {
    /// `minecraft:white_stained_glass_pane` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct WhiteStainedGlassPane {
        /// `west` state.
        pub west : bool,
        /// `east` state.
        pub east : bool,
        /// `south` state.
        pub south : bool,
        /// `north` state.
        pub north : bool,
        /// `waterlogged` state.
        pub waterlogged : bool,
    }
    impl super::Block for WhiteStainedGlassPane {
    }
    impl From<WhiteStainedGlassPane> for super::BlockState {
        fn from(value : WhiteStainedGlassPane) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:white_wool` block.
pub mod white_wool {
    /// `minecraft:white_wool` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct WhiteWool;
    impl super::Block for WhiteWool {
    }
    impl From<WhiteWool> for super::BlockState {
        fn from(value : WhiteWool) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:blue_orchid` block.
pub mod blue_orchid {
    /// `minecraft:blue_orchid` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct BlueOrchid;
    impl super::Block for BlueOrchid {
    }
    impl From<BlueOrchid> for super::BlockState {
        fn from(value : BlueOrchid) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:deepslate_iron_ore` block.
pub mod deepslate_iron_ore {
    /// `minecraft:deepslate_iron_ore` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct DeepslateIronOre;
    impl super::Block for DeepslateIronOre {
    }
    impl From<DeepslateIronOre> for super::BlockState {
        fn from(value : DeepslateIronOre) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:smooth_sandstone` block.
pub mod smooth_sandstone {
    /// `minecraft:smooth_sandstone` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct SmoothSandstone;
    impl super::Block for SmoothSandstone {
    }
    impl From<SmoothSandstone> for super::BlockState {
        fn from(value : SmoothSandstone) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:grass_block` block.
pub mod grass_block {
    /// `minecraft:grass_block` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct GrassBlock {
        /// `snowy` state.
        pub snowy : bool,
    }
    impl super::Block for GrassBlock {
    }
    impl From<GrassBlock> for super::BlockState {
        fn from(value : GrassBlock) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:jungle_fence` block.
pub mod jungle_fence {
    /// `minecraft:jungle_fence` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct JungleFence {
        /// `east` state.
        pub east : bool,
        /// `west` state.
        pub west : bool,
        /// `waterlogged` state.
        pub waterlogged : bool,
        /// `south` state.
        pub south : bool,
        /// `north` state.
        pub north : bool,
    }
    impl super::Block for JungleFence {
    }
    impl From<JungleFence> for super::BlockState {
        fn from(value : JungleFence) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:cyan_wool` block.
pub mod cyan_wool {
    /// `minecraft:cyan_wool` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct CyanWool;
    impl super::Block for CyanWool {
    }
    impl From<CyanWool> for super::BlockState {
        fn from(value : CyanWool) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:cobweb` block.
pub mod cobweb {
    /// `minecraft:cobweb` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct Cobweb;
    impl super::Block for Cobweb {
    }
    impl From<Cobweb> for super::BlockState {
        fn from(value : Cobweb) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:brown_candle` block.
pub mod brown_candle {
    /// `minecraft:brown_candle` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct BrownCandle {
        /// `lit` state.
        pub lit : bool,
        /// `candles` state.
        pub candles : Candles,
        /// `waterlogged` state.
        pub waterlogged : bool,
    }
    /// `candles` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Candles {
        /// `1` variant.
        N1,
        /// `2` variant.
        N2,
        /// `3` variant.
        N3,
        /// `4` variant.
        N4,
    }
    impl super::Block for BrownCandle {
    }
    impl From<BrownCandle> for super::BlockState {
        fn from(value : BrownCandle) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:pale_oak_stairs` block.
pub mod pale_oak_stairs {
    /// `minecraft:pale_oak_stairs` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct PaleOakStairs {
        /// `facing` state.
        pub facing : Facing,
        /// `shape` state.
        pub shape : Shape,
        /// `waterlogged` state.
        pub waterlogged : bool,
        /// `half` state.
        pub half : Half,
    }
    /// `facing` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Facing {
        /// `north` variant.
        North,
        /// `south` variant.
        South,
        /// `west` variant.
        West,
        /// `east` variant.
        East,
    }
    /// `shape` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Shape {
        /// `straight` variant.
        Straight,
        /// `inner_left` variant.
        InnerLeft,
        /// `inner_right` variant.
        InnerRight,
        /// `outer_left` variant.
        OuterLeft,
        /// `outer_right` variant.
        OuterRight,
    }
    /// `half` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Half {
        /// `top` variant.
        Top,
        /// `bottom` variant.
        Bottom,
    }
    impl super::Block for PaleOakStairs {
    }
    impl From<PaleOakStairs> for super::BlockState {
        fn from(value : PaleOakStairs) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:quartz_pillar` block.
pub mod quartz_pillar {
    /// `minecraft:quartz_pillar` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct QuartzPillar {
        /// `axis` state.
        pub axis : Axis,
    }
    /// `axis` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Axis {
        /// `x` variant.
        X,
        /// `y` variant.
        Y,
        /// `z` variant.
        Z,
    }
    impl super::Block for QuartzPillar {
    }
    impl From<QuartzPillar> for super::BlockState {
        fn from(value : QuartzPillar) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:granite_slab` block.
pub mod granite_slab {
    /// `minecraft:granite_slab` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct GraniteSlab {
        /// `waterlogged` state.
        pub waterlogged : bool,
        /// `type` state.
        pub kind : Kind,
    }
    /// `type` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Kind {
        /// `top` variant.
        Top,
        /// `bottom` variant.
        Bottom,
        /// `double` variant.
        Double,
    }
    impl super::Block for GraniteSlab {
    }
    impl From<GraniteSlab> for super::BlockState {
        fn from(value : GraniteSlab) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:fire_coral` block.
pub mod fire_coral {
    /// `minecraft:fire_coral` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct FireCoral {
        /// `waterlogged` state.
        pub waterlogged : bool,
    }
    impl super::Block for FireCoral {
    }
    impl From<FireCoral> for super::BlockState {
        fn from(value : FireCoral) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:creaking_heart` block.
pub mod creaking_heart {
    /// `minecraft:creaking_heart` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct CreakingHeart {
        /// `axis` state.
        pub axis : Axis,
        /// `natural` state.
        pub natural : bool,
        /// `creaking_heart_state` state.
        pub creaking_heart_state : CreakingHeartState,
    }
    /// `axis` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Axis {
        /// `x` variant.
        X,
        /// `y` variant.
        Y,
        /// `z` variant.
        Z,
    }
    /// `creaking_heart_state` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum CreakingHeartState {
        /// `uprooted` variant.
        Uprooted,
        /// `dormant` variant.
        Dormant,
        /// `awake` variant.
        Awake,
    }
    impl super::Block for CreakingHeart {
    }
    impl From<CreakingHeart> for super::BlockState {
        fn from(value : CreakingHeart) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:infested_cobblestone` block.
pub mod infested_cobblestone {
    /// `minecraft:infested_cobblestone` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct InfestedCobblestone;
    impl super::Block for InfestedCobblestone {
    }
    impl From<InfestedCobblestone> for super::BlockState {
        fn from(value : InfestedCobblestone) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:tinted_glass` block.
pub mod tinted_glass {
    /// `minecraft:tinted_glass` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct TintedGlass;
    impl super::Block for TintedGlass {
    }
    impl From<TintedGlass> for super::BlockState {
        fn from(value : TintedGlass) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:oak_wall_hanging_sign` block.
pub mod oak_wall_hanging_sign {
    /// `minecraft:oak_wall_hanging_sign` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct OakWallHangingSign {
        /// `waterlogged` state.
        pub waterlogged : bool,
        /// `facing` state.
        pub facing : Facing,
    }
    /// `facing` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Facing {
        /// `north` variant.
        North,
        /// `south` variant.
        South,
        /// `west` variant.
        West,
        /// `east` variant.
        East,
    }
    impl super::Block for OakWallHangingSign {
    }
    impl From<OakWallHangingSign> for super::BlockState {
        fn from(value : OakWallHangingSign) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:yellow_glazed_terracotta` block.
pub mod yellow_glazed_terracotta {
    /// `minecraft:yellow_glazed_terracotta` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct YellowGlazedTerracotta {
        /// `facing` state.
        pub facing : Facing,
    }
    /// `facing` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Facing {
        /// `north` variant.
        North,
        /// `south` variant.
        South,
        /// `west` variant.
        West,
        /// `east` variant.
        East,
    }
    impl super::Block for YellowGlazedTerracotta {
    }
    impl From<YellowGlazedTerracotta> for super::BlockState {
        fn from(value : YellowGlazedTerracotta) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:mangrove_wall_hanging_sign` block.
pub mod mangrove_wall_hanging_sign {
    /// `minecraft:mangrove_wall_hanging_sign` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct MangroveWallHangingSign {
        /// `facing` state.
        pub facing : Facing,
        /// `waterlogged` state.
        pub waterlogged : bool,
    }
    /// `facing` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Facing {
        /// `north` variant.
        North,
        /// `south` variant.
        South,
        /// `west` variant.
        West,
        /// `east` variant.
        East,
    }
    impl super::Block for MangroveWallHangingSign {
    }
    impl From<MangroveWallHangingSign> for super::BlockState {
        fn from(value : MangroveWallHangingSign) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:pink_wall_banner` block.
pub mod pink_wall_banner {
    /// `minecraft:pink_wall_banner` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct PinkWallBanner {
        /// `facing` state.
        pub facing : Facing,
    }
    /// `facing` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Facing {
        /// `north` variant.
        North,
        /// `south` variant.
        South,
        /// `west` variant.
        West,
        /// `east` variant.
        East,
    }
    impl super::Block for PinkWallBanner {
    }
    impl From<PinkWallBanner> for super::BlockState {
        fn from(value : PinkWallBanner) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:jungle_fence_gate` block.
pub mod jungle_fence_gate {
    /// `minecraft:jungle_fence_gate` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct JungleFenceGate {
        /// `powered` state.
        pub powered : bool,
        /// `facing` state.
        pub facing : Facing,
        /// `open` state.
        pub open : bool,
        /// `in_wall` state.
        pub in_wall : bool,
    }
    /// `facing` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Facing {
        /// `north` variant.
        North,
        /// `south` variant.
        South,
        /// `west` variant.
        West,
        /// `east` variant.
        East,
    }
    impl super::Block for JungleFenceGate {
    }
    impl From<JungleFenceGate> for super::BlockState {
        fn from(value : JungleFenceGate) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:potted_crimson_fungus` block.
pub mod potted_crimson_fungus {
    /// `minecraft:potted_crimson_fungus` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct PottedCrimsonFungus;
    impl super::Block for PottedCrimsonFungus {
    }
    impl From<PottedCrimsonFungus> for super::BlockState {
        fn from(value : PottedCrimsonFungus) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:mangrove_roots` block.
pub mod mangrove_roots {
    /// `minecraft:mangrove_roots` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct MangroveRoots {
        /// `waterlogged` state.
        pub waterlogged : bool,
    }
    impl super::Block for MangroveRoots {
    }
    impl From<MangroveRoots> for super::BlockState {
        fn from(value : MangroveRoots) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:pink_bed` block.
pub mod pink_bed {
    /// `minecraft:pink_bed` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct PinkBed {
        /// `occupied` state.
        pub occupied : bool,
        /// `facing` state.
        pub facing : Facing,
        /// `part` state.
        pub part : Part,
    }
    /// `facing` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Facing {
        /// `north` variant.
        North,
        /// `south` variant.
        South,
        /// `west` variant.
        West,
        /// `east` variant.
        East,
    }
    /// `part` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Part {
        /// `head` variant.
        Head,
        /// `foot` variant.
        Foot,
    }
    impl super::Block for PinkBed {
    }
    impl From<PinkBed> for super::BlockState {
        fn from(value : PinkBed) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:infested_stone` block.
pub mod infested_stone {
    /// `minecraft:infested_stone` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct InfestedStone;
    impl super::Block for InfestedStone {
    }
    impl From<InfestedStone> for super::BlockState {
        fn from(value : InfestedStone) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:stone_button` block.
pub mod stone_button {
    /// `minecraft:stone_button` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct StoneButton {
        /// `face` state.
        pub face : Face,
        /// `facing` state.
        pub facing : Facing,
        /// `powered` state.
        pub powered : bool,
    }
    /// `face` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Face {
        /// `floor` variant.
        Floor,
        /// `wall` variant.
        Wall,
        /// `ceiling` variant.
        Ceiling,
    }
    /// `facing` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Facing {
        /// `north` variant.
        North,
        /// `south` variant.
        South,
        /// `west` variant.
        West,
        /// `east` variant.
        East,
    }
    impl super::Block for StoneButton {
    }
    impl From<StoneButton> for super::BlockState {
        fn from(value : StoneButton) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:green_carpet` block.
pub mod green_carpet {
    /// `minecraft:green_carpet` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct GreenCarpet;
    impl super::Block for GreenCarpet {
    }
    impl From<GreenCarpet> for super::BlockState {
        fn from(value : GreenCarpet) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:green_wall_banner` block.
pub mod green_wall_banner {
    /// `minecraft:green_wall_banner` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct GreenWallBanner {
        /// `facing` state.
        pub facing : Facing,
    }
    /// `facing` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Facing {
        /// `north` variant.
        North,
        /// `south` variant.
        South,
        /// `west` variant.
        West,
        /// `east` variant.
        East,
    }
    impl super::Block for GreenWallBanner {
    }
    impl From<GreenWallBanner> for super::BlockState {
        fn from(value : GreenWallBanner) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:waxed_exposed_copper_door` block.
pub mod waxed_exposed_copper_door {
    /// `minecraft:waxed_exposed_copper_door` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct WaxedExposedCopperDoor {
        /// `open` state.
        pub open : bool,
        /// `facing` state.
        pub facing : Facing,
        /// `powered` state.
        pub powered : bool,
        /// `half` state.
        pub half : Half,
        /// `hinge` state.
        pub hinge : Hinge,
    }
    /// `facing` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Facing {
        /// `north` variant.
        North,
        /// `south` variant.
        South,
        /// `west` variant.
        West,
        /// `east` variant.
        East,
    }
    /// `half` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Half {
        /// `upper` variant.
        Upper,
        /// `lower` variant.
        Lower,
    }
    /// `hinge` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Hinge {
        /// `left` variant.
        Left,
        /// `right` variant.
        Right,
    }
    impl super::Block for WaxedExposedCopperDoor {
    }
    impl From<WaxedExposedCopperDoor> for super::BlockState {
        fn from(value : WaxedExposedCopperDoor) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:lime_banner` block.
pub mod lime_banner {
    /// `minecraft:lime_banner` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct LimeBanner {
        /// `rotation` state.
        pub rotation : Rotation,
    }
    /// `rotation` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Rotation {
        /// `0` variant.
        N0,
        /// `1` variant.
        N1,
        /// `2` variant.
        N2,
        /// `3` variant.
        N3,
        /// `4` variant.
        N4,
        /// `5` variant.
        N5,
        /// `6` variant.
        N6,
        /// `7` variant.
        N7,
        /// `8` variant.
        N8,
        /// `9` variant.
        N9,
        /// `10` variant.
        N10,
        /// `11` variant.
        N11,
        /// `12` variant.
        N12,
        /// `13` variant.
        N13,
        /// `14` variant.
        N14,
        /// `15` variant.
        N15,
    }
    impl super::Block for LimeBanner {
    }
    impl From<LimeBanner> for super::BlockState {
        fn from(value : LimeBanner) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:deepslate_gold_ore` block.
pub mod deepslate_gold_ore {
    /// `minecraft:deepslate_gold_ore` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct DeepslateGoldOre;
    impl super::Block for DeepslateGoldOre {
    }
    impl From<DeepslateGoldOre> for super::BlockState {
        fn from(value : DeepslateGoldOre) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:polished_blackstone_brick_slab` block.
pub mod polished_blackstone_brick_slab {
    /// `minecraft:polished_blackstone_brick_slab` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct PolishedBlackstoneBrickSlab {
        /// `type` state.
        pub kind : Kind,
        /// `waterlogged` state.
        pub waterlogged : bool,
    }
    /// `type` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Kind {
        /// `top` variant.
        Top,
        /// `bottom` variant.
        Bottom,
        /// `double` variant.
        Double,
    }
    impl super::Block for PolishedBlackstoneBrickSlab {
    }
    impl From<PolishedBlackstoneBrickSlab> for super::BlockState {
        fn from(value : PolishedBlackstoneBrickSlab) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:crimson_planks` block.
pub mod crimson_planks {
    /// `minecraft:crimson_planks` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct CrimsonPlanks;
    impl super::Block for CrimsonPlanks {
    }
    impl From<CrimsonPlanks> for super::BlockState {
        fn from(value : CrimsonPlanks) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:red_wall_banner` block.
pub mod red_wall_banner {
    /// `minecraft:red_wall_banner` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct RedWallBanner {
        /// `facing` state.
        pub facing : Facing,
    }
    /// `facing` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Facing {
        /// `north` variant.
        North,
        /// `south` variant.
        South,
        /// `west` variant.
        West,
        /// `east` variant.
        East,
    }
    impl super::Block for RedWallBanner {
    }
    impl From<RedWallBanner> for super::BlockState {
        fn from(value : RedWallBanner) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:stonecutter` block.
pub mod stonecutter {
    /// `minecraft:stonecutter` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct Stonecutter {
        /// `facing` state.
        pub facing : Facing,
    }
    /// `facing` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Facing {
        /// `north` variant.
        North,
        /// `south` variant.
        South,
        /// `west` variant.
        West,
        /// `east` variant.
        East,
    }
    impl super::Block for Stonecutter {
    }
    impl From<Stonecutter> for super::BlockState {
        fn from(value : Stonecutter) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:magenta_glazed_terracotta` block.
pub mod magenta_glazed_terracotta {
    /// `minecraft:magenta_glazed_terracotta` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct MagentaGlazedTerracotta {
        /// `facing` state.
        pub facing : Facing,
    }
    /// `facing` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Facing {
        /// `north` variant.
        North,
        /// `south` variant.
        South,
        /// `west` variant.
        West,
        /// `east` variant.
        East,
    }
    impl super::Block for MagentaGlazedTerracotta {
    }
    impl From<MagentaGlazedTerracotta> for super::BlockState {
        fn from(value : MagentaGlazedTerracotta) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:beetroots` block.
pub mod beetroots {
    /// `minecraft:beetroots` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct Beetroots {
        /// `age` state.
        pub age : Age,
    }
    /// `age` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Age {
        /// `0` variant.
        N0,
        /// `1` variant.
        N1,
        /// `2` variant.
        N2,
        /// `3` variant.
        N3,
    }
    impl super::Block for Beetroots {
    }
    impl From<Beetroots> for super::BlockState {
        fn from(value : Beetroots) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:dragon_wall_head` block.
pub mod dragon_wall_head {
    /// `minecraft:dragon_wall_head` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct DragonWallHead {
        /// `facing` state.
        pub facing : Facing,
        /// `powered` state.
        pub powered : bool,
    }
    /// `facing` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Facing {
        /// `north` variant.
        North,
        /// `south` variant.
        South,
        /// `west` variant.
        West,
        /// `east` variant.
        East,
    }
    impl super::Block for DragonWallHead {
    }
    impl From<DragonWallHead> for super::BlockState {
        fn from(value : DragonWallHead) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:suspicious_gravel` block.
pub mod suspicious_gravel {
    /// `minecraft:suspicious_gravel` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct SuspiciousGravel {
        /// `dusted` state.
        pub dusted : Dusted,
    }
    /// `dusted` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Dusted {
        /// `0` variant.
        N0,
        /// `1` variant.
        N1,
        /// `2` variant.
        N2,
        /// `3` variant.
        N3,
    }
    impl super::Block for SuspiciousGravel {
    }
    impl From<SuspiciousGravel> for super::BlockState {
        fn from(value : SuspiciousGravel) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:snow_block` block.
pub mod snow_block {
    /// `minecraft:snow_block` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct SnowBlock;
    impl super::Block for SnowBlock {
    }
    impl From<SnowBlock> for super::BlockState {
        fn from(value : SnowBlock) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:yellow_carpet` block.
pub mod yellow_carpet {
    /// `minecraft:yellow_carpet` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct YellowCarpet;
    impl super::Block for YellowCarpet {
    }
    impl From<YellowCarpet> for super::BlockState {
        fn from(value : YellowCarpet) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:warped_wall_hanging_sign` block.
pub mod warped_wall_hanging_sign {
    /// `minecraft:warped_wall_hanging_sign` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct WarpedWallHangingSign {
        /// `facing` state.
        pub facing : Facing,
        /// `waterlogged` state.
        pub waterlogged : bool,
    }
    /// `facing` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Facing {
        /// `north` variant.
        North,
        /// `south` variant.
        South,
        /// `west` variant.
        West,
        /// `east` variant.
        East,
    }
    impl super::Block for WarpedWallHangingSign {
    }
    impl From<WarpedWallHangingSign> for super::BlockState {
        fn from(value : WarpedWallHangingSign) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:ice` block.
pub mod ice {
    /// `minecraft:ice` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct Ice;
    impl super::Block for Ice {
    }
    impl From<Ice> for super::BlockState {
        fn from(value : Ice) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:potted_acacia_sapling` block.
pub mod potted_acacia_sapling {
    /// `minecraft:potted_acacia_sapling` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct PottedAcaciaSapling;
    impl super::Block for PottedAcaciaSapling {
    }
    impl From<PottedAcaciaSapling> for super::BlockState {
        fn from(value : PottedAcaciaSapling) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:stone_pressure_plate` block.
pub mod stone_pressure_plate {
    /// `minecraft:stone_pressure_plate` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct StonePressurePlate {
        /// `powered` state.
        pub powered : bool,
    }
    impl super::Block for StonePressurePlate {
    }
    impl From<StonePressurePlate> for super::BlockState {
        fn from(value : StonePressurePlate) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:cobbled_deepslate` block.
pub mod cobbled_deepslate {
    /// `minecraft:cobbled_deepslate` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct CobbledDeepslate;
    impl super::Block for CobbledDeepslate {
    }
    impl From<CobbledDeepslate> for super::BlockState {
        fn from(value : CobbledDeepslate) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:lever` block.
pub mod lever {
    /// `minecraft:lever` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct Lever {
        /// `facing` state.
        pub facing : Facing,
        /// `powered` state.
        pub powered : bool,
        /// `face` state.
        pub face : Face,
    }
    /// `facing` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Facing {
        /// `north` variant.
        North,
        /// `south` variant.
        South,
        /// `west` variant.
        West,
        /// `east` variant.
        East,
    }
    /// `face` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Face {
        /// `floor` variant.
        Floor,
        /// `wall` variant.
        Wall,
        /// `ceiling` variant.
        Ceiling,
    }
    impl super::Block for Lever {
    }
    impl From<Lever> for super::BlockState {
        fn from(value : Lever) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:shulker_box` block.
pub mod shulker_box {
    /// `minecraft:shulker_box` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct ShulkerBox {
        /// `facing` state.
        pub facing : Facing,
    }
    /// `facing` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Facing {
        /// `north` variant.
        North,
        /// `east` variant.
        East,
        /// `south` variant.
        South,
        /// `west` variant.
        West,
        /// `up` variant.
        Up,
        /// `down` variant.
        Down,
    }
    impl super::Block for ShulkerBox {
    }
    impl From<ShulkerBox> for super::BlockState {
        fn from(value : ShulkerBox) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:oxidized_cut_copper` block.
pub mod oxidized_cut_copper {
    /// `minecraft:oxidized_cut_copper` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct OxidizedCutCopper;
    impl super::Block for OxidizedCutCopper {
    }
    impl From<OxidizedCutCopper> for super::BlockState {
        fn from(value : OxidizedCutCopper) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:soul_lantern` block.
pub mod soul_lantern {
    /// `minecraft:soul_lantern` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct SoulLantern {
        /// `waterlogged` state.
        pub waterlogged : bool,
        /// `hanging` state.
        pub hanging : bool,
    }
    impl super::Block for SoulLantern {
    }
    impl From<SoulLantern> for super::BlockState {
        fn from(value : SoulLantern) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:dead_bubble_coral_fan` block.
pub mod dead_bubble_coral_fan {
    /// `minecraft:dead_bubble_coral_fan` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct DeadBubbleCoralFan {
        /// `waterlogged` state.
        pub waterlogged : bool,
    }
    impl super::Block for DeadBubbleCoralFan {
    }
    impl From<DeadBubbleCoralFan> for super::BlockState {
        fn from(value : DeadBubbleCoralFan) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:glowstone` block.
pub mod glowstone {
    /// `minecraft:glowstone` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct Glowstone;
    impl super::Block for Glowstone {
    }
    impl From<Glowstone> for super::BlockState {
        fn from(value : Glowstone) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:infested_stone_bricks` block.
pub mod infested_stone_bricks {
    /// `minecraft:infested_stone_bricks` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct InfestedStoneBricks;
    impl super::Block for InfestedStoneBricks {
    }
    impl From<InfestedStoneBricks> for super::BlockState {
        fn from(value : InfestedStoneBricks) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:jungle_trapdoor` block.
pub mod jungle_trapdoor {
    /// `minecraft:jungle_trapdoor` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct JungleTrapdoor {
        /// `facing` state.
        pub facing : Facing,
        /// `waterlogged` state.
        pub waterlogged : bool,
        /// `half` state.
        pub half : Half,
        /// `open` state.
        pub open : bool,
        /// `powered` state.
        pub powered : bool,
    }
    /// `facing` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Facing {
        /// `north` variant.
        North,
        /// `south` variant.
        South,
        /// `west` variant.
        West,
        /// `east` variant.
        East,
    }
    /// `half` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Half {
        /// `top` variant.
        Top,
        /// `bottom` variant.
        Bottom,
    }
    impl super::Block for JungleTrapdoor {
    }
    impl From<JungleTrapdoor> for super::BlockState {
        fn from(value : JungleTrapdoor) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:mossy_stone_brick_slab` block.
pub mod mossy_stone_brick_slab {
    /// `minecraft:mossy_stone_brick_slab` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct MossyStoneBrickSlab {
        /// `waterlogged` state.
        pub waterlogged : bool,
        /// `type` state.
        pub kind : Kind,
    }
    /// `type` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Kind {
        /// `top` variant.
        Top,
        /// `bottom` variant.
        Bottom,
        /// `double` variant.
        Double,
    }
    impl super::Block for MossyStoneBrickSlab {
    }
    impl From<MossyStoneBrickSlab> for super::BlockState {
        fn from(value : MossyStoneBrickSlab) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:damaged_anvil` block.
pub mod damaged_anvil {
    /// `minecraft:damaged_anvil` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct DamagedAnvil {
        /// `facing` state.
        pub facing : Facing,
    }
    /// `facing` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Facing {
        /// `north` variant.
        North,
        /// `south` variant.
        South,
        /// `west` variant.
        West,
        /// `east` variant.
        East,
    }
    impl super::Block for DamagedAnvil {
    }
    impl From<DamagedAnvil> for super::BlockState {
        fn from(value : DamagedAnvil) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:jungle_wall_hanging_sign` block.
pub mod jungle_wall_hanging_sign {
    /// `minecraft:jungle_wall_hanging_sign` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct JungleWallHangingSign {
        /// `facing` state.
        pub facing : Facing,
        /// `waterlogged` state.
        pub waterlogged : bool,
    }
    /// `facing` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Facing {
        /// `north` variant.
        North,
        /// `south` variant.
        South,
        /// `west` variant.
        West,
        /// `east` variant.
        East,
    }
    impl super::Block for JungleWallHangingSign {
    }
    impl From<JungleWallHangingSign> for super::BlockState {
        fn from(value : JungleWallHangingSign) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:spruce_sapling` block.
pub mod spruce_sapling {
    /// `minecraft:spruce_sapling` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct SpruceSapling {
        /// `stage` state.
        pub stage : Stage,
    }
    /// `stage` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Stage {
        /// `0` variant.
        N0,
        /// `1` variant.
        N1,
    }
    impl super::Block for SpruceSapling {
    }
    impl From<SpruceSapling> for super::BlockState {
        fn from(value : SpruceSapling) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:attached_pumpkin_stem` block.
pub mod attached_pumpkin_stem {
    /// `minecraft:attached_pumpkin_stem` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct AttachedPumpkinStem {
        /// `facing` state.
        pub facing : Facing,
    }
    /// `facing` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Facing {
        /// `north` variant.
        North,
        /// `south` variant.
        South,
        /// `west` variant.
        West,
        /// `east` variant.
        East,
    }
    impl super::Block for AttachedPumpkinStem {
    }
    impl From<AttachedPumpkinStem> for super::BlockState {
        fn from(value : AttachedPumpkinStem) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:deepslate_brick_slab` block.
pub mod deepslate_brick_slab {
    /// `minecraft:deepslate_brick_slab` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct DeepslateBrickSlab {
        /// `type` state.
        pub kind : Kind,
        /// `waterlogged` state.
        pub waterlogged : bool,
    }
    /// `type` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Kind {
        /// `top` variant.
        Top,
        /// `bottom` variant.
        Bottom,
        /// `double` variant.
        Double,
    }
    impl super::Block for DeepslateBrickSlab {
    }
    impl From<DeepslateBrickSlab> for super::BlockState {
        fn from(value : DeepslateBrickSlab) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:stripped_crimson_hyphae` block.
pub mod stripped_crimson_hyphae {
    /// `minecraft:stripped_crimson_hyphae` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct StrippedCrimsonHyphae {
        /// `axis` state.
        pub axis : Axis,
    }
    /// `axis` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Axis {
        /// `x` variant.
        X,
        /// `y` variant.
        Y,
        /// `z` variant.
        Z,
    }
    impl super::Block for StrippedCrimsonHyphae {
    }
    impl From<StrippedCrimsonHyphae> for super::BlockState {
        fn from(value : StrippedCrimsonHyphae) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:purple_shulker_box` block.
pub mod purple_shulker_box {
    /// `minecraft:purple_shulker_box` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct PurpleShulkerBox {
        /// `facing` state.
        pub facing : Facing,
    }
    /// `facing` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Facing {
        /// `north` variant.
        North,
        /// `east` variant.
        East,
        /// `south` variant.
        South,
        /// `west` variant.
        West,
        /// `up` variant.
        Up,
        /// `down` variant.
        Down,
    }
    impl super::Block for PurpleShulkerBox {
    }
    impl From<PurpleShulkerBox> for super::BlockState {
        fn from(value : PurpleShulkerBox) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:copper_grate` block.
pub mod copper_grate {
    /// `minecraft:copper_grate` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct CopperGrate {
        /// `waterlogged` state.
        pub waterlogged : bool,
    }
    impl super::Block for CopperGrate {
    }
    impl From<CopperGrate> for super::BlockState {
        fn from(value : CopperGrate) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:light_blue_concrete_powder` block.
pub mod light_blue_concrete_powder {
    /// `minecraft:light_blue_concrete_powder` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct LightBlueConcretePowder;
    impl super::Block for LightBlueConcretePowder {
    }
    impl From<LightBlueConcretePowder> for super::BlockState {
        fn from(value : LightBlueConcretePowder) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:red_nether_brick_slab` block.
pub mod red_nether_brick_slab {
    /// `minecraft:red_nether_brick_slab` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct RedNetherBrickSlab {
        /// `type` state.
        pub kind : Kind,
        /// `waterlogged` state.
        pub waterlogged : bool,
    }
    /// `type` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Kind {
        /// `top` variant.
        Top,
        /// `bottom` variant.
        Bottom,
        /// `double` variant.
        Double,
    }
    impl super::Block for RedNetherBrickSlab {
    }
    impl From<RedNetherBrickSlab> for super::BlockState {
        fn from(value : RedNetherBrickSlab) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:small_amethyst_bud` block.
pub mod small_amethyst_bud {
    /// `minecraft:small_amethyst_bud` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct SmallAmethystBud {
        /// `waterlogged` state.
        pub waterlogged : bool,
        /// `facing` state.
        pub facing : Facing,
    }
    /// `facing` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Facing {
        /// `north` variant.
        North,
        /// `east` variant.
        East,
        /// `south` variant.
        South,
        /// `west` variant.
        West,
        /// `up` variant.
        Up,
        /// `down` variant.
        Down,
    }
    impl super::Block for SmallAmethystBud {
    }
    impl From<SmallAmethystBud> for super::BlockState {
        fn from(value : SmallAmethystBud) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:amethyst_cluster` block.
pub mod amethyst_cluster {
    /// `minecraft:amethyst_cluster` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct AmethystCluster {
        /// `facing` state.
        pub facing : Facing,
        /// `waterlogged` state.
        pub waterlogged : bool,
    }
    /// `facing` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Facing {
        /// `north` variant.
        North,
        /// `east` variant.
        East,
        /// `south` variant.
        South,
        /// `west` variant.
        West,
        /// `up` variant.
        Up,
        /// `down` variant.
        Down,
    }
    impl super::Block for AmethystCluster {
    }
    impl From<AmethystCluster> for super::BlockState {
        fn from(value : AmethystCluster) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:emerald_ore` block.
pub mod emerald_ore {
    /// `minecraft:emerald_ore` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct EmeraldOre;
    impl super::Block for EmeraldOre {
    }
    impl From<EmeraldOre> for super::BlockState {
        fn from(value : EmeraldOre) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:spruce_fence` block.
pub mod spruce_fence {
    /// `minecraft:spruce_fence` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct SpruceFence {
        /// `west` state.
        pub west : bool,
        /// `waterlogged` state.
        pub waterlogged : bool,
        /// `south` state.
        pub south : bool,
        /// `east` state.
        pub east : bool,
        /// `north` state.
        pub north : bool,
    }
    impl super::Block for SpruceFence {
    }
    impl From<SpruceFence> for super::BlockState {
        fn from(value : SpruceFence) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:open_eyeblossom` block.
pub mod open_eyeblossom {
    /// `minecraft:open_eyeblossom` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct OpenEyeblossom;
    impl super::Block for OpenEyeblossom {
    }
    impl From<OpenEyeblossom> for super::BlockState {
        fn from(value : OpenEyeblossom) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:spruce_hanging_sign` block.
pub mod spruce_hanging_sign {
    /// `minecraft:spruce_hanging_sign` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct SpruceHangingSign {
        /// `attached` state.
        pub attached : bool,
        /// `rotation` state.
        pub rotation : Rotation,
        /// `waterlogged` state.
        pub waterlogged : bool,
    }
    /// `rotation` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Rotation {
        /// `0` variant.
        N0,
        /// `1` variant.
        N1,
        /// `2` variant.
        N2,
        /// `3` variant.
        N3,
        /// `4` variant.
        N4,
        /// `5` variant.
        N5,
        /// `6` variant.
        N6,
        /// `7` variant.
        N7,
        /// `8` variant.
        N8,
        /// `9` variant.
        N9,
        /// `10` variant.
        N10,
        /// `11` variant.
        N11,
        /// `12` variant.
        N12,
        /// `13` variant.
        N13,
        /// `14` variant.
        N14,
        /// `15` variant.
        N15,
    }
    impl super::Block for SpruceHangingSign {
    }
    impl From<SpruceHangingSign> for super::BlockState {
        fn from(value : SpruceHangingSign) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:brown_carpet` block.
pub mod brown_carpet {
    /// `minecraft:brown_carpet` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct BrownCarpet;
    impl super::Block for BrownCarpet {
    }
    impl From<BrownCarpet> for super::BlockState {
        fn from(value : BrownCarpet) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:acacia_door` block.
pub mod acacia_door {
    /// `minecraft:acacia_door` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct AcaciaDoor {
        /// `facing` state.
        pub facing : Facing,
        /// `open` state.
        pub open : bool,
        /// `half` state.
        pub half : Half,
        /// `hinge` state.
        pub hinge : Hinge,
        /// `powered` state.
        pub powered : bool,
    }
    /// `facing` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Facing {
        /// `north` variant.
        North,
        /// `south` variant.
        South,
        /// `west` variant.
        West,
        /// `east` variant.
        East,
    }
    /// `half` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Half {
        /// `upper` variant.
        Upper,
        /// `lower` variant.
        Lower,
    }
    /// `hinge` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Hinge {
        /// `left` variant.
        Left,
        /// `right` variant.
        Right,
    }
    impl super::Block for AcaciaDoor {
    }
    impl From<AcaciaDoor> for super::BlockState {
        fn from(value : AcaciaDoor) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:ladder` block.
pub mod ladder {
    /// `minecraft:ladder` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct Ladder {
        /// `facing` state.
        pub facing : Facing,
        /// `waterlogged` state.
        pub waterlogged : bool,
    }
    /// `facing` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Facing {
        /// `north` variant.
        North,
        /// `south` variant.
        South,
        /// `west` variant.
        West,
        /// `east` variant.
        East,
    }
    impl super::Block for Ladder {
    }
    impl From<Ladder> for super::BlockState {
        fn from(value : Ladder) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:large_fern` block.
pub mod large_fern {
    /// `minecraft:large_fern` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct LargeFern {
        /// `half` state.
        pub half : Half,
    }
    /// `half` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Half {
        /// `upper` variant.
        Upper,
        /// `lower` variant.
        Lower,
    }
    impl super::Block for LargeFern {
    }
    impl From<LargeFern> for super::BlockState {
        fn from(value : LargeFern) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:pink_petals` block.
pub mod pink_petals {
    /// `minecraft:pink_petals` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct PinkPetals {
        /// `facing` state.
        pub facing : Facing,
        /// `flower_amount` state.
        pub flower_amount : FlowerAmount,
    }
    /// `facing` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Facing {
        /// `north` variant.
        North,
        /// `south` variant.
        South,
        /// `west` variant.
        West,
        /// `east` variant.
        East,
    }
    /// `flower_amount` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum FlowerAmount {
        /// `1` variant.
        N1,
        /// `2` variant.
        N2,
        /// `3` variant.
        N3,
        /// `4` variant.
        N4,
    }
    impl super::Block for PinkPetals {
    }
    impl From<PinkPetals> for super::BlockState {
        fn from(value : PinkPetals) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:polished_granite_slab` block.
pub mod polished_granite_slab {
    /// `minecraft:polished_granite_slab` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct PolishedGraniteSlab {
        /// `type` state.
        pub kind : Kind,
        /// `waterlogged` state.
        pub waterlogged : bool,
    }
    /// `type` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Kind {
        /// `top` variant.
        Top,
        /// `bottom` variant.
        Bottom,
        /// `double` variant.
        Double,
    }
    impl super::Block for PolishedGraniteSlab {
    }
    impl From<PolishedGraniteSlab> for super::BlockState {
        fn from(value : PolishedGraniteSlab) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:potted_brown_mushroom` block.
pub mod potted_brown_mushroom {
    /// `minecraft:potted_brown_mushroom` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct PottedBrownMushroom;
    impl super::Block for PottedBrownMushroom {
    }
    impl From<PottedBrownMushroom> for super::BlockState {
        fn from(value : PottedBrownMushroom) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:bamboo_trapdoor` block.
pub mod bamboo_trapdoor {
    /// `minecraft:bamboo_trapdoor` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct BambooTrapdoor {
        /// `half` state.
        pub half : Half,
        /// `facing` state.
        pub facing : Facing,
        /// `powered` state.
        pub powered : bool,
        /// `waterlogged` state.
        pub waterlogged : bool,
        /// `open` state.
        pub open : bool,
    }
    /// `half` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Half {
        /// `top` variant.
        Top,
        /// `bottom` variant.
        Bottom,
    }
    /// `facing` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Facing {
        /// `north` variant.
        North,
        /// `south` variant.
        South,
        /// `west` variant.
        West,
        /// `east` variant.
        East,
    }
    impl super::Block for BambooTrapdoor {
    }
    impl From<BambooTrapdoor> for super::BlockState {
        fn from(value : BambooTrapdoor) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:cobblestone_stairs` block.
pub mod cobblestone_stairs {
    /// `minecraft:cobblestone_stairs` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct CobblestoneStairs {
        /// `half` state.
        pub half : Half,
        /// `facing` state.
        pub facing : Facing,
        /// `shape` state.
        pub shape : Shape,
        /// `waterlogged` state.
        pub waterlogged : bool,
    }
    /// `half` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Half {
        /// `top` variant.
        Top,
        /// `bottom` variant.
        Bottom,
    }
    /// `facing` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Facing {
        /// `north` variant.
        North,
        /// `south` variant.
        South,
        /// `west` variant.
        West,
        /// `east` variant.
        East,
    }
    /// `shape` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Shape {
        /// `straight` variant.
        Straight,
        /// `inner_left` variant.
        InnerLeft,
        /// `inner_right` variant.
        InnerRight,
        /// `outer_left` variant.
        OuterLeft,
        /// `outer_right` variant.
        OuterRight,
    }
    impl super::Block for CobblestoneStairs {
    }
    impl From<CobblestoneStairs> for super::BlockState {
        fn from(value : CobblestoneStairs) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:light_gray_banner` block.
pub mod light_gray_banner {
    /// `minecraft:light_gray_banner` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct LightGrayBanner {
        /// `rotation` state.
        pub rotation : Rotation,
    }
    /// `rotation` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Rotation {
        /// `0` variant.
        N0,
        /// `1` variant.
        N1,
        /// `2` variant.
        N2,
        /// `3` variant.
        N3,
        /// `4` variant.
        N4,
        /// `5` variant.
        N5,
        /// `6` variant.
        N6,
        /// `7` variant.
        N7,
        /// `8` variant.
        N8,
        /// `9` variant.
        N9,
        /// `10` variant.
        N10,
        /// `11` variant.
        N11,
        /// `12` variant.
        N12,
        /// `13` variant.
        N13,
        /// `14` variant.
        N14,
        /// `15` variant.
        N15,
    }
    impl super::Block for LightGrayBanner {
    }
    impl From<LightGrayBanner> for super::BlockState {
        fn from(value : LightGrayBanner) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:cake` block.
pub mod cake {
    /// `minecraft:cake` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct Cake {
        /// `bites` state.
        pub bites : Bites,
    }
    /// `bites` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Bites {
        /// `0` variant.
        N0,
        /// `1` variant.
        N1,
        /// `2` variant.
        N2,
        /// `3` variant.
        N3,
        /// `4` variant.
        N4,
        /// `5` variant.
        N5,
        /// `6` variant.
        N6,
    }
    impl super::Block for Cake {
    }
    impl From<Cake> for super::BlockState {
        fn from(value : Cake) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:purpur_stairs` block.
pub mod purpur_stairs {
    /// `minecraft:purpur_stairs` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct PurpurStairs {
        /// `waterlogged` state.
        pub waterlogged : bool,
        /// `half` state.
        pub half : Half,
        /// `facing` state.
        pub facing : Facing,
        /// `shape` state.
        pub shape : Shape,
    }
    /// `half` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Half {
        /// `top` variant.
        Top,
        /// `bottom` variant.
        Bottom,
    }
    /// `facing` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Facing {
        /// `north` variant.
        North,
        /// `south` variant.
        South,
        /// `west` variant.
        West,
        /// `east` variant.
        East,
    }
    /// `shape` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Shape {
        /// `straight` variant.
        Straight,
        /// `inner_left` variant.
        InnerLeft,
        /// `inner_right` variant.
        InnerRight,
        /// `outer_left` variant.
        OuterLeft,
        /// `outer_right` variant.
        OuterRight,
    }
    impl super::Block for PurpurStairs {
    }
    impl From<PurpurStairs> for super::BlockState {
        fn from(value : PurpurStairs) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:white_banner` block.
pub mod white_banner {
    /// `minecraft:white_banner` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct WhiteBanner {
        /// `rotation` state.
        pub rotation : Rotation,
    }
    /// `rotation` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Rotation {
        /// `0` variant.
        N0,
        /// `1` variant.
        N1,
        /// `2` variant.
        N2,
        /// `3` variant.
        N3,
        /// `4` variant.
        N4,
        /// `5` variant.
        N5,
        /// `6` variant.
        N6,
        /// `7` variant.
        N7,
        /// `8` variant.
        N8,
        /// `9` variant.
        N9,
        /// `10` variant.
        N10,
        /// `11` variant.
        N11,
        /// `12` variant.
        N12,
        /// `13` variant.
        N13,
        /// `14` variant.
        N14,
        /// `15` variant.
        N15,
    }
    impl super::Block for WhiteBanner {
    }
    impl From<WhiteBanner> for super::BlockState {
        fn from(value : WhiteBanner) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:gray_concrete_powder` block.
pub mod gray_concrete_powder {
    /// `minecraft:gray_concrete_powder` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct GrayConcretePowder;
    impl super::Block for GrayConcretePowder {
    }
    impl From<GrayConcretePowder> for super::BlockState {
        fn from(value : GrayConcretePowder) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:gray_stained_glass_pane` block.
pub mod gray_stained_glass_pane {
    /// `minecraft:gray_stained_glass_pane` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct GrayStainedGlassPane {
        /// `waterlogged` state.
        pub waterlogged : bool,
        /// `west` state.
        pub west : bool,
        /// `east` state.
        pub east : bool,
        /// `north` state.
        pub north : bool,
        /// `south` state.
        pub south : bool,
    }
    impl super::Block for GrayStainedGlassPane {
    }
    impl From<GrayStainedGlassPane> for super::BlockState {
        fn from(value : GrayStainedGlassPane) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:black_wool` block.
pub mod black_wool {
    /// `minecraft:black_wool` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct BlackWool;
    impl super::Block for BlackWool {
    }
    impl From<BlackWool> for super::BlockState {
        fn from(value : BlackWool) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:light_gray_wall_banner` block.
pub mod light_gray_wall_banner {
    /// `minecraft:light_gray_wall_banner` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct LightGrayWallBanner {
        /// `facing` state.
        pub facing : Facing,
    }
    /// `facing` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Facing {
        /// `north` variant.
        North,
        /// `south` variant.
        South,
        /// `west` variant.
        West,
        /// `east` variant.
        East,
    }
    impl super::Block for LightGrayWallBanner {
    }
    impl From<LightGrayWallBanner> for super::BlockState {
        fn from(value : LightGrayWallBanner) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:small_dripleaf` block.
pub mod small_dripleaf {
    /// `minecraft:small_dripleaf` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct SmallDripleaf {
        /// `facing` state.
        pub facing : Facing,
        /// `waterlogged` state.
        pub waterlogged : bool,
        /// `half` state.
        pub half : Half,
    }
    /// `facing` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Facing {
        /// `north` variant.
        North,
        /// `south` variant.
        South,
        /// `west` variant.
        West,
        /// `east` variant.
        East,
    }
    /// `half` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Half {
        /// `upper` variant.
        Upper,
        /// `lower` variant.
        Lower,
    }
    impl super::Block for SmallDripleaf {
    }
    impl From<SmallDripleaf> for super::BlockState {
        fn from(value : SmallDripleaf) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:pink_tulip` block.
pub mod pink_tulip {
    /// `minecraft:pink_tulip` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct PinkTulip;
    impl super::Block for PinkTulip {
    }
    impl From<PinkTulip> for super::BlockState {
        fn from(value : PinkTulip) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:moss_block` block.
pub mod moss_block {
    /// `minecraft:moss_block` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct MossBlock;
    impl super::Block for MossBlock {
    }
    impl From<MossBlock> for super::BlockState {
        fn from(value : MossBlock) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:blue_terracotta` block.
pub mod blue_terracotta {
    /// `minecraft:blue_terracotta` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct BlueTerracotta;
    impl super::Block for BlueTerracotta {
    }
    impl From<BlueTerracotta> for super::BlockState {
        fn from(value : BlueTerracotta) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:weeping_vines_plant` block.
pub mod weeping_vines_plant {
    /// `minecraft:weeping_vines_plant` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct WeepingVinesPlant;
    impl super::Block for WeepingVinesPlant {
    }
    impl From<WeepingVinesPlant> for super::BlockState {
        fn from(value : WeepingVinesPlant) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:twisting_vines` block.
pub mod twisting_vines {
    /// `minecraft:twisting_vines` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct TwistingVines {
        /// `age` state.
        pub age : Age,
    }
    /// `age` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Age {
        /// `0` variant.
        N0,
        /// `1` variant.
        N1,
        /// `2` variant.
        N2,
        /// `3` variant.
        N3,
        /// `4` variant.
        N4,
        /// `5` variant.
        N5,
        /// `6` variant.
        N6,
        /// `7` variant.
        N7,
        /// `8` variant.
        N8,
        /// `9` variant.
        N9,
        /// `10` variant.
        N10,
        /// `11` variant.
        N11,
        /// `12` variant.
        N12,
        /// `13` variant.
        N13,
        /// `14` variant.
        N14,
        /// `15` variant.
        N15,
        /// `16` variant.
        N16,
        /// `17` variant.
        N17,
        /// `18` variant.
        N18,
        /// `19` variant.
        N19,
        /// `20` variant.
        N20,
        /// `21` variant.
        N21,
        /// `22` variant.
        N22,
        /// `23` variant.
        N23,
        /// `24` variant.
        N24,
        /// `25` variant.
        N25,
    }
    impl super::Block for TwistingVines {
    }
    impl From<TwistingVines> for super::BlockState {
        fn from(value : TwistingVines) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:nether_brick_stairs` block.
pub mod nether_brick_stairs {
    /// `minecraft:nether_brick_stairs` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct NetherBrickStairs {
        /// `waterlogged` state.
        pub waterlogged : bool,
        /// `facing` state.
        pub facing : Facing,
        /// `shape` state.
        pub shape : Shape,
        /// `half` state.
        pub half : Half,
    }
    /// `facing` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Facing {
        /// `north` variant.
        North,
        /// `south` variant.
        South,
        /// `west` variant.
        West,
        /// `east` variant.
        East,
    }
    /// `shape` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Shape {
        /// `straight` variant.
        Straight,
        /// `inner_left` variant.
        InnerLeft,
        /// `inner_right` variant.
        InnerRight,
        /// `outer_left` variant.
        OuterLeft,
        /// `outer_right` variant.
        OuterRight,
    }
    /// `half` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Half {
        /// `top` variant.
        Top,
        /// `bottom` variant.
        Bottom,
    }
    impl super::Block for NetherBrickStairs {
    }
    impl From<NetherBrickStairs> for super::BlockState {
        fn from(value : NetherBrickStairs) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:oxidized_copper_bulb` block.
pub mod oxidized_copper_bulb {
    /// `minecraft:oxidized_copper_bulb` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct OxidizedCopperBulb {
        /// `lit` state.
        pub lit : bool,
        /// `powered` state.
        pub powered : bool,
    }
    impl super::Block for OxidizedCopperBulb {
    }
    impl From<OxidizedCopperBulb> for super::BlockState {
        fn from(value : OxidizedCopperBulb) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:potted_oxeye_daisy` block.
pub mod potted_oxeye_daisy {
    /// `minecraft:potted_oxeye_daisy` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct PottedOxeyeDaisy;
    impl super::Block for PottedOxeyeDaisy {
    }
    impl From<PottedOxeyeDaisy> for super::BlockState {
        fn from(value : PottedOxeyeDaisy) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:potted_pink_tulip` block.
pub mod potted_pink_tulip {
    /// `minecraft:potted_pink_tulip` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct PottedPinkTulip;
    impl super::Block for PottedPinkTulip {
    }
    impl From<PottedPinkTulip> for super::BlockState {
        fn from(value : PottedPinkTulip) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:dark_oak_wall_sign` block.
pub mod dark_oak_wall_sign {
    /// `minecraft:dark_oak_wall_sign` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct DarkOakWallSign {
        /// `waterlogged` state.
        pub waterlogged : bool,
        /// `facing` state.
        pub facing : Facing,
    }
    /// `facing` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Facing {
        /// `north` variant.
        North,
        /// `south` variant.
        South,
        /// `west` variant.
        West,
        /// `east` variant.
        East,
    }
    impl super::Block for DarkOakWallSign {
    }
    impl From<DarkOakWallSign> for super::BlockState {
        fn from(value : DarkOakWallSign) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:raw_copper_block` block.
pub mod raw_copper_block {
    /// `minecraft:raw_copper_block` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct RawCopperBlock;
    impl super::Block for RawCopperBlock {
    }
    impl From<RawCopperBlock> for super::BlockState {
        fn from(value : RawCopperBlock) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:deepslate_bricks` block.
pub mod deepslate_bricks {
    /// `minecraft:deepslate_bricks` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct DeepslateBricks;
    impl super::Block for DeepslateBricks {
    }
    impl From<DeepslateBricks> for super::BlockState {
        fn from(value : DeepslateBricks) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:zombie_wall_head` block.
pub mod zombie_wall_head {
    /// `minecraft:zombie_wall_head` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct ZombieWallHead {
        /// `facing` state.
        pub facing : Facing,
        /// `powered` state.
        pub powered : bool,
    }
    /// `facing` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Facing {
        /// `north` variant.
        North,
        /// `south` variant.
        South,
        /// `west` variant.
        West,
        /// `east` variant.
        East,
    }
    impl super::Block for ZombieWallHead {
    }
    impl From<ZombieWallHead> for super::BlockState {
        fn from(value : ZombieWallHead) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:tube_coral_fan` block.
pub mod tube_coral_fan {
    /// `minecraft:tube_coral_fan` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct TubeCoralFan {
        /// `waterlogged` state.
        pub waterlogged : bool,
    }
    impl super::Block for TubeCoralFan {
    }
    impl From<TubeCoralFan> for super::BlockState {
        fn from(value : TubeCoralFan) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:sweet_berry_bush` block.
pub mod sweet_berry_bush {
    /// `minecraft:sweet_berry_bush` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct SweetBerryBush {
        /// `age` state.
        pub age : Age,
    }
    /// `age` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Age {
        /// `0` variant.
        N0,
        /// `1` variant.
        N1,
        /// `2` variant.
        N2,
        /// `3` variant.
        N3,
    }
    impl super::Block for SweetBerryBush {
    }
    impl From<SweetBerryBush> for super::BlockState {
        fn from(value : SweetBerryBush) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:acacia_log` block.
pub mod acacia_log {
    /// `minecraft:acacia_log` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct AcaciaLog {
        /// `axis` state.
        pub axis : Axis,
    }
    /// `axis` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Axis {
        /// `x` variant.
        X,
        /// `y` variant.
        Y,
        /// `z` variant.
        Z,
    }
    impl super::Block for AcaciaLog {
    }
    impl From<AcaciaLog> for super::BlockState {
        fn from(value : AcaciaLog) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:end_stone_brick_stairs` block.
pub mod end_stone_brick_stairs {
    /// `minecraft:end_stone_brick_stairs` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct EndStoneBrickStairs {
        /// `waterlogged` state.
        pub waterlogged : bool,
        /// `facing` state.
        pub facing : Facing,
        /// `shape` state.
        pub shape : Shape,
        /// `half` state.
        pub half : Half,
    }
    /// `facing` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Facing {
        /// `north` variant.
        North,
        /// `south` variant.
        South,
        /// `west` variant.
        West,
        /// `east` variant.
        East,
    }
    /// `shape` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Shape {
        /// `straight` variant.
        Straight,
        /// `inner_left` variant.
        InnerLeft,
        /// `inner_right` variant.
        InnerRight,
        /// `outer_left` variant.
        OuterLeft,
        /// `outer_right` variant.
        OuterRight,
    }
    /// `half` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Half {
        /// `top` variant.
        Top,
        /// `bottom` variant.
        Bottom,
    }
    impl super::Block for EndStoneBrickStairs {
    }
    impl From<EndStoneBrickStairs> for super::BlockState {
        fn from(value : EndStoneBrickStairs) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:cherry_button` block.
pub mod cherry_button {
    /// `minecraft:cherry_button` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct CherryButton {
        /// `facing` state.
        pub facing : Facing,
        /// `powered` state.
        pub powered : bool,
        /// `face` state.
        pub face : Face,
    }
    /// `facing` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Facing {
        /// `north` variant.
        North,
        /// `south` variant.
        South,
        /// `west` variant.
        West,
        /// `east` variant.
        East,
    }
    /// `face` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Face {
        /// `floor` variant.
        Floor,
        /// `wall` variant.
        Wall,
        /// `ceiling` variant.
        Ceiling,
    }
    impl super::Block for CherryButton {
    }
    impl From<CherryButton> for super::BlockState {
        fn from(value : CherryButton) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:twisting_vines_plant` block.
pub mod twisting_vines_plant {
    /// `minecraft:twisting_vines_plant` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct TwistingVinesPlant;
    impl super::Block for TwistingVinesPlant {
    }
    impl From<TwistingVinesPlant> for super::BlockState {
        fn from(value : TwistingVinesPlant) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:green_candle` block.
pub mod green_candle {
    /// `minecraft:green_candle` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct GreenCandle {
        /// `lit` state.
        pub lit : bool,
        /// `waterlogged` state.
        pub waterlogged : bool,
        /// `candles` state.
        pub candles : Candles,
    }
    /// `candles` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Candles {
        /// `1` variant.
        N1,
        /// `2` variant.
        N2,
        /// `3` variant.
        N3,
        /// `4` variant.
        N4,
    }
    impl super::Block for GreenCandle {
    }
    impl From<GreenCandle> for super::BlockState {
        fn from(value : GreenCandle) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:stripped_dark_oak_log` block.
pub mod stripped_dark_oak_log {
    /// `minecraft:stripped_dark_oak_log` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct StrippedDarkOakLog {
        /// `axis` state.
        pub axis : Axis,
    }
    /// `axis` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Axis {
        /// `x` variant.
        X,
        /// `y` variant.
        Y,
        /// `z` variant.
        Z,
    }
    impl super::Block for StrippedDarkOakLog {
    }
    impl From<StrippedDarkOakLog> for super::BlockState {
        fn from(value : StrippedDarkOakLog) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:petrified_oak_slab` block.
pub mod petrified_oak_slab {
    /// `minecraft:petrified_oak_slab` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct PetrifiedOakSlab {
        /// `waterlogged` state.
        pub waterlogged : bool,
        /// `type` state.
        pub kind : Kind,
    }
    /// `type` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Kind {
        /// `top` variant.
        Top,
        /// `bottom` variant.
        Bottom,
        /// `double` variant.
        Double,
    }
    impl super::Block for PetrifiedOakSlab {
    }
    impl From<PetrifiedOakSlab> for super::BlockState {
        fn from(value : PetrifiedOakSlab) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:sculk` block.
pub mod sculk {
    /// `minecraft:sculk` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct Sculk;
    impl super::Block for Sculk {
    }
    impl From<Sculk> for super::BlockState {
        fn from(value : Sculk) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:netherrack` block.
pub mod netherrack {
    /// `minecraft:netherrack` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct Netherrack;
    impl super::Block for Netherrack {
    }
    impl From<Netherrack> for super::BlockState {
        fn from(value : Netherrack) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:test_block` block.
pub mod test_block {
    /// `minecraft:test_block` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct TestBlock {
        /// `mode` state.
        pub mode : Mode,
    }
    /// `mode` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Mode {
        /// `start` variant.
        Start,
        /// `log` variant.
        Log,
        /// `fail` variant.
        Fail,
        /// `accept` variant.
        Accept,
    }
    impl super::Block for TestBlock {
    }
    impl From<TestBlock> for super::BlockState {
        fn from(value : TestBlock) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:blue_bed` block.
pub mod blue_bed {
    /// `minecraft:blue_bed` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct BlueBed {
        /// `facing` state.
        pub facing : Facing,
        /// `occupied` state.
        pub occupied : bool,
        /// `part` state.
        pub part : Part,
    }
    /// `facing` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Facing {
        /// `north` variant.
        North,
        /// `south` variant.
        South,
        /// `west` variant.
        West,
        /// `east` variant.
        East,
    }
    /// `part` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Part {
        /// `head` variant.
        Head,
        /// `foot` variant.
        Foot,
    }
    impl super::Block for BlueBed {
    }
    impl From<BlueBed> for super::BlockState {
        fn from(value : BlueBed) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:oak_door` block.
pub mod oak_door {
    /// `minecraft:oak_door` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct OakDoor {
        /// `facing` state.
        pub facing : Facing,
        /// `powered` state.
        pub powered : bool,
        /// `open` state.
        pub open : bool,
        /// `hinge` state.
        pub hinge : Hinge,
        /// `half` state.
        pub half : Half,
    }
    /// `facing` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Facing {
        /// `north` variant.
        North,
        /// `south` variant.
        South,
        /// `west` variant.
        West,
        /// `east` variant.
        East,
    }
    /// `hinge` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Hinge {
        /// `left` variant.
        Left,
        /// `right` variant.
        Right,
    }
    /// `half` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Half {
        /// `upper` variant.
        Upper,
        /// `lower` variant.
        Lower,
    }
    impl super::Block for OakDoor {
    }
    impl From<OakDoor> for super::BlockState {
        fn from(value : OakDoor) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:copper_door` block.
pub mod copper_door {
    /// `minecraft:copper_door` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct CopperDoor {
        /// `facing` state.
        pub facing : Facing,
        /// `half` state.
        pub half : Half,
        /// `hinge` state.
        pub hinge : Hinge,
        /// `open` state.
        pub open : bool,
        /// `powered` state.
        pub powered : bool,
    }
    /// `facing` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Facing {
        /// `north` variant.
        North,
        /// `south` variant.
        South,
        /// `west` variant.
        West,
        /// `east` variant.
        East,
    }
    /// `half` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Half {
        /// `upper` variant.
        Upper,
        /// `lower` variant.
        Lower,
    }
    /// `hinge` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Hinge {
        /// `left` variant.
        Left,
        /// `right` variant.
        Right,
    }
    impl super::Block for CopperDoor {
    }
    impl From<CopperDoor> for super::BlockState {
        fn from(value : CopperDoor) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:lava` block.
pub mod lava {
    /// `minecraft:lava` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct Lava {
        /// `level` state.
        pub level : Level,
    }
    /// `level` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Level {
        /// `0` variant.
        N0,
        /// `1` variant.
        N1,
        /// `2` variant.
        N2,
        /// `3` variant.
        N3,
        /// `4` variant.
        N4,
        /// `5` variant.
        N5,
        /// `6` variant.
        N6,
        /// `7` variant.
        N7,
        /// `8` variant.
        N8,
        /// `9` variant.
        N9,
        /// `10` variant.
        N10,
        /// `11` variant.
        N11,
        /// `12` variant.
        N12,
        /// `13` variant.
        N13,
        /// `14` variant.
        N14,
        /// `15` variant.
        N15,
    }
    impl super::Block for Lava {
    }
    impl From<Lava> for super::BlockState {
        fn from(value : Lava) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:crimson_trapdoor` block.
pub mod crimson_trapdoor {
    /// `minecraft:crimson_trapdoor` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct CrimsonTrapdoor {
        /// `open` state.
        pub open : bool,
        /// `half` state.
        pub half : Half,
        /// `powered` state.
        pub powered : bool,
        /// `facing` state.
        pub facing : Facing,
        /// `waterlogged` state.
        pub waterlogged : bool,
    }
    /// `half` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Half {
        /// `top` variant.
        Top,
        /// `bottom` variant.
        Bottom,
    }
    /// `facing` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Facing {
        /// `north` variant.
        North,
        /// `south` variant.
        South,
        /// `west` variant.
        West,
        /// `east` variant.
        East,
    }
    impl super::Block for CrimsonTrapdoor {
    }
    impl From<CrimsonTrapdoor> for super::BlockState {
        fn from(value : CrimsonTrapdoor) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:lava_cauldron` block.
pub mod lava_cauldron {
    /// `minecraft:lava_cauldron` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct LavaCauldron;
    impl super::Block for LavaCauldron {
    }
    impl From<LavaCauldron> for super::BlockState {
        fn from(value : LavaCauldron) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:prismarine_slab` block.
pub mod prismarine_slab {
    /// `minecraft:prismarine_slab` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct PrismarineSlab {
        /// `type` state.
        pub kind : Kind,
        /// `waterlogged` state.
        pub waterlogged : bool,
    }
    /// `type` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Kind {
        /// `top` variant.
        Top,
        /// `bottom` variant.
        Bottom,
        /// `double` variant.
        Double,
    }
    impl super::Block for PrismarineSlab {
    }
    impl From<PrismarineSlab> for super::BlockState {
        fn from(value : PrismarineSlab) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:blue_candle_cake` block.
pub mod blue_candle_cake {
    /// `minecraft:blue_candle_cake` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct BlueCandleCake {
        /// `lit` state.
        pub lit : bool,
    }
    impl super::Block for BlueCandleCake {
    }
    impl From<BlueCandleCake> for super::BlockState {
        fn from(value : BlueCandleCake) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:dead_fire_coral_fan` block.
pub mod dead_fire_coral_fan {
    /// `minecraft:dead_fire_coral_fan` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct DeadFireCoralFan {
        /// `waterlogged` state.
        pub waterlogged : bool,
    }
    impl super::Block for DeadFireCoralFan {
    }
    impl From<DeadFireCoralFan> for super::BlockState {
        fn from(value : DeadFireCoralFan) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:furnace` block.
pub mod furnace {
    /// `minecraft:furnace` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct Furnace {
        /// `lit` state.
        pub lit : bool,
        /// `facing` state.
        pub facing : Facing,
    }
    /// `facing` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Facing {
        /// `north` variant.
        North,
        /// `south` variant.
        South,
        /// `west` variant.
        West,
        /// `east` variant.
        East,
    }
    impl super::Block for Furnace {
    }
    impl From<Furnace> for super::BlockState {
        fn from(value : Furnace) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:mangrove_pressure_plate` block.
pub mod mangrove_pressure_plate {
    /// `minecraft:mangrove_pressure_plate` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct MangrovePressurePlate {
        /// `powered` state.
        pub powered : bool,
    }
    impl super::Block for MangrovePressurePlate {
    }
    impl From<MangrovePressurePlate> for super::BlockState {
        fn from(value : MangrovePressurePlate) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:sandstone_stairs` block.
pub mod sandstone_stairs {
    /// `minecraft:sandstone_stairs` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct SandstoneStairs {
        /// `waterlogged` state.
        pub waterlogged : bool,
        /// `half` state.
        pub half : Half,
        /// `shape` state.
        pub shape : Shape,
        /// `facing` state.
        pub facing : Facing,
    }
    /// `half` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Half {
        /// `top` variant.
        Top,
        /// `bottom` variant.
        Bottom,
    }
    /// `shape` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Shape {
        /// `straight` variant.
        Straight,
        /// `inner_left` variant.
        InnerLeft,
        /// `inner_right` variant.
        InnerRight,
        /// `outer_left` variant.
        OuterLeft,
        /// `outer_right` variant.
        OuterRight,
    }
    /// `facing` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Facing {
        /// `north` variant.
        North,
        /// `south` variant.
        South,
        /// `west` variant.
        West,
        /// `east` variant.
        East,
    }
    impl super::Block for SandstoneStairs {
    }
    impl From<SandstoneStairs> for super::BlockState {
        fn from(value : SandstoneStairs) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:barrier` block.
pub mod barrier {
    /// `minecraft:barrier` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct Barrier {
        /// `waterlogged` state.
        pub waterlogged : bool,
    }
    impl super::Block for Barrier {
    }
    impl From<Barrier> for super::BlockState {
        fn from(value : Barrier) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:bush` block.
pub mod bush {
    /// `minecraft:bush` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct Bush;
    impl super::Block for Bush {
    }
    impl From<Bush> for super::BlockState {
        fn from(value : Bush) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:redstone_block` block.
pub mod redstone_block {
    /// `minecraft:redstone_block` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct RedstoneBlock;
    impl super::Block for RedstoneBlock {
    }
    impl From<RedstoneBlock> for super::BlockState {
        fn from(value : RedstoneBlock) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:flowering_azalea` block.
pub mod flowering_azalea {
    /// `minecraft:flowering_azalea` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct FloweringAzalea;
    impl super::Block for FloweringAzalea {
    }
    impl From<FloweringAzalea> for super::BlockState {
        fn from(value : FloweringAzalea) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:jungle_slab` block.
pub mod jungle_slab {
    /// `minecraft:jungle_slab` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct JungleSlab {
        /// `waterlogged` state.
        pub waterlogged : bool,
        /// `type` state.
        pub kind : Kind,
    }
    /// `type` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Kind {
        /// `top` variant.
        Top,
        /// `bottom` variant.
        Bottom,
        /// `double` variant.
        Double,
    }
    impl super::Block for JungleSlab {
    }
    impl From<JungleSlab> for super::BlockState {
        fn from(value : JungleSlab) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:waxed_oxidized_copper` block.
pub mod waxed_oxidized_copper {
    /// `minecraft:waxed_oxidized_copper` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct WaxedOxidizedCopper;
    impl super::Block for WaxedOxidizedCopper {
    }
    impl From<WaxedOxidizedCopper> for super::BlockState {
        fn from(value : WaxedOxidizedCopper) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:black_shulker_box` block.
pub mod black_shulker_box {
    /// `minecraft:black_shulker_box` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct BlackShulkerBox {
        /// `facing` state.
        pub facing : Facing,
    }
    /// `facing` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Facing {
        /// `north` variant.
        North,
        /// `east` variant.
        East,
        /// `south` variant.
        South,
        /// `west` variant.
        West,
        /// `up` variant.
        Up,
        /// `down` variant.
        Down,
    }
    impl super::Block for BlackShulkerBox {
    }
    impl From<BlackShulkerBox> for super::BlockState {
        fn from(value : BlackShulkerBox) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:fire_coral_wall_fan` block.
pub mod fire_coral_wall_fan {
    /// `minecraft:fire_coral_wall_fan` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct FireCoralWallFan {
        /// `waterlogged` state.
        pub waterlogged : bool,
        /// `facing` state.
        pub facing : Facing,
    }
    /// `facing` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Facing {
        /// `north` variant.
        North,
        /// `south` variant.
        South,
        /// `west` variant.
        West,
        /// `east` variant.
        East,
    }
    impl super::Block for FireCoralWallFan {
    }
    impl From<FireCoralWallFan> for super::BlockState {
        fn from(value : FireCoralWallFan) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:warped_fence` block.
pub mod warped_fence {
    /// `minecraft:warped_fence` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct WarpedFence {
        /// `south` state.
        pub south : bool,
        /// `west` state.
        pub west : bool,
        /// `east` state.
        pub east : bool,
        /// `north` state.
        pub north : bool,
        /// `waterlogged` state.
        pub waterlogged : bool,
    }
    impl super::Block for WarpedFence {
    }
    impl From<WarpedFence> for super::BlockState {
        fn from(value : WarpedFence) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:bookshelf` block.
pub mod bookshelf {
    /// `minecraft:bookshelf` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct Bookshelf;
    impl super::Block for Bookshelf {
    }
    impl From<Bookshelf> for super::BlockState {
        fn from(value : Bookshelf) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:orange_terracotta` block.
pub mod orange_terracotta {
    /// `minecraft:orange_terracotta` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct OrangeTerracotta;
    impl super::Block for OrangeTerracotta {
    }
    impl From<OrangeTerracotta> for super::BlockState {
        fn from(value : OrangeTerracotta) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:stripped_cherry_wood` block.
pub mod stripped_cherry_wood {
    /// `minecraft:stripped_cherry_wood` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct StrippedCherryWood {
        /// `axis` state.
        pub axis : Axis,
    }
    /// `axis` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Axis {
        /// `x` variant.
        X,
        /// `y` variant.
        Y,
        /// `z` variant.
        Z,
    }
    impl super::Block for StrippedCherryWood {
    }
    impl From<StrippedCherryWood> for super::BlockState {
        fn from(value : StrippedCherryWood) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:pale_oak_leaves` block.
pub mod pale_oak_leaves {
    /// `minecraft:pale_oak_leaves` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct PaleOakLeaves {
        /// `distance` state.
        pub distance : Distance,
        /// `persistent` state.
        pub persistent : bool,
        /// `waterlogged` state.
        pub waterlogged : bool,
    }
    /// `distance` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Distance {
        /// `1` variant.
        N1,
        /// `2` variant.
        N2,
        /// `3` variant.
        N3,
        /// `4` variant.
        N4,
        /// `5` variant.
        N5,
        /// `6` variant.
        N6,
        /// `7` variant.
        N7,
    }
    impl super::Block for PaleOakLeaves {
    }
    impl From<PaleOakLeaves> for super::BlockState {
        fn from(value : PaleOakLeaves) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:cyan_glazed_terracotta` block.
pub mod cyan_glazed_terracotta {
    /// `minecraft:cyan_glazed_terracotta` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct CyanGlazedTerracotta {
        /// `facing` state.
        pub facing : Facing,
    }
    /// `facing` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Facing {
        /// `north` variant.
        North,
        /// `south` variant.
        South,
        /// `west` variant.
        West,
        /// `east` variant.
        East,
    }
    impl super::Block for CyanGlazedTerracotta {
    }
    impl From<CyanGlazedTerracotta> for super::BlockState {
        fn from(value : CyanGlazedTerracotta) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:birch_fence_gate` block.
pub mod birch_fence_gate {
    /// `minecraft:birch_fence_gate` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct BirchFenceGate {
        /// `powered` state.
        pub powered : bool,
        /// `facing` state.
        pub facing : Facing,
        /// `in_wall` state.
        pub in_wall : bool,
        /// `open` state.
        pub open : bool,
    }
    /// `facing` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Facing {
        /// `north` variant.
        North,
        /// `south` variant.
        South,
        /// `west` variant.
        West,
        /// `east` variant.
        East,
    }
    impl super::Block for BirchFenceGate {
    }
    impl From<BirchFenceGate> for super::BlockState {
        fn from(value : BirchFenceGate) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:crimson_button` block.
pub mod crimson_button {
    /// `minecraft:crimson_button` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct CrimsonButton {
        /// `face` state.
        pub face : Face,
        /// `facing` state.
        pub facing : Facing,
        /// `powered` state.
        pub powered : bool,
    }
    /// `face` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Face {
        /// `floor` variant.
        Floor,
        /// `wall` variant.
        Wall,
        /// `ceiling` variant.
        Ceiling,
    }
    /// `facing` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Facing {
        /// `north` variant.
        North,
        /// `south` variant.
        South,
        /// `west` variant.
        West,
        /// `east` variant.
        East,
    }
    impl super::Block for CrimsonButton {
    }
    impl From<CrimsonButton> for super::BlockState {
        fn from(value : CrimsonButton) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:deepslate_redstone_ore` block.
pub mod deepslate_redstone_ore {
    /// `minecraft:deepslate_redstone_ore` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct DeepslateRedstoneOre {
        /// `lit` state.
        pub lit : bool,
    }
    impl super::Block for DeepslateRedstoneOre {
    }
    impl From<DeepslateRedstoneOre> for super::BlockState {
        fn from(value : DeepslateRedstoneOre) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:cherry_trapdoor` block.
pub mod cherry_trapdoor {
    /// `minecraft:cherry_trapdoor` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct CherryTrapdoor {
        /// `half` state.
        pub half : Half,
        /// `open` state.
        pub open : bool,
        /// `facing` state.
        pub facing : Facing,
        /// `waterlogged` state.
        pub waterlogged : bool,
        /// `powered` state.
        pub powered : bool,
    }
    /// `half` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Half {
        /// `top` variant.
        Top,
        /// `bottom` variant.
        Bottom,
    }
    /// `facing` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Facing {
        /// `north` variant.
        North,
        /// `south` variant.
        South,
        /// `west` variant.
        West,
        /// `east` variant.
        East,
    }
    impl super::Block for CherryTrapdoor {
    }
    impl From<CherryTrapdoor> for super::BlockState {
        fn from(value : CherryTrapdoor) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:mossy_stone_bricks` block.
pub mod mossy_stone_bricks {
    /// `minecraft:mossy_stone_bricks` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct MossyStoneBricks;
    impl super::Block for MossyStoneBricks {
    }
    impl From<MossyStoneBricks> for super::BlockState {
        fn from(value : MossyStoneBricks) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:nether_bricks` block.
pub mod nether_bricks {
    /// `minecraft:nether_bricks` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct NetherBricks;
    impl super::Block for NetherBricks {
    }
    impl From<NetherBricks> for super::BlockState {
        fn from(value : NetherBricks) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:acacia_fence` block.
pub mod acacia_fence {
    /// `minecraft:acacia_fence` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct AcaciaFence {
        /// `north` state.
        pub north : bool,
        /// `west` state.
        pub west : bool,
        /// `east` state.
        pub east : bool,
        /// `waterlogged` state.
        pub waterlogged : bool,
        /// `south` state.
        pub south : bool,
    }
    impl super::Block for AcaciaFence {
    }
    impl From<AcaciaFence> for super::BlockState {
        fn from(value : AcaciaFence) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:tuff_brick_stairs` block.
pub mod tuff_brick_stairs {
    /// `minecraft:tuff_brick_stairs` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct TuffBrickStairs {
        /// `half` state.
        pub half : Half,
        /// `facing` state.
        pub facing : Facing,
        /// `shape` state.
        pub shape : Shape,
        /// `waterlogged` state.
        pub waterlogged : bool,
    }
    /// `half` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Half {
        /// `top` variant.
        Top,
        /// `bottom` variant.
        Bottom,
    }
    /// `facing` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Facing {
        /// `north` variant.
        North,
        /// `south` variant.
        South,
        /// `west` variant.
        West,
        /// `east` variant.
        East,
    }
    /// `shape` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Shape {
        /// `straight` variant.
        Straight,
        /// `inner_left` variant.
        InnerLeft,
        /// `inner_right` variant.
        InnerRight,
        /// `outer_left` variant.
        OuterLeft,
        /// `outer_right` variant.
        OuterRight,
    }
    impl super::Block for TuffBrickStairs {
    }
    impl From<TuffBrickStairs> for super::BlockState {
        fn from(value : TuffBrickStairs) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:brick_stairs` block.
pub mod brick_stairs {
    /// `minecraft:brick_stairs` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct BrickStairs {
        /// `half` state.
        pub half : Half,
        /// `shape` state.
        pub shape : Shape,
        /// `waterlogged` state.
        pub waterlogged : bool,
        /// `facing` state.
        pub facing : Facing,
    }
    /// `half` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Half {
        /// `top` variant.
        Top,
        /// `bottom` variant.
        Bottom,
    }
    /// `shape` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Shape {
        /// `straight` variant.
        Straight,
        /// `inner_left` variant.
        InnerLeft,
        /// `inner_right` variant.
        InnerRight,
        /// `outer_left` variant.
        OuterLeft,
        /// `outer_right` variant.
        OuterRight,
    }
    /// `facing` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Facing {
        /// `north` variant.
        North,
        /// `south` variant.
        South,
        /// `west` variant.
        West,
        /// `east` variant.
        East,
    }
    impl super::Block for BrickStairs {
    }
    impl From<BrickStairs> for super::BlockState {
        fn from(value : BrickStairs) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:cyan_stained_glass_pane` block.
pub mod cyan_stained_glass_pane {
    /// `minecraft:cyan_stained_glass_pane` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct CyanStainedGlassPane {
        /// `west` state.
        pub west : bool,
        /// `east` state.
        pub east : bool,
        /// `waterlogged` state.
        pub waterlogged : bool,
        /// `south` state.
        pub south : bool,
        /// `north` state.
        pub north : bool,
    }
    impl super::Block for CyanStainedGlassPane {
    }
    impl From<CyanStainedGlassPane> for super::BlockState {
        fn from(value : CyanStainedGlassPane) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:packed_ice` block.
pub mod packed_ice {
    /// `minecraft:packed_ice` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct PackedIce;
    impl super::Block for PackedIce {
    }
    impl From<PackedIce> for super::BlockState {
        fn from(value : PackedIce) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:exposed_cut_copper_slab` block.
pub mod exposed_cut_copper_slab {
    /// `minecraft:exposed_cut_copper_slab` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct ExposedCutCopperSlab {
        /// `type` state.
        pub kind : Kind,
        /// `waterlogged` state.
        pub waterlogged : bool,
    }
    /// `type` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Kind {
        /// `top` variant.
        Top,
        /// `bottom` variant.
        Bottom,
        /// `double` variant.
        Double,
    }
    impl super::Block for ExposedCutCopperSlab {
    }
    impl From<ExposedCutCopperSlab> for super::BlockState {
        fn from(value : ExposedCutCopperSlab) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:orange_concrete_powder` block.
pub mod orange_concrete_powder {
    /// `minecraft:orange_concrete_powder` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct OrangeConcretePowder;
    impl super::Block for OrangeConcretePowder {
    }
    impl From<OrangeConcretePowder> for super::BlockState {
        fn from(value : OrangeConcretePowder) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:birch_pressure_plate` block.
pub mod birch_pressure_plate {
    /// `minecraft:birch_pressure_plate` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct BirchPressurePlate {
        /// `powered` state.
        pub powered : bool,
    }
    impl super::Block for BirchPressurePlate {
    }
    impl From<BirchPressurePlate> for super::BlockState {
        fn from(value : BirchPressurePlate) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:exposed_copper_bulb` block.
pub mod exposed_copper_bulb {
    /// `minecraft:exposed_copper_bulb` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct ExposedCopperBulb {
        /// `lit` state.
        pub lit : bool,
        /// `powered` state.
        pub powered : bool,
    }
    impl super::Block for ExposedCopperBulb {
    }
    impl From<ExposedCopperBulb> for super::BlockState {
        fn from(value : ExposedCopperBulb) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:magenta_stained_glass_pane` block.
pub mod magenta_stained_glass_pane {
    /// `minecraft:magenta_stained_glass_pane` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct MagentaStainedGlassPane {
        /// `north` state.
        pub north : bool,
        /// `south` state.
        pub south : bool,
        /// `west` state.
        pub west : bool,
        /// `east` state.
        pub east : bool,
        /// `waterlogged` state.
        pub waterlogged : bool,
    }
    impl super::Block for MagentaStainedGlassPane {
    }
    impl From<MagentaStainedGlassPane> for super::BlockState {
        fn from(value : MagentaStainedGlassPane) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:weathered_cut_copper_slab` block.
pub mod weathered_cut_copper_slab {
    /// `minecraft:weathered_cut_copper_slab` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct WeatheredCutCopperSlab {
        /// `type` state.
        pub kind : Kind,
        /// `waterlogged` state.
        pub waterlogged : bool,
    }
    /// `type` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Kind {
        /// `top` variant.
        Top,
        /// `bottom` variant.
        Bottom,
        /// `double` variant.
        Double,
    }
    impl super::Block for WeatheredCutCopperSlab {
    }
    impl From<WeatheredCutCopperSlab> for super::BlockState {
        fn from(value : WeatheredCutCopperSlab) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:andesite` block.
pub mod andesite {
    /// `minecraft:andesite` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct Andesite;
    impl super::Block for Andesite {
    }
    impl From<Andesite> for super::BlockState {
        fn from(value : Andesite) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:gray_candle_cake` block.
pub mod gray_candle_cake {
    /// `minecraft:gray_candle_cake` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct GrayCandleCake {
        /// `lit` state.
        pub lit : bool,
    }
    impl super::Block for GrayCandleCake {
    }
    impl From<GrayCandleCake> for super::BlockState {
        fn from(value : GrayCandleCake) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:slime_block` block.
pub mod slime_block {
    /// `minecraft:slime_block` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct SlimeBlock;
    impl super::Block for SlimeBlock {
    }
    impl From<SlimeBlock> for super::BlockState {
        fn from(value : SlimeBlock) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:chiseled_deepslate` block.
pub mod chiseled_deepslate {
    /// `minecraft:chiseled_deepslate` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct ChiseledDeepslate;
    impl super::Block for ChiseledDeepslate {
    }
    impl From<ChiseledDeepslate> for super::BlockState {
        fn from(value : ChiseledDeepslate) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:jungle_wall_sign` block.
pub mod jungle_wall_sign {
    /// `minecraft:jungle_wall_sign` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct JungleWallSign {
        /// `facing` state.
        pub facing : Facing,
        /// `waterlogged` state.
        pub waterlogged : bool,
    }
    /// `facing` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Facing {
        /// `north` variant.
        North,
        /// `south` variant.
        South,
        /// `west` variant.
        West,
        /// `east` variant.
        East,
    }
    impl super::Block for JungleWallSign {
    }
    impl From<JungleWallSign> for super::BlockState {
        fn from(value : JungleWallSign) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:cyan_stained_glass` block.
pub mod cyan_stained_glass {
    /// `minecraft:cyan_stained_glass` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct CyanStainedGlass;
    impl super::Block for CyanStainedGlass {
    }
    impl From<CyanStainedGlass> for super::BlockState {
        fn from(value : CyanStainedGlass) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:acacia_button` block.
pub mod acacia_button {
    /// `minecraft:acacia_button` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct AcaciaButton {
        /// `face` state.
        pub face : Face,
        /// `powered` state.
        pub powered : bool,
        /// `facing` state.
        pub facing : Facing,
    }
    /// `face` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Face {
        /// `floor` variant.
        Floor,
        /// `wall` variant.
        Wall,
        /// `ceiling` variant.
        Ceiling,
    }
    /// `facing` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Facing {
        /// `north` variant.
        North,
        /// `south` variant.
        South,
        /// `west` variant.
        West,
        /// `east` variant.
        East,
    }
    impl super::Block for AcaciaButton {
    }
    impl From<AcaciaButton> for super::BlockState {
        fn from(value : AcaciaButton) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:bamboo_button` block.
pub mod bamboo_button {
    /// `minecraft:bamboo_button` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct BambooButton {
        /// `facing` state.
        pub facing : Facing,
        /// `face` state.
        pub face : Face,
        /// `powered` state.
        pub powered : bool,
    }
    /// `facing` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Facing {
        /// `north` variant.
        North,
        /// `south` variant.
        South,
        /// `west` variant.
        West,
        /// `east` variant.
        East,
    }
    /// `face` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Face {
        /// `floor` variant.
        Floor,
        /// `wall` variant.
        Wall,
        /// `ceiling` variant.
        Ceiling,
    }
    impl super::Block for BambooButton {
    }
    impl From<BambooButton> for super::BlockState {
        fn from(value : BambooButton) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:green_candle_cake` block.
pub mod green_candle_cake {
    /// `minecraft:green_candle_cake` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct GreenCandleCake {
        /// `lit` state.
        pub lit : bool,
    }
    impl super::Block for GreenCandleCake {
    }
    impl From<GreenCandleCake> for super::BlockState {
        fn from(value : GreenCandleCake) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:lily_of_the_valley` block.
pub mod lily_of_the_valley {
    /// `minecraft:lily_of_the_valley` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct LilyOfTheValley;
    impl super::Block for LilyOfTheValley {
    }
    impl From<LilyOfTheValley> for super::BlockState {
        fn from(value : LilyOfTheValley) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:trial_spawner` block.
pub mod trial_spawner {
    /// `minecraft:trial_spawner` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct TrialSpawner {
        /// `ominous` state.
        pub ominous : bool,
        /// `trial_spawner_state` state.
        pub trial_spawner_state : TrialSpawnerState,
    }
    /// `trial_spawner_state` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum TrialSpawnerState {
        /// `inactive` variant.
        Inactive,
        /// `waiting_for_players` variant.
        WaitingForPlayers,
        /// `active` variant.
        Active,
        /// `waiting_for_reward_ejection` variant.
        WaitingForRewardEjection,
        /// `ejecting_reward` variant.
        EjectingReward,
        /// `cooldown` variant.
        Cooldown,
    }
    impl super::Block for TrialSpawner {
    }
    impl From<TrialSpawner> for super::BlockState {
        fn from(value : TrialSpawner) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:mangrove_stairs` block.
pub mod mangrove_stairs {
    /// `minecraft:mangrove_stairs` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct MangroveStairs {
        /// `facing` state.
        pub facing : Facing,
        /// `half` state.
        pub half : Half,
        /// `shape` state.
        pub shape : Shape,
        /// `waterlogged` state.
        pub waterlogged : bool,
    }
    /// `facing` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Facing {
        /// `north` variant.
        North,
        /// `south` variant.
        South,
        /// `west` variant.
        West,
        /// `east` variant.
        East,
    }
    /// `half` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Half {
        /// `top` variant.
        Top,
        /// `bottom` variant.
        Bottom,
    }
    /// `shape` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Shape {
        /// `straight` variant.
        Straight,
        /// `inner_left` variant.
        InnerLeft,
        /// `inner_right` variant.
        InnerRight,
        /// `outer_left` variant.
        OuterLeft,
        /// `outer_right` variant.
        OuterRight,
    }
    impl super::Block for MangroveStairs {
    }
    impl From<MangroveStairs> for super::BlockState {
        fn from(value : MangroveStairs) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:cherry_planks` block.
pub mod cherry_planks {
    /// `minecraft:cherry_planks` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct CherryPlanks;
    impl super::Block for CherryPlanks {
    }
    impl From<CherryPlanks> for super::BlockState {
        fn from(value : CherryPlanks) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:birch_hanging_sign` block.
pub mod birch_hanging_sign {
    /// `minecraft:birch_hanging_sign` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct BirchHangingSign {
        /// `waterlogged` state.
        pub waterlogged : bool,
        /// `attached` state.
        pub attached : bool,
        /// `rotation` state.
        pub rotation : Rotation,
    }
    /// `rotation` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Rotation {
        /// `0` variant.
        N0,
        /// `1` variant.
        N1,
        /// `2` variant.
        N2,
        /// `3` variant.
        N3,
        /// `4` variant.
        N4,
        /// `5` variant.
        N5,
        /// `6` variant.
        N6,
        /// `7` variant.
        N7,
        /// `8` variant.
        N8,
        /// `9` variant.
        N9,
        /// `10` variant.
        N10,
        /// `11` variant.
        N11,
        /// `12` variant.
        N12,
        /// `13` variant.
        N13,
        /// `14` variant.
        N14,
        /// `15` variant.
        N15,
    }
    impl super::Block for BirchHangingSign {
    }
    impl From<BirchHangingSign> for super::BlockState {
        fn from(value : BirchHangingSign) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:waxed_exposed_cut_copper` block.
pub mod waxed_exposed_cut_copper {
    /// `minecraft:waxed_exposed_cut_copper` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct WaxedExposedCutCopper;
    impl super::Block for WaxedExposedCutCopper {
    }
    impl From<WaxedExposedCutCopper> for super::BlockState {
        fn from(value : WaxedExposedCutCopper) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:cyan_wall_banner` block.
pub mod cyan_wall_banner {
    /// `minecraft:cyan_wall_banner` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct CyanWallBanner {
        /// `facing` state.
        pub facing : Facing,
    }
    /// `facing` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Facing {
        /// `north` variant.
        North,
        /// `south` variant.
        South,
        /// `west` variant.
        West,
        /// `east` variant.
        East,
    }
    impl super::Block for CyanWallBanner {
    }
    impl From<CyanWallBanner> for super::BlockState {
        fn from(value : CyanWallBanner) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:gray_concrete` block.
pub mod gray_concrete {
    /// `minecraft:gray_concrete` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct GrayConcrete;
    impl super::Block for GrayConcrete {
    }
    impl From<GrayConcrete> for super::BlockState {
        fn from(value : GrayConcrete) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:tuff` block.
pub mod tuff {
    /// `minecraft:tuff` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct Tuff;
    impl super::Block for Tuff {
    }
    impl From<Tuff> for super::BlockState {
        fn from(value : Tuff) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:bubble_coral_fan` block.
pub mod bubble_coral_fan {
    /// `minecraft:bubble_coral_fan` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct BubbleCoralFan {
        /// `waterlogged` state.
        pub waterlogged : bool,
    }
    impl super::Block for BubbleCoralFan {
    }
    impl From<BubbleCoralFan> for super::BlockState {
        fn from(value : BubbleCoralFan) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:dark_oak_log` block.
pub mod dark_oak_log {
    /// `minecraft:dark_oak_log` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct DarkOakLog {
        /// `axis` state.
        pub axis : Axis,
    }
    /// `axis` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Axis {
        /// `x` variant.
        X,
        /// `y` variant.
        Y,
        /// `z` variant.
        Z,
    }
    impl super::Block for DarkOakLog {
    }
    impl From<DarkOakLog> for super::BlockState {
        fn from(value : DarkOakLog) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:lectern` block.
pub mod lectern {
    /// `minecraft:lectern` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct Lectern {
        /// `powered` state.
        pub powered : bool,
        /// `facing` state.
        pub facing : Facing,
        /// `has_book` state.
        pub has_book : bool,
    }
    /// `facing` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Facing {
        /// `north` variant.
        North,
        /// `south` variant.
        South,
        /// `west` variant.
        West,
        /// `east` variant.
        East,
    }
    impl super::Block for Lectern {
    }
    impl From<Lectern> for super::BlockState {
        fn from(value : Lectern) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:stripped_warped_stem` block.
pub mod stripped_warped_stem {
    /// `minecraft:stripped_warped_stem` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct StrippedWarpedStem {
        /// `axis` state.
        pub axis : Axis,
    }
    /// `axis` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Axis {
        /// `x` variant.
        X,
        /// `y` variant.
        Y,
        /// `z` variant.
        Z,
    }
    impl super::Block for StrippedWarpedStem {
    }
    impl From<StrippedWarpedStem> for super::BlockState {
        fn from(value : StrippedWarpedStem) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:gilded_blackstone` block.
pub mod gilded_blackstone {
    /// `minecraft:gilded_blackstone` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct GildedBlackstone;
    impl super::Block for GildedBlackstone {
    }
    impl From<GildedBlackstone> for super::BlockState {
        fn from(value : GildedBlackstone) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:spore_blossom` block.
pub mod spore_blossom {
    /// `minecraft:spore_blossom` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct SporeBlossom;
    impl super::Block for SporeBlossom {
    }
    impl From<SporeBlossom> for super::BlockState {
        fn from(value : SporeBlossom) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:skeleton_wall_skull` block.
pub mod skeleton_wall_skull {
    /// `minecraft:skeleton_wall_skull` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct SkeletonWallSkull {
        /// `powered` state.
        pub powered : bool,
        /// `facing` state.
        pub facing : Facing,
    }
    /// `facing` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Facing {
        /// `north` variant.
        North,
        /// `south` variant.
        South,
        /// `west` variant.
        West,
        /// `east` variant.
        East,
    }
    impl super::Block for SkeletonWallSkull {
    }
    impl From<SkeletonWallSkull> for super::BlockState {
        fn from(value : SkeletonWallSkull) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:orange_banner` block.
pub mod orange_banner {
    /// `minecraft:orange_banner` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct OrangeBanner {
        /// `rotation` state.
        pub rotation : Rotation,
    }
    /// `rotation` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Rotation {
        /// `0` variant.
        N0,
        /// `1` variant.
        N1,
        /// `2` variant.
        N2,
        /// `3` variant.
        N3,
        /// `4` variant.
        N4,
        /// `5` variant.
        N5,
        /// `6` variant.
        N6,
        /// `7` variant.
        N7,
        /// `8` variant.
        N8,
        /// `9` variant.
        N9,
        /// `10` variant.
        N10,
        /// `11` variant.
        N11,
        /// `12` variant.
        N12,
        /// `13` variant.
        N13,
        /// `14` variant.
        N14,
        /// `15` variant.
        N15,
    }
    impl super::Block for OrangeBanner {
    }
    impl From<OrangeBanner> for super::BlockState {
        fn from(value : OrangeBanner) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:iron_block` block.
pub mod iron_block {
    /// `minecraft:iron_block` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct IronBlock;
    impl super::Block for IronBlock {
    }
    impl From<IronBlock> for super::BlockState {
        fn from(value : IronBlock) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:poppy` block.
pub mod poppy {
    /// `minecraft:poppy` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct Poppy;
    impl super::Block for Poppy {
    }
    impl From<Poppy> for super::BlockState {
        fn from(value : Poppy) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:brown_concrete_powder` block.
pub mod brown_concrete_powder {
    /// `minecraft:brown_concrete_powder` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct BrownConcretePowder;
    impl super::Block for BrownConcretePowder {
    }
    impl From<BrownConcretePowder> for super::BlockState {
        fn from(value : BrownConcretePowder) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:structure_block` block.
pub mod structure_block {
    /// `minecraft:structure_block` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct StructureBlock {
        /// `mode` state.
        pub mode : Mode,
    }
    /// `mode` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Mode {
        /// `save` variant.
        Save,
        /// `load` variant.
        Load,
        /// `corner` variant.
        Corner,
        /// `data` variant.
        Data,
    }
    impl super::Block for StructureBlock {
    }
    impl From<StructureBlock> for super::BlockState {
        fn from(value : StructureBlock) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:crafting_table` block.
pub mod crafting_table {
    /// `minecraft:crafting_table` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct CraftingTable;
    impl super::Block for CraftingTable {
    }
    impl From<CraftingTable> for super::BlockState {
        fn from(value : CraftingTable) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:acacia_sign` block.
pub mod acacia_sign {
    /// `minecraft:acacia_sign` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct AcaciaSign {
        /// `rotation` state.
        pub rotation : Rotation,
        /// `waterlogged` state.
        pub waterlogged : bool,
    }
    /// `rotation` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Rotation {
        /// `0` variant.
        N0,
        /// `1` variant.
        N1,
        /// `2` variant.
        N2,
        /// `3` variant.
        N3,
        /// `4` variant.
        N4,
        /// `5` variant.
        N5,
        /// `6` variant.
        N6,
        /// `7` variant.
        N7,
        /// `8` variant.
        N8,
        /// `9` variant.
        N9,
        /// `10` variant.
        N10,
        /// `11` variant.
        N11,
        /// `12` variant.
        N12,
        /// `13` variant.
        N13,
        /// `14` variant.
        N14,
        /// `15` variant.
        N15,
    }
    impl super::Block for AcaciaSign {
    }
    impl From<AcaciaSign> for super::BlockState {
        fn from(value : AcaciaSign) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:potted_dandelion` block.
pub mod potted_dandelion {
    /// `minecraft:potted_dandelion` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct PottedDandelion;
    impl super::Block for PottedDandelion {
    }
    impl From<PottedDandelion> for super::BlockState {
        fn from(value : PottedDandelion) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:sculk_vein` block.
pub mod sculk_vein {
    /// `minecraft:sculk_vein` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct SculkVein {
        /// `south` state.
        pub south : bool,
        /// `up` state.
        pub up : bool,
        /// `east` state.
        pub east : bool,
        /// `waterlogged` state.
        pub waterlogged : bool,
        /// `west` state.
        pub west : bool,
        /// `north` state.
        pub north : bool,
        /// `down` state.
        pub down : bool,
    }
    impl super::Block for SculkVein {
    }
    impl From<SculkVein> for super::BlockState {
        fn from(value : SculkVein) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:warped_fence_gate` block.
pub mod warped_fence_gate {
    /// `minecraft:warped_fence_gate` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct WarpedFenceGate {
        /// `facing` state.
        pub facing : Facing,
        /// `powered` state.
        pub powered : bool,
        /// `in_wall` state.
        pub in_wall : bool,
        /// `open` state.
        pub open : bool,
    }
    /// `facing` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Facing {
        /// `north` variant.
        North,
        /// `south` variant.
        South,
        /// `west` variant.
        West,
        /// `east` variant.
        East,
    }
    impl super::Block for WarpedFenceGate {
    }
    impl From<WarpedFenceGate> for super::BlockState {
        fn from(value : WarpedFenceGate) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:weathered_cut_copper_stairs` block.
pub mod weathered_cut_copper_stairs {
    /// `minecraft:weathered_cut_copper_stairs` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct WeatheredCutCopperStairs {
        /// `half` state.
        pub half : Half,
        /// `shape` state.
        pub shape : Shape,
        /// `waterlogged` state.
        pub waterlogged : bool,
        /// `facing` state.
        pub facing : Facing,
    }
    /// `half` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Half {
        /// `top` variant.
        Top,
        /// `bottom` variant.
        Bottom,
    }
    /// `shape` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Shape {
        /// `straight` variant.
        Straight,
        /// `inner_left` variant.
        InnerLeft,
        /// `inner_right` variant.
        InnerRight,
        /// `outer_left` variant.
        OuterLeft,
        /// `outer_right` variant.
        OuterRight,
    }
    /// `facing` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Facing {
        /// `north` variant.
        North,
        /// `south` variant.
        South,
        /// `west` variant.
        West,
        /// `east` variant.
        East,
    }
    impl super::Block for WeatheredCutCopperStairs {
    }
    impl From<WeatheredCutCopperStairs> for super::BlockState {
        fn from(value : WeatheredCutCopperStairs) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:raw_gold_block` block.
pub mod raw_gold_block {
    /// `minecraft:raw_gold_block` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct RawGoldBlock;
    impl super::Block for RawGoldBlock {
    }
    impl From<RawGoldBlock> for super::BlockState {
        fn from(value : RawGoldBlock) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:chiseled_quartz_block` block.
pub mod chiseled_quartz_block {
    /// `minecraft:chiseled_quartz_block` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct ChiseledQuartzBlock;
    impl super::Block for ChiseledQuartzBlock {
    }
    impl From<ChiseledQuartzBlock> for super::BlockState {
        fn from(value : ChiseledQuartzBlock) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:honeycomb_block` block.
pub mod honeycomb_block {
    /// `minecraft:honeycomb_block` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct HoneycombBlock;
    impl super::Block for HoneycombBlock {
    }
    impl From<HoneycombBlock> for super::BlockState {
        fn from(value : HoneycombBlock) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:yellow_concrete_powder` block.
pub mod yellow_concrete_powder {
    /// `minecraft:yellow_concrete_powder` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct YellowConcretePowder;
    impl super::Block for YellowConcretePowder {
    }
    impl From<YellowConcretePowder> for super::BlockState {
        fn from(value : YellowConcretePowder) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:red_candle_cake` block.
pub mod red_candle_cake {
    /// `minecraft:red_candle_cake` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct RedCandleCake {
        /// `lit` state.
        pub lit : bool,
    }
    impl super::Block for RedCandleCake {
    }
    impl From<RedCandleCake> for super::BlockState {
        fn from(value : RedCandleCake) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:sculk_sensor` block.
pub mod sculk_sensor {
    /// `minecraft:sculk_sensor` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct SculkSensor {
        /// `sculk_sensor_phase` state.
        pub sculk_sensor_phase : SculkSensorPhase,
        /// `power` state.
        pub power : Power,
        /// `waterlogged` state.
        pub waterlogged : bool,
    }
    /// `sculk_sensor_phase` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum SculkSensorPhase {
        /// `inactive` variant.
        Inactive,
        /// `active` variant.
        Active,
        /// `cooldown` variant.
        Cooldown,
    }
    /// `power` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Power {
        /// `0` variant.
        N0,
        /// `1` variant.
        N1,
        /// `2` variant.
        N2,
        /// `3` variant.
        N3,
        /// `4` variant.
        N4,
        /// `5` variant.
        N5,
        /// `6` variant.
        N6,
        /// `7` variant.
        N7,
        /// `8` variant.
        N8,
        /// `9` variant.
        N9,
        /// `10` variant.
        N10,
        /// `11` variant.
        N11,
        /// `12` variant.
        N12,
        /// `13` variant.
        N13,
        /// `14` variant.
        N14,
        /// `15` variant.
        N15,
    }
    impl super::Block for SculkSensor {
    }
    impl From<SculkSensor> for super::BlockState {
        fn from(value : SculkSensor) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:warped_nylium` block.
pub mod warped_nylium {
    /// `minecraft:warped_nylium` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct WarpedNylium;
    impl super::Block for WarpedNylium {
    }
    impl From<WarpedNylium> for super::BlockState {
        fn from(value : WarpedNylium) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:oak_hanging_sign` block.
pub mod oak_hanging_sign {
    /// `minecraft:oak_hanging_sign` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct OakHangingSign {
        /// `waterlogged` state.
        pub waterlogged : bool,
        /// `attached` state.
        pub attached : bool,
        /// `rotation` state.
        pub rotation : Rotation,
    }
    /// `rotation` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Rotation {
        /// `0` variant.
        N0,
        /// `1` variant.
        N1,
        /// `2` variant.
        N2,
        /// `3` variant.
        N3,
        /// `4` variant.
        N4,
        /// `5` variant.
        N5,
        /// `6` variant.
        N6,
        /// `7` variant.
        N7,
        /// `8` variant.
        N8,
        /// `9` variant.
        N9,
        /// `10` variant.
        N10,
        /// `11` variant.
        N11,
        /// `12` variant.
        N12,
        /// `13` variant.
        N13,
        /// `14` variant.
        N14,
        /// `15` variant.
        N15,
    }
    impl super::Block for OakHangingSign {
    }
    impl From<OakHangingSign> for super::BlockState {
        fn from(value : OakHangingSign) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:redstone_lamp` block.
pub mod redstone_lamp {
    /// `minecraft:redstone_lamp` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct RedstoneLamp {
        /// `lit` state.
        pub lit : bool,
    }
    impl super::Block for RedstoneLamp {
    }
    impl From<RedstoneLamp> for super::BlockState {
        fn from(value : RedstoneLamp) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:cracked_polished_blackstone_bricks` block.
pub mod cracked_polished_blackstone_bricks {
    /// `minecraft:cracked_polished_blackstone_bricks` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct CrackedPolishedBlackstoneBricks;
    impl super::Block for CrackedPolishedBlackstoneBricks {
    }
    impl From<CrackedPolishedBlackstoneBricks> for super::BlockState {
        fn from(value : CrackedPolishedBlackstoneBricks) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:stripped_mangrove_log` block.
pub mod stripped_mangrove_log {
    /// `minecraft:stripped_mangrove_log` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct StrippedMangroveLog {
        /// `axis` state.
        pub axis : Axis,
    }
    /// `axis` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Axis {
        /// `x` variant.
        X,
        /// `y` variant.
        Y,
        /// `z` variant.
        Z,
    }
    impl super::Block for StrippedMangroveLog {
    }
    impl From<StrippedMangroveLog> for super::BlockState {
        fn from(value : StrippedMangroveLog) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:sea_pickle` block.
pub mod sea_pickle {
    /// `minecraft:sea_pickle` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct SeaPickle {
        /// `waterlogged` state.
        pub waterlogged : bool,
        /// `pickles` state.
        pub pickles : Pickles,
    }
    /// `pickles` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Pickles {
        /// `1` variant.
        N1,
        /// `2` variant.
        N2,
        /// `3` variant.
        N3,
        /// `4` variant.
        N4,
    }
    impl super::Block for SeaPickle {
    }
    impl From<SeaPickle> for super::BlockState {
        fn from(value : SeaPickle) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:red_bed` block.
pub mod red_bed {
    /// `minecraft:red_bed` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct RedBed {
        /// `part` state.
        pub part : Part,
        /// `occupied` state.
        pub occupied : bool,
        /// `facing` state.
        pub facing : Facing,
    }
    /// `part` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Part {
        /// `head` variant.
        Head,
        /// `foot` variant.
        Foot,
    }
    /// `facing` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Facing {
        /// `north` variant.
        North,
        /// `south` variant.
        South,
        /// `west` variant.
        West,
        /// `east` variant.
        East,
    }
    impl super::Block for RedBed {
    }
    impl From<RedBed> for super::BlockState {
        fn from(value : RedBed) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:stripped_warped_hyphae` block.
pub mod stripped_warped_hyphae {
    /// `minecraft:stripped_warped_hyphae` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct StrippedWarpedHyphae {
        /// `axis` state.
        pub axis : Axis,
    }
    /// `axis` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Axis {
        /// `x` variant.
        X,
        /// `y` variant.
        Y,
        /// `z` variant.
        Z,
    }
    impl super::Block for StrippedWarpedHyphae {
    }
    impl From<StrippedWarpedHyphae> for super::BlockState {
        fn from(value : StrippedWarpedHyphae) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:green_terracotta` block.
pub mod green_terracotta {
    /// `minecraft:green_terracotta` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct GreenTerracotta;
    impl super::Block for GreenTerracotta {
    }
    impl From<GreenTerracotta> for super::BlockState {
        fn from(value : GreenTerracotta) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:birch_wall_sign` block.
pub mod birch_wall_sign {
    /// `minecraft:birch_wall_sign` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct BirchWallSign {
        /// `waterlogged` state.
        pub waterlogged : bool,
        /// `facing` state.
        pub facing : Facing,
    }
    /// `facing` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Facing {
        /// `north` variant.
        North,
        /// `south` variant.
        South,
        /// `west` variant.
        West,
        /// `east` variant.
        East,
    }
    impl super::Block for BirchWallSign {
    }
    impl From<BirchWallSign> for super::BlockState {
        fn from(value : BirchWallSign) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:tuff_brick_slab` block.
pub mod tuff_brick_slab {
    /// `minecraft:tuff_brick_slab` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct TuffBrickSlab {
        /// `waterlogged` state.
        pub waterlogged : bool,
        /// `type` state.
        pub kind : Kind,
    }
    /// `type` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Kind {
        /// `top` variant.
        Top,
        /// `bottom` variant.
        Bottom,
        /// `double` variant.
        Double,
    }
    impl super::Block for TuffBrickSlab {
    }
    impl From<TuffBrickSlab> for super::BlockState {
        fn from(value : TuffBrickSlab) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:cut_copper_stairs` block.
pub mod cut_copper_stairs {
    /// `minecraft:cut_copper_stairs` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct CutCopperStairs {
        /// `facing` state.
        pub facing : Facing,
        /// `half` state.
        pub half : Half,
        /// `shape` state.
        pub shape : Shape,
        /// `waterlogged` state.
        pub waterlogged : bool,
    }
    /// `facing` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Facing {
        /// `north` variant.
        North,
        /// `south` variant.
        South,
        /// `west` variant.
        West,
        /// `east` variant.
        East,
    }
    /// `half` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Half {
        /// `top` variant.
        Top,
        /// `bottom` variant.
        Bottom,
    }
    /// `shape` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Shape {
        /// `straight` variant.
        Straight,
        /// `inner_left` variant.
        InnerLeft,
        /// `inner_right` variant.
        InnerRight,
        /// `outer_left` variant.
        OuterLeft,
        /// `outer_right` variant.
        OuterRight,
    }
    impl super::Block for CutCopperStairs {
    }
    impl From<CutCopperStairs> for super::BlockState {
        fn from(value : CutCopperStairs) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:mangrove_wood` block.
pub mod mangrove_wood {
    /// `minecraft:mangrove_wood` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct MangroveWood {
        /// `axis` state.
        pub axis : Axis,
    }
    /// `axis` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Axis {
        /// `x` variant.
        X,
        /// `y` variant.
        Y,
        /// `z` variant.
        Z,
    }
    impl super::Block for MangroveWood {
    }
    impl From<MangroveWood> for super::BlockState {
        fn from(value : MangroveWood) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:stripped_oak_wood` block.
pub mod stripped_oak_wood {
    /// `minecraft:stripped_oak_wood` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct StrippedOakWood {
        /// `axis` state.
        pub axis : Axis,
    }
    /// `axis` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Axis {
        /// `x` variant.
        X,
        /// `y` variant.
        Y,
        /// `z` variant.
        Z,
    }
    impl super::Block for StrippedOakWood {
    }
    impl From<StrippedOakWood> for super::BlockState {
        fn from(value : StrippedOakWood) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:copper_bulb` block.
pub mod copper_bulb {
    /// `minecraft:copper_bulb` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct CopperBulb {
        /// `lit` state.
        pub lit : bool,
        /// `powered` state.
        pub powered : bool,
    }
    impl super::Block for CopperBulb {
    }
    impl From<CopperBulb> for super::BlockState {
        fn from(value : CopperBulb) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:note_block` block.
pub mod note_block {
    /// `minecraft:note_block` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct NoteBlock {
        /// `instrument` state.
        pub instrument : Instrument,
        /// `powered` state.
        pub powered : bool,
        /// `note` state.
        pub note : Note,
    }
    /// `instrument` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Instrument {
        /// `harp` variant.
        Harp,
        /// `basedrum` variant.
        Basedrum,
        /// `snare` variant.
        Snare,
        /// `hat` variant.
        Hat,
        /// `bass` variant.
        Bass,
        /// `flute` variant.
        Flute,
        /// `bell` variant.
        Bell,
        /// `guitar` variant.
        Guitar,
        /// `chime` variant.
        Chime,
        /// `xylophone` variant.
        Xylophone,
        /// `iron_xylophone` variant.
        IronXylophone,
        /// `cow_bell` variant.
        CowBell,
        /// `didgeridoo` variant.
        Didgeridoo,
        /// `bit` variant.
        Bit,
        /// `banjo` variant.
        Banjo,
        /// `pling` variant.
        Pling,
        /// `zombie` variant.
        Zombie,
        /// `skeleton` variant.
        Skeleton,
        /// `creeper` variant.
        Creeper,
        /// `dragon` variant.
        Dragon,
        /// `wither_skeleton` variant.
        WitherSkeleton,
        /// `piglin` variant.
        Piglin,
        /// `custom_head` variant.
        CustomHead,
    }
    /// `note` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Note {
        /// `0` variant.
        N0,
        /// `1` variant.
        N1,
        /// `2` variant.
        N2,
        /// `3` variant.
        N3,
        /// `4` variant.
        N4,
        /// `5` variant.
        N5,
        /// `6` variant.
        N6,
        /// `7` variant.
        N7,
        /// `8` variant.
        N8,
        /// `9` variant.
        N9,
        /// `10` variant.
        N10,
        /// `11` variant.
        N11,
        /// `12` variant.
        N12,
        /// `13` variant.
        N13,
        /// `14` variant.
        N14,
        /// `15` variant.
        N15,
        /// `16` variant.
        N16,
        /// `17` variant.
        N17,
        /// `18` variant.
        N18,
        /// `19` variant.
        N19,
        /// `20` variant.
        N20,
        /// `21` variant.
        N21,
        /// `22` variant.
        N22,
        /// `23` variant.
        N23,
        /// `24` variant.
        N24,
    }
    impl super::Block for NoteBlock {
    }
    impl From<NoteBlock> for super::BlockState {
        fn from(value : NoteBlock) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:acacia_trapdoor` block.
pub mod acacia_trapdoor {
    /// `minecraft:acacia_trapdoor` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct AcaciaTrapdoor {
        /// `open` state.
        pub open : bool,
        /// `facing` state.
        pub facing : Facing,
        /// `powered` state.
        pub powered : bool,
        /// `waterlogged` state.
        pub waterlogged : bool,
        /// `half` state.
        pub half : Half,
    }
    /// `facing` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Facing {
        /// `north` variant.
        North,
        /// `south` variant.
        South,
        /// `west` variant.
        West,
        /// `east` variant.
        East,
    }
    /// `half` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Half {
        /// `top` variant.
        Top,
        /// `bottom` variant.
        Bottom,
    }
    impl super::Block for AcaciaTrapdoor {
    }
    impl From<AcaciaTrapdoor> for super::BlockState {
        fn from(value : AcaciaTrapdoor) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:gold_ore` block.
pub mod gold_ore {
    /// `minecraft:gold_ore` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct GoldOre;
    impl super::Block for GoldOre {
    }
    impl From<GoldOre> for super::BlockState {
        fn from(value : GoldOre) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:potted_dead_bush` block.
pub mod potted_dead_bush {
    /// `minecraft:potted_dead_bush` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct PottedDeadBush;
    impl super::Block for PottedDeadBush {
    }
    impl From<PottedDeadBush> for super::BlockState {
        fn from(value : PottedDeadBush) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:pale_oak_trapdoor` block.
pub mod pale_oak_trapdoor {
    /// `minecraft:pale_oak_trapdoor` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct PaleOakTrapdoor {
        /// `open` state.
        pub open : bool,
        /// `half` state.
        pub half : Half,
        /// `powered` state.
        pub powered : bool,
        /// `facing` state.
        pub facing : Facing,
        /// `waterlogged` state.
        pub waterlogged : bool,
    }
    /// `half` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Half {
        /// `top` variant.
        Top,
        /// `bottom` variant.
        Bottom,
    }
    /// `facing` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Facing {
        /// `north` variant.
        North,
        /// `south` variant.
        South,
        /// `west` variant.
        West,
        /// `east` variant.
        East,
    }
    impl super::Block for PaleOakTrapdoor {
    }
    impl From<PaleOakTrapdoor> for super::BlockState {
        fn from(value : PaleOakTrapdoor) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:pink_carpet` block.
pub mod pink_carpet {
    /// `minecraft:pink_carpet` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct PinkCarpet;
    impl super::Block for PinkCarpet {
    }
    impl From<PinkCarpet> for super::BlockState {
        fn from(value : PinkCarpet) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:redstone_ore` block.
pub mod redstone_ore {
    /// `minecraft:redstone_ore` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct RedstoneOre {
        /// `lit` state.
        pub lit : bool,
    }
    impl super::Block for RedstoneOre {
    }
    impl From<RedstoneOre> for super::BlockState {
        fn from(value : RedstoneOre) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:dark_oak_stairs` block.
pub mod dark_oak_stairs {
    /// `minecraft:dark_oak_stairs` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct DarkOakStairs {
        /// `facing` state.
        pub facing : Facing,
        /// `waterlogged` state.
        pub waterlogged : bool,
        /// `half` state.
        pub half : Half,
        /// `shape` state.
        pub shape : Shape,
    }
    /// `facing` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Facing {
        /// `north` variant.
        North,
        /// `south` variant.
        South,
        /// `west` variant.
        West,
        /// `east` variant.
        East,
    }
    /// `half` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Half {
        /// `top` variant.
        Top,
        /// `bottom` variant.
        Bottom,
    }
    /// `shape` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Shape {
        /// `straight` variant.
        Straight,
        /// `inner_left` variant.
        InnerLeft,
        /// `inner_right` variant.
        InnerRight,
        /// `outer_left` variant.
        OuterLeft,
        /// `outer_right` variant.
        OuterRight,
    }
    impl super::Block for DarkOakStairs {
    }
    impl From<DarkOakStairs> for super::BlockState {
        fn from(value : DarkOakStairs) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:chain_command_block` block.
pub mod chain_command_block {
    /// `minecraft:chain_command_block` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct ChainCommandBlock {
        /// `facing` state.
        pub facing : Facing,
        /// `conditional` state.
        pub conditional : bool,
    }
    /// `facing` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Facing {
        /// `north` variant.
        North,
        /// `east` variant.
        East,
        /// `south` variant.
        South,
        /// `west` variant.
        West,
        /// `up` variant.
        Up,
        /// `down` variant.
        Down,
    }
    impl super::Block for ChainCommandBlock {
    }
    impl From<ChainCommandBlock> for super::BlockState {
        fn from(value : ChainCommandBlock) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:deepslate_brick_wall` block.
pub mod deepslate_brick_wall {
    /// `minecraft:deepslate_brick_wall` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct DeepslateBrickWall {
        /// `west` state.
        pub west : West,
        /// `east` state.
        pub east : East,
        /// `south` state.
        pub south : South,
        /// `waterlogged` state.
        pub waterlogged : bool,
        /// `north` state.
        pub north : North,
        /// `up` state.
        pub up : bool,
    }
    /// `west` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum West {
        /// `none` variant.
        None,
        /// `low` variant.
        Low,
        /// `tall` variant.
        Tall,
    }
    /// `east` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum East {
        /// `none` variant.
        None,
        /// `low` variant.
        Low,
        /// `tall` variant.
        Tall,
    }
    /// `south` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum South {
        /// `none` variant.
        None,
        /// `low` variant.
        Low,
        /// `tall` variant.
        Tall,
    }
    /// `north` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum North {
        /// `none` variant.
        None,
        /// `low` variant.
        Low,
        /// `tall` variant.
        Tall,
    }
    impl super::Block for DeepslateBrickWall {
    }
    impl From<DeepslateBrickWall> for super::BlockState {
        fn from(value : DeepslateBrickWall) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:end_gateway` block.
pub mod end_gateway {
    /// `minecraft:end_gateway` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct EndGateway;
    impl super::Block for EndGateway {
    }
    impl From<EndGateway> for super::BlockState {
        fn from(value : EndGateway) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:weathered_cut_copper` block.
pub mod weathered_cut_copper {
    /// `minecraft:weathered_cut_copper` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct WeatheredCutCopper;
    impl super::Block for WeatheredCutCopper {
    }
    impl From<WeatheredCutCopper> for super::BlockState {
        fn from(value : WeatheredCutCopper) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:potted_mangrove_propagule` block.
pub mod potted_mangrove_propagule {
    /// `minecraft:potted_mangrove_propagule` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct PottedMangrovePropagule;
    impl super::Block for PottedMangrovePropagule {
    }
    impl From<PottedMangrovePropagule> for super::BlockState {
        fn from(value : PottedMangrovePropagule) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:fire_coral_block` block.
pub mod fire_coral_block {
    /// `minecraft:fire_coral_block` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct FireCoralBlock;
    impl super::Block for FireCoralBlock {
    }
    impl From<FireCoralBlock> for super::BlockState {
        fn from(value : FireCoralBlock) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:crafter` block.
pub mod crafter {
    /// `minecraft:crafter` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct Crafter {
        /// `orientation` state.
        pub orientation : Orientation,
        /// `triggered` state.
        pub triggered : bool,
        /// `crafting` state.
        pub crafting : bool,
    }
    /// `orientation` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Orientation {
        /// `down_east` variant.
        DownEast,
        /// `down_north` variant.
        DownNorth,
        /// `down_south` variant.
        DownSouth,
        /// `down_west` variant.
        DownWest,
        /// `up_east` variant.
        UpEast,
        /// `up_north` variant.
        UpNorth,
        /// `up_south` variant.
        UpSouth,
        /// `up_west` variant.
        UpWest,
        /// `west_up` variant.
        WestUp,
        /// `east_up` variant.
        EastUp,
        /// `north_up` variant.
        NorthUp,
        /// `south_up` variant.
        SouthUp,
    }
    impl super::Block for Crafter {
    }
    impl From<Crafter> for super::BlockState {
        fn from(value : Crafter) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:deepslate_brick_stairs` block.
pub mod deepslate_brick_stairs {
    /// `minecraft:deepslate_brick_stairs` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct DeepslateBrickStairs {
        /// `shape` state.
        pub shape : Shape,
        /// `facing` state.
        pub facing : Facing,
        /// `waterlogged` state.
        pub waterlogged : bool,
        /// `half` state.
        pub half : Half,
    }
    /// `shape` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Shape {
        /// `straight` variant.
        Straight,
        /// `inner_left` variant.
        InnerLeft,
        /// `inner_right` variant.
        InnerRight,
        /// `outer_left` variant.
        OuterLeft,
        /// `outer_right` variant.
        OuterRight,
    }
    /// `facing` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Facing {
        /// `north` variant.
        North,
        /// `south` variant.
        South,
        /// `west` variant.
        West,
        /// `east` variant.
        East,
    }
    /// `half` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Half {
        /// `top` variant.
        Top,
        /// `bottom` variant.
        Bottom,
    }
    impl super::Block for DeepslateBrickStairs {
    }
    impl From<DeepslateBrickStairs> for super::BlockState {
        fn from(value : DeepslateBrickStairs) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:nether_brick_slab` block.
pub mod nether_brick_slab {
    /// `minecraft:nether_brick_slab` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct NetherBrickSlab {
        /// `waterlogged` state.
        pub waterlogged : bool,
        /// `type` state.
        pub kind : Kind,
    }
    /// `type` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Kind {
        /// `top` variant.
        Top,
        /// `bottom` variant.
        Bottom,
        /// `double` variant.
        Double,
    }
    impl super::Block for NetherBrickSlab {
    }
    impl From<NetherBrickSlab> for super::BlockState {
        fn from(value : NetherBrickSlab) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:brown_glazed_terracotta` block.
pub mod brown_glazed_terracotta {
    /// `minecraft:brown_glazed_terracotta` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct BrownGlazedTerracotta {
        /// `facing` state.
        pub facing : Facing,
    }
    /// `facing` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Facing {
        /// `north` variant.
        North,
        /// `south` variant.
        South,
        /// `west` variant.
        West,
        /// `east` variant.
        East,
    }
    impl super::Block for BrownGlazedTerracotta {
    }
    impl From<BrownGlazedTerracotta> for super::BlockState {
        fn from(value : BrownGlazedTerracotta) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:purple_concrete` block.
pub mod purple_concrete {
    /// `minecraft:purple_concrete` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct PurpleConcrete;
    impl super::Block for PurpleConcrete {
    }
    impl From<PurpleConcrete> for super::BlockState {
        fn from(value : PurpleConcrete) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:blue_shulker_box` block.
pub mod blue_shulker_box {
    /// `minecraft:blue_shulker_box` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct BlueShulkerBox {
        /// `facing` state.
        pub facing : Facing,
    }
    /// `facing` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Facing {
        /// `north` variant.
        North,
        /// `east` variant.
        East,
        /// `south` variant.
        South,
        /// `west` variant.
        West,
        /// `up` variant.
        Up,
        /// `down` variant.
        Down,
    }
    impl super::Block for BlueShulkerBox {
    }
    impl From<BlueShulkerBox> for super::BlockState {
        fn from(value : BlueShulkerBox) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:polished_blackstone_slab` block.
pub mod polished_blackstone_slab {
    /// `minecraft:polished_blackstone_slab` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct PolishedBlackstoneSlab {
        /// `waterlogged` state.
        pub waterlogged : bool,
        /// `type` state.
        pub kind : Kind,
    }
    /// `type` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Kind {
        /// `top` variant.
        Top,
        /// `bottom` variant.
        Bottom,
        /// `double` variant.
        Double,
    }
    impl super::Block for PolishedBlackstoneSlab {
    }
    impl From<PolishedBlackstoneSlab> for super::BlockState {
        fn from(value : PolishedBlackstoneSlab) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:oak_fence` block.
pub mod oak_fence {
    /// `minecraft:oak_fence` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct OakFence {
        /// `waterlogged` state.
        pub waterlogged : bool,
        /// `east` state.
        pub east : bool,
        /// `south` state.
        pub south : bool,
        /// `west` state.
        pub west : bool,
        /// `north` state.
        pub north : bool,
    }
    impl super::Block for OakFence {
    }
    impl From<OakFence> for super::BlockState {
        fn from(value : OakFence) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:dead_brain_coral_wall_fan` block.
pub mod dead_brain_coral_wall_fan {
    /// `minecraft:dead_brain_coral_wall_fan` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct DeadBrainCoralWallFan {
        /// `waterlogged` state.
        pub waterlogged : bool,
        /// `facing` state.
        pub facing : Facing,
    }
    /// `facing` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Facing {
        /// `north` variant.
        North,
        /// `south` variant.
        South,
        /// `west` variant.
        West,
        /// `east` variant.
        East,
    }
    impl super::Block for DeadBrainCoralWallFan {
    }
    impl From<DeadBrainCoralWallFan> for super::BlockState {
        fn from(value : DeadBrainCoralWallFan) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:light_blue_candle` block.
pub mod light_blue_candle {
    /// `minecraft:light_blue_candle` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct LightBlueCandle {
        /// `waterlogged` state.
        pub waterlogged : bool,
        /// `candles` state.
        pub candles : Candles,
        /// `lit` state.
        pub lit : bool,
    }
    /// `candles` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Candles {
        /// `1` variant.
        N1,
        /// `2` variant.
        N2,
        /// `3` variant.
        N3,
        /// `4` variant.
        N4,
    }
    impl super::Block for LightBlueCandle {
    }
    impl From<LightBlueCandle> for super::BlockState {
        fn from(value : LightBlueCandle) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:chiseled_stone_bricks` block.
pub mod chiseled_stone_bricks {
    /// `minecraft:chiseled_stone_bricks` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct ChiseledStoneBricks;
    impl super::Block for ChiseledStoneBricks {
    }
    impl From<ChiseledStoneBricks> for super::BlockState {
        fn from(value : ChiseledStoneBricks) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:end_portal_frame` block.
pub mod end_portal_frame {
    /// `minecraft:end_portal_frame` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct EndPortalFrame {
        /// `facing` state.
        pub facing : Facing,
        /// `eye` state.
        pub eye : bool,
    }
    /// `facing` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Facing {
        /// `north` variant.
        North,
        /// `south` variant.
        South,
        /// `west` variant.
        West,
        /// `east` variant.
        East,
    }
    impl super::Block for EndPortalFrame {
    }
    impl From<EndPortalFrame> for super::BlockState {
        fn from(value : EndPortalFrame) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:mangrove_wall_sign` block.
pub mod mangrove_wall_sign {
    /// `minecraft:mangrove_wall_sign` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct MangroveWallSign {
        /// `waterlogged` state.
        pub waterlogged : bool,
        /// `facing` state.
        pub facing : Facing,
    }
    /// `facing` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Facing {
        /// `north` variant.
        North,
        /// `south` variant.
        South,
        /// `west` variant.
        West,
        /// `east` variant.
        East,
    }
    impl super::Block for MangroveWallSign {
    }
    impl From<MangroveWallSign> for super::BlockState {
        fn from(value : MangroveWallSign) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:stone_brick_stairs` block.
pub mod stone_brick_stairs {
    /// `minecraft:stone_brick_stairs` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct StoneBrickStairs {
        /// `shape` state.
        pub shape : Shape,
        /// `facing` state.
        pub facing : Facing,
        /// `half` state.
        pub half : Half,
        /// `waterlogged` state.
        pub waterlogged : bool,
    }
    /// `shape` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Shape {
        /// `straight` variant.
        Straight,
        /// `inner_left` variant.
        InnerLeft,
        /// `inner_right` variant.
        InnerRight,
        /// `outer_left` variant.
        OuterLeft,
        /// `outer_right` variant.
        OuterRight,
    }
    /// `facing` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Facing {
        /// `north` variant.
        North,
        /// `south` variant.
        South,
        /// `west` variant.
        West,
        /// `east` variant.
        East,
    }
    /// `half` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Half {
        /// `top` variant.
        Top,
        /// `bottom` variant.
        Bottom,
    }
    impl super::Block for StoneBrickStairs {
    }
    impl From<StoneBrickStairs> for super::BlockState {
        fn from(value : StoneBrickStairs) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:vault` block.
pub mod vault {
    /// `minecraft:vault` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct Vault {
        /// `facing` state.
        pub facing : Facing,
        /// `ominous` state.
        pub ominous : bool,
        /// `vault_state` state.
        pub vault_state : VaultState,
    }
    /// `facing` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Facing {
        /// `north` variant.
        North,
        /// `south` variant.
        South,
        /// `west` variant.
        West,
        /// `east` variant.
        East,
    }
    /// `vault_state` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum VaultState {
        /// `inactive` variant.
        Inactive,
        /// `active` variant.
        Active,
        /// `unlocking` variant.
        Unlocking,
        /// `ejecting` variant.
        Ejecting,
    }
    impl super::Block for Vault {
    }
    impl From<Vault> for super::BlockState {
        fn from(value : Vault) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:bee_nest` block.
pub mod bee_nest {
    /// `minecraft:bee_nest` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct BeeNest {
        /// `facing` state.
        pub facing : Facing,
        /// `honey_level` state.
        pub honey_level : HoneyLevel,
    }
    /// `facing` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Facing {
        /// `north` variant.
        North,
        /// `south` variant.
        South,
        /// `west` variant.
        West,
        /// `east` variant.
        East,
    }
    /// `honey_level` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum HoneyLevel {
        /// `0` variant.
        N0,
        /// `1` variant.
        N1,
        /// `2` variant.
        N2,
        /// `3` variant.
        N3,
        /// `4` variant.
        N4,
        /// `5` variant.
        N5,
    }
    impl super::Block for BeeNest {
    }
    impl From<BeeNest> for super::BlockState {
        fn from(value : BeeNest) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:potted_cherry_sapling` block.
pub mod potted_cherry_sapling {
    /// `minecraft:potted_cherry_sapling` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct PottedCherrySapling;
    impl super::Block for PottedCherrySapling {
    }
    impl From<PottedCherrySapling> for super::BlockState {
        fn from(value : PottedCherrySapling) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:red_glazed_terracotta` block.
pub mod red_glazed_terracotta {
    /// `minecraft:red_glazed_terracotta` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct RedGlazedTerracotta {
        /// `facing` state.
        pub facing : Facing,
    }
    /// `facing` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Facing {
        /// `north` variant.
        North,
        /// `south` variant.
        South,
        /// `west` variant.
        West,
        /// `east` variant.
        East,
    }
    impl super::Block for RedGlazedTerracotta {
    }
    impl From<RedGlazedTerracotta> for super::BlockState {
        fn from(value : RedGlazedTerracotta) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:cut_copper` block.
pub mod cut_copper {
    /// `minecraft:cut_copper` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct CutCopper;
    impl super::Block for CutCopper {
    }
    impl From<CutCopper> for super::BlockState {
        fn from(value : CutCopper) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:dark_oak_pressure_plate` block.
pub mod dark_oak_pressure_plate {
    /// `minecraft:dark_oak_pressure_plate` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct DarkOakPressurePlate {
        /// `powered` state.
        pub powered : bool,
    }
    impl super::Block for DarkOakPressurePlate {
    }
    impl From<DarkOakPressurePlate> for super::BlockState {
        fn from(value : DarkOakPressurePlate) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:polished_blackstone_button` block.
pub mod polished_blackstone_button {
    /// `minecraft:polished_blackstone_button` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct PolishedBlackstoneButton {
        /// `face` state.
        pub face : Face,
        /// `powered` state.
        pub powered : bool,
        /// `facing` state.
        pub facing : Facing,
    }
    /// `face` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Face {
        /// `floor` variant.
        Floor,
        /// `wall` variant.
        Wall,
        /// `ceiling` variant.
        Ceiling,
    }
    /// `facing` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Facing {
        /// `north` variant.
        North,
        /// `south` variant.
        South,
        /// `west` variant.
        West,
        /// `east` variant.
        East,
    }
    impl super::Block for PolishedBlackstoneButton {
    }
    impl From<PolishedBlackstoneButton> for super::BlockState {
        fn from(value : PolishedBlackstoneButton) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:lime_glazed_terracotta` block.
pub mod lime_glazed_terracotta {
    /// `minecraft:lime_glazed_terracotta` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct LimeGlazedTerracotta {
        /// `facing` state.
        pub facing : Facing,
    }
    /// `facing` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Facing {
        /// `north` variant.
        North,
        /// `south` variant.
        South,
        /// `west` variant.
        West,
        /// `east` variant.
        East,
    }
    impl super::Block for LimeGlazedTerracotta {
    }
    impl From<LimeGlazedTerracotta> for super::BlockState {
        fn from(value : LimeGlazedTerracotta) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:purple_stained_glass` block.
pub mod purple_stained_glass {
    /// `minecraft:purple_stained_glass` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct PurpleStainedGlass;
    impl super::Block for PurpleStainedGlass {
    }
    impl From<PurpleStainedGlass> for super::BlockState {
        fn from(value : PurpleStainedGlass) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:prismarine_brick_slab` block.
pub mod prismarine_brick_slab {
    /// `minecraft:prismarine_brick_slab` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct PrismarineBrickSlab {
        /// `type` state.
        pub kind : Kind,
        /// `waterlogged` state.
        pub waterlogged : bool,
    }
    /// `type` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Kind {
        /// `top` variant.
        Top,
        /// `bottom` variant.
        Bottom,
        /// `double` variant.
        Double,
    }
    impl super::Block for PrismarineBrickSlab {
    }
    impl From<PrismarineBrickSlab> for super::BlockState {
        fn from(value : PrismarineBrickSlab) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:tripwire_hook` block.
pub mod tripwire_hook {
    /// `minecraft:tripwire_hook` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct TripwireHook {
        /// `attached` state.
        pub attached : bool,
        /// `powered` state.
        pub powered : bool,
        /// `facing` state.
        pub facing : Facing,
    }
    /// `facing` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Facing {
        /// `north` variant.
        North,
        /// `south` variant.
        South,
        /// `west` variant.
        West,
        /// `east` variant.
        East,
    }
    impl super::Block for TripwireHook {
    }
    impl From<TripwireHook> for super::BlockState {
        fn from(value : TripwireHook) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:oak_fence_gate` block.
pub mod oak_fence_gate {
    /// `minecraft:oak_fence_gate` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct OakFenceGate {
        /// `facing` state.
        pub facing : Facing,
        /// `powered` state.
        pub powered : bool,
        /// `in_wall` state.
        pub in_wall : bool,
        /// `open` state.
        pub open : bool,
    }
    /// `facing` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Facing {
        /// `north` variant.
        North,
        /// `south` variant.
        South,
        /// `west` variant.
        West,
        /// `east` variant.
        East,
    }
    impl super::Block for OakFenceGate {
    }
    impl From<OakFenceGate> for super::BlockState {
        fn from(value : OakFenceGate) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:dark_oak_wall_hanging_sign` block.
pub mod dark_oak_wall_hanging_sign {
    /// `minecraft:dark_oak_wall_hanging_sign` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct DarkOakWallHangingSign {
        /// `facing` state.
        pub facing : Facing,
        /// `waterlogged` state.
        pub waterlogged : bool,
    }
    /// `facing` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Facing {
        /// `north` variant.
        North,
        /// `south` variant.
        South,
        /// `west` variant.
        West,
        /// `east` variant.
        East,
    }
    impl super::Block for DarkOakWallHangingSign {
    }
    impl From<DarkOakWallHangingSign> for super::BlockState {
        fn from(value : DarkOakWallHangingSign) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:observer` block.
pub mod observer {
    /// `minecraft:observer` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct Observer {
        /// `facing` state.
        pub facing : Facing,
        /// `powered` state.
        pub powered : bool,
    }
    /// `facing` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Facing {
        /// `north` variant.
        North,
        /// `east` variant.
        East,
        /// `south` variant.
        South,
        /// `west` variant.
        West,
        /// `up` variant.
        Up,
        /// `down` variant.
        Down,
    }
    impl super::Block for Observer {
    }
    impl From<Observer> for super::BlockState {
        fn from(value : Observer) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:dark_oak_planks` block.
pub mod dark_oak_planks {
    /// `minecraft:dark_oak_planks` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct DarkOakPlanks;
    impl super::Block for DarkOakPlanks {
    }
    impl From<DarkOakPlanks> for super::BlockState {
        fn from(value : DarkOakPlanks) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:tube_coral_block` block.
pub mod tube_coral_block {
    /// `minecraft:tube_coral_block` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct TubeCoralBlock;
    impl super::Block for TubeCoralBlock {
    }
    impl From<TubeCoralBlock> for super::BlockState {
        fn from(value : TubeCoralBlock) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:melon_stem` block.
pub mod melon_stem {
    /// `minecraft:melon_stem` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct MelonStem {
        /// `age` state.
        pub age : Age,
    }
    /// `age` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Age {
        /// `0` variant.
        N0,
        /// `1` variant.
        N1,
        /// `2` variant.
        N2,
        /// `3` variant.
        N3,
        /// `4` variant.
        N4,
        /// `5` variant.
        N5,
        /// `6` variant.
        N6,
        /// `7` variant.
        N7,
    }
    impl super::Block for MelonStem {
    }
    impl From<MelonStem> for super::BlockState {
        fn from(value : MelonStem) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:target` block.
pub mod target {
    /// `minecraft:target` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct Target {
        /// `power` state.
        pub power : Power,
    }
    /// `power` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Power {
        /// `0` variant.
        N0,
        /// `1` variant.
        N1,
        /// `2` variant.
        N2,
        /// `3` variant.
        N3,
        /// `4` variant.
        N4,
        /// `5` variant.
        N5,
        /// `6` variant.
        N6,
        /// `7` variant.
        N7,
        /// `8` variant.
        N8,
        /// `9` variant.
        N9,
        /// `10` variant.
        N10,
        /// `11` variant.
        N11,
        /// `12` variant.
        N12,
        /// `13` variant.
        N13,
        /// `14` variant.
        N14,
        /// `15` variant.
        N15,
    }
    impl super::Block for Target {
    }
    impl From<Target> for super::BlockState {
        fn from(value : Target) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:mushroom_stem` block.
pub mod mushroom_stem {
    /// `minecraft:mushroom_stem` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct MushroomStem {
        /// `east` state.
        pub east : bool,
        /// `west` state.
        pub west : bool,
        /// `up` state.
        pub up : bool,
        /// `south` state.
        pub south : bool,
        /// `north` state.
        pub north : bool,
        /// `down` state.
        pub down : bool,
    }
    impl super::Block for MushroomStem {
    }
    impl From<MushroomStem> for super::BlockState {
        fn from(value : MushroomStem) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:end_rod` block.
pub mod end_rod {
    /// `minecraft:end_rod` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct EndRod {
        /// `facing` state.
        pub facing : Facing,
    }
    /// `facing` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Facing {
        /// `north` variant.
        North,
        /// `east` variant.
        East,
        /// `south` variant.
        South,
        /// `west` variant.
        West,
        /// `up` variant.
        Up,
        /// `down` variant.
        Down,
    }
    impl super::Block for EndRod {
    }
    impl From<EndRod> for super::BlockState {
        fn from(value : EndRod) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:cut_sandstone` block.
pub mod cut_sandstone {
    /// `minecraft:cut_sandstone` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct CutSandstone;
    impl super::Block for CutSandstone {
    }
    impl From<CutSandstone> for super::BlockState {
        fn from(value : CutSandstone) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:deepslate_tile_wall` block.
pub mod deepslate_tile_wall {
    /// `minecraft:deepslate_tile_wall` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct DeepslateTileWall {
        /// `west` state.
        pub west : West,
        /// `up` state.
        pub up : bool,
        /// `waterlogged` state.
        pub waterlogged : bool,
        /// `north` state.
        pub north : North,
        /// `east` state.
        pub east : East,
        /// `south` state.
        pub south : South,
    }
    /// `west` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum West {
        /// `none` variant.
        None,
        /// `low` variant.
        Low,
        /// `tall` variant.
        Tall,
    }
    /// `north` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum North {
        /// `none` variant.
        None,
        /// `low` variant.
        Low,
        /// `tall` variant.
        Tall,
    }
    /// `east` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum East {
        /// `none` variant.
        None,
        /// `low` variant.
        Low,
        /// `tall` variant.
        Tall,
    }
    /// `south` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum South {
        /// `none` variant.
        None,
        /// `low` variant.
        Low,
        /// `tall` variant.
        Tall,
    }
    impl super::Block for DeepslateTileWall {
    }
    impl From<DeepslateTileWall> for super::BlockState {
        fn from(value : DeepslateTileWall) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:exposed_cut_copper` block.
pub mod exposed_cut_copper {
    /// `minecraft:exposed_cut_copper` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct ExposedCutCopper;
    impl super::Block for ExposedCutCopper {
    }
    impl From<ExposedCutCopper> for super::BlockState {
        fn from(value : ExposedCutCopper) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:orange_wall_banner` block.
pub mod orange_wall_banner {
    /// `minecraft:orange_wall_banner` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct OrangeWallBanner {
        /// `facing` state.
        pub facing : Facing,
    }
    /// `facing` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Facing {
        /// `north` variant.
        North,
        /// `south` variant.
        South,
        /// `west` variant.
        West,
        /// `east` variant.
        East,
    }
    impl super::Block for OrangeWallBanner {
    }
    impl From<OrangeWallBanner> for super::BlockState {
        fn from(value : OrangeWallBanner) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:polished_blackstone` block.
pub mod polished_blackstone {
    /// `minecraft:polished_blackstone` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct PolishedBlackstone;
    impl super::Block for PolishedBlackstone {
    }
    impl From<PolishedBlackstone> for super::BlockState {
        fn from(value : PolishedBlackstone) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:spruce_slab` block.
pub mod spruce_slab {
    /// `minecraft:spruce_slab` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct SpruceSlab {
        /// `waterlogged` state.
        pub waterlogged : bool,
        /// `type` state.
        pub kind : Kind,
    }
    /// `type` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Kind {
        /// `top` variant.
        Top,
        /// `bottom` variant.
        Bottom,
        /// `double` variant.
        Double,
    }
    impl super::Block for SpruceSlab {
    }
    impl From<SpruceSlab> for super::BlockState {
        fn from(value : SpruceSlab) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:mangrove_trapdoor` block.
pub mod mangrove_trapdoor {
    /// `minecraft:mangrove_trapdoor` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct MangroveTrapdoor {
        /// `open` state.
        pub open : bool,
        /// `facing` state.
        pub facing : Facing,
        /// `half` state.
        pub half : Half,
        /// `waterlogged` state.
        pub waterlogged : bool,
        /// `powered` state.
        pub powered : bool,
    }
    /// `facing` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Facing {
        /// `north` variant.
        North,
        /// `south` variant.
        South,
        /// `west` variant.
        West,
        /// `east` variant.
        East,
    }
    /// `half` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Half {
        /// `top` variant.
        Top,
        /// `bottom` variant.
        Bottom,
    }
    impl super::Block for MangroveTrapdoor {
    }
    impl From<MangroveTrapdoor> for super::BlockState {
        fn from(value : MangroveTrapdoor) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:pale_oak_fence` block.
pub mod pale_oak_fence {
    /// `minecraft:pale_oak_fence` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct PaleOakFence {
        /// `south` state.
        pub south : bool,
        /// `waterlogged` state.
        pub waterlogged : bool,
        /// `west` state.
        pub west : bool,
        /// `north` state.
        pub north : bool,
        /// `east` state.
        pub east : bool,
    }
    impl super::Block for PaleOakFence {
    }
    impl From<PaleOakFence> for super::BlockState {
        fn from(value : PaleOakFence) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:rose_bush` block.
pub mod rose_bush {
    /// `minecraft:rose_bush` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct RoseBush {
        /// `half` state.
        pub half : Half,
    }
    /// `half` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Half {
        /// `upper` variant.
        Upper,
        /// `lower` variant.
        Lower,
    }
    impl super::Block for RoseBush {
    }
    impl From<RoseBush> for super::BlockState {
        fn from(value : RoseBush) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:pale_hanging_moss` block.
pub mod pale_hanging_moss {
    /// `minecraft:pale_hanging_moss` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct PaleHangingMoss {
        /// `tip` state.
        pub tip : bool,
    }
    impl super::Block for PaleHangingMoss {
    }
    impl From<PaleHangingMoss> for super::BlockState {
        fn from(value : PaleHangingMoss) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:fire_coral_fan` block.
pub mod fire_coral_fan {
    /// `minecraft:fire_coral_fan` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct FireCoralFan {
        /// `waterlogged` state.
        pub waterlogged : bool,
    }
    impl super::Block for FireCoralFan {
    }
    impl From<FireCoralFan> for super::BlockState {
        fn from(value : FireCoralFan) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:potted_azure_bluet` block.
pub mod potted_azure_bluet {
    /// `minecraft:potted_azure_bluet` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct PottedAzureBluet;
    impl super::Block for PottedAzureBluet {
    }
    impl From<PottedAzureBluet> for super::BlockState {
        fn from(value : PottedAzureBluet) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:cyan_carpet` block.
pub mod cyan_carpet {
    /// `minecraft:cyan_carpet` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct CyanCarpet;
    impl super::Block for CyanCarpet {
    }
    impl From<CyanCarpet> for super::BlockState {
        fn from(value : CyanCarpet) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:black_bed` block.
pub mod black_bed {
    /// `minecraft:black_bed` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct BlackBed {
        /// `occupied` state.
        pub occupied : bool,
        /// `facing` state.
        pub facing : Facing,
        /// `part` state.
        pub part : Part,
    }
    /// `facing` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Facing {
        /// `north` variant.
        North,
        /// `south` variant.
        South,
        /// `west` variant.
        West,
        /// `east` variant.
        East,
    }
    /// `part` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Part {
        /// `head` variant.
        Head,
        /// `foot` variant.
        Foot,
    }
    impl super::Block for BlackBed {
    }
    impl From<BlackBed> for super::BlockState {
        fn from(value : BlackBed) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:blue_wall_banner` block.
pub mod blue_wall_banner {
    /// `minecraft:blue_wall_banner` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct BlueWallBanner {
        /// `facing` state.
        pub facing : Facing,
    }
    /// `facing` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Facing {
        /// `north` variant.
        North,
        /// `south` variant.
        South,
        /// `west` variant.
        West,
        /// `east` variant.
        East,
    }
    impl super::Block for BlueWallBanner {
    }
    impl From<BlueWallBanner> for super::BlockState {
        fn from(value : BlueWallBanner) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:cyan_candle_cake` block.
pub mod cyan_candle_cake {
    /// `minecraft:cyan_candle_cake` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct CyanCandleCake {
        /// `lit` state.
        pub lit : bool,
    }
    impl super::Block for CyanCandleCake {
    }
    impl From<CyanCandleCake> for super::BlockState {
        fn from(value : CyanCandleCake) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:nether_brick_fence` block.
pub mod nether_brick_fence {
    /// `minecraft:nether_brick_fence` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct NetherBrickFence {
        /// `west` state.
        pub west : bool,
        /// `north` state.
        pub north : bool,
        /// `east` state.
        pub east : bool,
        /// `south` state.
        pub south : bool,
        /// `waterlogged` state.
        pub waterlogged : bool,
    }
    impl super::Block for NetherBrickFence {
    }
    impl From<NetherBrickFence> for super::BlockState {
        fn from(value : NetherBrickFence) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:stripped_pale_oak_log` block.
pub mod stripped_pale_oak_log {
    /// `minecraft:stripped_pale_oak_log` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct StrippedPaleOakLog {
        /// `axis` state.
        pub axis : Axis,
    }
    /// `axis` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Axis {
        /// `x` variant.
        X,
        /// `y` variant.
        Y,
        /// `z` variant.
        Z,
    }
    impl super::Block for StrippedPaleOakLog {
    }
    impl From<StrippedPaleOakLog> for super::BlockState {
        fn from(value : StrippedPaleOakLog) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:wet_sponge` block.
pub mod wet_sponge {
    /// `minecraft:wet_sponge` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct WetSponge;
    impl super::Block for WetSponge {
    }
    impl From<WetSponge> for super::BlockState {
        fn from(value : WetSponge) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:gray_carpet` block.
pub mod gray_carpet {
    /// `minecraft:gray_carpet` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct GrayCarpet;
    impl super::Block for GrayCarpet {
    }
    impl From<GrayCarpet> for super::BlockState {
        fn from(value : GrayCarpet) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:birch_planks` block.
pub mod birch_planks {
    /// `minecraft:birch_planks` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct BirchPlanks;
    impl super::Block for BirchPlanks {
    }
    impl From<BirchPlanks> for super::BlockState {
        fn from(value : BirchPlanks) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:mangrove_planks` block.
pub mod mangrove_planks {
    /// `minecraft:mangrove_planks` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct MangrovePlanks;
    impl super::Block for MangrovePlanks {
    }
    impl From<MangrovePlanks> for super::BlockState {
        fn from(value : MangrovePlanks) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:composter` block.
pub mod composter {
    /// `minecraft:composter` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct Composter {
        /// `level` state.
        pub level : Level,
    }
    /// `level` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Level {
        /// `0` variant.
        N0,
        /// `1` variant.
        N1,
        /// `2` variant.
        N2,
        /// `3` variant.
        N3,
        /// `4` variant.
        N4,
        /// `5` variant.
        N5,
        /// `6` variant.
        N6,
        /// `7` variant.
        N7,
        /// `8` variant.
        N8,
    }
    impl super::Block for Composter {
    }
    impl From<Composter> for super::BlockState {
        fn from(value : Composter) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:red_nether_brick_stairs` block.
pub mod red_nether_brick_stairs {
    /// `minecraft:red_nether_brick_stairs` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct RedNetherBrickStairs {
        /// `shape` state.
        pub shape : Shape,
        /// `facing` state.
        pub facing : Facing,
        /// `waterlogged` state.
        pub waterlogged : bool,
        /// `half` state.
        pub half : Half,
    }
    /// `shape` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Shape {
        /// `straight` variant.
        Straight,
        /// `inner_left` variant.
        InnerLeft,
        /// `inner_right` variant.
        InnerRight,
        /// `outer_left` variant.
        OuterLeft,
        /// `outer_right` variant.
        OuterRight,
    }
    /// `facing` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Facing {
        /// `north` variant.
        North,
        /// `south` variant.
        South,
        /// `west` variant.
        West,
        /// `east` variant.
        East,
    }
    /// `half` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Half {
        /// `top` variant.
        Top,
        /// `bottom` variant.
        Bottom,
    }
    impl super::Block for RedNetherBrickStairs {
    }
    impl From<RedNetherBrickStairs> for super::BlockState {
        fn from(value : RedNetherBrickStairs) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:gray_stained_glass` block.
pub mod gray_stained_glass {
    /// `minecraft:gray_stained_glass` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct GrayStainedGlass;
    impl super::Block for GrayStainedGlass {
    }
    impl From<GrayStainedGlass> for super::BlockState {
        fn from(value : GrayStainedGlass) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:vine` block.
pub mod vine {
    /// `minecraft:vine` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct Vine {
        /// `north` state.
        pub north : bool,
        /// `south` state.
        pub south : bool,
        /// `east` state.
        pub east : bool,
        /// `up` state.
        pub up : bool,
        /// `west` state.
        pub west : bool,
    }
    impl super::Block for Vine {
    }
    impl From<Vine> for super::BlockState {
        fn from(value : Vine) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:cracked_deepslate_bricks` block.
pub mod cracked_deepslate_bricks {
    /// `minecraft:cracked_deepslate_bricks` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct CrackedDeepslateBricks;
    impl super::Block for CrackedDeepslateBricks {
    }
    impl From<CrackedDeepslateBricks> for super::BlockState {
        fn from(value : CrackedDeepslateBricks) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:oxeye_daisy` block.
pub mod oxeye_daisy {
    /// `minecraft:oxeye_daisy` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct OxeyeDaisy;
    impl super::Block for OxeyeDaisy {
    }
    impl From<OxeyeDaisy> for super::BlockState {
        fn from(value : OxeyeDaisy) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:warped_hanging_sign` block.
pub mod warped_hanging_sign {
    /// `minecraft:warped_hanging_sign` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct WarpedHangingSign {
        /// `rotation` state.
        pub rotation : Rotation,
        /// `waterlogged` state.
        pub waterlogged : bool,
        /// `attached` state.
        pub attached : bool,
    }
    /// `rotation` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Rotation {
        /// `0` variant.
        N0,
        /// `1` variant.
        N1,
        /// `2` variant.
        N2,
        /// `3` variant.
        N3,
        /// `4` variant.
        N4,
        /// `5` variant.
        N5,
        /// `6` variant.
        N6,
        /// `7` variant.
        N7,
        /// `8` variant.
        N8,
        /// `9` variant.
        N9,
        /// `10` variant.
        N10,
        /// `11` variant.
        N11,
        /// `12` variant.
        N12,
        /// `13` variant.
        N13,
        /// `14` variant.
        N14,
        /// `15` variant.
        N15,
    }
    impl super::Block for WarpedHangingSign {
    }
    impl From<WarpedHangingSign> for super::BlockState {
        fn from(value : WarpedHangingSign) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:deepslate_tile_slab` block.
pub mod deepslate_tile_slab {
    /// `minecraft:deepslate_tile_slab` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct DeepslateTileSlab {
        /// `waterlogged` state.
        pub waterlogged : bool,
        /// `type` state.
        pub kind : Kind,
    }
    /// `type` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Kind {
        /// `top` variant.
        Top,
        /// `bottom` variant.
        Bottom,
        /// `double` variant.
        Double,
    }
    impl super::Block for DeepslateTileSlab {
    }
    impl From<DeepslateTileSlab> for super::BlockState {
        fn from(value : DeepslateTileSlab) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:lime_candle` block.
pub mod lime_candle {
    /// `minecraft:lime_candle` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct LimeCandle {
        /// `lit` state.
        pub lit : bool,
        /// `waterlogged` state.
        pub waterlogged : bool,
        /// `candles` state.
        pub candles : Candles,
    }
    /// `candles` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Candles {
        /// `1` variant.
        N1,
        /// `2` variant.
        N2,
        /// `3` variant.
        N3,
        /// `4` variant.
        N4,
    }
    impl super::Block for LimeCandle {
    }
    impl From<LimeCandle> for super::BlockState {
        fn from(value : LimeCandle) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:magma_block` block.
pub mod magma_block {
    /// `minecraft:magma_block` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct MagmaBlock;
    impl super::Block for MagmaBlock {
    }
    impl From<MagmaBlock> for super::BlockState {
        fn from(value : MagmaBlock) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:bamboo_mosaic_stairs` block.
pub mod bamboo_mosaic_stairs {
    /// `minecraft:bamboo_mosaic_stairs` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct BambooMosaicStairs {
        /// `waterlogged` state.
        pub waterlogged : bool,
        /// `facing` state.
        pub facing : Facing,
        /// `half` state.
        pub half : Half,
        /// `shape` state.
        pub shape : Shape,
    }
    /// `facing` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Facing {
        /// `north` variant.
        North,
        /// `south` variant.
        South,
        /// `west` variant.
        West,
        /// `east` variant.
        East,
    }
    /// `half` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Half {
        /// `top` variant.
        Top,
        /// `bottom` variant.
        Bottom,
    }
    /// `shape` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Shape {
        /// `straight` variant.
        Straight,
        /// `inner_left` variant.
        InnerLeft,
        /// `inner_right` variant.
        InnerRight,
        /// `outer_left` variant.
        OuterLeft,
        /// `outer_right` variant.
        OuterRight,
    }
    impl super::Block for BambooMosaicStairs {
    }
    impl From<BambooMosaicStairs> for super::BlockState {
        fn from(value : BambooMosaicStairs) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:nether_brick_wall` block.
pub mod nether_brick_wall {
    /// `minecraft:nether_brick_wall` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct NetherBrickWall {
        /// `up` state.
        pub up : bool,
        /// `north` state.
        pub north : North,
        /// `waterlogged` state.
        pub waterlogged : bool,
        /// `west` state.
        pub west : West,
        /// `south` state.
        pub south : South,
        /// `east` state.
        pub east : East,
    }
    /// `north` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum North {
        /// `none` variant.
        None,
        /// `low` variant.
        Low,
        /// `tall` variant.
        Tall,
    }
    /// `west` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum West {
        /// `none` variant.
        None,
        /// `low` variant.
        Low,
        /// `tall` variant.
        Tall,
    }
    /// `south` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum South {
        /// `none` variant.
        None,
        /// `low` variant.
        Low,
        /// `tall` variant.
        Tall,
    }
    /// `east` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum East {
        /// `none` variant.
        None,
        /// `low` variant.
        Low,
        /// `tall` variant.
        Tall,
    }
    impl super::Block for NetherBrickWall {
    }
    impl From<NetherBrickWall> for super::BlockState {
        fn from(value : NetherBrickWall) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:shroomlight` block.
pub mod shroomlight {
    /// `minecraft:shroomlight` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct Shroomlight;
    impl super::Block for Shroomlight {
    }
    impl From<Shroomlight> for super::BlockState {
        fn from(value : Shroomlight) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:waxed_weathered_copper_door` block.
pub mod waxed_weathered_copper_door {
    /// `minecraft:waxed_weathered_copper_door` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct WaxedWeatheredCopperDoor {
        /// `open` state.
        pub open : bool,
        /// `powered` state.
        pub powered : bool,
        /// `half` state.
        pub half : Half,
        /// `hinge` state.
        pub hinge : Hinge,
        /// `facing` state.
        pub facing : Facing,
    }
    /// `half` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Half {
        /// `upper` variant.
        Upper,
        /// `lower` variant.
        Lower,
    }
    /// `hinge` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Hinge {
        /// `left` variant.
        Left,
        /// `right` variant.
        Right,
    }
    /// `facing` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Facing {
        /// `north` variant.
        North,
        /// `south` variant.
        South,
        /// `west` variant.
        West,
        /// `east` variant.
        East,
    }
    impl super::Block for WaxedWeatheredCopperDoor {
    }
    impl From<WaxedWeatheredCopperDoor> for super::BlockState {
        fn from(value : WaxedWeatheredCopperDoor) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:potted_red_mushroom` block.
pub mod potted_red_mushroom {
    /// `minecraft:potted_red_mushroom` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct PottedRedMushroom;
    impl super::Block for PottedRedMushroom {
    }
    impl From<PottedRedMushroom> for super::BlockState {
        fn from(value : PottedRedMushroom) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:polished_deepslate_slab` block.
pub mod polished_deepslate_slab {
    /// `minecraft:polished_deepslate_slab` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct PolishedDeepslateSlab {
        /// `waterlogged` state.
        pub waterlogged : bool,
        /// `type` state.
        pub kind : Kind,
    }
    /// `type` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Kind {
        /// `top` variant.
        Top,
        /// `bottom` variant.
        Bottom,
        /// `double` variant.
        Double,
    }
    impl super::Block for PolishedDeepslateSlab {
    }
    impl From<PolishedDeepslateSlab> for super::BlockState {
        fn from(value : PolishedDeepslateSlab) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:activator_rail` block.
pub mod activator_rail {
    /// `minecraft:activator_rail` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct ActivatorRail {
        /// `powered` state.
        pub powered : bool,
        /// `shape` state.
        pub shape : Shape,
        /// `waterlogged` state.
        pub waterlogged : bool,
    }
    /// `shape` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Shape {
        /// `north_south` variant.
        NorthSouth,
        /// `east_west` variant.
        EastWest,
        /// `ascending_east` variant.
        AscendingEast,
        /// `ascending_west` variant.
        AscendingWest,
        /// `ascending_north` variant.
        AscendingNorth,
        /// `ascending_south` variant.
        AscendingSouth,
    }
    impl super::Block for ActivatorRail {
    }
    impl From<ActivatorRail> for super::BlockState {
        fn from(value : ActivatorRail) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:dark_oak_leaves` block.
pub mod dark_oak_leaves {
    /// `minecraft:dark_oak_leaves` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct DarkOakLeaves {
        /// `distance` state.
        pub distance : Distance,
        /// `waterlogged` state.
        pub waterlogged : bool,
        /// `persistent` state.
        pub persistent : bool,
    }
    /// `distance` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Distance {
        /// `1` variant.
        N1,
        /// `2` variant.
        N2,
        /// `3` variant.
        N3,
        /// `4` variant.
        N4,
        /// `5` variant.
        N5,
        /// `6` variant.
        N6,
        /// `7` variant.
        N7,
    }
    impl super::Block for DarkOakLeaves {
    }
    impl From<DarkOakLeaves> for super::BlockState {
        fn from(value : DarkOakLeaves) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:blue_concrete_powder` block.
pub mod blue_concrete_powder {
    /// `minecraft:blue_concrete_powder` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct BlueConcretePowder;
    impl super::Block for BlueConcretePowder {
    }
    impl From<BlueConcretePowder> for super::BlockState {
        fn from(value : BlueConcretePowder) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:mud` block.
pub mod mud {
    /// `minecraft:mud` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct Mud;
    impl super::Block for Mud {
    }
    impl From<Mud> for super::BlockState {
        fn from(value : Mud) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:mangrove_button` block.
pub mod mangrove_button {
    /// `minecraft:mangrove_button` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct MangroveButton {
        /// `facing` state.
        pub facing : Facing,
        /// `face` state.
        pub face : Face,
        /// `powered` state.
        pub powered : bool,
    }
    /// `facing` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Facing {
        /// `north` variant.
        North,
        /// `south` variant.
        South,
        /// `west` variant.
        West,
        /// `east` variant.
        East,
    }
    /// `face` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Face {
        /// `floor` variant.
        Floor,
        /// `wall` variant.
        Wall,
        /// `ceiling` variant.
        Ceiling,
    }
    impl super::Block for MangroveButton {
    }
    impl From<MangroveButton> for super::BlockState {
        fn from(value : MangroveButton) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:exposed_copper_door` block.
pub mod exposed_copper_door {
    /// `minecraft:exposed_copper_door` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct ExposedCopperDoor {
        /// `facing` state.
        pub facing : Facing,
        /// `half` state.
        pub half : Half,
        /// `hinge` state.
        pub hinge : Hinge,
        /// `powered` state.
        pub powered : bool,
        /// `open` state.
        pub open : bool,
    }
    /// `facing` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Facing {
        /// `north` variant.
        North,
        /// `south` variant.
        South,
        /// `west` variant.
        West,
        /// `east` variant.
        East,
    }
    /// `half` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Half {
        /// `upper` variant.
        Upper,
        /// `lower` variant.
        Lower,
    }
    /// `hinge` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Hinge {
        /// `left` variant.
        Left,
        /// `right` variant.
        Right,
    }
    impl super::Block for ExposedCopperDoor {
    }
    impl From<ExposedCopperDoor> for super::BlockState {
        fn from(value : ExposedCopperDoor) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:player_head` block.
pub mod player_head {
    /// `minecraft:player_head` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct PlayerHead {
        /// `powered` state.
        pub powered : bool,
        /// `rotation` state.
        pub rotation : Rotation,
    }
    /// `rotation` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Rotation {
        /// `0` variant.
        N0,
        /// `1` variant.
        N1,
        /// `2` variant.
        N2,
        /// `3` variant.
        N3,
        /// `4` variant.
        N4,
        /// `5` variant.
        N5,
        /// `6` variant.
        N6,
        /// `7` variant.
        N7,
        /// `8` variant.
        N8,
        /// `9` variant.
        N9,
        /// `10` variant.
        N10,
        /// `11` variant.
        N11,
        /// `12` variant.
        N12,
        /// `13` variant.
        N13,
        /// `14` variant.
        N14,
        /// `15` variant.
        N15,
    }
    impl super::Block for PlayerHead {
    }
    impl From<PlayerHead> for super::BlockState {
        fn from(value : PlayerHead) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:reinforced_deepslate` block.
pub mod reinforced_deepslate {
    /// `minecraft:reinforced_deepslate` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct ReinforcedDeepslate;
    impl super::Block for ReinforcedDeepslate {
    }
    impl From<ReinforcedDeepslate> for super::BlockState {
        fn from(value : ReinforcedDeepslate) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:stripped_dark_oak_wood` block.
pub mod stripped_dark_oak_wood {
    /// `minecraft:stripped_dark_oak_wood` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct StrippedDarkOakWood {
        /// `axis` state.
        pub axis : Axis,
    }
    /// `axis` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Axis {
        /// `x` variant.
        X,
        /// `y` variant.
        Y,
        /// `z` variant.
        Z,
    }
    impl super::Block for StrippedDarkOakWood {
    }
    impl From<StrippedDarkOakWood> for super::BlockState {
        fn from(value : StrippedDarkOakWood) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:jungle_hanging_sign` block.
pub mod jungle_hanging_sign {
    /// `minecraft:jungle_hanging_sign` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct JungleHangingSign {
        /// `attached` state.
        pub attached : bool,
        /// `waterlogged` state.
        pub waterlogged : bool,
        /// `rotation` state.
        pub rotation : Rotation,
    }
    /// `rotation` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Rotation {
        /// `0` variant.
        N0,
        /// `1` variant.
        N1,
        /// `2` variant.
        N2,
        /// `3` variant.
        N3,
        /// `4` variant.
        N4,
        /// `5` variant.
        N5,
        /// `6` variant.
        N6,
        /// `7` variant.
        N7,
        /// `8` variant.
        N8,
        /// `9` variant.
        N9,
        /// `10` variant.
        N10,
        /// `11` variant.
        N11,
        /// `12` variant.
        N12,
        /// `13` variant.
        N13,
        /// `14` variant.
        N14,
        /// `15` variant.
        N15,
    }
    impl super::Block for JungleHangingSign {
    }
    impl From<JungleHangingSign> for super::BlockState {
        fn from(value : JungleHangingSign) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:crimson_hyphae` block.
pub mod crimson_hyphae {
    /// `minecraft:crimson_hyphae` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct CrimsonHyphae {
        /// `axis` state.
        pub axis : Axis,
    }
    /// `axis` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Axis {
        /// `x` variant.
        X,
        /// `y` variant.
        Y,
        /// `z` variant.
        Z,
    }
    impl super::Block for CrimsonHyphae {
    }
    impl From<CrimsonHyphae> for super::BlockState {
        fn from(value : CrimsonHyphae) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:clay` block.
pub mod clay {
    /// `minecraft:clay` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct Clay;
    impl super::Block for Clay {
    }
    impl From<Clay> for super::BlockState {
        fn from(value : Clay) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:purple_candle` block.
pub mod purple_candle {
    /// `minecraft:purple_candle` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct PurpleCandle {
        /// `waterlogged` state.
        pub waterlogged : bool,
        /// `lit` state.
        pub lit : bool,
        /// `candles` state.
        pub candles : Candles,
    }
    /// `candles` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Candles {
        /// `1` variant.
        N1,
        /// `2` variant.
        N2,
        /// `3` variant.
        N3,
        /// `4` variant.
        N4,
    }
    impl super::Block for PurpleCandle {
    }
    impl From<PurpleCandle> for super::BlockState {
        fn from(value : PurpleCandle) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:gray_terracotta` block.
pub mod gray_terracotta {
    /// `minecraft:gray_terracotta` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct GrayTerracotta;
    impl super::Block for GrayTerracotta {
    }
    impl From<GrayTerracotta> for super::BlockState {
        fn from(value : GrayTerracotta) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:powered_rail` block.
pub mod powered_rail {
    /// `minecraft:powered_rail` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct PoweredRail {
        /// `powered` state.
        pub powered : bool,
        /// `shape` state.
        pub shape : Shape,
        /// `waterlogged` state.
        pub waterlogged : bool,
    }
    /// `shape` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Shape {
        /// `north_south` variant.
        NorthSouth,
        /// `east_west` variant.
        EastWest,
        /// `ascending_east` variant.
        AscendingEast,
        /// `ascending_west` variant.
        AscendingWest,
        /// `ascending_north` variant.
        AscendingNorth,
        /// `ascending_south` variant.
        AscendingSouth,
    }
    impl super::Block for PoweredRail {
    }
    impl From<PoweredRail> for super::BlockState {
        fn from(value : PoweredRail) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:green_wool` block.
pub mod green_wool {
    /// `minecraft:green_wool` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct GreenWool;
    impl super::Block for GreenWool {
    }
    impl From<GreenWool> for super::BlockState {
        fn from(value : GreenWool) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:cherry_leaves` block.
pub mod cherry_leaves {
    /// `minecraft:cherry_leaves` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct CherryLeaves {
        /// `waterlogged` state.
        pub waterlogged : bool,
        /// `distance` state.
        pub distance : Distance,
        /// `persistent` state.
        pub persistent : bool,
    }
    /// `distance` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Distance {
        /// `1` variant.
        N1,
        /// `2` variant.
        N2,
        /// `3` variant.
        N3,
        /// `4` variant.
        N4,
        /// `5` variant.
        N5,
        /// `6` variant.
        N6,
        /// `7` variant.
        N7,
    }
    impl super::Block for CherryLeaves {
    }
    impl From<CherryLeaves> for super::BlockState {
        fn from(value : CherryLeaves) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:purpur_slab` block.
pub mod purpur_slab {
    /// `minecraft:purpur_slab` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct PurpurSlab {
        /// `type` state.
        pub kind : Kind,
        /// `waterlogged` state.
        pub waterlogged : bool,
    }
    /// `type` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Kind {
        /// `top` variant.
        Top,
        /// `bottom` variant.
        Bottom,
        /// `double` variant.
        Double,
    }
    impl super::Block for PurpurSlab {
    }
    impl From<PurpurSlab> for super::BlockState {
        fn from(value : PurpurSlab) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:cobblestone_slab` block.
pub mod cobblestone_slab {
    /// `minecraft:cobblestone_slab` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct CobblestoneSlab {
        /// `waterlogged` state.
        pub waterlogged : bool,
        /// `type` state.
        pub kind : Kind,
    }
    /// `type` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Kind {
        /// `top` variant.
        Top,
        /// `bottom` variant.
        Bottom,
        /// `double` variant.
        Double,
    }
    impl super::Block for CobblestoneSlab {
    }
    impl From<CobblestoneSlab> for super::BlockState {
        fn from(value : CobblestoneSlab) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:weathered_chiseled_copper` block.
pub mod weathered_chiseled_copper {
    /// `minecraft:weathered_chiseled_copper` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct WeatheredChiseledCopper;
    impl super::Block for WeatheredChiseledCopper {
    }
    impl From<WeatheredChiseledCopper> for super::BlockState {
        fn from(value : WeatheredChiseledCopper) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:cartography_table` block.
pub mod cartography_table {
    /// `minecraft:cartography_table` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct CartographyTable;
    impl super::Block for CartographyTable {
    }
    impl From<CartographyTable> for super::BlockState {
        fn from(value : CartographyTable) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:polished_andesite` block.
pub mod polished_andesite {
    /// `minecraft:polished_andesite` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct PolishedAndesite;
    impl super::Block for PolishedAndesite {
    }
    impl From<PolishedAndesite> for super::BlockState {
        fn from(value : PolishedAndesite) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:chiseled_nether_bricks` block.
pub mod chiseled_nether_bricks {
    /// `minecraft:chiseled_nether_bricks` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct ChiseledNetherBricks;
    impl super::Block for ChiseledNetherBricks {
    }
    impl From<ChiseledNetherBricks> for super::BlockState {
        fn from(value : ChiseledNetherBricks) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:light_blue_glazed_terracotta` block.
pub mod light_blue_glazed_terracotta {
    /// `minecraft:light_blue_glazed_terracotta` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct LightBlueGlazedTerracotta {
        /// `facing` state.
        pub facing : Facing,
    }
    /// `facing` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Facing {
        /// `north` variant.
        North,
        /// `south` variant.
        South,
        /// `west` variant.
        West,
        /// `east` variant.
        East,
    }
    impl super::Block for LightBlueGlazedTerracotta {
    }
    impl From<LightBlueGlazedTerracotta> for super::BlockState {
        fn from(value : LightBlueGlazedTerracotta) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:magenta_concrete_powder` block.
pub mod magenta_concrete_powder {
    /// `minecraft:magenta_concrete_powder` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct MagentaConcretePowder;
    impl super::Block for MagentaConcretePowder {
    }
    impl From<MagentaConcretePowder> for super::BlockState {
        fn from(value : MagentaConcretePowder) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:oak_slab` block.
pub mod oak_slab {
    /// `minecraft:oak_slab` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct OakSlab {
        /// `type` state.
        pub kind : Kind,
        /// `waterlogged` state.
        pub waterlogged : bool,
    }
    /// `type` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Kind {
        /// `top` variant.
        Top,
        /// `bottom` variant.
        Bottom,
        /// `double` variant.
        Double,
    }
    impl super::Block for OakSlab {
    }
    impl From<OakSlab> for super::BlockState {
        fn from(value : OakSlab) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:deepslate_tile_stairs` block.
pub mod deepslate_tile_stairs {
    /// `minecraft:deepslate_tile_stairs` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct DeepslateTileStairs {
        /// `facing` state.
        pub facing : Facing,
        /// `shape` state.
        pub shape : Shape,
        /// `waterlogged` state.
        pub waterlogged : bool,
        /// `half` state.
        pub half : Half,
    }
    /// `facing` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Facing {
        /// `north` variant.
        North,
        /// `south` variant.
        South,
        /// `west` variant.
        West,
        /// `east` variant.
        East,
    }
    /// `shape` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Shape {
        /// `straight` variant.
        Straight,
        /// `inner_left` variant.
        InnerLeft,
        /// `inner_right` variant.
        InnerRight,
        /// `outer_left` variant.
        OuterLeft,
        /// `outer_right` variant.
        OuterRight,
    }
    /// `half` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Half {
        /// `top` variant.
        Top,
        /// `bottom` variant.
        Bottom,
    }
    impl super::Block for DeepslateTileStairs {
    }
    impl From<DeepslateTileStairs> for super::BlockState {
        fn from(value : DeepslateTileStairs) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:packed_mud` block.
pub mod packed_mud {
    /// `minecraft:packed_mud` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct PackedMud;
    impl super::Block for PackedMud {
    }
    impl From<PackedMud> for super::BlockState {
        fn from(value : PackedMud) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:warped_trapdoor` block.
pub mod warped_trapdoor {
    /// `minecraft:warped_trapdoor` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct WarpedTrapdoor {
        /// `half` state.
        pub half : Half,
        /// `waterlogged` state.
        pub waterlogged : bool,
        /// `open` state.
        pub open : bool,
        /// `powered` state.
        pub powered : bool,
        /// `facing` state.
        pub facing : Facing,
    }
    /// `half` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Half {
        /// `top` variant.
        Top,
        /// `bottom` variant.
        Bottom,
    }
    /// `facing` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Facing {
        /// `north` variant.
        North,
        /// `south` variant.
        South,
        /// `west` variant.
        West,
        /// `east` variant.
        East,
    }
    impl super::Block for WarpedTrapdoor {
    }
    impl From<WarpedTrapdoor> for super::BlockState {
        fn from(value : WarpedTrapdoor) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:powder_snow_cauldron` block.
pub mod powder_snow_cauldron {
    /// `minecraft:powder_snow_cauldron` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct PowderSnowCauldron {
        /// `level` state.
        pub level : Level,
    }
    /// `level` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Level {
        /// `1` variant.
        N1,
        /// `2` variant.
        N2,
        /// `3` variant.
        N3,
    }
    impl super::Block for PowderSnowCauldron {
    }
    impl From<PowderSnowCauldron> for super::BlockState {
        fn from(value : PowderSnowCauldron) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:stripped_acacia_log` block.
pub mod stripped_acacia_log {
    /// `minecraft:stripped_acacia_log` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct StrippedAcaciaLog {
        /// `axis` state.
        pub axis : Axis,
    }
    /// `axis` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Axis {
        /// `x` variant.
        X,
        /// `y` variant.
        Y,
        /// `z` variant.
        Z,
    }
    impl super::Block for StrippedAcaciaLog {
    }
    impl From<StrippedAcaciaLog> for super::BlockState {
        fn from(value : StrippedAcaciaLog) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:wither_rose` block.
pub mod wither_rose {
    /// `minecraft:wither_rose` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct WitherRose;
    impl super::Block for WitherRose {
    }
    impl From<WitherRose> for super::BlockState {
        fn from(value : WitherRose) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:light_gray_shulker_box` block.
pub mod light_gray_shulker_box {
    /// `minecraft:light_gray_shulker_box` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct LightGrayShulkerBox {
        /// `facing` state.
        pub facing : Facing,
    }
    /// `facing` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Facing {
        /// `north` variant.
        North,
        /// `east` variant.
        East,
        /// `south` variant.
        South,
        /// `west` variant.
        West,
        /// `up` variant.
        Up,
        /// `down` variant.
        Down,
    }
    impl super::Block for LightGrayShulkerBox {
    }
    impl From<LightGrayShulkerBox> for super::BlockState {
        fn from(value : LightGrayShulkerBox) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:crimson_stairs` block.
pub mod crimson_stairs {
    /// `minecraft:crimson_stairs` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct CrimsonStairs {
        /// `half` state.
        pub half : Half,
        /// `waterlogged` state.
        pub waterlogged : bool,
        /// `facing` state.
        pub facing : Facing,
        /// `shape` state.
        pub shape : Shape,
    }
    /// `half` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Half {
        /// `top` variant.
        Top,
        /// `bottom` variant.
        Bottom,
    }
    /// `facing` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Facing {
        /// `north` variant.
        North,
        /// `south` variant.
        South,
        /// `west` variant.
        West,
        /// `east` variant.
        East,
    }
    /// `shape` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Shape {
        /// `straight` variant.
        Straight,
        /// `inner_left` variant.
        InnerLeft,
        /// `inner_right` variant.
        InnerRight,
        /// `outer_left` variant.
        OuterLeft,
        /// `outer_right` variant.
        OuterRight,
    }
    impl super::Block for CrimsonStairs {
    }
    impl From<CrimsonStairs> for super::BlockState {
        fn from(value : CrimsonStairs) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:tuff_wall` block.
pub mod tuff_wall {
    /// `minecraft:tuff_wall` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct TuffWall {
        /// `west` state.
        pub west : West,
        /// `waterlogged` state.
        pub waterlogged : bool,
        /// `up` state.
        pub up : bool,
        /// `south` state.
        pub south : South,
        /// `north` state.
        pub north : North,
        /// `east` state.
        pub east : East,
    }
    /// `west` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum West {
        /// `none` variant.
        None,
        /// `low` variant.
        Low,
        /// `tall` variant.
        Tall,
    }
    /// `south` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum South {
        /// `none` variant.
        None,
        /// `low` variant.
        Low,
        /// `tall` variant.
        Tall,
    }
    /// `north` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum North {
        /// `none` variant.
        None,
        /// `low` variant.
        Low,
        /// `tall` variant.
        Tall,
    }
    /// `east` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum East {
        /// `none` variant.
        None,
        /// `low` variant.
        Low,
        /// `tall` variant.
        Tall,
    }
    impl super::Block for TuffWall {
    }
    impl From<TuffWall> for super::BlockState {
        fn from(value : TuffWall) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:white_glazed_terracotta` block.
pub mod white_glazed_terracotta {
    /// `minecraft:white_glazed_terracotta` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct WhiteGlazedTerracotta {
        /// `facing` state.
        pub facing : Facing,
    }
    /// `facing` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Facing {
        /// `north` variant.
        North,
        /// `south` variant.
        South,
        /// `west` variant.
        West,
        /// `east` variant.
        East,
    }
    impl super::Block for WhiteGlazedTerracotta {
    }
    impl From<WhiteGlazedTerracotta> for super::BlockState {
        fn from(value : WhiteGlazedTerracotta) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:blackstone_stairs` block.
pub mod blackstone_stairs {
    /// `minecraft:blackstone_stairs` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct BlackstoneStairs {
        /// `shape` state.
        pub shape : Shape,
        /// `waterlogged` state.
        pub waterlogged : bool,
        /// `facing` state.
        pub facing : Facing,
        /// `half` state.
        pub half : Half,
    }
    /// `shape` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Shape {
        /// `straight` variant.
        Straight,
        /// `inner_left` variant.
        InnerLeft,
        /// `inner_right` variant.
        InnerRight,
        /// `outer_left` variant.
        OuterLeft,
        /// `outer_right` variant.
        OuterRight,
    }
    /// `facing` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Facing {
        /// `north` variant.
        North,
        /// `south` variant.
        South,
        /// `west` variant.
        West,
        /// `east` variant.
        East,
    }
    /// `half` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Half {
        /// `top` variant.
        Top,
        /// `bottom` variant.
        Bottom,
    }
    impl super::Block for BlackstoneStairs {
    }
    impl From<BlackstoneStairs> for super::BlockState {
        fn from(value : BlackstoneStairs) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:polished_granite` block.
pub mod polished_granite {
    /// `minecraft:polished_granite` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct PolishedGranite;
    impl super::Block for PolishedGranite {
    }
    impl From<PolishedGranite> for super::BlockState {
        fn from(value : PolishedGranite) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:stripped_spruce_wood` block.
pub mod stripped_spruce_wood {
    /// `minecraft:stripped_spruce_wood` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct StrippedSpruceWood {
        /// `axis` state.
        pub axis : Axis,
    }
    /// `axis` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Axis {
        /// `x` variant.
        X,
        /// `y` variant.
        Y,
        /// `z` variant.
        Z,
    }
    impl super::Block for StrippedSpruceWood {
    }
    impl From<StrippedSpruceWood> for super::BlockState {
        fn from(value : StrippedSpruceWood) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:campfire` block.
pub mod campfire {
    /// `minecraft:campfire` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct Campfire {
        /// `facing` state.
        pub facing : Facing,
        /// `signal_fire` state.
        pub signal_fire : bool,
        /// `lit` state.
        pub lit : bool,
        /// `waterlogged` state.
        pub waterlogged : bool,
    }
    /// `facing` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Facing {
        /// `north` variant.
        North,
        /// `south` variant.
        South,
        /// `west` variant.
        West,
        /// `east` variant.
        East,
    }
    impl super::Block for Campfire {
    }
    impl From<Campfire> for super::BlockState {
        fn from(value : Campfire) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:white_terracotta` block.
pub mod white_terracotta {
    /// `minecraft:white_terracotta` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct WhiteTerracotta;
    impl super::Block for WhiteTerracotta {
    }
    impl From<WhiteTerracotta> for super::BlockState {
        fn from(value : WhiteTerracotta) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:jungle_sign` block.
pub mod jungle_sign {
    /// `minecraft:jungle_sign` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct JungleSign {
        /// `rotation` state.
        pub rotation : Rotation,
        /// `waterlogged` state.
        pub waterlogged : bool,
    }
    /// `rotation` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Rotation {
        /// `0` variant.
        N0,
        /// `1` variant.
        N1,
        /// `2` variant.
        N2,
        /// `3` variant.
        N3,
        /// `4` variant.
        N4,
        /// `5` variant.
        N5,
        /// `6` variant.
        N6,
        /// `7` variant.
        N7,
        /// `8` variant.
        N8,
        /// `9` variant.
        N9,
        /// `10` variant.
        N10,
        /// `11` variant.
        N11,
        /// `12` variant.
        N12,
        /// `13` variant.
        N13,
        /// `14` variant.
        N14,
        /// `15` variant.
        N15,
    }
    impl super::Block for JungleSign {
    }
    impl From<JungleSign> for super::BlockState {
        fn from(value : JungleSign) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:stone_stairs` block.
pub mod stone_stairs {
    /// `minecraft:stone_stairs` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct StoneStairs {
        /// `half` state.
        pub half : Half,
        /// `facing` state.
        pub facing : Facing,
        /// `waterlogged` state.
        pub waterlogged : bool,
        /// `shape` state.
        pub shape : Shape,
    }
    /// `half` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Half {
        /// `top` variant.
        Top,
        /// `bottom` variant.
        Bottom,
    }
    /// `facing` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Facing {
        /// `north` variant.
        North,
        /// `south` variant.
        South,
        /// `west` variant.
        West,
        /// `east` variant.
        East,
    }
    /// `shape` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Shape {
        /// `straight` variant.
        Straight,
        /// `inner_left` variant.
        InnerLeft,
        /// `inner_right` variant.
        InnerRight,
        /// `outer_left` variant.
        OuterLeft,
        /// `outer_right` variant.
        OuterRight,
    }
    impl super::Block for StoneStairs {
    }
    impl From<StoneStairs> for super::BlockState {
        fn from(value : StoneStairs) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:waxed_cut_copper_slab` block.
pub mod waxed_cut_copper_slab {
    /// `minecraft:waxed_cut_copper_slab` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct WaxedCutCopperSlab {
        /// `waterlogged` state.
        pub waterlogged : bool,
        /// `type` state.
        pub kind : Kind,
    }
    /// `type` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Kind {
        /// `top` variant.
        Top,
        /// `bottom` variant.
        Bottom,
        /// `double` variant.
        Double,
    }
    impl super::Block for WaxedCutCopperSlab {
    }
    impl From<WaxedCutCopperSlab> for super::BlockState {
        fn from(value : WaxedCutCopperSlab) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:potted_birch_sapling` block.
pub mod potted_birch_sapling {
    /// `minecraft:potted_birch_sapling` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct PottedBirchSapling;
    impl super::Block for PottedBirchSapling {
    }
    impl From<PottedBirchSapling> for super::BlockState {
        fn from(value : PottedBirchSapling) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:magenta_wool` block.
pub mod magenta_wool {
    /// `minecraft:magenta_wool` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct MagentaWool;
    impl super::Block for MagentaWool {
    }
    impl From<MagentaWool> for super::BlockState {
        fn from(value : MagentaWool) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:light_blue_shulker_box` block.
pub mod light_blue_shulker_box {
    /// `minecraft:light_blue_shulker_box` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct LightBlueShulkerBox {
        /// `facing` state.
        pub facing : Facing,
    }
    /// `facing` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Facing {
        /// `north` variant.
        North,
        /// `east` variant.
        East,
        /// `south` variant.
        South,
        /// `west` variant.
        West,
        /// `up` variant.
        Up,
        /// `down` variant.
        Down,
    }
    impl super::Block for LightBlueShulkerBox {
    }
    impl From<LightBlueShulkerBox> for super::BlockState {
        fn from(value : LightBlueShulkerBox) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:crimson_fungus` block.
pub mod crimson_fungus {
    /// `minecraft:crimson_fungus` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct CrimsonFungus;
    impl super::Block for CrimsonFungus {
    }
    impl From<CrimsonFungus> for super::BlockState {
        fn from(value : CrimsonFungus) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:light_gray_terracotta` block.
pub mod light_gray_terracotta {
    /// `minecraft:light_gray_terracotta` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct LightGrayTerracotta;
    impl super::Block for LightGrayTerracotta {
    }
    impl From<LightGrayTerracotta> for super::BlockState {
        fn from(value : LightGrayTerracotta) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:beacon` block.
pub mod beacon {
    /// `minecraft:beacon` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct Beacon;
    impl super::Block for Beacon {
    }
    impl From<Beacon> for super::BlockState {
        fn from(value : Beacon) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:light_blue_terracotta` block.
pub mod light_blue_terracotta {
    /// `minecraft:light_blue_terracotta` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct LightBlueTerracotta;
    impl super::Block for LightBlueTerracotta {
    }
    impl From<LightBlueTerracotta> for super::BlockState {
        fn from(value : LightBlueTerracotta) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:brewing_stand` block.
pub mod brewing_stand {
    /// `minecraft:brewing_stand` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct BrewingStand {
        /// `has_bottle_1` state.
        pub has_bottle_1 : bool,
        /// `has_bottle_2` state.
        pub has_bottle_2 : bool,
        /// `has_bottle_0` state.
        pub has_bottle_0 : bool,
    }
    impl super::Block for BrewingStand {
    }
    impl From<BrewingStand> for super::BlockState {
        fn from(value : BrewingStand) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:waxed_copper_grate` block.
pub mod waxed_copper_grate {
    /// `minecraft:waxed_copper_grate` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct WaxedCopperGrate {
        /// `waterlogged` state.
        pub waterlogged : bool,
    }
    impl super::Block for WaxedCopperGrate {
    }
    impl From<WaxedCopperGrate> for super::BlockState {
        fn from(value : WaxedCopperGrate) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:waxed_exposed_copper_bulb` block.
pub mod waxed_exposed_copper_bulb {
    /// `minecraft:waxed_exposed_copper_bulb` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct WaxedExposedCopperBulb {
        /// `lit` state.
        pub lit : bool,
        /// `powered` state.
        pub powered : bool,
    }
    impl super::Block for WaxedExposedCopperBulb {
    }
    impl From<WaxedExposedCopperBulb> for super::BlockState {
        fn from(value : WaxedExposedCopperBulb) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:purple_terracotta` block.
pub mod purple_terracotta {
    /// `minecraft:purple_terracotta` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct PurpleTerracotta;
    impl super::Block for PurpleTerracotta {
    }
    impl From<PurpleTerracotta> for super::BlockState {
        fn from(value : PurpleTerracotta) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:pink_wool` block.
pub mod pink_wool {
    /// `minecraft:pink_wool` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct PinkWool;
    impl super::Block for PinkWool {
    }
    impl From<PinkWool> for super::BlockState {
        fn from(value : PinkWool) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:crimson_roots` block.
pub mod crimson_roots {
    /// `minecraft:crimson_roots` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct CrimsonRoots;
    impl super::Block for CrimsonRoots {
    }
    impl From<CrimsonRoots> for super::BlockState {
        fn from(value : CrimsonRoots) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:glass` block.
pub mod glass {
    /// `minecraft:glass` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct Glass;
    impl super::Block for Glass {
    }
    impl From<Glass> for super::BlockState {
        fn from(value : Glass) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:smooth_quartz_stairs` block.
pub mod smooth_quartz_stairs {
    /// `minecraft:smooth_quartz_stairs` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct SmoothQuartzStairs {
        /// `waterlogged` state.
        pub waterlogged : bool,
        /// `half` state.
        pub half : Half,
        /// `facing` state.
        pub facing : Facing,
        /// `shape` state.
        pub shape : Shape,
    }
    /// `half` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Half {
        /// `top` variant.
        Top,
        /// `bottom` variant.
        Bottom,
    }
    /// `facing` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Facing {
        /// `north` variant.
        North,
        /// `south` variant.
        South,
        /// `west` variant.
        West,
        /// `east` variant.
        East,
    }
    /// `shape` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Shape {
        /// `straight` variant.
        Straight,
        /// `inner_left` variant.
        InnerLeft,
        /// `inner_right` variant.
        InnerRight,
        /// `outer_left` variant.
        OuterLeft,
        /// `outer_right` variant.
        OuterRight,
    }
    impl super::Block for SmoothQuartzStairs {
    }
    impl From<SmoothQuartzStairs> for super::BlockState {
        fn from(value : SmoothQuartzStairs) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:red_candle` block.
pub mod red_candle {
    /// `minecraft:red_candle` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct RedCandle {
        /// `candles` state.
        pub candles : Candles,
        /// `waterlogged` state.
        pub waterlogged : bool,
        /// `lit` state.
        pub lit : bool,
    }
    /// `candles` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Candles {
        /// `1` variant.
        N1,
        /// `2` variant.
        N2,
        /// `3` variant.
        N3,
        /// `4` variant.
        N4,
    }
    impl super::Block for RedCandle {
    }
    impl From<RedCandle> for super::BlockState {
        fn from(value : RedCandle) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:warped_fungus` block.
pub mod warped_fungus {
    /// `minecraft:warped_fungus` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct WarpedFungus;
    impl super::Block for WarpedFungus {
    }
    impl From<WarpedFungus> for super::BlockState {
        fn from(value : WarpedFungus) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:dark_prismarine_slab` block.
pub mod dark_prismarine_slab {
    /// `minecraft:dark_prismarine_slab` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct DarkPrismarineSlab {
        /// `waterlogged` state.
        pub waterlogged : bool,
        /// `type` state.
        pub kind : Kind,
    }
    /// `type` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Kind {
        /// `top` variant.
        Top,
        /// `bottom` variant.
        Bottom,
        /// `double` variant.
        Double,
    }
    impl super::Block for DarkPrismarineSlab {
    }
    impl From<DarkPrismarineSlab> for super::BlockState {
        fn from(value : DarkPrismarineSlab) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:redstone_wall_torch` block.
pub mod redstone_wall_torch {
    /// `minecraft:redstone_wall_torch` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct RedstoneWallTorch {
        /// `facing` state.
        pub facing : Facing,
        /// `lit` state.
        pub lit : bool,
    }
    /// `facing` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Facing {
        /// `north` variant.
        North,
        /// `south` variant.
        South,
        /// `west` variant.
        West,
        /// `east` variant.
        East,
    }
    impl super::Block for RedstoneWallTorch {
    }
    impl From<RedstoneWallTorch> for super::BlockState {
        fn from(value : RedstoneWallTorch) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:oak_stairs` block.
pub mod oak_stairs {
    /// `minecraft:oak_stairs` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct OakStairs {
        /// `facing` state.
        pub facing : Facing,
        /// `shape` state.
        pub shape : Shape,
        /// `half` state.
        pub half : Half,
        /// `waterlogged` state.
        pub waterlogged : bool,
    }
    /// `facing` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Facing {
        /// `north` variant.
        North,
        /// `south` variant.
        South,
        /// `west` variant.
        West,
        /// `east` variant.
        East,
    }
    /// `shape` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Shape {
        /// `straight` variant.
        Straight,
        /// `inner_left` variant.
        InnerLeft,
        /// `inner_right` variant.
        InnerRight,
        /// `outer_left` variant.
        OuterLeft,
        /// `outer_right` variant.
        OuterRight,
    }
    /// `half` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Half {
        /// `top` variant.
        Top,
        /// `bottom` variant.
        Bottom,
    }
    impl super::Block for OakStairs {
    }
    impl From<OakStairs> for super::BlockState {
        fn from(value : OakStairs) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:yellow_terracotta` block.
pub mod yellow_terracotta {
    /// `minecraft:yellow_terracotta` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct YellowTerracotta;
    impl super::Block for YellowTerracotta {
    }
    impl From<YellowTerracotta> for super::BlockState {
        fn from(value : YellowTerracotta) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:magenta_concrete` block.
pub mod magenta_concrete {
    /// `minecraft:magenta_concrete` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct MagentaConcrete;
    impl super::Block for MagentaConcrete {
    }
    impl From<MagentaConcrete> for super::BlockState {
        fn from(value : MagentaConcrete) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:red_stained_glass_pane` block.
pub mod red_stained_glass_pane {
    /// `minecraft:red_stained_glass_pane` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct RedStainedGlassPane {
        /// `north` state.
        pub north : bool,
        /// `waterlogged` state.
        pub waterlogged : bool,
        /// `south` state.
        pub south : bool,
        /// `east` state.
        pub east : bool,
        /// `west` state.
        pub west : bool,
    }
    impl super::Block for RedStainedGlassPane {
    }
    impl From<RedStainedGlassPane> for super::BlockState {
        fn from(value : RedStainedGlassPane) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:soul_torch` block.
pub mod soul_torch {
    /// `minecraft:soul_torch` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct SoulTorch;
    impl super::Block for SoulTorch {
    }
    impl From<SoulTorch> for super::BlockState {
        fn from(value : SoulTorch) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:acacia_fence_gate` block.
pub mod acacia_fence_gate {
    /// `minecraft:acacia_fence_gate` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct AcaciaFenceGate {
        /// `facing` state.
        pub facing : Facing,
        /// `powered` state.
        pub powered : bool,
        /// `open` state.
        pub open : bool,
        /// `in_wall` state.
        pub in_wall : bool,
    }
    /// `facing` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Facing {
        /// `north` variant.
        North,
        /// `south` variant.
        South,
        /// `west` variant.
        West,
        /// `east` variant.
        East,
    }
    impl super::Block for AcaciaFenceGate {
    }
    impl From<AcaciaFenceGate> for super::BlockState {
        fn from(value : AcaciaFenceGate) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:waxed_copper_trapdoor` block.
pub mod waxed_copper_trapdoor {
    /// `minecraft:waxed_copper_trapdoor` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct WaxedCopperTrapdoor {
        /// `powered` state.
        pub powered : bool,
        /// `facing` state.
        pub facing : Facing,
        /// `open` state.
        pub open : bool,
        /// `half` state.
        pub half : Half,
        /// `waterlogged` state.
        pub waterlogged : bool,
    }
    /// `facing` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Facing {
        /// `north` variant.
        North,
        /// `south` variant.
        South,
        /// `west` variant.
        West,
        /// `east` variant.
        East,
    }
    /// `half` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Half {
        /// `top` variant.
        Top,
        /// `bottom` variant.
        Bottom,
    }
    impl super::Block for WaxedCopperTrapdoor {
    }
    impl From<WaxedCopperTrapdoor> for super::BlockState {
        fn from(value : WaxedCopperTrapdoor) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:stone_brick_wall` block.
pub mod stone_brick_wall {
    /// `minecraft:stone_brick_wall` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct StoneBrickWall {
        /// `south` state.
        pub south : South,
        /// `west` state.
        pub west : West,
        /// `north` state.
        pub north : North,
        /// `waterlogged` state.
        pub waterlogged : bool,
        /// `east` state.
        pub east : East,
        /// `up` state.
        pub up : bool,
    }
    /// `south` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum South {
        /// `none` variant.
        None,
        /// `low` variant.
        Low,
        /// `tall` variant.
        Tall,
    }
    /// `west` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum West {
        /// `none` variant.
        None,
        /// `low` variant.
        Low,
        /// `tall` variant.
        Tall,
    }
    /// `north` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum North {
        /// `none` variant.
        None,
        /// `low` variant.
        Low,
        /// `tall` variant.
        Tall,
    }
    /// `east` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum East {
        /// `none` variant.
        None,
        /// `low` variant.
        Low,
        /// `tall` variant.
        Tall,
    }
    impl super::Block for StoneBrickWall {
    }
    impl From<StoneBrickWall> for super::BlockState {
        fn from(value : StoneBrickWall) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:potted_lily_of_the_valley` block.
pub mod potted_lily_of_the_valley {
    /// `minecraft:potted_lily_of_the_valley` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct PottedLilyOfTheValley;
    impl super::Block for PottedLilyOfTheValley {
    }
    impl From<PottedLilyOfTheValley> for super::BlockState {
        fn from(value : PottedLilyOfTheValley) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:orange_stained_glass` block.
pub mod orange_stained_glass {
    /// `minecraft:orange_stained_glass` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct OrangeStainedGlass;
    impl super::Block for OrangeStainedGlass {
    }
    impl From<OrangeStainedGlass> for super::BlockState {
        fn from(value : OrangeStainedGlass) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:spruce_button` block.
pub mod spruce_button {
    /// `minecraft:spruce_button` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct SpruceButton {
        /// `powered` state.
        pub powered : bool,
        /// `facing` state.
        pub facing : Facing,
        /// `face` state.
        pub face : Face,
    }
    /// `facing` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Facing {
        /// `north` variant.
        North,
        /// `south` variant.
        South,
        /// `west` variant.
        West,
        /// `east` variant.
        East,
    }
    /// `face` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Face {
        /// `floor` variant.
        Floor,
        /// `wall` variant.
        Wall,
        /// `ceiling` variant.
        Ceiling,
    }
    impl super::Block for SpruceButton {
    }
    impl From<SpruceButton> for super::BlockState {
        fn from(value : SpruceButton) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:gray_shulker_box` block.
pub mod gray_shulker_box {
    /// `minecraft:gray_shulker_box` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct GrayShulkerBox {
        /// `facing` state.
        pub facing : Facing,
    }
    /// `facing` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Facing {
        /// `north` variant.
        North,
        /// `east` variant.
        East,
        /// `south` variant.
        South,
        /// `west` variant.
        West,
        /// `up` variant.
        Up,
        /// `down` variant.
        Down,
    }
    impl super::Block for GrayShulkerBox {
    }
    impl From<GrayShulkerBox> for super::BlockState {
        fn from(value : GrayShulkerBox) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:purple_stained_glass_pane` block.
pub mod purple_stained_glass_pane {
    /// `minecraft:purple_stained_glass_pane` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct PurpleStainedGlassPane {
        /// `south` state.
        pub south : bool,
        /// `east` state.
        pub east : bool,
        /// `waterlogged` state.
        pub waterlogged : bool,
        /// `north` state.
        pub north : bool,
        /// `west` state.
        pub west : bool,
    }
    impl super::Block for PurpleStainedGlassPane {
    }
    impl From<PurpleStainedGlassPane> for super::BlockState {
        fn from(value : PurpleStainedGlassPane) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:spruce_wall_sign` block.
pub mod spruce_wall_sign {
    /// `minecraft:spruce_wall_sign` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct SpruceWallSign {
        /// `facing` state.
        pub facing : Facing,
        /// `waterlogged` state.
        pub waterlogged : bool,
    }
    /// `facing` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Facing {
        /// `north` variant.
        North,
        /// `south` variant.
        South,
        /// `west` variant.
        West,
        /// `east` variant.
        East,
    }
    impl super::Block for SpruceWallSign {
    }
    impl From<SpruceWallSign> for super::BlockState {
        fn from(value : SpruceWallSign) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:granite_wall` block.
pub mod granite_wall {
    /// `minecraft:granite_wall` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct GraniteWall {
        /// `south` state.
        pub south : South,
        /// `up` state.
        pub up : bool,
        /// `waterlogged` state.
        pub waterlogged : bool,
        /// `east` state.
        pub east : East,
        /// `west` state.
        pub west : West,
        /// `north` state.
        pub north : North,
    }
    /// `south` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum South {
        /// `none` variant.
        None,
        /// `low` variant.
        Low,
        /// `tall` variant.
        Tall,
    }
    /// `east` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum East {
        /// `none` variant.
        None,
        /// `low` variant.
        Low,
        /// `tall` variant.
        Tall,
    }
    /// `west` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum West {
        /// `none` variant.
        None,
        /// `low` variant.
        Low,
        /// `tall` variant.
        Tall,
    }
    /// `north` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum North {
        /// `none` variant.
        None,
        /// `low` variant.
        Low,
        /// `tall` variant.
        Tall,
    }
    impl super::Block for GraniteWall {
    }
    impl From<GraniteWall> for super::BlockState {
        fn from(value : GraniteWall) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:lime_bed` block.
pub mod lime_bed {
    /// `minecraft:lime_bed` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct LimeBed {
        /// `part` state.
        pub part : Part,
        /// `occupied` state.
        pub occupied : bool,
        /// `facing` state.
        pub facing : Facing,
    }
    /// `part` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Part {
        /// `head` variant.
        Head,
        /// `foot` variant.
        Foot,
    }
    /// `facing` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Facing {
        /// `north` variant.
        North,
        /// `south` variant.
        South,
        /// `west` variant.
        West,
        /// `east` variant.
        East,
    }
    impl super::Block for LimeBed {
    }
    impl From<LimeBed> for super::BlockState {
        fn from(value : LimeBed) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:bamboo_block` block.
pub mod bamboo_block {
    /// `minecraft:bamboo_block` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct BambooBlock {
        /// `axis` state.
        pub axis : Axis,
    }
    /// `axis` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Axis {
        /// `x` variant.
        X,
        /// `y` variant.
        Y,
        /// `z` variant.
        Z,
    }
    impl super::Block for BambooBlock {
    }
    impl From<BambooBlock> for super::BlockState {
        fn from(value : BambooBlock) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:brown_wool` block.
pub mod brown_wool {
    /// `minecraft:brown_wool` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct BrownWool;
    impl super::Block for BrownWool {
    }
    impl From<BrownWool> for super::BlockState {
        fn from(value : BrownWool) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:dark_oak_sapling` block.
pub mod dark_oak_sapling {
    /// `minecraft:dark_oak_sapling` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct DarkOakSapling {
        /// `stage` state.
        pub stage : Stage,
    }
    /// `stage` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Stage {
        /// `0` variant.
        N0,
        /// `1` variant.
        N1,
    }
    impl super::Block for DarkOakSapling {
    }
    impl From<DarkOakSapling> for super::BlockState {
        fn from(value : DarkOakSapling) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:pink_terracotta` block.
pub mod pink_terracotta {
    /// `minecraft:pink_terracotta` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct PinkTerracotta;
    impl super::Block for PinkTerracotta {
    }
    impl From<PinkTerracotta> for super::BlockState {
        fn from(value : PinkTerracotta) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:purple_concrete_powder` block.
pub mod purple_concrete_powder {
    /// `minecraft:purple_concrete_powder` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct PurpleConcretePowder;
    impl super::Block for PurpleConcretePowder {
    }
    impl From<PurpleConcretePowder> for super::BlockState {
        fn from(value : PurpleConcretePowder) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:kelp` block.
pub mod kelp {
    /// `minecraft:kelp` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct Kelp {
        /// `age` state.
        pub age : Age,
    }
    /// `age` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Age {
        /// `0` variant.
        N0,
        /// `1` variant.
        N1,
        /// `2` variant.
        N2,
        /// `3` variant.
        N3,
        /// `4` variant.
        N4,
        /// `5` variant.
        N5,
        /// `6` variant.
        N6,
        /// `7` variant.
        N7,
        /// `8` variant.
        N8,
        /// `9` variant.
        N9,
        /// `10` variant.
        N10,
        /// `11` variant.
        N11,
        /// `12` variant.
        N12,
        /// `13` variant.
        N13,
        /// `14` variant.
        N14,
        /// `15` variant.
        N15,
        /// `16` variant.
        N16,
        /// `17` variant.
        N17,
        /// `18` variant.
        N18,
        /// `19` variant.
        N19,
        /// `20` variant.
        N20,
        /// `21` variant.
        N21,
        /// `22` variant.
        N22,
        /// `23` variant.
        N23,
        /// `24` variant.
        N24,
        /// `25` variant.
        N25,
    }
    impl super::Block for Kelp {
    }
    impl From<Kelp> for super::BlockState {
        fn from(value : Kelp) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:light_gray_concrete` block.
pub mod light_gray_concrete {
    /// `minecraft:light_gray_concrete` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct LightGrayConcrete;
    impl super::Block for LightGrayConcrete {
    }
    impl From<LightGrayConcrete> for super::BlockState {
        fn from(value : LightGrayConcrete) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:warped_button` block.
pub mod warped_button {
    /// `minecraft:warped_button` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct WarpedButton {
        /// `face` state.
        pub face : Face,
        /// `powered` state.
        pub powered : bool,
        /// `facing` state.
        pub facing : Facing,
    }
    /// `face` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Face {
        /// `floor` variant.
        Floor,
        /// `wall` variant.
        Wall,
        /// `ceiling` variant.
        Ceiling,
    }
    /// `facing` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Facing {
        /// `north` variant.
        North,
        /// `south` variant.
        South,
        /// `west` variant.
        West,
        /// `east` variant.
        East,
    }
    impl super::Block for WarpedButton {
    }
    impl From<WarpedButton> for super::BlockState {
        fn from(value : WarpedButton) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:acacia_sapling` block.
pub mod acacia_sapling {
    /// `minecraft:acacia_sapling` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct AcaciaSapling {
        /// `stage` state.
        pub stage : Stage,
    }
    /// `stage` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Stage {
        /// `0` variant.
        N0,
        /// `1` variant.
        N1,
    }
    impl super::Block for AcaciaSapling {
    }
    impl From<AcaciaSapling> for super::BlockState {
        fn from(value : AcaciaSapling) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:calcite` block.
pub mod calcite {
    /// `minecraft:calcite` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct Calcite;
    impl super::Block for Calcite {
    }
    impl From<Calcite> for super::BlockState {
        fn from(value : Calcite) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:exposed_copper` block.
pub mod exposed_copper {
    /// `minecraft:exposed_copper` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct ExposedCopper;
    impl super::Block for ExposedCopper {
    }
    impl From<ExposedCopper> for super::BlockState {
        fn from(value : ExposedCopper) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:red_nether_bricks` block.
pub mod red_nether_bricks {
    /// `minecraft:red_nether_bricks` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct RedNetherBricks;
    impl super::Block for RedNetherBricks {
    }
    impl From<RedNetherBricks> for super::BlockState {
        fn from(value : RedNetherBricks) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:bubble_coral_block` block.
pub mod bubble_coral_block {
    /// `minecraft:bubble_coral_block` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct BubbleCoralBlock;
    impl super::Block for BubbleCoralBlock {
    }
    impl From<BubbleCoralBlock> for super::BlockState {
        fn from(value : BubbleCoralBlock) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:candle` block.
pub mod candle {
    /// `minecraft:candle` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct Candle {
        /// `waterlogged` state.
        pub waterlogged : bool,
        /// `lit` state.
        pub lit : bool,
        /// `candles` state.
        pub candles : Candles,
    }
    /// `candles` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Candles {
        /// `1` variant.
        N1,
        /// `2` variant.
        N2,
        /// `3` variant.
        N3,
        /// `4` variant.
        N4,
    }
    impl super::Block for Candle {
    }
    impl From<Candle> for super::BlockState {
        fn from(value : Candle) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:white_tulip` block.
pub mod white_tulip {
    /// `minecraft:white_tulip` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct WhiteTulip;
    impl super::Block for WhiteTulip {
    }
    impl From<WhiteTulip> for super::BlockState {
        fn from(value : WhiteTulip) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:dragon_head` block.
pub mod dragon_head {
    /// `minecraft:dragon_head` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct DragonHead {
        /// `powered` state.
        pub powered : bool,
        /// `rotation` state.
        pub rotation : Rotation,
    }
    /// `rotation` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Rotation {
        /// `0` variant.
        N0,
        /// `1` variant.
        N1,
        /// `2` variant.
        N2,
        /// `3` variant.
        N3,
        /// `4` variant.
        N4,
        /// `5` variant.
        N5,
        /// `6` variant.
        N6,
        /// `7` variant.
        N7,
        /// `8` variant.
        N8,
        /// `9` variant.
        N9,
        /// `10` variant.
        N10,
        /// `11` variant.
        N11,
        /// `12` variant.
        N12,
        /// `13` variant.
        N13,
        /// `14` variant.
        N14,
        /// `15` variant.
        N15,
    }
    impl super::Block for DragonHead {
    }
    impl From<DragonHead> for super::BlockState {
        fn from(value : DragonHead) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:dark_prismarine_stairs` block.
pub mod dark_prismarine_stairs {
    /// `minecraft:dark_prismarine_stairs` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct DarkPrismarineStairs {
        /// `shape` state.
        pub shape : Shape,
        /// `waterlogged` state.
        pub waterlogged : bool,
        /// `half` state.
        pub half : Half,
        /// `facing` state.
        pub facing : Facing,
    }
    /// `shape` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Shape {
        /// `straight` variant.
        Straight,
        /// `inner_left` variant.
        InnerLeft,
        /// `inner_right` variant.
        InnerRight,
        /// `outer_left` variant.
        OuterLeft,
        /// `outer_right` variant.
        OuterRight,
    }
    /// `half` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Half {
        /// `top` variant.
        Top,
        /// `bottom` variant.
        Bottom,
    }
    /// `facing` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Facing {
        /// `north` variant.
        North,
        /// `south` variant.
        South,
        /// `west` variant.
        West,
        /// `east` variant.
        East,
    }
    impl super::Block for DarkPrismarineStairs {
    }
    impl From<DarkPrismarineStairs> for super::BlockState {
        fn from(value : DarkPrismarineStairs) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:waxed_oxidized_cut_copper_stairs` block.
pub mod waxed_oxidized_cut_copper_stairs {
    /// `minecraft:waxed_oxidized_cut_copper_stairs` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct WaxedOxidizedCutCopperStairs {
        /// `waterlogged` state.
        pub waterlogged : bool,
        /// `facing` state.
        pub facing : Facing,
        /// `shape` state.
        pub shape : Shape,
        /// `half` state.
        pub half : Half,
    }
    /// `facing` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Facing {
        /// `north` variant.
        North,
        /// `south` variant.
        South,
        /// `west` variant.
        West,
        /// `east` variant.
        East,
    }
    /// `shape` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Shape {
        /// `straight` variant.
        Straight,
        /// `inner_left` variant.
        InnerLeft,
        /// `inner_right` variant.
        InnerRight,
        /// `outer_left` variant.
        OuterLeft,
        /// `outer_right` variant.
        OuterRight,
    }
    /// `half` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Half {
        /// `top` variant.
        Top,
        /// `bottom` variant.
        Bottom,
    }
    impl super::Block for WaxedOxidizedCutCopperStairs {
    }
    impl From<WaxedOxidizedCutCopperStairs> for super::BlockState {
        fn from(value : WaxedOxidizedCutCopperStairs) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:weathered_copper_bulb` block.
pub mod weathered_copper_bulb {
    /// `minecraft:weathered_copper_bulb` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct WeatheredCopperBulb {
        /// `powered` state.
        pub powered : bool,
        /// `lit` state.
        pub lit : bool,
    }
    impl super::Block for WeatheredCopperBulb {
    }
    impl From<WeatheredCopperBulb> for super::BlockState {
        fn from(value : WeatheredCopperBulb) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:brain_coral_fan` block.
pub mod brain_coral_fan {
    /// `minecraft:brain_coral_fan` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct BrainCoralFan {
        /// `waterlogged` state.
        pub waterlogged : bool,
    }
    impl super::Block for BrainCoralFan {
    }
    impl From<BrainCoralFan> for super::BlockState {
        fn from(value : BrainCoralFan) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:brick_slab` block.
pub mod brick_slab {
    /// `minecraft:brick_slab` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct BrickSlab {
        /// `type` state.
        pub kind : Kind,
        /// `waterlogged` state.
        pub waterlogged : bool,
    }
    /// `type` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Kind {
        /// `top` variant.
        Top,
        /// `bottom` variant.
        Bottom,
        /// `double` variant.
        Double,
    }
    impl super::Block for BrickSlab {
    }
    impl From<BrickSlab> for super::BlockState {
        fn from(value : BrickSlab) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:chiseled_copper` block.
pub mod chiseled_copper {
    /// `minecraft:chiseled_copper` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct ChiseledCopper;
    impl super::Block for ChiseledCopper {
    }
    impl From<ChiseledCopper> for super::BlockState {
        fn from(value : ChiseledCopper) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:pale_oak_log` block.
pub mod pale_oak_log {
    /// `minecraft:pale_oak_log` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct PaleOakLog {
        /// `axis` state.
        pub axis : Axis,
    }
    /// `axis` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Axis {
        /// `x` variant.
        X,
        /// `y` variant.
        Y,
        /// `z` variant.
        Z,
    }
    impl super::Block for PaleOakLog {
    }
    impl From<PaleOakLog> for super::BlockState {
        fn from(value : PaleOakLog) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:soul_campfire` block.
pub mod soul_campfire {
    /// `minecraft:soul_campfire` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct SoulCampfire {
        /// `facing` state.
        pub facing : Facing,
        /// `lit` state.
        pub lit : bool,
        /// `waterlogged` state.
        pub waterlogged : bool,
        /// `signal_fire` state.
        pub signal_fire : bool,
    }
    /// `facing` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Facing {
        /// `north` variant.
        North,
        /// `south` variant.
        South,
        /// `west` variant.
        West,
        /// `east` variant.
        East,
    }
    impl super::Block for SoulCampfire {
    }
    impl From<SoulCampfire> for super::BlockState {
        fn from(value : SoulCampfire) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:azure_bluet` block.
pub mod azure_bluet {
    /// `minecraft:azure_bluet` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct AzureBluet;
    impl super::Block for AzureBluet {
    }
    impl From<AzureBluet> for super::BlockState {
        fn from(value : AzureBluet) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:red_sandstone_slab` block.
pub mod red_sandstone_slab {
    /// `minecraft:red_sandstone_slab` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct RedSandstoneSlab {
        /// `type` state.
        pub kind : Kind,
        /// `waterlogged` state.
        pub waterlogged : bool,
    }
    /// `type` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Kind {
        /// `top` variant.
        Top,
        /// `bottom` variant.
        Bottom,
        /// `double` variant.
        Double,
    }
    impl super::Block for RedSandstoneSlab {
    }
    impl From<RedSandstoneSlab> for super::BlockState {
        fn from(value : RedSandstoneSlab) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:void_air` block.
pub mod void_air {
    /// `minecraft:void_air` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct VoidAir;
    impl super::Block for VoidAir {
    }
    impl From<VoidAir> for super::BlockState {
        fn from(value : VoidAir) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:yellow_candle` block.
pub mod yellow_candle {
    /// `minecraft:yellow_candle` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct YellowCandle {
        /// `candles` state.
        pub candles : Candles,
        /// `lit` state.
        pub lit : bool,
        /// `waterlogged` state.
        pub waterlogged : bool,
    }
    /// `candles` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Candles {
        /// `1` variant.
        N1,
        /// `2` variant.
        N2,
        /// `3` variant.
        N3,
        /// `4` variant.
        N4,
    }
    impl super::Block for YellowCandle {
    }
    impl From<YellowCandle> for super::BlockState {
        fn from(value : YellowCandle) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:bamboo_wall_sign` block.
pub mod bamboo_wall_sign {
    /// `minecraft:bamboo_wall_sign` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct BambooWallSign {
        /// `waterlogged` state.
        pub waterlogged : bool,
        /// `facing` state.
        pub facing : Facing,
    }
    /// `facing` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Facing {
        /// `north` variant.
        North,
        /// `south` variant.
        South,
        /// `west` variant.
        West,
        /// `east` variant.
        East,
    }
    impl super::Block for BambooWallSign {
    }
    impl From<BambooWallSign> for super::BlockState {
        fn from(value : BambooWallSign) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:light_gray_stained_glass_pane` block.
pub mod light_gray_stained_glass_pane {
    /// `minecraft:light_gray_stained_glass_pane` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct LightGrayStainedGlassPane {
        /// `east` state.
        pub east : bool,
        /// `north` state.
        pub north : bool,
        /// `waterlogged` state.
        pub waterlogged : bool,
        /// `west` state.
        pub west : bool,
        /// `south` state.
        pub south : bool,
    }
    impl super::Block for LightGrayStainedGlassPane {
    }
    impl From<LightGrayStainedGlassPane> for super::BlockState {
        fn from(value : LightGrayStainedGlassPane) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:orange_shulker_box` block.
pub mod orange_shulker_box {
    /// `minecraft:orange_shulker_box` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct OrangeShulkerBox {
        /// `facing` state.
        pub facing : Facing,
    }
    /// `facing` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Facing {
        /// `north` variant.
        North,
        /// `east` variant.
        East,
        /// `south` variant.
        South,
        /// `west` variant.
        West,
        /// `up` variant.
        Up,
        /// `down` variant.
        Down,
    }
    impl super::Block for OrangeShulkerBox {
    }
    impl From<OrangeShulkerBox> for super::BlockState {
        fn from(value : OrangeShulkerBox) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:green_bed` block.
pub mod green_bed {
    /// `minecraft:green_bed` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct GreenBed {
        /// `part` state.
        pub part : Part,
        /// `occupied` state.
        pub occupied : bool,
        /// `facing` state.
        pub facing : Facing,
    }
    /// `part` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Part {
        /// `head` variant.
        Head,
        /// `foot` variant.
        Foot,
    }
    /// `facing` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Facing {
        /// `north` variant.
        North,
        /// `south` variant.
        South,
        /// `west` variant.
        West,
        /// `east` variant.
        East,
    }
    impl super::Block for GreenBed {
    }
    impl From<GreenBed> for super::BlockState {
        fn from(value : GreenBed) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:bamboo_sign` block.
pub mod bamboo_sign {
    /// `minecraft:bamboo_sign` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct BambooSign {
        /// `rotation` state.
        pub rotation : Rotation,
        /// `waterlogged` state.
        pub waterlogged : bool,
    }
    /// `rotation` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Rotation {
        /// `0` variant.
        N0,
        /// `1` variant.
        N1,
        /// `2` variant.
        N2,
        /// `3` variant.
        N3,
        /// `4` variant.
        N4,
        /// `5` variant.
        N5,
        /// `6` variant.
        N6,
        /// `7` variant.
        N7,
        /// `8` variant.
        N8,
        /// `9` variant.
        N9,
        /// `10` variant.
        N10,
        /// `11` variant.
        N11,
        /// `12` variant.
        N12,
        /// `13` variant.
        N13,
        /// `14` variant.
        N14,
        /// `15` variant.
        N15,
    }
    impl super::Block for BambooSign {
    }
    impl From<BambooSign> for super::BlockState {
        fn from(value : BambooSign) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:purpur_pillar` block.
pub mod purpur_pillar {
    /// `minecraft:purpur_pillar` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct PurpurPillar {
        /// `axis` state.
        pub axis : Axis,
    }
    /// `axis` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Axis {
        /// `x` variant.
        X,
        /// `y` variant.
        Y,
        /// `z` variant.
        Z,
    }
    impl super::Block for PurpurPillar {
    }
    impl From<PurpurPillar> for super::BlockState {
        fn from(value : PurpurPillar) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:black_concrete` block.
pub mod black_concrete {
    /// `minecraft:black_concrete` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct BlackConcrete;
    impl super::Block for BlackConcrete {
    }
    impl From<BlackConcrete> for super::BlockState {
        fn from(value : BlackConcrete) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:amethyst_block` block.
pub mod amethyst_block {
    /// `minecraft:amethyst_block` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct AmethystBlock;
    impl super::Block for AmethystBlock {
    }
    impl From<AmethystBlock> for super::BlockState {
        fn from(value : AmethystBlock) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:bamboo` block.
pub mod bamboo {
    /// `minecraft:bamboo` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct Bamboo {
        /// `leaves` state.
        pub leaves : Leaves,
        /// `age` state.
        pub age : Age,
        /// `stage` state.
        pub stage : Stage,
    }
    /// `leaves` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Leaves {
        /// `none` variant.
        None,
        /// `small` variant.
        Small,
        /// `large` variant.
        Large,
    }
    /// `age` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Age {
        /// `0` variant.
        N0,
        /// `1` variant.
        N1,
    }
    /// `stage` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Stage {
        /// `0` variant.
        N0,
        /// `1` variant.
        N1,
    }
    impl super::Block for Bamboo {
    }
    impl From<Bamboo> for super::BlockState {
        fn from(value : Bamboo) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:smooth_sandstone_slab` block.
pub mod smooth_sandstone_slab {
    /// `minecraft:smooth_sandstone_slab` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct SmoothSandstoneSlab {
        /// `waterlogged` state.
        pub waterlogged : bool,
        /// `type` state.
        pub kind : Kind,
    }
    /// `type` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Kind {
        /// `top` variant.
        Top,
        /// `bottom` variant.
        Bottom,
        /// `double` variant.
        Double,
    }
    impl super::Block for SmoothSandstoneSlab {
    }
    impl From<SmoothSandstoneSlab> for super::BlockState {
        fn from(value : SmoothSandstoneSlab) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:creeper_wall_head` block.
pub mod creeper_wall_head {
    /// `minecraft:creeper_wall_head` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct CreeperWallHead {
        /// `facing` state.
        pub facing : Facing,
        /// `powered` state.
        pub powered : bool,
    }
    /// `facing` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Facing {
        /// `north` variant.
        North,
        /// `south` variant.
        South,
        /// `west` variant.
        West,
        /// `east` variant.
        East,
    }
    impl super::Block for CreeperWallHead {
    }
    impl From<CreeperWallHead> for super::BlockState {
        fn from(value : CreeperWallHead) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:crimson_wall_hanging_sign` block.
pub mod crimson_wall_hanging_sign {
    /// `minecraft:crimson_wall_hanging_sign` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct CrimsonWallHangingSign {
        /// `waterlogged` state.
        pub waterlogged : bool,
        /// `facing` state.
        pub facing : Facing,
    }
    /// `facing` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Facing {
        /// `north` variant.
        North,
        /// `south` variant.
        South,
        /// `west` variant.
        West,
        /// `east` variant.
        East,
    }
    impl super::Block for CrimsonWallHangingSign {
    }
    impl From<CrimsonWallHangingSign> for super::BlockState {
        fn from(value : CrimsonWallHangingSign) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:red_sandstone` block.
pub mod red_sandstone {
    /// `minecraft:red_sandstone` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct RedSandstone;
    impl super::Block for RedSandstone {
    }
    impl From<RedSandstone> for super::BlockState {
        fn from(value : RedSandstone) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:seagrass` block.
pub mod seagrass {
    /// `minecraft:seagrass` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct Seagrass;
    impl super::Block for Seagrass {
    }
    impl From<Seagrass> for super::BlockState {
        fn from(value : Seagrass) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:sugar_cane` block.
pub mod sugar_cane {
    /// `minecraft:sugar_cane` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct SugarCane {
        /// `age` state.
        pub age : Age,
    }
    /// `age` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Age {
        /// `0` variant.
        N0,
        /// `1` variant.
        N1,
        /// `2` variant.
        N2,
        /// `3` variant.
        N3,
        /// `4` variant.
        N4,
        /// `5` variant.
        N5,
        /// `6` variant.
        N6,
        /// `7` variant.
        N7,
        /// `8` variant.
        N8,
        /// `9` variant.
        N9,
        /// `10` variant.
        N10,
        /// `11` variant.
        N11,
        /// `12` variant.
        N12,
        /// `13` variant.
        N13,
        /// `14` variant.
        N14,
        /// `15` variant.
        N15,
    }
    impl super::Block for SugarCane {
    }
    impl From<SugarCane> for super::BlockState {
        fn from(value : SugarCane) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:light_blue_banner` block.
pub mod light_blue_banner {
    /// `minecraft:light_blue_banner` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct LightBlueBanner {
        /// `rotation` state.
        pub rotation : Rotation,
    }
    /// `rotation` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Rotation {
        /// `0` variant.
        N0,
        /// `1` variant.
        N1,
        /// `2` variant.
        N2,
        /// `3` variant.
        N3,
        /// `4` variant.
        N4,
        /// `5` variant.
        N5,
        /// `6` variant.
        N6,
        /// `7` variant.
        N7,
        /// `8` variant.
        N8,
        /// `9` variant.
        N9,
        /// `10` variant.
        N10,
        /// `11` variant.
        N11,
        /// `12` variant.
        N12,
        /// `13` variant.
        N13,
        /// `14` variant.
        N14,
        /// `15` variant.
        N15,
    }
    impl super::Block for LightBlueBanner {
    }
    impl From<LightBlueBanner> for super::BlockState {
        fn from(value : LightBlueBanner) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:cherry_fence` block.
pub mod cherry_fence {
    /// `minecraft:cherry_fence` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct CherryFence {
        /// `west` state.
        pub west : bool,
        /// `east` state.
        pub east : bool,
        /// `waterlogged` state.
        pub waterlogged : bool,
        /// `south` state.
        pub south : bool,
        /// `north` state.
        pub north : bool,
    }
    impl super::Block for CherryFence {
    }
    impl From<CherryFence> for super::BlockState {
        fn from(value : CherryFence) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:blackstone` block.
pub mod blackstone {
    /// `minecraft:blackstone` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct Blackstone;
    impl super::Block for Blackstone {
    }
    impl From<Blackstone> for super::BlockState {
        fn from(value : Blackstone) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:cobbled_deepslate_slab` block.
pub mod cobbled_deepslate_slab {
    /// `minecraft:cobbled_deepslate_slab` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct CobbledDeepslateSlab {
        /// `waterlogged` state.
        pub waterlogged : bool,
        /// `type` state.
        pub kind : Kind,
    }
    /// `type` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Kind {
        /// `top` variant.
        Top,
        /// `bottom` variant.
        Bottom,
        /// `double` variant.
        Double,
    }
    impl super::Block for CobbledDeepslateSlab {
    }
    impl From<CobbledDeepslateSlab> for super::BlockState {
        fn from(value : CobbledDeepslateSlab) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:dead_bubble_coral` block.
pub mod dead_bubble_coral {
    /// `minecraft:dead_bubble_coral` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct DeadBubbleCoral {
        /// `waterlogged` state.
        pub waterlogged : bool,
    }
    impl super::Block for DeadBubbleCoral {
    }
    impl From<DeadBubbleCoral> for super::BlockState {
        fn from(value : DeadBubbleCoral) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:pitcher_plant` block.
pub mod pitcher_plant {
    /// `minecraft:pitcher_plant` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct PitcherPlant {
        /// `half` state.
        pub half : Half,
    }
    /// `half` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Half {
        /// `upper` variant.
        Upper,
        /// `lower` variant.
        Lower,
    }
    impl super::Block for PitcherPlant {
    }
    impl From<PitcherPlant> for super::BlockState {
        fn from(value : PitcherPlant) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:oak_planks` block.
pub mod oak_planks {
    /// `minecraft:oak_planks` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct OakPlanks;
    impl super::Block for OakPlanks {
    }
    impl From<OakPlanks> for super::BlockState {
        fn from(value : OakPlanks) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:bamboo_mosaic` block.
pub mod bamboo_mosaic {
    /// `minecraft:bamboo_mosaic` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct BambooMosaic;
    impl super::Block for BambooMosaic {
    }
    impl From<BambooMosaic> for super::BlockState {
        fn from(value : BambooMosaic) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:cherry_sign` block.
pub mod cherry_sign {
    /// `minecraft:cherry_sign` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct CherrySign {
        /// `rotation` state.
        pub rotation : Rotation,
        /// `waterlogged` state.
        pub waterlogged : bool,
    }
    /// `rotation` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Rotation {
        /// `0` variant.
        N0,
        /// `1` variant.
        N1,
        /// `2` variant.
        N2,
        /// `3` variant.
        N3,
        /// `4` variant.
        N4,
        /// `5` variant.
        N5,
        /// `6` variant.
        N6,
        /// `7` variant.
        N7,
        /// `8` variant.
        N8,
        /// `9` variant.
        N9,
        /// `10` variant.
        N10,
        /// `11` variant.
        N11,
        /// `12` variant.
        N12,
        /// `13` variant.
        N13,
        /// `14` variant.
        N14,
        /// `15` variant.
        N15,
    }
    impl super::Block for CherrySign {
    }
    impl From<CherrySign> for super::BlockState {
        fn from(value : CherrySign) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:crimson_hanging_sign` block.
pub mod crimson_hanging_sign {
    /// `minecraft:crimson_hanging_sign` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct CrimsonHangingSign {
        /// `waterlogged` state.
        pub waterlogged : bool,
        /// `rotation` state.
        pub rotation : Rotation,
        /// `attached` state.
        pub attached : bool,
    }
    /// `rotation` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Rotation {
        /// `0` variant.
        N0,
        /// `1` variant.
        N1,
        /// `2` variant.
        N2,
        /// `3` variant.
        N3,
        /// `4` variant.
        N4,
        /// `5` variant.
        N5,
        /// `6` variant.
        N6,
        /// `7` variant.
        N7,
        /// `8` variant.
        N8,
        /// `9` variant.
        N9,
        /// `10` variant.
        N10,
        /// `11` variant.
        N11,
        /// `12` variant.
        N12,
        /// `13` variant.
        N13,
        /// `14` variant.
        N14,
        /// `15` variant.
        N15,
    }
    impl super::Block for CrimsonHangingSign {
    }
    impl From<CrimsonHangingSign> for super::BlockState {
        fn from(value : CrimsonHangingSign) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:light_blue_stained_glass_pane` block.
pub mod light_blue_stained_glass_pane {
    /// `minecraft:light_blue_stained_glass_pane` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct LightBlueStainedGlassPane {
        /// `south` state.
        pub south : bool,
        /// `north` state.
        pub north : bool,
        /// `waterlogged` state.
        pub waterlogged : bool,
        /// `east` state.
        pub east : bool,
        /// `west` state.
        pub west : bool,
    }
    impl super::Block for LightBlueStainedGlassPane {
    }
    impl From<LightBlueStainedGlassPane> for super::BlockState {
        fn from(value : LightBlueStainedGlassPane) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:black_terracotta` block.
pub mod black_terracotta {
    /// `minecraft:black_terracotta` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct BlackTerracotta;
    impl super::Block for BlackTerracotta {
    }
    impl From<BlackTerracotta> for super::BlockState {
        fn from(value : BlackTerracotta) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:light_blue_stained_glass` block.
pub mod light_blue_stained_glass {
    /// `minecraft:light_blue_stained_glass` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct LightBlueStainedGlass;
    impl super::Block for LightBlueStainedGlass {
    }
    impl From<LightBlueStainedGlass> for super::BlockState {
        fn from(value : LightBlueStainedGlass) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:torch` block.
pub mod torch {
    /// `minecraft:torch` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct Torch;
    impl super::Block for Torch {
    }
    impl From<Torch> for super::BlockState {
        fn from(value : Torch) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:warped_sign` block.
pub mod warped_sign {
    /// `minecraft:warped_sign` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct WarpedSign {
        /// `rotation` state.
        pub rotation : Rotation,
        /// `waterlogged` state.
        pub waterlogged : bool,
    }
    /// `rotation` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Rotation {
        /// `0` variant.
        N0,
        /// `1` variant.
        N1,
        /// `2` variant.
        N2,
        /// `3` variant.
        N3,
        /// `4` variant.
        N4,
        /// `5` variant.
        N5,
        /// `6` variant.
        N6,
        /// `7` variant.
        N7,
        /// `8` variant.
        N8,
        /// `9` variant.
        N9,
        /// `10` variant.
        N10,
        /// `11` variant.
        N11,
        /// `12` variant.
        N12,
        /// `13` variant.
        N13,
        /// `14` variant.
        N14,
        /// `15` variant.
        N15,
    }
    impl super::Block for WarpedSign {
    }
    impl From<WarpedSign> for super::BlockState {
        fn from(value : WarpedSign) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:crimson_wall_sign` block.
pub mod crimson_wall_sign {
    /// `minecraft:crimson_wall_sign` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct CrimsonWallSign {
        /// `facing` state.
        pub facing : Facing,
        /// `waterlogged` state.
        pub waterlogged : bool,
    }
    /// `facing` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Facing {
        /// `north` variant.
        North,
        /// `south` variant.
        South,
        /// `west` variant.
        West,
        /// `east` variant.
        East,
    }
    impl super::Block for CrimsonWallSign {
    }
    impl From<CrimsonWallSign> for super::BlockState {
        fn from(value : CrimsonWallSign) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:repeating_command_block` block.
pub mod repeating_command_block {
    /// `minecraft:repeating_command_block` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct RepeatingCommandBlock {
        /// `conditional` state.
        pub conditional : bool,
        /// `facing` state.
        pub facing : Facing,
    }
    /// `facing` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Facing {
        /// `north` variant.
        North,
        /// `east` variant.
        East,
        /// `south` variant.
        South,
        /// `west` variant.
        West,
        /// `up` variant.
        Up,
        /// `down` variant.
        Down,
    }
    impl super::Block for RepeatingCommandBlock {
    }
    impl From<RepeatingCommandBlock> for super::BlockState {
        fn from(value : RepeatingCommandBlock) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:horn_coral_fan` block.
pub mod horn_coral_fan {
    /// `minecraft:horn_coral_fan` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct HornCoralFan {
        /// `waterlogged` state.
        pub waterlogged : bool,
    }
    impl super::Block for HornCoralFan {
    }
    impl From<HornCoralFan> for super::BlockState {
        fn from(value : HornCoralFan) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:torchflower_crop` block.
pub mod torchflower_crop {
    /// `minecraft:torchflower_crop` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct TorchflowerCrop {
        /// `age` state.
        pub age : Age,
    }
    /// `age` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Age {
        /// `0` variant.
        N0,
        /// `1` variant.
        N1,
    }
    impl super::Block for TorchflowerCrop {
    }
    impl From<TorchflowerCrop> for super::BlockState {
        fn from(value : TorchflowerCrop) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:jungle_leaves` block.
pub mod jungle_leaves {
    /// `minecraft:jungle_leaves` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct JungleLeaves {
        /// `waterlogged` state.
        pub waterlogged : bool,
        /// `distance` state.
        pub distance : Distance,
        /// `persistent` state.
        pub persistent : bool,
    }
    /// `distance` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Distance {
        /// `1` variant.
        N1,
        /// `2` variant.
        N2,
        /// `3` variant.
        N3,
        /// `4` variant.
        N4,
        /// `5` variant.
        N5,
        /// `6` variant.
        N6,
        /// `7` variant.
        N7,
    }
    impl super::Block for JungleLeaves {
    }
    impl From<JungleLeaves> for super::BlockState {
        fn from(value : JungleLeaves) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:mangrove_propagule` block.
pub mod mangrove_propagule {
    /// `minecraft:mangrove_propagule` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct MangrovePropagule {
        /// `stage` state.
        pub stage : Stage,
        /// `hanging` state.
        pub hanging : bool,
        /// `age` state.
        pub age : Age,
        /// `waterlogged` state.
        pub waterlogged : bool,
    }
    /// `stage` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Stage {
        /// `0` variant.
        N0,
        /// `1` variant.
        N1,
    }
    /// `age` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Age {
        /// `0` variant.
        N0,
        /// `1` variant.
        N1,
        /// `2` variant.
        N2,
        /// `3` variant.
        N3,
        /// `4` variant.
        N4,
    }
    impl super::Block for MangrovePropagule {
    }
    impl From<MangrovePropagule> for super::BlockState {
        fn from(value : MangrovePropagule) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:bamboo_planks` block.
pub mod bamboo_planks {
    /// `minecraft:bamboo_planks` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct BambooPlanks;
    impl super::Block for BambooPlanks {
    }
    impl From<BambooPlanks> for super::BlockState {
        fn from(value : BambooPlanks) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:stripped_pale_oak_wood` block.
pub mod stripped_pale_oak_wood {
    /// `minecraft:stripped_pale_oak_wood` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct StrippedPaleOakWood {
        /// `axis` state.
        pub axis : Axis,
    }
    /// `axis` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Axis {
        /// `x` variant.
        X,
        /// `y` variant.
        Y,
        /// `z` variant.
        Z,
    }
    impl super::Block for StrippedPaleOakWood {
    }
    impl From<StrippedPaleOakWood> for super::BlockState {
        fn from(value : StrippedPaleOakWood) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:peony` block.
pub mod peony {
    /// `minecraft:peony` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct Peony {
        /// `half` state.
        pub half : Half,
    }
    /// `half` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Half {
        /// `upper` variant.
        Upper,
        /// `lower` variant.
        Lower,
    }
    impl super::Block for Peony {
    }
    impl From<Peony> for super::BlockState {
        fn from(value : Peony) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:potted_red_tulip` block.
pub mod potted_red_tulip {
    /// `minecraft:potted_red_tulip` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct PottedRedTulip;
    impl super::Block for PottedRedTulip {
    }
    impl From<PottedRedTulip> for super::BlockState {
        fn from(value : PottedRedTulip) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:copper_trapdoor` block.
pub mod copper_trapdoor {
    /// `minecraft:copper_trapdoor` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct CopperTrapdoor {
        /// `powered` state.
        pub powered : bool,
        /// `waterlogged` state.
        pub waterlogged : bool,
        /// `facing` state.
        pub facing : Facing,
        /// `open` state.
        pub open : bool,
        /// `half` state.
        pub half : Half,
    }
    /// `facing` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Facing {
        /// `north` variant.
        North,
        /// `south` variant.
        South,
        /// `west` variant.
        West,
        /// `east` variant.
        East,
    }
    /// `half` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Half {
        /// `top` variant.
        Top,
        /// `bottom` variant.
        Bottom,
    }
    impl super::Block for CopperTrapdoor {
    }
    impl From<CopperTrapdoor> for super::BlockState {
        fn from(value : CopperTrapdoor) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:blue_candle` block.
pub mod blue_candle {
    /// `minecraft:blue_candle` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct BlueCandle {
        /// `waterlogged` state.
        pub waterlogged : bool,
        /// `candles` state.
        pub candles : Candles,
        /// `lit` state.
        pub lit : bool,
    }
    /// `candles` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Candles {
        /// `1` variant.
        N1,
        /// `2` variant.
        N2,
        /// `3` variant.
        N3,
        /// `4` variant.
        N4,
    }
    impl super::Block for BlueCandle {
    }
    impl From<BlueCandle> for super::BlockState {
        fn from(value : BlueCandle) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:grindstone` block.
pub mod grindstone {
    /// `minecraft:grindstone` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct Grindstone {
        /// `facing` state.
        pub facing : Facing,
        /// `face` state.
        pub face : Face,
    }
    /// `facing` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Facing {
        /// `north` variant.
        North,
        /// `south` variant.
        South,
        /// `west` variant.
        West,
        /// `east` variant.
        East,
    }
    /// `face` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Face {
        /// `floor` variant.
        Floor,
        /// `wall` variant.
        Wall,
        /// `ceiling` variant.
        Ceiling,
    }
    impl super::Block for Grindstone {
    }
    impl From<Grindstone> for super::BlockState {
        fn from(value : Grindstone) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:birch_slab` block.
pub mod birch_slab {
    /// `minecraft:birch_slab` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct BirchSlab {
        /// `type` state.
        pub kind : Kind,
        /// `waterlogged` state.
        pub waterlogged : bool,
    }
    /// `type` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Kind {
        /// `top` variant.
        Top,
        /// `bottom` variant.
        Bottom,
        /// `double` variant.
        Double,
    }
    impl super::Block for BirchSlab {
    }
    impl From<BirchSlab> for super::BlockState {
        fn from(value : BirchSlab) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:light_gray_candle` block.
pub mod light_gray_candle {
    /// `minecraft:light_gray_candle` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct LightGrayCandle {
        /// `candles` state.
        pub candles : Candles,
        /// `lit` state.
        pub lit : bool,
        /// `waterlogged` state.
        pub waterlogged : bool,
    }
    /// `candles` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Candles {
        /// `1` variant.
        N1,
        /// `2` variant.
        N2,
        /// `3` variant.
        N3,
        /// `4` variant.
        N4,
    }
    impl super::Block for LightGrayCandle {
    }
    impl From<LightGrayCandle> for super::BlockState {
        fn from(value : LightGrayCandle) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:potatoes` block.
pub mod potatoes {
    /// `minecraft:potatoes` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct Potatoes {
        /// `age` state.
        pub age : Age,
    }
    /// `age` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Age {
        /// `0` variant.
        N0,
        /// `1` variant.
        N1,
        /// `2` variant.
        N2,
        /// `3` variant.
        N3,
        /// `4` variant.
        N4,
        /// `5` variant.
        N5,
        /// `6` variant.
        N6,
        /// `7` variant.
        N7,
    }
    impl super::Block for Potatoes {
    }
    impl From<Potatoes> for super::BlockState {
        fn from(value : Potatoes) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:chiseled_tuff_bricks` block.
pub mod chiseled_tuff_bricks {
    /// `minecraft:chiseled_tuff_bricks` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct ChiseledTuffBricks;
    impl super::Block for ChiseledTuffBricks {
    }
    impl From<ChiseledTuffBricks> for super::BlockState {
        fn from(value : ChiseledTuffBricks) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:spruce_stairs` block.
pub mod spruce_stairs {
    /// `minecraft:spruce_stairs` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct SpruceStairs {
        /// `facing` state.
        pub facing : Facing,
        /// `half` state.
        pub half : Half,
        /// `shape` state.
        pub shape : Shape,
        /// `waterlogged` state.
        pub waterlogged : bool,
    }
    /// `facing` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Facing {
        /// `north` variant.
        North,
        /// `south` variant.
        South,
        /// `west` variant.
        West,
        /// `east` variant.
        East,
    }
    /// `half` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Half {
        /// `top` variant.
        Top,
        /// `bottom` variant.
        Bottom,
    }
    /// `shape` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Shape {
        /// `straight` variant.
        Straight,
        /// `inner_left` variant.
        InnerLeft,
        /// `inner_right` variant.
        InnerRight,
        /// `outer_left` variant.
        OuterLeft,
        /// `outer_right` variant.
        OuterRight,
    }
    impl super::Block for SpruceStairs {
    }
    impl From<SpruceStairs> for super::BlockState {
        fn from(value : SpruceStairs) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:basalt` block.
pub mod basalt {
    /// `minecraft:basalt` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct Basalt {
        /// `axis` state.
        pub axis : Axis,
    }
    /// `axis` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Axis {
        /// `x` variant.
        X,
        /// `y` variant.
        Y,
        /// `z` variant.
        Z,
    }
    impl super::Block for Basalt {
    }
    impl From<Basalt> for super::BlockState {
        fn from(value : Basalt) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:waxed_oxidized_cut_copper` block.
pub mod waxed_oxidized_cut_copper {
    /// `minecraft:waxed_oxidized_cut_copper` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct WaxedOxidizedCutCopper;
    impl super::Block for WaxedOxidizedCutCopper {
    }
    impl From<WaxedOxidizedCutCopper> for super::BlockState {
        fn from(value : WaxedOxidizedCutCopper) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:acacia_hanging_sign` block.
pub mod acacia_hanging_sign {
    /// `minecraft:acacia_hanging_sign` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct AcaciaHangingSign {
        /// `rotation` state.
        pub rotation : Rotation,
        /// `waterlogged` state.
        pub waterlogged : bool,
        /// `attached` state.
        pub attached : bool,
    }
    /// `rotation` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Rotation {
        /// `0` variant.
        N0,
        /// `1` variant.
        N1,
        /// `2` variant.
        N2,
        /// `3` variant.
        N3,
        /// `4` variant.
        N4,
        /// `5` variant.
        N5,
        /// `6` variant.
        N6,
        /// `7` variant.
        N7,
        /// `8` variant.
        N8,
        /// `9` variant.
        N9,
        /// `10` variant.
        N10,
        /// `11` variant.
        N11,
        /// `12` variant.
        N12,
        /// `13` variant.
        N13,
        /// `14` variant.
        N14,
        /// `15` variant.
        N15,
    }
    impl super::Block for AcaciaHangingSign {
    }
    impl From<AcaciaHangingSign> for super::BlockState {
        fn from(value : AcaciaHangingSign) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:chiseled_sandstone` block.
pub mod chiseled_sandstone {
    /// `minecraft:chiseled_sandstone` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct ChiseledSandstone;
    impl super::Block for ChiseledSandstone {
    }
    impl From<ChiseledSandstone> for super::BlockState {
        fn from(value : ChiseledSandstone) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:big_dripleaf_stem` block.
pub mod big_dripleaf_stem {
    /// `minecraft:big_dripleaf_stem` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct BigDripleafStem {
        /// `waterlogged` state.
        pub waterlogged : bool,
        /// `facing` state.
        pub facing : Facing,
    }
    /// `facing` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Facing {
        /// `north` variant.
        North,
        /// `south` variant.
        South,
        /// `west` variant.
        West,
        /// `east` variant.
        East,
    }
    impl super::Block for BigDripleafStem {
    }
    impl From<BigDripleafStem> for super::BlockState {
        fn from(value : BigDripleafStem) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:comparator` block.
pub mod comparator {
    /// `minecraft:comparator` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct Comparator {
        /// `facing` state.
        pub facing : Facing,
        /// `powered` state.
        pub powered : bool,
        /// `mode` state.
        pub mode : Mode,
    }
    /// `facing` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Facing {
        /// `north` variant.
        North,
        /// `south` variant.
        South,
        /// `west` variant.
        West,
        /// `east` variant.
        East,
    }
    /// `mode` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Mode {
        /// `compare` variant.
        Compare,
        /// `subtract` variant.
        Subtract,
    }
    impl super::Block for Comparator {
    }
    impl From<Comparator> for super::BlockState {
        fn from(value : Comparator) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:light_gray_wool` block.
pub mod light_gray_wool {
    /// `minecraft:light_gray_wool` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct LightGrayWool;
    impl super::Block for LightGrayWool {
    }
    impl From<LightGrayWool> for super::BlockState {
        fn from(value : LightGrayWool) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:oak_pressure_plate` block.
pub mod oak_pressure_plate {
    /// `minecraft:oak_pressure_plate` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct OakPressurePlate {
        /// `powered` state.
        pub powered : bool,
    }
    impl super::Block for OakPressurePlate {
    }
    impl From<OakPressurePlate> for super::BlockState {
        fn from(value : OakPressurePlate) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:birch_log` block.
pub mod birch_log {
    /// `minecraft:birch_log` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct BirchLog {
        /// `axis` state.
        pub axis : Axis,
    }
    /// `axis` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Axis {
        /// `x` variant.
        X,
        /// `y` variant.
        Y,
        /// `z` variant.
        Z,
    }
    impl super::Block for BirchLog {
    }
    impl From<BirchLog> for super::BlockState {
        fn from(value : BirchLog) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:brown_stained_glass_pane` block.
pub mod brown_stained_glass_pane {
    /// `minecraft:brown_stained_glass_pane` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct BrownStainedGlassPane {
        /// `north` state.
        pub north : bool,
        /// `east` state.
        pub east : bool,
        /// `south` state.
        pub south : bool,
        /// `waterlogged` state.
        pub waterlogged : bool,
        /// `west` state.
        pub west : bool,
    }
    impl super::Block for BrownStainedGlassPane {
    }
    impl From<BrownStainedGlassPane> for super::BlockState {
        fn from(value : BrownStainedGlassPane) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:dead_tube_coral_wall_fan` block.
pub mod dead_tube_coral_wall_fan {
    /// `minecraft:dead_tube_coral_wall_fan` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct DeadTubeCoralWallFan {
        /// `waterlogged` state.
        pub waterlogged : bool,
        /// `facing` state.
        pub facing : Facing,
    }
    /// `facing` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Facing {
        /// `north` variant.
        North,
        /// `south` variant.
        South,
        /// `west` variant.
        West,
        /// `east` variant.
        East,
    }
    impl super::Block for DeadTubeCoralWallFan {
    }
    impl From<DeadTubeCoralWallFan> for super::BlockState {
        fn from(value : DeadTubeCoralWallFan) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:acacia_leaves` block.
pub mod acacia_leaves {
    /// `minecraft:acacia_leaves` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct AcaciaLeaves {
        /// `distance` state.
        pub distance : Distance,
        /// `persistent` state.
        pub persistent : bool,
        /// `waterlogged` state.
        pub waterlogged : bool,
    }
    /// `distance` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Distance {
        /// `1` variant.
        N1,
        /// `2` variant.
        N2,
        /// `3` variant.
        N3,
        /// `4` variant.
        N4,
        /// `5` variant.
        N5,
        /// `6` variant.
        N6,
        /// `7` variant.
        N7,
    }
    impl super::Block for AcaciaLeaves {
    }
    impl From<AcaciaLeaves> for super::BlockState {
        fn from(value : AcaciaLeaves) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:exposed_copper_trapdoor` block.
pub mod exposed_copper_trapdoor {
    /// `minecraft:exposed_copper_trapdoor` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct ExposedCopperTrapdoor {
        /// `powered` state.
        pub powered : bool,
        /// `open` state.
        pub open : bool,
        /// `waterlogged` state.
        pub waterlogged : bool,
        /// `half` state.
        pub half : Half,
        /// `facing` state.
        pub facing : Facing,
    }
    /// `half` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Half {
        /// `top` variant.
        Top,
        /// `bottom` variant.
        Bottom,
    }
    /// `facing` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Facing {
        /// `north` variant.
        North,
        /// `south` variant.
        South,
        /// `west` variant.
        West,
        /// `east` variant.
        East,
    }
    impl super::Block for ExposedCopperTrapdoor {
    }
    impl From<ExposedCopperTrapdoor> for super::BlockState {
        fn from(value : ExposedCopperTrapdoor) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:lime_concrete` block.
pub mod lime_concrete {
    /// `minecraft:lime_concrete` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct LimeConcrete;
    impl super::Block for LimeConcrete {
    }
    impl From<LimeConcrete> for super::BlockState {
        fn from(value : LimeConcrete) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:potted_crimson_roots` block.
pub mod potted_crimson_roots {
    /// `minecraft:potted_crimson_roots` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct PottedCrimsonRoots;
    impl super::Block for PottedCrimsonRoots {
    }
    impl From<PottedCrimsonRoots> for super::BlockState {
        fn from(value : PottedCrimsonRoots) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:potted_warped_fungus` block.
pub mod potted_warped_fungus {
    /// `minecraft:potted_warped_fungus` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct PottedWarpedFungus;
    impl super::Block for PottedWarpedFungus {
    }
    impl From<PottedWarpedFungus> for super::BlockState {
        fn from(value : PottedWarpedFungus) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:deepslate_coal_ore` block.
pub mod deepslate_coal_ore {
    /// `minecraft:deepslate_coal_ore` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct DeepslateCoalOre;
    impl super::Block for DeepslateCoalOre {
    }
    impl From<DeepslateCoalOre> for super::BlockState {
        fn from(value : DeepslateCoalOre) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:repeater` block.
pub mod repeater {
    /// `minecraft:repeater` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct Repeater {
        /// `locked` state.
        pub locked : bool,
        /// `facing` state.
        pub facing : Facing,
        /// `powered` state.
        pub powered : bool,
        /// `delay` state.
        pub delay : Delay,
    }
    /// `facing` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Facing {
        /// `north` variant.
        North,
        /// `south` variant.
        South,
        /// `west` variant.
        West,
        /// `east` variant.
        East,
    }
    /// `delay` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Delay {
        /// `1` variant.
        N1,
        /// `2` variant.
        N2,
        /// `3` variant.
        N3,
        /// `4` variant.
        N4,
    }
    impl super::Block for Repeater {
    }
    impl From<Repeater> for super::BlockState {
        fn from(value : Repeater) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:birch_wall_hanging_sign` block.
pub mod birch_wall_hanging_sign {
    /// `minecraft:birch_wall_hanging_sign` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct BirchWallHangingSign {
        /// `waterlogged` state.
        pub waterlogged : bool,
        /// `facing` state.
        pub facing : Facing,
    }
    /// `facing` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Facing {
        /// `north` variant.
        North,
        /// `south` variant.
        South,
        /// `west` variant.
        West,
        /// `east` variant.
        East,
    }
    impl super::Block for BirchWallHangingSign {
    }
    impl From<BirchWallHangingSign> for super::BlockState {
        fn from(value : BirchWallHangingSign) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:spruce_trapdoor` block.
pub mod spruce_trapdoor {
    /// `minecraft:spruce_trapdoor` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct SpruceTrapdoor {
        /// `facing` state.
        pub facing : Facing,
        /// `powered` state.
        pub powered : bool,
        /// `half` state.
        pub half : Half,
        /// `open` state.
        pub open : bool,
        /// `waterlogged` state.
        pub waterlogged : bool,
    }
    /// `facing` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Facing {
        /// `north` variant.
        North,
        /// `south` variant.
        South,
        /// `west` variant.
        West,
        /// `east` variant.
        East,
    }
    /// `half` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Half {
        /// `top` variant.
        Top,
        /// `bottom` variant.
        Bottom,
    }
    impl super::Block for SpruceTrapdoor {
    }
    impl From<SpruceTrapdoor> for super::BlockState {
        fn from(value : SpruceTrapdoor) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:dark_oak_wood` block.
pub mod dark_oak_wood {
    /// `minecraft:dark_oak_wood` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct DarkOakWood {
        /// `axis` state.
        pub axis : Axis,
    }
    /// `axis` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Axis {
        /// `x` variant.
        X,
        /// `y` variant.
        Y,
        /// `z` variant.
        Z,
    }
    impl super::Block for DarkOakWood {
    }
    impl From<DarkOakWood> for super::BlockState {
        fn from(value : DarkOakWood) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:bedrock` block.
pub mod bedrock {
    /// `minecraft:bedrock` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct Bedrock;
    impl super::Block for Bedrock {
    }
    impl From<Bedrock> for super::BlockState {
        fn from(value : Bedrock) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:brain_coral_wall_fan` block.
pub mod brain_coral_wall_fan {
    /// `minecraft:brain_coral_wall_fan` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct BrainCoralWallFan {
        /// `facing` state.
        pub facing : Facing,
        /// `waterlogged` state.
        pub waterlogged : bool,
    }
    /// `facing` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Facing {
        /// `north` variant.
        North,
        /// `south` variant.
        South,
        /// `west` variant.
        West,
        /// `east` variant.
        East,
    }
    impl super::Block for BrainCoralWallFan {
    }
    impl From<BrainCoralWallFan> for super::BlockState {
        fn from(value : BrainCoralWallFan) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:purple_banner` block.
pub mod purple_banner {
    /// `minecraft:purple_banner` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct PurpleBanner {
        /// `rotation` state.
        pub rotation : Rotation,
    }
    /// `rotation` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Rotation {
        /// `0` variant.
        N0,
        /// `1` variant.
        N1,
        /// `2` variant.
        N2,
        /// `3` variant.
        N3,
        /// `4` variant.
        N4,
        /// `5` variant.
        N5,
        /// `6` variant.
        N6,
        /// `7` variant.
        N7,
        /// `8` variant.
        N8,
        /// `9` variant.
        N9,
        /// `10` variant.
        N10,
        /// `11` variant.
        N11,
        /// `12` variant.
        N12,
        /// `13` variant.
        N13,
        /// `14` variant.
        N14,
        /// `15` variant.
        N15,
    }
    impl super::Block for PurpleBanner {
    }
    impl From<PurpleBanner> for super::BlockState {
        fn from(value : PurpleBanner) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:white_carpet` block.
pub mod white_carpet {
    /// `minecraft:white_carpet` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct WhiteCarpet;
    impl super::Block for WhiteCarpet {
    }
    impl From<WhiteCarpet> for super::BlockState {
        fn from(value : WhiteCarpet) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:yellow_wool` block.
pub mod yellow_wool {
    /// `minecraft:yellow_wool` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct YellowWool;
    impl super::Block for YellowWool {
    }
    impl From<YellowWool> for super::BlockState {
        fn from(value : YellowWool) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:light_blue_concrete` block.
pub mod light_blue_concrete {
    /// `minecraft:light_blue_concrete` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct LightBlueConcrete;
    impl super::Block for LightBlueConcrete {
    }
    impl From<LightBlueConcrete> for super::BlockState {
        fn from(value : LightBlueConcrete) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:moving_piston` block.
pub mod moving_piston {
    /// `minecraft:moving_piston` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct MovingPiston {
        /// `facing` state.
        pub facing : Facing,
        /// `type` state.
        pub kind : Kind,
    }
    /// `facing` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Facing {
        /// `north` variant.
        North,
        /// `east` variant.
        East,
        /// `south` variant.
        South,
        /// `west` variant.
        West,
        /// `up` variant.
        Up,
        /// `down` variant.
        Down,
    }
    /// `type` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Kind {
        /// `normal` variant.
        Normal,
        /// `sticky` variant.
        Sticky,
    }
    impl super::Block for MovingPiston {
    }
    impl From<MovingPiston> for super::BlockState {
        fn from(value : MovingPiston) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:dead_fire_coral_wall_fan` block.
pub mod dead_fire_coral_wall_fan {
    /// `minecraft:dead_fire_coral_wall_fan` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct DeadFireCoralWallFan {
        /// `waterlogged` state.
        pub waterlogged : bool,
        /// `facing` state.
        pub facing : Facing,
    }
    /// `facing` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Facing {
        /// `north` variant.
        North,
        /// `south` variant.
        South,
        /// `west` variant.
        West,
        /// `east` variant.
        East,
    }
    impl super::Block for DeadFireCoralWallFan {
    }
    impl From<DeadFireCoralWallFan> for super::BlockState {
        fn from(value : DeadFireCoralWallFan) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:red_shulker_box` block.
pub mod red_shulker_box {
    /// `minecraft:red_shulker_box` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct RedShulkerBox {
        /// `facing` state.
        pub facing : Facing,
    }
    /// `facing` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Facing {
        /// `north` variant.
        North,
        /// `east` variant.
        East,
        /// `south` variant.
        South,
        /// `west` variant.
        West,
        /// `up` variant.
        Up,
        /// `down` variant.
        Down,
    }
    impl super::Block for RedShulkerBox {
    }
    impl From<RedShulkerBox> for super::BlockState {
        fn from(value : RedShulkerBox) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:orange_stained_glass_pane` block.
pub mod orange_stained_glass_pane {
    /// `minecraft:orange_stained_glass_pane` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct OrangeStainedGlassPane {
        /// `south` state.
        pub south : bool,
        /// `north` state.
        pub north : bool,
        /// `east` state.
        pub east : bool,
        /// `west` state.
        pub west : bool,
        /// `waterlogged` state.
        pub waterlogged : bool,
    }
    impl super::Block for OrangeStainedGlassPane {
    }
    impl From<OrangeStainedGlassPane> for super::BlockState {
        fn from(value : OrangeStainedGlassPane) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:chorus_flower` block.
pub mod chorus_flower {
    /// `minecraft:chorus_flower` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct ChorusFlower {
        /// `age` state.
        pub age : Age,
    }
    /// `age` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Age {
        /// `0` variant.
        N0,
        /// `1` variant.
        N1,
        /// `2` variant.
        N2,
        /// `3` variant.
        N3,
        /// `4` variant.
        N4,
        /// `5` variant.
        N5,
    }
    impl super::Block for ChorusFlower {
    }
    impl From<ChorusFlower> for super::BlockState {
        fn from(value : ChorusFlower) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:bamboo_wall_hanging_sign` block.
pub mod bamboo_wall_hanging_sign {
    /// `minecraft:bamboo_wall_hanging_sign` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct BambooWallHangingSign {
        /// `waterlogged` state.
        pub waterlogged : bool,
        /// `facing` state.
        pub facing : Facing,
    }
    /// `facing` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Facing {
        /// `north` variant.
        North,
        /// `south` variant.
        South,
        /// `west` variant.
        West,
        /// `east` variant.
        East,
    }
    impl super::Block for BambooWallHangingSign {
    }
    impl From<BambooWallHangingSign> for super::BlockState {
        fn from(value : BambooWallHangingSign) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:mangrove_door` block.
pub mod mangrove_door {
    /// `minecraft:mangrove_door` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct MangroveDoor {
        /// `half` state.
        pub half : Half,
        /// `facing` state.
        pub facing : Facing,
        /// `open` state.
        pub open : bool,
        /// `powered` state.
        pub powered : bool,
        /// `hinge` state.
        pub hinge : Hinge,
    }
    /// `half` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Half {
        /// `upper` variant.
        Upper,
        /// `lower` variant.
        Lower,
    }
    /// `facing` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Facing {
        /// `north` variant.
        North,
        /// `south` variant.
        South,
        /// `west` variant.
        West,
        /// `east` variant.
        East,
    }
    /// `hinge` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Hinge {
        /// `left` variant.
        Left,
        /// `right` variant.
        Right,
    }
    impl super::Block for MangroveDoor {
    }
    impl From<MangroveDoor> for super::BlockState {
        fn from(value : MangroveDoor) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:verdant_froglight` block.
pub mod verdant_froglight {
    /// `minecraft:verdant_froglight` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct VerdantFroglight {
        /// `axis` state.
        pub axis : Axis,
    }
    /// `axis` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Axis {
        /// `x` variant.
        X,
        /// `y` variant.
        Y,
        /// `z` variant.
        Z,
    }
    impl super::Block for VerdantFroglight {
    }
    impl From<VerdantFroglight> for super::BlockState {
        fn from(value : VerdantFroglight) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:cherry_pressure_plate` block.
pub mod cherry_pressure_plate {
    /// `minecraft:cherry_pressure_plate` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct CherryPressurePlate {
        /// `powered` state.
        pub powered : bool,
    }
    impl super::Block for CherryPressurePlate {
    }
    impl From<CherryPressurePlate> for super::BlockState {
        fn from(value : CherryPressurePlate) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:acacia_slab` block.
pub mod acacia_slab {
    /// `minecraft:acacia_slab` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct AcaciaSlab {
        /// `type` state.
        pub kind : Kind,
        /// `waterlogged` state.
        pub waterlogged : bool,
    }
    /// `type` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Kind {
        /// `top` variant.
        Top,
        /// `bottom` variant.
        Bottom,
        /// `double` variant.
        Double,
    }
    impl super::Block for AcaciaSlab {
    }
    impl From<AcaciaSlab> for super::BlockState {
        fn from(value : AcaciaSlab) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:soul_wall_torch` block.
pub mod soul_wall_torch {
    /// `minecraft:soul_wall_torch` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct SoulWallTorch {
        /// `facing` state.
        pub facing : Facing,
    }
    /// `facing` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Facing {
        /// `north` variant.
        North,
        /// `south` variant.
        South,
        /// `west` variant.
        West,
        /// `east` variant.
        East,
    }
    impl super::Block for SoulWallTorch {
    }
    impl From<SoulWallTorch> for super::BlockState {
        fn from(value : SoulWallTorch) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:attached_melon_stem` block.
pub mod attached_melon_stem {
    /// `minecraft:attached_melon_stem` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct AttachedMelonStem {
        /// `facing` state.
        pub facing : Facing,
    }
    /// `facing` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Facing {
        /// `north` variant.
        North,
        /// `south` variant.
        South,
        /// `west` variant.
        West,
        /// `east` variant.
        East,
    }
    impl super::Block for AttachedMelonStem {
    }
    impl From<AttachedMelonStem> for super::BlockState {
        fn from(value : AttachedMelonStem) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:diorite_slab` block.
pub mod diorite_slab {
    /// `minecraft:diorite_slab` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct DioriteSlab {
        /// `type` state.
        pub kind : Kind,
        /// `waterlogged` state.
        pub waterlogged : bool,
    }
    /// `type` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Kind {
        /// `top` variant.
        Top,
        /// `bottom` variant.
        Bottom,
        /// `double` variant.
        Double,
    }
    impl super::Block for DioriteSlab {
    }
    impl From<DioriteSlab> for super::BlockState {
        fn from(value : DioriteSlab) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:lime_candle_cake` block.
pub mod lime_candle_cake {
    /// `minecraft:lime_candle_cake` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct LimeCandleCake {
        /// `lit` state.
        pub lit : bool,
    }
    impl super::Block for LimeCandleCake {
    }
    impl From<LimeCandleCake> for super::BlockState {
        fn from(value : LimeCandleCake) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:birch_wood` block.
pub mod birch_wood {
    /// `minecraft:birch_wood` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct BirchWood {
        /// `axis` state.
        pub axis : Axis,
    }
    /// `axis` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Axis {
        /// `x` variant.
        X,
        /// `y` variant.
        Y,
        /// `z` variant.
        Z,
    }
    impl super::Block for BirchWood {
    }
    impl From<BirchWood> for super::BlockState {
        fn from(value : BirchWood) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:polished_blackstone_wall` block.
pub mod polished_blackstone_wall {
    /// `minecraft:polished_blackstone_wall` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct PolishedBlackstoneWall {
        /// `up` state.
        pub up : bool,
        /// `south` state.
        pub south : South,
        /// `waterlogged` state.
        pub waterlogged : bool,
        /// `west` state.
        pub west : West,
        /// `north` state.
        pub north : North,
        /// `east` state.
        pub east : East,
    }
    /// `south` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum South {
        /// `none` variant.
        None,
        /// `low` variant.
        Low,
        /// `tall` variant.
        Tall,
    }
    /// `west` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum West {
        /// `none` variant.
        None,
        /// `low` variant.
        Low,
        /// `tall` variant.
        Tall,
    }
    /// `north` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum North {
        /// `none` variant.
        None,
        /// `low` variant.
        Low,
        /// `tall` variant.
        Tall,
    }
    /// `east` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum East {
        /// `none` variant.
        None,
        /// `low` variant.
        Low,
        /// `tall` variant.
        Tall,
    }
    impl super::Block for PolishedBlackstoneWall {
    }
    impl From<PolishedBlackstoneWall> for super::BlockState {
        fn from(value : PolishedBlackstoneWall) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:dead_tube_coral_block` block.
pub mod dead_tube_coral_block {
    /// `minecraft:dead_tube_coral_block` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct DeadTubeCoralBlock;
    impl super::Block for DeadTubeCoralBlock {
    }
    impl From<DeadTubeCoralBlock> for super::BlockState {
        fn from(value : DeadTubeCoralBlock) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:magenta_candle` block.
pub mod magenta_candle {
    /// `minecraft:magenta_candle` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct MagentaCandle {
        /// `waterlogged` state.
        pub waterlogged : bool,
        /// `lit` state.
        pub lit : bool,
        /// `candles` state.
        pub candles : Candles,
    }
    /// `candles` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Candles {
        /// `1` variant.
        N1,
        /// `2` variant.
        N2,
        /// `3` variant.
        N3,
        /// `4` variant.
        N4,
    }
    impl super::Block for MagentaCandle {
    }
    impl From<MagentaCandle> for super::BlockState {
        fn from(value : MagentaCandle) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:spruce_wood` block.
pub mod spruce_wood {
    /// `minecraft:spruce_wood` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct SpruceWood {
        /// `axis` state.
        pub axis : Axis,
    }
    /// `axis` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Axis {
        /// `x` variant.
        X,
        /// `y` variant.
        Y,
        /// `z` variant.
        Z,
    }
    impl super::Block for SpruceWood {
    }
    impl From<SpruceWood> for super::BlockState {
        fn from(value : SpruceWood) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:tuff_brick_wall` block.
pub mod tuff_brick_wall {
    /// `minecraft:tuff_brick_wall` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct TuffBrickWall {
        /// `up` state.
        pub up : bool,
        /// `east` state.
        pub east : East,
        /// `west` state.
        pub west : West,
        /// `waterlogged` state.
        pub waterlogged : bool,
        /// `north` state.
        pub north : North,
        /// `south` state.
        pub south : South,
    }
    /// `east` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum East {
        /// `none` variant.
        None,
        /// `low` variant.
        Low,
        /// `tall` variant.
        Tall,
    }
    /// `west` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum West {
        /// `none` variant.
        None,
        /// `low` variant.
        Low,
        /// `tall` variant.
        Tall,
    }
    /// `north` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum North {
        /// `none` variant.
        None,
        /// `low` variant.
        Low,
        /// `tall` variant.
        Tall,
    }
    /// `south` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum South {
        /// `none` variant.
        None,
        /// `low` variant.
        Low,
        /// `tall` variant.
        Tall,
    }
    impl super::Block for TuffBrickWall {
    }
    impl From<TuffBrickWall> for super::BlockState {
        fn from(value : TuffBrickWall) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:blue_carpet` block.
pub mod blue_carpet {
    /// `minecraft:blue_carpet` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct BlueCarpet;
    impl super::Block for BlueCarpet {
    }
    impl From<BlueCarpet> for super::BlockState {
        fn from(value : BlueCarpet) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:cyan_candle` block.
pub mod cyan_candle {
    /// `minecraft:cyan_candle` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct CyanCandle {
        /// `waterlogged` state.
        pub waterlogged : bool,
        /// `candles` state.
        pub candles : Candles,
        /// `lit` state.
        pub lit : bool,
    }
    /// `candles` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Candles {
        /// `1` variant.
        N1,
        /// `2` variant.
        N2,
        /// `3` variant.
        N3,
        /// `4` variant.
        N4,
    }
    impl super::Block for CyanCandle {
    }
    impl From<CyanCandle> for super::BlockState {
        fn from(value : CyanCandle) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:chorus_plant` block.
pub mod chorus_plant {
    /// `minecraft:chorus_plant` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct ChorusPlant {
        /// `west` state.
        pub west : bool,
        /// `east` state.
        pub east : bool,
        /// `up` state.
        pub up : bool,
        /// `down` state.
        pub down : bool,
        /// `north` state.
        pub north : bool,
        /// `south` state.
        pub south : bool,
    }
    impl super::Block for ChorusPlant {
    }
    impl From<ChorusPlant> for super::BlockState {
        fn from(value : ChorusPlant) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:purpur_block` block.
pub mod purpur_block {
    /// `minecraft:purpur_block` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct PurpurBlock;
    impl super::Block for PurpurBlock {
    }
    impl From<PurpurBlock> for super::BlockState {
        fn from(value : PurpurBlock) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:light_blue_carpet` block.
pub mod light_blue_carpet {
    /// `minecraft:light_blue_carpet` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct LightBlueCarpet;
    impl super::Block for LightBlueCarpet {
    }
    impl From<LightBlueCarpet> for super::BlockState {
        fn from(value : LightBlueCarpet) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:birch_trapdoor` block.
pub mod birch_trapdoor {
    /// `minecraft:birch_trapdoor` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct BirchTrapdoor {
        /// `facing` state.
        pub facing : Facing,
        /// `half` state.
        pub half : Half,
        /// `open` state.
        pub open : bool,
        /// `powered` state.
        pub powered : bool,
        /// `waterlogged` state.
        pub waterlogged : bool,
    }
    /// `facing` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Facing {
        /// `north` variant.
        North,
        /// `south` variant.
        South,
        /// `west` variant.
        West,
        /// `east` variant.
        East,
    }
    /// `half` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Half {
        /// `top` variant.
        Top,
        /// `bottom` variant.
        Bottom,
    }
    impl super::Block for BirchTrapdoor {
    }
    impl From<BirchTrapdoor> for super::BlockState {
        fn from(value : BirchTrapdoor) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:wither_skeleton_wall_skull` block.
pub mod wither_skeleton_wall_skull {
    /// `minecraft:wither_skeleton_wall_skull` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct WitherSkeletonWallSkull {
        /// `facing` state.
        pub facing : Facing,
        /// `powered` state.
        pub powered : bool,
    }
    /// `facing` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Facing {
        /// `north` variant.
        North,
        /// `south` variant.
        South,
        /// `west` variant.
        West,
        /// `east` variant.
        East,
    }
    impl super::Block for WitherSkeletonWallSkull {
    }
    impl From<WitherSkeletonWallSkull> for super::BlockState {
        fn from(value : WitherSkeletonWallSkull) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:warped_roots` block.
pub mod warped_roots {
    /// `minecraft:warped_roots` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct WarpedRoots;
    impl super::Block for WarpedRoots {
    }
    impl From<WarpedRoots> for super::BlockState {
        fn from(value : WarpedRoots) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:dead_fire_coral_block` block.
pub mod dead_fire_coral_block {
    /// `minecraft:dead_fire_coral_block` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct DeadFireCoralBlock;
    impl super::Block for DeadFireCoralBlock {
    }
    impl From<DeadFireCoralBlock> for super::BlockState {
        fn from(value : DeadFireCoralBlock) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:black_candle` block.
pub mod black_candle {
    /// `minecraft:black_candle` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct BlackCandle {
        /// `candles` state.
        pub candles : Candles,
        /// `waterlogged` state.
        pub waterlogged : bool,
        /// `lit` state.
        pub lit : bool,
    }
    /// `candles` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Candles {
        /// `1` variant.
        N1,
        /// `2` variant.
        N2,
        /// `3` variant.
        N3,
        /// `4` variant.
        N4,
    }
    impl super::Block for BlackCandle {
    }
    impl From<BlackCandle> for super::BlockState {
        fn from(value : BlackCandle) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:potted_dark_oak_sapling` block.
pub mod potted_dark_oak_sapling {
    /// `minecraft:potted_dark_oak_sapling` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct PottedDarkOakSapling;
    impl super::Block for PottedDarkOakSapling {
    }
    impl From<PottedDarkOakSapling> for super::BlockState {
        fn from(value : PottedDarkOakSapling) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:bamboo_fence_gate` block.
pub mod bamboo_fence_gate {
    /// `minecraft:bamboo_fence_gate` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct BambooFenceGate {
        /// `in_wall` state.
        pub in_wall : bool,
        /// `open` state.
        pub open : bool,
        /// `facing` state.
        pub facing : Facing,
        /// `powered` state.
        pub powered : bool,
    }
    /// `facing` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Facing {
        /// `north` variant.
        North,
        /// `south` variant.
        South,
        /// `west` variant.
        West,
        /// `east` variant.
        East,
    }
    impl super::Block for BambooFenceGate {
    }
    impl From<BambooFenceGate> for super::BlockState {
        fn from(value : BambooFenceGate) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:blast_furnace` block.
pub mod blast_furnace {
    /// `minecraft:blast_furnace` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct BlastFurnace {
        /// `facing` state.
        pub facing : Facing,
        /// `lit` state.
        pub lit : bool,
    }
    /// `facing` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Facing {
        /// `north` variant.
        North,
        /// `south` variant.
        South,
        /// `west` variant.
        West,
        /// `east` variant.
        East,
    }
    impl super::Block for BlastFurnace {
    }
    impl From<BlastFurnace> for super::BlockState {
        fn from(value : BlastFurnace) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:mud_brick_slab` block.
pub mod mud_brick_slab {
    /// `minecraft:mud_brick_slab` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct MudBrickSlab {
        /// `type` state.
        pub kind : Kind,
        /// `waterlogged` state.
        pub waterlogged : bool,
    }
    /// `type` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Kind {
        /// `top` variant.
        Top,
        /// `bottom` variant.
        Bottom,
        /// `double` variant.
        Double,
    }
    impl super::Block for MudBrickSlab {
    }
    impl From<MudBrickSlab> for super::BlockState {
        fn from(value : MudBrickSlab) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:waxed_oxidized_cut_copper_slab` block.
pub mod waxed_oxidized_cut_copper_slab {
    /// `minecraft:waxed_oxidized_cut_copper_slab` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct WaxedOxidizedCutCopperSlab {
        /// `type` state.
        pub kind : Kind,
        /// `waterlogged` state.
        pub waterlogged : bool,
    }
    /// `type` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Kind {
        /// `top` variant.
        Top,
        /// `bottom` variant.
        Bottom,
        /// `double` variant.
        Double,
    }
    impl super::Block for WaxedOxidizedCutCopperSlab {
    }
    impl From<WaxedOxidizedCutCopperSlab> for super::BlockState {
        fn from(value : WaxedOxidizedCutCopperSlab) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:oxidized_cut_copper_stairs` block.
pub mod oxidized_cut_copper_stairs {
    /// `minecraft:oxidized_cut_copper_stairs` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct OxidizedCutCopperStairs {
        /// `half` state.
        pub half : Half,
        /// `facing` state.
        pub facing : Facing,
        /// `shape` state.
        pub shape : Shape,
        /// `waterlogged` state.
        pub waterlogged : bool,
    }
    /// `half` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Half {
        /// `top` variant.
        Top,
        /// `bottom` variant.
        Bottom,
    }
    /// `facing` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Facing {
        /// `north` variant.
        North,
        /// `south` variant.
        South,
        /// `west` variant.
        West,
        /// `east` variant.
        East,
    }
    /// `shape` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Shape {
        /// `straight` variant.
        Straight,
        /// `inner_left` variant.
        InnerLeft,
        /// `inner_right` variant.
        InnerRight,
        /// `outer_left` variant.
        OuterLeft,
        /// `outer_right` variant.
        OuterRight,
    }
    impl super::Block for OxidizedCutCopperStairs {
    }
    impl From<OxidizedCutCopperStairs> for super::BlockState {
        fn from(value : OxidizedCutCopperStairs) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:green_glazed_terracotta` block.
pub mod green_glazed_terracotta {
    /// `minecraft:green_glazed_terracotta` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct GreenGlazedTerracotta {
        /// `facing` state.
        pub facing : Facing,
    }
    /// `facing` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Facing {
        /// `north` variant.
        North,
        /// `south` variant.
        South,
        /// `west` variant.
        West,
        /// `east` variant.
        East,
    }
    impl super::Block for GreenGlazedTerracotta {
    }
    impl From<GreenGlazedTerracotta> for super::BlockState {
        fn from(value : GreenGlazedTerracotta) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:polished_blackstone_brick_stairs` block.
pub mod polished_blackstone_brick_stairs {
    /// `minecraft:polished_blackstone_brick_stairs` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct PolishedBlackstoneBrickStairs {
        /// `waterlogged` state.
        pub waterlogged : bool,
        /// `half` state.
        pub half : Half,
        /// `facing` state.
        pub facing : Facing,
        /// `shape` state.
        pub shape : Shape,
    }
    /// `half` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Half {
        /// `top` variant.
        Top,
        /// `bottom` variant.
        Bottom,
    }
    /// `facing` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Facing {
        /// `north` variant.
        North,
        /// `south` variant.
        South,
        /// `west` variant.
        West,
        /// `east` variant.
        East,
    }
    /// `shape` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Shape {
        /// `straight` variant.
        Straight,
        /// `inner_left` variant.
        InnerLeft,
        /// `inner_right` variant.
        InnerRight,
        /// `outer_left` variant.
        OuterLeft,
        /// `outer_right` variant.
        OuterRight,
    }
    impl super::Block for PolishedBlackstoneBrickStairs {
    }
    impl From<PolishedBlackstoneBrickStairs> for super::BlockState {
        fn from(value : PolishedBlackstoneBrickStairs) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:tnt` block.
pub mod tnt {
    /// `minecraft:tnt` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct Tnt {
        /// `unstable` state.
        pub unstable : bool,
    }
    impl super::Block for Tnt {
    }
    impl From<Tnt> for super::BlockState {
        fn from(value : Tnt) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:purple_bed` block.
pub mod purple_bed {
    /// `minecraft:purple_bed` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct PurpleBed {
        /// `occupied` state.
        pub occupied : bool,
        /// `facing` state.
        pub facing : Facing,
        /// `part` state.
        pub part : Part,
    }
    /// `facing` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Facing {
        /// `north` variant.
        North,
        /// `south` variant.
        South,
        /// `west` variant.
        West,
        /// `east` variant.
        East,
    }
    /// `part` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Part {
        /// `head` variant.
        Head,
        /// `foot` variant.
        Foot,
    }
    impl super::Block for PurpleBed {
    }
    impl From<PurpleBed> for super::BlockState {
        fn from(value : PurpleBed) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:cactus` block.
pub mod cactus {
    /// `minecraft:cactus` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct Cactus {
        /// `age` state.
        pub age : Age,
    }
    /// `age` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Age {
        /// `0` variant.
        N0,
        /// `1` variant.
        N1,
        /// `2` variant.
        N2,
        /// `3` variant.
        N3,
        /// `4` variant.
        N4,
        /// `5` variant.
        N5,
        /// `6` variant.
        N6,
        /// `7` variant.
        N7,
        /// `8` variant.
        N8,
        /// `9` variant.
        N9,
        /// `10` variant.
        N10,
        /// `11` variant.
        N11,
        /// `12` variant.
        N12,
        /// `13` variant.
        N13,
        /// `14` variant.
        N14,
        /// `15` variant.
        N15,
    }
    impl super::Block for Cactus {
    }
    impl From<Cactus> for super::BlockState {
        fn from(value : Cactus) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:mangrove_slab` block.
pub mod mangrove_slab {
    /// `minecraft:mangrove_slab` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct MangroveSlab {
        /// `waterlogged` state.
        pub waterlogged : bool,
        /// `type` state.
        pub kind : Kind,
    }
    /// `type` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Kind {
        /// `top` variant.
        Top,
        /// `bottom` variant.
        Bottom,
        /// `double` variant.
        Double,
    }
    impl super::Block for MangroveSlab {
    }
    impl From<MangroveSlab> for super::BlockState {
        fn from(value : MangroveSlab) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:sniffer_egg` block.
pub mod sniffer_egg {
    /// `minecraft:sniffer_egg` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct SnifferEgg {
        /// `hatch` state.
        pub hatch : Hatch,
    }
    /// `hatch` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Hatch {
        /// `0` variant.
        N0,
        /// `1` variant.
        N1,
        /// `2` variant.
        N2,
    }
    impl super::Block for SnifferEgg {
    }
    impl From<SnifferEgg> for super::BlockState {
        fn from(value : SnifferEgg) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:pink_shulker_box` block.
pub mod pink_shulker_box {
    /// `minecraft:pink_shulker_box` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct PinkShulkerBox {
        /// `facing` state.
        pub facing : Facing,
    }
    /// `facing` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Facing {
        /// `north` variant.
        North,
        /// `east` variant.
        East,
        /// `south` variant.
        South,
        /// `west` variant.
        West,
        /// `up` variant.
        Up,
        /// `down` variant.
        Down,
    }
    impl super::Block for PinkShulkerBox {
    }
    impl From<PinkShulkerBox> for super::BlockState {
        fn from(value : PinkShulkerBox) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:cherry_wall_sign` block.
pub mod cherry_wall_sign {
    /// `minecraft:cherry_wall_sign` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct CherryWallSign {
        /// `facing` state.
        pub facing : Facing,
        /// `waterlogged` state.
        pub waterlogged : bool,
    }
    /// `facing` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Facing {
        /// `north` variant.
        North,
        /// `south` variant.
        South,
        /// `west` variant.
        West,
        /// `east` variant.
        East,
    }
    impl super::Block for CherryWallSign {
    }
    impl From<CherryWallSign> for super::BlockState {
        fn from(value : CherryWallSign) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:potted_poppy` block.
pub mod potted_poppy {
    /// `minecraft:potted_poppy` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct PottedPoppy;
    impl super::Block for PottedPoppy {
    }
    impl From<PottedPoppy> for super::BlockState {
        fn from(value : PottedPoppy) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:yellow_shulker_box` block.
pub mod yellow_shulker_box {
    /// `minecraft:yellow_shulker_box` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct YellowShulkerBox {
        /// `facing` state.
        pub facing : Facing,
    }
    /// `facing` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Facing {
        /// `north` variant.
        North,
        /// `east` variant.
        East,
        /// `south` variant.
        South,
        /// `west` variant.
        West,
        /// `up` variant.
        Up,
        /// `down` variant.
        Down,
    }
    impl super::Block for YellowShulkerBox {
    }
    impl From<YellowShulkerBox> for super::BlockState {
        fn from(value : YellowShulkerBox) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:waxed_exposed_copper_trapdoor` block.
pub mod waxed_exposed_copper_trapdoor {
    /// `minecraft:waxed_exposed_copper_trapdoor` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct WaxedExposedCopperTrapdoor {
        /// `waterlogged` state.
        pub waterlogged : bool,
        /// `open` state.
        pub open : bool,
        /// `facing` state.
        pub facing : Facing,
        /// `half` state.
        pub half : Half,
        /// `powered` state.
        pub powered : bool,
    }
    /// `facing` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Facing {
        /// `north` variant.
        North,
        /// `south` variant.
        South,
        /// `west` variant.
        West,
        /// `east` variant.
        East,
    }
    /// `half` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Half {
        /// `top` variant.
        Top,
        /// `bottom` variant.
        Bottom,
    }
    impl super::Block for WaxedExposedCopperTrapdoor {
    }
    impl From<WaxedExposedCopperTrapdoor> for super::BlockState {
        fn from(value : WaxedExposedCopperTrapdoor) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:ender_chest` block.
pub mod ender_chest {
    /// `minecraft:ender_chest` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct EnderChest {
        /// `waterlogged` state.
        pub waterlogged : bool,
        /// `facing` state.
        pub facing : Facing,
    }
    /// `facing` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Facing {
        /// `north` variant.
        North,
        /// `south` variant.
        South,
        /// `west` variant.
        West,
        /// `east` variant.
        East,
    }
    impl super::Block for EnderChest {
    }
    impl From<EnderChest> for super::BlockState {
        fn from(value : EnderChest) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:polished_diorite` block.
pub mod polished_diorite {
    /// `minecraft:polished_diorite` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct PolishedDiorite;
    impl super::Block for PolishedDiorite {
    }
    impl From<PolishedDiorite> for super::BlockState {
        fn from(value : PolishedDiorite) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:brown_candle_cake` block.
pub mod brown_candle_cake {
    /// `minecraft:brown_candle_cake` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct BrownCandleCake {
        /// `lit` state.
        pub lit : bool,
    }
    impl super::Block for BrownCandleCake {
    }
    impl From<BrownCandleCake> for super::BlockState {
        fn from(value : BrownCandleCake) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:infested_chiseled_stone_bricks` block.
pub mod infested_chiseled_stone_bricks {
    /// `minecraft:infested_chiseled_stone_bricks` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct InfestedChiseledStoneBricks;
    impl super::Block for InfestedChiseledStoneBricks {
    }
    impl From<InfestedChiseledStoneBricks> for super::BlockState {
        fn from(value : InfestedChiseledStoneBricks) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:magenta_bed` block.
pub mod magenta_bed {
    /// `minecraft:magenta_bed` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct MagentaBed {
        /// `facing` state.
        pub facing : Facing,
        /// `part` state.
        pub part : Part,
        /// `occupied` state.
        pub occupied : bool,
    }
    /// `facing` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Facing {
        /// `north` variant.
        North,
        /// `south` variant.
        South,
        /// `west` variant.
        West,
        /// `east` variant.
        East,
    }
    /// `part` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Part {
        /// `head` variant.
        Head,
        /// `foot` variant.
        Foot,
    }
    impl super::Block for MagentaBed {
    }
    impl From<MagentaBed> for super::BlockState {
        fn from(value : MagentaBed) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:polished_tuff_stairs` block.
pub mod polished_tuff_stairs {
    /// `minecraft:polished_tuff_stairs` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct PolishedTuffStairs {
        /// `waterlogged` state.
        pub waterlogged : bool,
        /// `shape` state.
        pub shape : Shape,
        /// `facing` state.
        pub facing : Facing,
        /// `half` state.
        pub half : Half,
    }
    /// `shape` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Shape {
        /// `straight` variant.
        Straight,
        /// `inner_left` variant.
        InnerLeft,
        /// `inner_right` variant.
        InnerRight,
        /// `outer_left` variant.
        OuterLeft,
        /// `outer_right` variant.
        OuterRight,
    }
    /// `facing` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Facing {
        /// `north` variant.
        North,
        /// `south` variant.
        South,
        /// `west` variant.
        West,
        /// `east` variant.
        East,
    }
    /// `half` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Half {
        /// `top` variant.
        Top,
        /// `bottom` variant.
        Bottom,
    }
    impl super::Block for PolishedTuffStairs {
    }
    impl From<PolishedTuffStairs> for super::BlockState {
        fn from(value : PolishedTuffStairs) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:cherry_door` block.
pub mod cherry_door {
    /// `minecraft:cherry_door` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct CherryDoor {
        /// `powered` state.
        pub powered : bool,
        /// `half` state.
        pub half : Half,
        /// `hinge` state.
        pub hinge : Hinge,
        /// `facing` state.
        pub facing : Facing,
        /// `open` state.
        pub open : bool,
    }
    /// `half` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Half {
        /// `upper` variant.
        Upper,
        /// `lower` variant.
        Lower,
    }
    /// `hinge` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Hinge {
        /// `left` variant.
        Left,
        /// `right` variant.
        Right,
    }
    /// `facing` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Facing {
        /// `north` variant.
        North,
        /// `south` variant.
        South,
        /// `west` variant.
        West,
        /// `east` variant.
        East,
    }
    impl super::Block for CherryDoor {
    }
    impl From<CherryDoor> for super::BlockState {
        fn from(value : CherryDoor) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:dandelion` block.
pub mod dandelion {
    /// `minecraft:dandelion` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct Dandelion;
    impl super::Block for Dandelion {
    }
    impl From<Dandelion> for super::BlockState {
        fn from(value : Dandelion) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:gray_glazed_terracotta` block.
pub mod gray_glazed_terracotta {
    /// `minecraft:gray_glazed_terracotta` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct GrayGlazedTerracotta {
        /// `facing` state.
        pub facing : Facing,
    }
    /// `facing` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Facing {
        /// `north` variant.
        North,
        /// `south` variant.
        South,
        /// `west` variant.
        West,
        /// `east` variant.
        East,
    }
    impl super::Block for GrayGlazedTerracotta {
    }
    impl From<GrayGlazedTerracotta> for super::BlockState {
        fn from(value : GrayGlazedTerracotta) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:light_blue_candle_cake` block.
pub mod light_blue_candle_cake {
    /// `minecraft:light_blue_candle_cake` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct LightBlueCandleCake {
        /// `lit` state.
        pub lit : bool,
    }
    impl super::Block for LightBlueCandleCake {
    }
    impl From<LightBlueCandleCake> for super::BlockState {
        fn from(value : LightBlueCandleCake) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:potted_cactus` block.
pub mod potted_cactus {
    /// `minecraft:potted_cactus` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct PottedCactus;
    impl super::Block for PottedCactus {
    }
    impl From<PottedCactus> for super::BlockState {
        fn from(value : PottedCactus) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:pale_oak_slab` block.
pub mod pale_oak_slab {
    /// `minecraft:pale_oak_slab` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct PaleOakSlab {
        /// `waterlogged` state.
        pub waterlogged : bool,
        /// `type` state.
        pub kind : Kind,
    }
    /// `type` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Kind {
        /// `top` variant.
        Top,
        /// `bottom` variant.
        Bottom,
        /// `double` variant.
        Double,
    }
    impl super::Block for PaleOakSlab {
    }
    impl From<PaleOakSlab> for super::BlockState {
        fn from(value : PaleOakSlab) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:dead_bubble_coral_wall_fan` block.
pub mod dead_bubble_coral_wall_fan {
    /// `minecraft:dead_bubble_coral_wall_fan` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct DeadBubbleCoralWallFan {
        /// `facing` state.
        pub facing : Facing,
        /// `waterlogged` state.
        pub waterlogged : bool,
    }
    /// `facing` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Facing {
        /// `north` variant.
        North,
        /// `south` variant.
        South,
        /// `west` variant.
        West,
        /// `east` variant.
        East,
    }
    impl super::Block for DeadBubbleCoralWallFan {
    }
    impl From<DeadBubbleCoralWallFan> for super::BlockState {
        fn from(value : DeadBubbleCoralWallFan) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:short_grass` block.
pub mod short_grass {
    /// `minecraft:short_grass` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct ShortGrass;
    impl super::Block for ShortGrass {
    }
    impl From<ShortGrass> for super::BlockState {
        fn from(value : ShortGrass) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:heavy_core` block.
pub mod heavy_core {
    /// `minecraft:heavy_core` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct HeavyCore {
        /// `waterlogged` state.
        pub waterlogged : bool,
    }
    impl super::Block for HeavyCore {
    }
    impl From<HeavyCore> for super::BlockState {
        fn from(value : HeavyCore) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:potted_spruce_sapling` block.
pub mod potted_spruce_sapling {
    /// `minecraft:potted_spruce_sapling` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct PottedSpruceSapling;
    impl super::Block for PottedSpruceSapling {
    }
    impl From<PottedSpruceSapling> for super::BlockState {
        fn from(value : PottedSpruceSapling) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:deepslate_tiles` block.
pub mod deepslate_tiles {
    /// `minecraft:deepslate_tiles` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct DeepslateTiles;
    impl super::Block for DeepslateTiles {
    }
    impl From<DeepslateTiles> for super::BlockState {
        fn from(value : DeepslateTiles) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:chiseled_bookshelf` block.
pub mod chiseled_bookshelf {
    /// `minecraft:chiseled_bookshelf` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct ChiseledBookshelf {
        /// `facing` state.
        pub facing : Facing,
        /// `slot_2_occupied` state.
        pub slot_2_occupied : bool,
        /// `slot_1_occupied` state.
        pub slot_1_occupied : bool,
        /// `slot_0_occupied` state.
        pub slot_0_occupied : bool,
        /// `slot_4_occupied` state.
        pub slot_4_occupied : bool,
        /// `slot_5_occupied` state.
        pub slot_5_occupied : bool,
        /// `slot_3_occupied` state.
        pub slot_3_occupied : bool,
    }
    /// `facing` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Facing {
        /// `north` variant.
        North,
        /// `south` variant.
        South,
        /// `west` variant.
        West,
        /// `east` variant.
        East,
    }
    impl super::Block for ChiseledBookshelf {
    }
    impl From<ChiseledBookshelf> for super::BlockState {
        fn from(value : ChiseledBookshelf) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:pink_banner` block.
pub mod pink_banner {
    /// `minecraft:pink_banner` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct PinkBanner {
        /// `rotation` state.
        pub rotation : Rotation,
    }
    /// `rotation` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Rotation {
        /// `0` variant.
        N0,
        /// `1` variant.
        N1,
        /// `2` variant.
        N2,
        /// `3` variant.
        N3,
        /// `4` variant.
        N4,
        /// `5` variant.
        N5,
        /// `6` variant.
        N6,
        /// `7` variant.
        N7,
        /// `8` variant.
        N8,
        /// `9` variant.
        N9,
        /// `10` variant.
        N10,
        /// `11` variant.
        N11,
        /// `12` variant.
        N12,
        /// `13` variant.
        N13,
        /// `14` variant.
        N14,
        /// `15` variant.
        N15,
    }
    impl super::Block for PinkBanner {
    }
    impl From<PinkBanner> for super::BlockState {
        fn from(value : PinkBanner) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:pink_concrete_powder` block.
pub mod pink_concrete_powder {
    /// `minecraft:pink_concrete_powder` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct PinkConcretePowder;
    impl super::Block for PinkConcretePowder {
    }
    impl From<PinkConcretePowder> for super::BlockState {
        fn from(value : PinkConcretePowder) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:cauldron` block.
pub mod cauldron {
    /// `minecraft:cauldron` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct Cauldron;
    impl super::Block for Cauldron {
    }
    impl From<Cauldron> for super::BlockState {
        fn from(value : Cauldron) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:spruce_planks` block.
pub mod spruce_planks {
    /// `minecraft:spruce_planks` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct SprucePlanks;
    impl super::Block for SprucePlanks {
    }
    impl From<SprucePlanks> for super::BlockState {
        fn from(value : SprucePlanks) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:polished_tuff_wall` block.
pub mod polished_tuff_wall {
    /// `minecraft:polished_tuff_wall` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct PolishedTuffWall {
        /// `waterlogged` state.
        pub waterlogged : bool,
        /// `west` state.
        pub west : West,
        /// `north` state.
        pub north : North,
        /// `east` state.
        pub east : East,
        /// `south` state.
        pub south : South,
        /// `up` state.
        pub up : bool,
    }
    /// `west` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum West {
        /// `none` variant.
        None,
        /// `low` variant.
        Low,
        /// `tall` variant.
        Tall,
    }
    /// `north` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum North {
        /// `none` variant.
        None,
        /// `low` variant.
        Low,
        /// `tall` variant.
        Tall,
    }
    /// `east` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum East {
        /// `none` variant.
        None,
        /// `low` variant.
        Low,
        /// `tall` variant.
        Tall,
    }
    /// `south` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum South {
        /// `none` variant.
        None,
        /// `low` variant.
        Low,
        /// `tall` variant.
        Tall,
    }
    impl super::Block for PolishedTuffWall {
    }
    impl From<PolishedTuffWall> for super::BlockState {
        fn from(value : PolishedTuffWall) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:birch_button` block.
pub mod birch_button {
    /// `minecraft:birch_button` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct BirchButton {
        /// `powered` state.
        pub powered : bool,
        /// `face` state.
        pub face : Face,
        /// `facing` state.
        pub facing : Facing,
    }
    /// `face` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Face {
        /// `floor` variant.
        Floor,
        /// `wall` variant.
        Wall,
        /// `ceiling` variant.
        Ceiling,
    }
    /// `facing` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Facing {
        /// `north` variant.
        North,
        /// `south` variant.
        South,
        /// `west` variant.
        West,
        /// `east` variant.
        East,
    }
    impl super::Block for BirchButton {
    }
    impl From<BirchButton> for super::BlockState {
        fn from(value : BirchButton) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:stripped_acacia_wood` block.
pub mod stripped_acacia_wood {
    /// `minecraft:stripped_acacia_wood` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct StrippedAcaciaWood {
        /// `axis` state.
        pub axis : Axis,
    }
    /// `axis` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Axis {
        /// `x` variant.
        X,
        /// `y` variant.
        Y,
        /// `z` variant.
        Z,
    }
    impl super::Block for StrippedAcaciaWood {
    }
    impl From<StrippedAcaciaWood> for super::BlockState {
        fn from(value : StrippedAcaciaWood) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:stripped_oak_log` block.
pub mod stripped_oak_log {
    /// `minecraft:stripped_oak_log` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct StrippedOakLog {
        /// `axis` state.
        pub axis : Axis,
    }
    /// `axis` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Axis {
        /// `x` variant.
        X,
        /// `y` variant.
        Y,
        /// `z` variant.
        Z,
    }
    impl super::Block for StrippedOakLog {
    }
    impl From<StrippedOakLog> for super::BlockState {
        fn from(value : StrippedOakLog) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:tuff_bricks` block.
pub mod tuff_bricks {
    /// `minecraft:tuff_bricks` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct TuffBricks;
    impl super::Block for TuffBricks {
    }
    impl From<TuffBricks> for super::BlockState {
        fn from(value : TuffBricks) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:tuff_stairs` block.
pub mod tuff_stairs {
    /// `minecraft:tuff_stairs` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct TuffStairs {
        /// `waterlogged` state.
        pub waterlogged : bool,
        /// `facing` state.
        pub facing : Facing,
        /// `half` state.
        pub half : Half,
        /// `shape` state.
        pub shape : Shape,
    }
    /// `facing` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Facing {
        /// `north` variant.
        North,
        /// `south` variant.
        South,
        /// `west` variant.
        West,
        /// `east` variant.
        East,
    }
    /// `half` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Half {
        /// `top` variant.
        Top,
        /// `bottom` variant.
        Bottom,
    }
    /// `shape` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Shape {
        /// `straight` variant.
        Straight,
        /// `inner_left` variant.
        InnerLeft,
        /// `inner_right` variant.
        InnerRight,
        /// `outer_left` variant.
        OuterLeft,
        /// `outer_right` variant.
        OuterRight,
    }
    impl super::Block for TuffStairs {
    }
    impl From<TuffStairs> for super::BlockState {
        fn from(value : TuffStairs) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:oak_trapdoor` block.
pub mod oak_trapdoor {
    /// `minecraft:oak_trapdoor` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct OakTrapdoor {
        /// `waterlogged` state.
        pub waterlogged : bool,
        /// `facing` state.
        pub facing : Facing,
        /// `half` state.
        pub half : Half,
        /// `open` state.
        pub open : bool,
        /// `powered` state.
        pub powered : bool,
    }
    /// `facing` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Facing {
        /// `north` variant.
        North,
        /// `south` variant.
        South,
        /// `west` variant.
        West,
        /// `east` variant.
        East,
    }
    /// `half` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Half {
        /// `top` variant.
        Top,
        /// `bottom` variant.
        Bottom,
    }
    impl super::Block for OakTrapdoor {
    }
    impl From<OakTrapdoor> for super::BlockState {
        fn from(value : OakTrapdoor) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:brown_bed` block.
pub mod brown_bed {
    /// `minecraft:brown_bed` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct BrownBed {
        /// `occupied` state.
        pub occupied : bool,
        /// `facing` state.
        pub facing : Facing,
        /// `part` state.
        pub part : Part,
    }
    /// `facing` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Facing {
        /// `north` variant.
        North,
        /// `south` variant.
        South,
        /// `west` variant.
        West,
        /// `east` variant.
        East,
    }
    /// `part` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Part {
        /// `head` variant.
        Head,
        /// `foot` variant.
        Foot,
    }
    impl super::Block for BrownBed {
    }
    impl From<BrownBed> for super::BlockState {
        fn from(value : BrownBed) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:polished_basalt` block.
pub mod polished_basalt {
    /// `minecraft:polished_basalt` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct PolishedBasalt {
        /// `axis` state.
        pub axis : Axis,
    }
    /// `axis` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Axis {
        /// `x` variant.
        X,
        /// `y` variant.
        Y,
        /// `z` variant.
        Z,
    }
    impl super::Block for PolishedBasalt {
    }
    impl From<PolishedBasalt> for super::BlockState {
        fn from(value : PolishedBasalt) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:leaf_litter` block.
pub mod leaf_litter {
    /// `minecraft:leaf_litter` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct LeafLitter {
        /// `segment_amount` state.
        pub segment_amount : SegmentAmount,
        /// `facing` state.
        pub facing : Facing,
    }
    /// `segment_amount` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum SegmentAmount {
        /// `1` variant.
        N1,
        /// `2` variant.
        N2,
        /// `3` variant.
        N3,
        /// `4` variant.
        N4,
    }
    /// `facing` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Facing {
        /// `north` variant.
        North,
        /// `south` variant.
        South,
        /// `west` variant.
        West,
        /// `east` variant.
        East,
    }
    impl super::Block for LeafLitter {
    }
    impl From<LeafLitter> for super::BlockState {
        fn from(value : LeafLitter) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:waxed_cut_copper` block.
pub mod waxed_cut_copper {
    /// `minecraft:waxed_cut_copper` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct WaxedCutCopper;
    impl super::Block for WaxedCutCopper {
    }
    impl From<WaxedCutCopper> for super::BlockState {
        fn from(value : WaxedCutCopper) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:waxed_weathered_cut_copper_slab` block.
pub mod waxed_weathered_cut_copper_slab {
    /// `minecraft:waxed_weathered_cut_copper_slab` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct WaxedWeatheredCutCopperSlab {
        /// `type` state.
        pub kind : Kind,
        /// `waterlogged` state.
        pub waterlogged : bool,
    }
    /// `type` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Kind {
        /// `top` variant.
        Top,
        /// `bottom` variant.
        Bottom,
        /// `double` variant.
        Double,
    }
    impl super::Block for WaxedWeatheredCutCopperSlab {
    }
    impl From<WaxedWeatheredCutCopperSlab> for super::BlockState {
        fn from(value : WaxedWeatheredCutCopperSlab) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:white_concrete` block.
pub mod white_concrete {
    /// `minecraft:white_concrete` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct WhiteConcrete;
    impl super::Block for WhiteConcrete {
    }
    impl From<WhiteConcrete> for super::BlockState {
        fn from(value : WhiteConcrete) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:cherry_slab` block.
pub mod cherry_slab {
    /// `minecraft:cherry_slab` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct CherrySlab {
        /// `waterlogged` state.
        pub waterlogged : bool,
        /// `type` state.
        pub kind : Kind,
    }
    /// `type` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Kind {
        /// `top` variant.
        Top,
        /// `bottom` variant.
        Bottom,
        /// `double` variant.
        Double,
    }
    impl super::Block for CherrySlab {
    }
    impl From<CherrySlab> for super::BlockState {
        fn from(value : CherrySlab) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:oxidized_copper_trapdoor` block.
pub mod oxidized_copper_trapdoor {
    /// `minecraft:oxidized_copper_trapdoor` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct OxidizedCopperTrapdoor {
        /// `open` state.
        pub open : bool,
        /// `powered` state.
        pub powered : bool,
        /// `waterlogged` state.
        pub waterlogged : bool,
        /// `half` state.
        pub half : Half,
        /// `facing` state.
        pub facing : Facing,
    }
    /// `half` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Half {
        /// `top` variant.
        Top,
        /// `bottom` variant.
        Bottom,
    }
    /// `facing` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Facing {
        /// `north` variant.
        North,
        /// `south` variant.
        South,
        /// `west` variant.
        West,
        /// `east` variant.
        East,
    }
    impl super::Block for OxidizedCopperTrapdoor {
    }
    impl From<OxidizedCopperTrapdoor> for super::BlockState {
        fn from(value : OxidizedCopperTrapdoor) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:smooth_red_sandstone_stairs` block.
pub mod smooth_red_sandstone_stairs {
    /// `minecraft:smooth_red_sandstone_stairs` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct SmoothRedSandstoneStairs {
        /// `shape` state.
        pub shape : Shape,
        /// `half` state.
        pub half : Half,
        /// `facing` state.
        pub facing : Facing,
        /// `waterlogged` state.
        pub waterlogged : bool,
    }
    /// `shape` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Shape {
        /// `straight` variant.
        Straight,
        /// `inner_left` variant.
        InnerLeft,
        /// `inner_right` variant.
        InnerRight,
        /// `outer_left` variant.
        OuterLeft,
        /// `outer_right` variant.
        OuterRight,
    }
    /// `half` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Half {
        /// `top` variant.
        Top,
        /// `bottom` variant.
        Bottom,
    }
    /// `facing` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Facing {
        /// `north` variant.
        North,
        /// `south` variant.
        South,
        /// `west` variant.
        West,
        /// `east` variant.
        East,
    }
    impl super::Block for SmoothRedSandstoneStairs {
    }
    impl From<SmoothRedSandstoneStairs> for super::BlockState {
        fn from(value : SmoothRedSandstoneStairs) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:oak_button` block.
pub mod oak_button {
    /// `minecraft:oak_button` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct OakButton {
        /// `face` state.
        pub face : Face,
        /// `facing` state.
        pub facing : Facing,
        /// `powered` state.
        pub powered : bool,
    }
    /// `face` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Face {
        /// `floor` variant.
        Floor,
        /// `wall` variant.
        Wall,
        /// `ceiling` variant.
        Ceiling,
    }
    /// `facing` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Facing {
        /// `north` variant.
        North,
        /// `south` variant.
        South,
        /// `west` variant.
        West,
        /// `east` variant.
        East,
    }
    impl super::Block for OakButton {
    }
    impl From<OakButton> for super::BlockState {
        fn from(value : OakButton) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:light` block.
pub mod light {
    /// `minecraft:light` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct Light {
        /// `level` state.
        pub level : Level,
        /// `waterlogged` state.
        pub waterlogged : bool,
    }
    /// `level` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Level {
        /// `0` variant.
        N0,
        /// `1` variant.
        N1,
        /// `2` variant.
        N2,
        /// `3` variant.
        N3,
        /// `4` variant.
        N4,
        /// `5` variant.
        N5,
        /// `6` variant.
        N6,
        /// `7` variant.
        N7,
        /// `8` variant.
        N8,
        /// `9` variant.
        N9,
        /// `10` variant.
        N10,
        /// `11` variant.
        N11,
        /// `12` variant.
        N12,
        /// `13` variant.
        N13,
        /// `14` variant.
        N14,
        /// `15` variant.
        N15,
    }
    impl super::Block for Light {
    }
    impl From<Light> for super::BlockState {
        fn from(value : Light) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:spruce_door` block.
pub mod spruce_door {
    /// `minecraft:spruce_door` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct SpruceDoor {
        /// `half` state.
        pub half : Half,
        /// `hinge` state.
        pub hinge : Hinge,
        /// `open` state.
        pub open : bool,
        /// `facing` state.
        pub facing : Facing,
        /// `powered` state.
        pub powered : bool,
    }
    /// `half` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Half {
        /// `upper` variant.
        Upper,
        /// `lower` variant.
        Lower,
    }
    /// `hinge` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Hinge {
        /// `left` variant.
        Left,
        /// `right` variant.
        Right,
    }
    /// `facing` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Facing {
        /// `north` variant.
        North,
        /// `south` variant.
        South,
        /// `west` variant.
        West,
        /// `east` variant.
        East,
    }
    impl super::Block for SpruceDoor {
    }
    impl From<SpruceDoor> for super::BlockState {
        fn from(value : SpruceDoor) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:oak_leaves` block.
pub mod oak_leaves {
    /// `minecraft:oak_leaves` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct OakLeaves {
        /// `persistent` state.
        pub persistent : bool,
        /// `waterlogged` state.
        pub waterlogged : bool,
        /// `distance` state.
        pub distance : Distance,
    }
    /// `distance` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Distance {
        /// `1` variant.
        N1,
        /// `2` variant.
        N2,
        /// `3` variant.
        N3,
        /// `4` variant.
        N4,
        /// `5` variant.
        N5,
        /// `6` variant.
        N6,
        /// `7` variant.
        N7,
    }
    impl super::Block for OakLeaves {
    }
    impl From<OakLeaves> for super::BlockState {
        fn from(value : OakLeaves) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:cobblestone_wall` block.
pub mod cobblestone_wall {
    /// `minecraft:cobblestone_wall` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct CobblestoneWall {
        /// `south` state.
        pub south : South,
        /// `waterlogged` state.
        pub waterlogged : bool,
        /// `north` state.
        pub north : North,
        /// `east` state.
        pub east : East,
        /// `up` state.
        pub up : bool,
        /// `west` state.
        pub west : West,
    }
    /// `south` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum South {
        /// `none` variant.
        None,
        /// `low` variant.
        Low,
        /// `tall` variant.
        Tall,
    }
    /// `north` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum North {
        /// `none` variant.
        None,
        /// `low` variant.
        Low,
        /// `tall` variant.
        Tall,
    }
    /// `east` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum East {
        /// `none` variant.
        None,
        /// `low` variant.
        Low,
        /// `tall` variant.
        Tall,
    }
    /// `west` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum West {
        /// `none` variant.
        None,
        /// `low` variant.
        Low,
        /// `tall` variant.
        Tall,
    }
    impl super::Block for CobblestoneWall {
    }
    impl From<CobblestoneWall> for super::BlockState {
        fn from(value : CobblestoneWall) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:sticky_piston` block.
pub mod sticky_piston {
    /// `minecraft:sticky_piston` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct StickyPiston {
        /// `extended` state.
        pub extended : bool,
        /// `facing` state.
        pub facing : Facing,
    }
    /// `facing` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Facing {
        /// `north` variant.
        North,
        /// `east` variant.
        East,
        /// `south` variant.
        South,
        /// `west` variant.
        West,
        /// `up` variant.
        Up,
        /// `down` variant.
        Down,
    }
    impl super::Block for StickyPiston {
    }
    impl From<StickyPiston> for super::BlockState {
        fn from(value : StickyPiston) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:smooth_quartz` block.
pub mod smooth_quartz {
    /// `minecraft:smooth_quartz` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct SmoothQuartz;
    impl super::Block for SmoothQuartz {
    }
    impl From<SmoothQuartz> for super::BlockState {
        fn from(value : SmoothQuartz) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:lime_wool` block.
pub mod lime_wool {
    /// `minecraft:lime_wool` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct LimeWool;
    impl super::Block for LimeWool {
    }
    impl From<LimeWool> for super::BlockState {
        fn from(value : LimeWool) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:purple_wall_banner` block.
pub mod purple_wall_banner {
    /// `minecraft:purple_wall_banner` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct PurpleWallBanner {
        /// `facing` state.
        pub facing : Facing,
    }
    /// `facing` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Facing {
        /// `north` variant.
        North,
        /// `south` variant.
        South,
        /// `west` variant.
        West,
        /// `east` variant.
        East,
    }
    impl super::Block for PurpleWallBanner {
    }
    impl From<PurpleWallBanner> for super::BlockState {
        fn from(value : PurpleWallBanner) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:respawn_anchor` block.
pub mod respawn_anchor {
    /// `minecraft:respawn_anchor` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct RespawnAnchor {
        /// `charges` state.
        pub charges : Charges,
    }
    /// `charges` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Charges {
        /// `0` variant.
        N0,
        /// `1` variant.
        N1,
        /// `2` variant.
        N2,
        /// `3` variant.
        N3,
        /// `4` variant.
        N4,
    }
    impl super::Block for RespawnAnchor {
    }
    impl From<RespawnAnchor> for super::BlockState {
        fn from(value : RespawnAnchor) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:carrots` block.
pub mod carrots {
    /// `minecraft:carrots` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct Carrots {
        /// `age` state.
        pub age : Age,
    }
    /// `age` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Age {
        /// `0` variant.
        N0,
        /// `1` variant.
        N1,
        /// `2` variant.
        N2,
        /// `3` variant.
        N3,
        /// `4` variant.
        N4,
        /// `5` variant.
        N5,
        /// `6` variant.
        N6,
        /// `7` variant.
        N7,
    }
    impl super::Block for Carrots {
    }
    impl From<Carrots> for super::BlockState {
        fn from(value : Carrots) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:chest` block.
pub mod chest {
    /// `minecraft:chest` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct Chest {
        /// `facing` state.
        pub facing : Facing,
        /// `waterlogged` state.
        pub waterlogged : bool,
        /// `type` state.
        pub kind : Kind,
    }
    /// `facing` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Facing {
        /// `north` variant.
        North,
        /// `south` variant.
        South,
        /// `west` variant.
        West,
        /// `east` variant.
        East,
    }
    /// `type` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Kind {
        /// `single` variant.
        Single,
        /// `left` variant.
        Left,
        /// `right` variant.
        Right,
    }
    impl super::Block for Chest {
    }
    impl From<Chest> for super::BlockState {
        fn from(value : Chest) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:bone_block` block.
pub mod bone_block {
    /// `minecraft:bone_block` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct BoneBlock {
        /// `axis` state.
        pub axis : Axis,
    }
    /// `axis` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Axis {
        /// `x` variant.
        X,
        /// `y` variant.
        Y,
        /// `z` variant.
        Z,
    }
    impl super::Block for BoneBlock {
    }
    impl From<BoneBlock> for super::BlockState {
        fn from(value : BoneBlock) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:podzol` block.
pub mod podzol {
    /// `minecraft:podzol` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct Podzol {
        /// `snowy` state.
        pub snowy : bool,
    }
    impl super::Block for Podzol {
    }
    impl From<Podzol> for super::BlockState {
        fn from(value : Podzol) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:nether_quartz_ore` block.
pub mod nether_quartz_ore {
    /// `minecraft:nether_quartz_ore` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct NetherQuartzOre;
    impl super::Block for NetherQuartzOre {
    }
    impl From<NetherQuartzOre> for super::BlockState {
        fn from(value : NetherQuartzOre) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:smooth_sandstone_stairs` block.
pub mod smooth_sandstone_stairs {
    /// `minecraft:smooth_sandstone_stairs` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct SmoothSandstoneStairs {
        /// `shape` state.
        pub shape : Shape,
        /// `facing` state.
        pub facing : Facing,
        /// `half` state.
        pub half : Half,
        /// `waterlogged` state.
        pub waterlogged : bool,
    }
    /// `shape` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Shape {
        /// `straight` variant.
        Straight,
        /// `inner_left` variant.
        InnerLeft,
        /// `inner_right` variant.
        InnerRight,
        /// `outer_left` variant.
        OuterLeft,
        /// `outer_right` variant.
        OuterRight,
    }
    /// `facing` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Facing {
        /// `north` variant.
        North,
        /// `south` variant.
        South,
        /// `west` variant.
        West,
        /// `east` variant.
        East,
    }
    /// `half` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Half {
        /// `top` variant.
        Top,
        /// `bottom` variant.
        Bottom,
    }
    impl super::Block for SmoothSandstoneStairs {
    }
    impl From<SmoothSandstoneStairs> for super::BlockState {
        fn from(value : SmoothSandstoneStairs) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:red_stained_glass` block.
pub mod red_stained_glass {
    /// `minecraft:red_stained_glass` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct RedStainedGlass;
    impl super::Block for RedStainedGlass {
    }
    impl From<RedStainedGlass> for super::BlockState {
        fn from(value : RedStainedGlass) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:calibrated_sculk_sensor` block.
pub mod calibrated_sculk_sensor {
    /// `minecraft:calibrated_sculk_sensor` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct CalibratedSculkSensor {
        /// `facing` state.
        pub facing : Facing,
        /// `power` state.
        pub power : Power,
        /// `sculk_sensor_phase` state.
        pub sculk_sensor_phase : SculkSensorPhase,
        /// `waterlogged` state.
        pub waterlogged : bool,
    }
    /// `facing` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Facing {
        /// `north` variant.
        North,
        /// `south` variant.
        South,
        /// `west` variant.
        West,
        /// `east` variant.
        East,
    }
    /// `power` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Power {
        /// `0` variant.
        N0,
        /// `1` variant.
        N1,
        /// `2` variant.
        N2,
        /// `3` variant.
        N3,
        /// `4` variant.
        N4,
        /// `5` variant.
        N5,
        /// `6` variant.
        N6,
        /// `7` variant.
        N7,
        /// `8` variant.
        N8,
        /// `9` variant.
        N9,
        /// `10` variant.
        N10,
        /// `11` variant.
        N11,
        /// `12` variant.
        N12,
        /// `13` variant.
        N13,
        /// `14` variant.
        N14,
        /// `15` variant.
        N15,
    }
    /// `sculk_sensor_phase` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum SculkSensorPhase {
        /// `inactive` variant.
        Inactive,
        /// `active` variant.
        Active,
        /// `cooldown` variant.
        Cooldown,
    }
    impl super::Block for CalibratedSculkSensor {
    }
    impl From<CalibratedSculkSensor> for super::BlockState {
        fn from(value : CalibratedSculkSensor) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:cactus_flower` block.
pub mod cactus_flower {
    /// `minecraft:cactus_flower` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct CactusFlower;
    impl super::Block for CactusFlower {
    }
    impl From<CactusFlower> for super::BlockState {
        fn from(value : CactusFlower) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:dragon_egg` block.
pub mod dragon_egg {
    /// `minecraft:dragon_egg` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct DragonEgg;
    impl super::Block for DragonEgg {
    }
    impl From<DragonEgg> for super::BlockState {
        fn from(value : DragonEgg) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:light_weighted_pressure_plate` block.
pub mod light_weighted_pressure_plate {
    /// `minecraft:light_weighted_pressure_plate` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct LightWeightedPressurePlate {
        /// `power` state.
        pub power : Power,
    }
    /// `power` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Power {
        /// `0` variant.
        N0,
        /// `1` variant.
        N1,
        /// `2` variant.
        N2,
        /// `3` variant.
        N3,
        /// `4` variant.
        N4,
        /// `5` variant.
        N5,
        /// `6` variant.
        N6,
        /// `7` variant.
        N7,
        /// `8` variant.
        N8,
        /// `9` variant.
        N9,
        /// `10` variant.
        N10,
        /// `11` variant.
        N11,
        /// `12` variant.
        N12,
        /// `13` variant.
        N13,
        /// `14` variant.
        N14,
        /// `15` variant.
        N15,
    }
    impl super::Block for LightWeightedPressurePlate {
    }
    impl From<LightWeightedPressurePlate> for super::BlockState {
        fn from(value : LightWeightedPressurePlate) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:lime_terracotta` block.
pub mod lime_terracotta {
    /// `minecraft:lime_terracotta` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct LimeTerracotta;
    impl super::Block for LimeTerracotta {
    }
    impl From<LimeTerracotta> for super::BlockState {
        fn from(value : LimeTerracotta) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:spruce_pressure_plate` block.
pub mod spruce_pressure_plate {
    /// `minecraft:spruce_pressure_plate` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct SprucePressurePlate {
        /// `powered` state.
        pub powered : bool,
    }
    impl super::Block for SprucePressurePlate {
    }
    impl From<SprucePressurePlate> for super::BlockState {
        fn from(value : SprucePressurePlate) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:granite_stairs` block.
pub mod granite_stairs {
    /// `minecraft:granite_stairs` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct GraniteStairs {
        /// `facing` state.
        pub facing : Facing,
        /// `waterlogged` state.
        pub waterlogged : bool,
        /// `shape` state.
        pub shape : Shape,
        /// `half` state.
        pub half : Half,
    }
    /// `facing` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Facing {
        /// `north` variant.
        North,
        /// `south` variant.
        South,
        /// `west` variant.
        West,
        /// `east` variant.
        East,
    }
    /// `shape` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Shape {
        /// `straight` variant.
        Straight,
        /// `inner_left` variant.
        InnerLeft,
        /// `inner_right` variant.
        InnerRight,
        /// `outer_left` variant.
        OuterLeft,
        /// `outer_right` variant.
        OuterRight,
    }
    /// `half` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Half {
        /// `top` variant.
        Top,
        /// `bottom` variant.
        Bottom,
    }
    impl super::Block for GraniteStairs {
    }
    impl From<GraniteStairs> for super::BlockState {
        fn from(value : GraniteStairs) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:waxed_weathered_copper` block.
pub mod waxed_weathered_copper {
    /// `minecraft:waxed_weathered_copper` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct WaxedWeatheredCopper;
    impl super::Block for WaxedWeatheredCopper {
    }
    impl From<WaxedWeatheredCopper> for super::BlockState {
        fn from(value : WaxedWeatheredCopper) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:spruce_fence_gate` block.
pub mod spruce_fence_gate {
    /// `minecraft:spruce_fence_gate` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct SpruceFenceGate {
        /// `powered` state.
        pub powered : bool,
        /// `facing` state.
        pub facing : Facing,
        /// `in_wall` state.
        pub in_wall : bool,
        /// `open` state.
        pub open : bool,
    }
    /// `facing` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Facing {
        /// `north` variant.
        North,
        /// `south` variant.
        South,
        /// `west` variant.
        West,
        /// `east` variant.
        East,
    }
    impl super::Block for SpruceFenceGate {
    }
    impl From<SpruceFenceGate> for super::BlockState {
        fn from(value : SpruceFenceGate) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:waxed_copper_block` block.
pub mod waxed_copper_block {
    /// `minecraft:waxed_copper_block` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct WaxedCopperBlock;
    impl super::Block for WaxedCopperBlock {
    }
    impl From<WaxedCopperBlock> for super::BlockState {
        fn from(value : WaxedCopperBlock) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:prismarine` block.
pub mod prismarine {
    /// `minecraft:prismarine` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct Prismarine;
    impl super::Block for Prismarine {
    }
    impl From<Prismarine> for super::BlockState {
        fn from(value : Prismarine) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:resin_brick_slab` block.
pub mod resin_brick_slab {
    /// `minecraft:resin_brick_slab` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct ResinBrickSlab {
        /// `waterlogged` state.
        pub waterlogged : bool,
        /// `type` state.
        pub kind : Kind,
    }
    /// `type` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Kind {
        /// `top` variant.
        Top,
        /// `bottom` variant.
        Bottom,
        /// `double` variant.
        Double,
    }
    impl super::Block for ResinBrickSlab {
    }
    impl From<ResinBrickSlab> for super::BlockState {
        fn from(value : ResinBrickSlab) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:gray_bed` block.
pub mod gray_bed {
    /// `minecraft:gray_bed` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct GrayBed {
        /// `facing` state.
        pub facing : Facing,
        /// `part` state.
        pub part : Part,
        /// `occupied` state.
        pub occupied : bool,
    }
    /// `facing` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Facing {
        /// `north` variant.
        North,
        /// `south` variant.
        South,
        /// `west` variant.
        West,
        /// `east` variant.
        East,
    }
    /// `part` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Part {
        /// `head` variant.
        Head,
        /// `foot` variant.
        Foot,
    }
    impl super::Block for GrayBed {
    }
    impl From<GrayBed> for super::BlockState {
        fn from(value : GrayBed) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:iron_door` block.
pub mod iron_door {
    /// `minecraft:iron_door` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct IronDoor {
        /// `facing` state.
        pub facing : Facing,
        /// `powered` state.
        pub powered : bool,
        /// `half` state.
        pub half : Half,
        /// `open` state.
        pub open : bool,
        /// `hinge` state.
        pub hinge : Hinge,
    }
    /// `facing` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Facing {
        /// `north` variant.
        North,
        /// `south` variant.
        South,
        /// `west` variant.
        West,
        /// `east` variant.
        East,
    }
    /// `half` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Half {
        /// `upper` variant.
        Upper,
        /// `lower` variant.
        Lower,
    }
    /// `hinge` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Hinge {
        /// `left` variant.
        Left,
        /// `right` variant.
        Right,
    }
    impl super::Block for IronDoor {
    }
    impl From<IronDoor> for super::BlockState {
        fn from(value : IronDoor) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:stone_brick_slab` block.
pub mod stone_brick_slab {
    /// `minecraft:stone_brick_slab` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct StoneBrickSlab {
        /// `type` state.
        pub kind : Kind,
        /// `waterlogged` state.
        pub waterlogged : bool,
    }
    /// `type` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Kind {
        /// `top` variant.
        Top,
        /// `bottom` variant.
        Bottom,
        /// `double` variant.
        Double,
    }
    impl super::Block for StoneBrickSlab {
    }
    impl From<StoneBrickSlab> for super::BlockState {
        fn from(value : StoneBrickSlab) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:waxed_weathered_chiseled_copper` block.
pub mod waxed_weathered_chiseled_copper {
    /// `minecraft:waxed_weathered_chiseled_copper` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct WaxedWeatheredChiseledCopper;
    impl super::Block for WaxedWeatheredChiseledCopper {
    }
    impl From<WaxedWeatheredChiseledCopper> for super::BlockState {
        fn from(value : WaxedWeatheredChiseledCopper) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:gold_block` block.
pub mod gold_block {
    /// `minecraft:gold_block` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct GoldBlock;
    impl super::Block for GoldBlock {
    }
    impl From<GoldBlock> for super::BlockState {
        fn from(value : GoldBlock) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:red_terracotta` block.
pub mod red_terracotta {
    /// `minecraft:red_terracotta` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct RedTerracotta;
    impl super::Block for RedTerracotta {
    }
    impl From<RedTerracotta> for super::BlockState {
        fn from(value : RedTerracotta) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:warped_stairs` block.
pub mod warped_stairs {
    /// `minecraft:warped_stairs` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct WarpedStairs {
        /// `shape` state.
        pub shape : Shape,
        /// `facing` state.
        pub facing : Facing,
        /// `half` state.
        pub half : Half,
        /// `waterlogged` state.
        pub waterlogged : bool,
    }
    /// `shape` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Shape {
        /// `straight` variant.
        Straight,
        /// `inner_left` variant.
        InnerLeft,
        /// `inner_right` variant.
        InnerRight,
        /// `outer_left` variant.
        OuterLeft,
        /// `outer_right` variant.
        OuterRight,
    }
    /// `facing` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Facing {
        /// `north` variant.
        North,
        /// `south` variant.
        South,
        /// `west` variant.
        West,
        /// `east` variant.
        East,
    }
    /// `half` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Half {
        /// `top` variant.
        Top,
        /// `bottom` variant.
        Bottom,
    }
    impl super::Block for WarpedStairs {
    }
    impl From<WarpedStairs> for super::BlockState {
        fn from(value : WarpedStairs) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:water_cauldron` block.
pub mod water_cauldron {
    /// `minecraft:water_cauldron` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct WaterCauldron {
        /// `level` state.
        pub level : Level,
    }
    /// `level` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Level {
        /// `1` variant.
        N1,
        /// `2` variant.
        N2,
        /// `3` variant.
        N3,
    }
    impl super::Block for WaterCauldron {
    }
    impl From<WaterCauldron> for super::BlockState {
        fn from(value : WaterCauldron) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:bamboo_hanging_sign` block.
pub mod bamboo_hanging_sign {
    /// `minecraft:bamboo_hanging_sign` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct BambooHangingSign {
        /// `waterlogged` state.
        pub waterlogged : bool,
        /// `attached` state.
        pub attached : bool,
        /// `rotation` state.
        pub rotation : Rotation,
    }
    /// `rotation` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Rotation {
        /// `0` variant.
        N0,
        /// `1` variant.
        N1,
        /// `2` variant.
        N2,
        /// `3` variant.
        N3,
        /// `4` variant.
        N4,
        /// `5` variant.
        N5,
        /// `6` variant.
        N6,
        /// `7` variant.
        N7,
        /// `8` variant.
        N8,
        /// `9` variant.
        N9,
        /// `10` variant.
        N10,
        /// `11` variant.
        N11,
        /// `12` variant.
        N12,
        /// `13` variant.
        N13,
        /// `14` variant.
        N14,
        /// `15` variant.
        N15,
    }
    impl super::Block for BambooHangingSign {
    }
    impl From<BambooHangingSign> for super::BlockState {
        fn from(value : BambooHangingSign) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:dark_prismarine` block.
pub mod dark_prismarine {
    /// `minecraft:dark_prismarine` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct DarkPrismarine;
    impl super::Block for DarkPrismarine {
    }
    impl From<DarkPrismarine> for super::BlockState {
        fn from(value : DarkPrismarine) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:pink_candle` block.
pub mod pink_candle {
    /// `minecraft:pink_candle` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct PinkCandle {
        /// `lit` state.
        pub lit : bool,
        /// `candles` state.
        pub candles : Candles,
        /// `waterlogged` state.
        pub waterlogged : bool,
    }
    /// `candles` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Candles {
        /// `1` variant.
        N1,
        /// `2` variant.
        N2,
        /// `3` variant.
        N3,
        /// `4` variant.
        N4,
    }
    impl super::Block for PinkCandle {
    }
    impl From<PinkCandle> for super::BlockState {
        fn from(value : PinkCandle) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:red_concrete` block.
pub mod red_concrete {
    /// `minecraft:red_concrete` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct RedConcrete;
    impl super::Block for RedConcrete {
    }
    impl From<RedConcrete> for super::BlockState {
        fn from(value : RedConcrete) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:red_sandstone_stairs` block.
pub mod red_sandstone_stairs {
    /// `minecraft:red_sandstone_stairs` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct RedSandstoneStairs {
        /// `half` state.
        pub half : Half,
        /// `waterlogged` state.
        pub waterlogged : bool,
        /// `shape` state.
        pub shape : Shape,
        /// `facing` state.
        pub facing : Facing,
    }
    /// `half` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Half {
        /// `top` variant.
        Top,
        /// `bottom` variant.
        Bottom,
    }
    /// `shape` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Shape {
        /// `straight` variant.
        Straight,
        /// `inner_left` variant.
        InnerLeft,
        /// `inner_right` variant.
        InnerRight,
        /// `outer_left` variant.
        OuterLeft,
        /// `outer_right` variant.
        OuterRight,
    }
    /// `facing` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Facing {
        /// `north` variant.
        North,
        /// `south` variant.
        South,
        /// `west` variant.
        West,
        /// `east` variant.
        East,
    }
    impl super::Block for RedSandstoneStairs {
    }
    impl From<RedSandstoneStairs> for super::BlockState {
        fn from(value : RedSandstoneStairs) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:cornflower` block.
pub mod cornflower {
    /// `minecraft:cornflower` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct Cornflower;
    impl super::Block for Cornflower {
    }
    impl From<Cornflower> for super::BlockState {
        fn from(value : Cornflower) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:crying_obsidian` block.
pub mod crying_obsidian {
    /// `minecraft:crying_obsidian` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct CryingObsidian;
    impl super::Block for CryingObsidian {
    }
    impl From<CryingObsidian> for super::BlockState {
        fn from(value : CryingObsidian) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:loom` block.
pub mod loom {
    /// `minecraft:loom` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct Loom {
        /// `facing` state.
        pub facing : Facing,
    }
    /// `facing` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Facing {
        /// `north` variant.
        North,
        /// `south` variant.
        South,
        /// `west` variant.
        West,
        /// `east` variant.
        East,
    }
    impl super::Block for Loom {
    }
    impl From<Loom> for super::BlockState {
        fn from(value : Loom) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:stripped_birch_wood` block.
pub mod stripped_birch_wood {
    /// `minecraft:stripped_birch_wood` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct StrippedBirchWood {
        /// `axis` state.
        pub axis : Axis,
    }
    /// `axis` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Axis {
        /// `x` variant.
        X,
        /// `y` variant.
        Y,
        /// `z` variant.
        Z,
    }
    impl super::Block for StrippedBirchWood {
    }
    impl From<StrippedBirchWood> for super::BlockState {
        fn from(value : StrippedBirchWood) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:waxed_copper_bulb` block.
pub mod waxed_copper_bulb {
    /// `minecraft:waxed_copper_bulb` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct WaxedCopperBulb {
        /// `powered` state.
        pub powered : bool,
        /// `lit` state.
        pub lit : bool,
    }
    impl super::Block for WaxedCopperBulb {
    }
    impl From<WaxedCopperBulb> for super::BlockState {
        fn from(value : WaxedCopperBulb) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:weathered_copper_trapdoor` block.
pub mod weathered_copper_trapdoor {
    /// `minecraft:weathered_copper_trapdoor` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct WeatheredCopperTrapdoor {
        /// `powered` state.
        pub powered : bool,
        /// `facing` state.
        pub facing : Facing,
        /// `waterlogged` state.
        pub waterlogged : bool,
        /// `half` state.
        pub half : Half,
        /// `open` state.
        pub open : bool,
    }
    /// `facing` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Facing {
        /// `north` variant.
        North,
        /// `south` variant.
        South,
        /// `west` variant.
        West,
        /// `east` variant.
        East,
    }
    /// `half` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Half {
        /// `top` variant.
        Top,
        /// `bottom` variant.
        Bottom,
    }
    impl super::Block for WeatheredCopperTrapdoor {
    }
    impl From<WeatheredCopperTrapdoor> for super::BlockState {
        fn from(value : WeatheredCopperTrapdoor) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:red_carpet` block.
pub mod red_carpet {
    /// `minecraft:red_carpet` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct RedCarpet;
    impl super::Block for RedCarpet {
    }
    impl From<RedCarpet> for super::BlockState {
        fn from(value : RedCarpet) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:stone_slab` block.
pub mod stone_slab {
    /// `minecraft:stone_slab` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct StoneSlab {
        /// `waterlogged` state.
        pub waterlogged : bool,
        /// `type` state.
        pub kind : Kind,
    }
    /// `type` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Kind {
        /// `top` variant.
        Top,
        /// `bottom` variant.
        Bottom,
        /// `double` variant.
        Double,
    }
    impl super::Block for StoneSlab {
    }
    impl From<StoneSlab> for super::BlockState {
        fn from(value : StoneSlab) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:waxed_exposed_copper` block.
pub mod waxed_exposed_copper {
    /// `minecraft:waxed_exposed_copper` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct WaxedExposedCopper;
    impl super::Block for WaxedExposedCopper {
    }
    impl From<WaxedExposedCopper> for super::BlockState {
        fn from(value : WaxedExposedCopper) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:crimson_stem` block.
pub mod crimson_stem {
    /// `minecraft:crimson_stem` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct CrimsonStem {
        /// `axis` state.
        pub axis : Axis,
    }
    /// `axis` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Axis {
        /// `x` variant.
        X,
        /// `y` variant.
        Y,
        /// `z` variant.
        Z,
    }
    impl super::Block for CrimsonStem {
    }
    impl From<CrimsonStem> for super::BlockState {
        fn from(value : CrimsonStem) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:player_wall_head` block.
pub mod player_wall_head {
    /// `minecraft:player_wall_head` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct PlayerWallHead {
        /// `facing` state.
        pub facing : Facing,
        /// `powered` state.
        pub powered : bool,
    }
    /// `facing` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Facing {
        /// `north` variant.
        North,
        /// `south` variant.
        South,
        /// `west` variant.
        West,
        /// `east` variant.
        East,
    }
    impl super::Block for PlayerWallHead {
    }
    impl From<PlayerWallHead> for super::BlockState {
        fn from(value : PlayerWallHead) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:potted_torchflower` block.
pub mod potted_torchflower {
    /// `minecraft:potted_torchflower` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct PottedTorchflower;
    impl super::Block for PottedTorchflower {
    }
    impl From<PottedTorchflower> for super::BlockState {
        fn from(value : PottedTorchflower) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:pumpkin` block.
pub mod pumpkin {
    /// `minecraft:pumpkin` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct Pumpkin;
    impl super::Block for Pumpkin {
    }
    impl From<Pumpkin> for super::BlockState {
        fn from(value : Pumpkin) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:warped_stem` block.
pub mod warped_stem {
    /// `minecraft:warped_stem` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct WarpedStem {
        /// `axis` state.
        pub axis : Axis,
    }
    /// `axis` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Axis {
        /// `x` variant.
        X,
        /// `y` variant.
        Y,
        /// `z` variant.
        Z,
    }
    impl super::Block for WarpedStem {
    }
    impl From<WarpedStem> for super::BlockState {
        fn from(value : WarpedStem) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:bubble_column` block.
pub mod bubble_column {
    /// `minecraft:bubble_column` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct BubbleColumn {
        /// `drag` state.
        pub drag : bool,
    }
    impl super::Block for BubbleColumn {
    }
    impl From<BubbleColumn> for super::BlockState {
        fn from(value : BubbleColumn) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:sandstone_wall` block.
pub mod sandstone_wall {
    /// `minecraft:sandstone_wall` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct SandstoneWall {
        /// `west` state.
        pub west : West,
        /// `east` state.
        pub east : East,
        /// `south` state.
        pub south : South,
        /// `up` state.
        pub up : bool,
        /// `north` state.
        pub north : North,
        /// `waterlogged` state.
        pub waterlogged : bool,
    }
    /// `west` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum West {
        /// `none` variant.
        None,
        /// `low` variant.
        Low,
        /// `tall` variant.
        Tall,
    }
    /// `east` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum East {
        /// `none` variant.
        None,
        /// `low` variant.
        Low,
        /// `tall` variant.
        Tall,
    }
    /// `south` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum South {
        /// `none` variant.
        None,
        /// `low` variant.
        Low,
        /// `tall` variant.
        Tall,
    }
    /// `north` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum North {
        /// `none` variant.
        None,
        /// `low` variant.
        Low,
        /// `tall` variant.
        Tall,
    }
    impl super::Block for SandstoneWall {
    }
    impl From<SandstoneWall> for super::BlockState {
        fn from(value : SandstoneWall) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:cherry_log` block.
pub mod cherry_log {
    /// `minecraft:cherry_log` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct CherryLog {
        /// `axis` state.
        pub axis : Axis,
    }
    /// `axis` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Axis {
        /// `x` variant.
        X,
        /// `y` variant.
        Y,
        /// `z` variant.
        Z,
    }
    impl super::Block for CherryLog {
    }
    impl From<CherryLog> for super::BlockState {
        fn from(value : CherryLog) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:pale_moss_block` block.
pub mod pale_moss_block {
    /// `minecraft:pale_moss_block` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct PaleMossBlock;
    impl super::Block for PaleMossBlock {
    }
    impl From<PaleMossBlock> for super::BlockState {
        fn from(value : PaleMossBlock) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:warped_door` block.
pub mod warped_door {
    /// `minecraft:warped_door` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct WarpedDoor {
        /// `hinge` state.
        pub hinge : Hinge,
        /// `facing` state.
        pub facing : Facing,
        /// `half` state.
        pub half : Half,
        /// `open` state.
        pub open : bool,
        /// `powered` state.
        pub powered : bool,
    }
    /// `hinge` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Hinge {
        /// `left` variant.
        Left,
        /// `right` variant.
        Right,
    }
    /// `facing` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Facing {
        /// `north` variant.
        North,
        /// `south` variant.
        South,
        /// `west` variant.
        West,
        /// `east` variant.
        East,
    }
    /// `half` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Half {
        /// `upper` variant.
        Upper,
        /// `lower` variant.
        Lower,
    }
    impl super::Block for WarpedDoor {
    }
    impl From<WarpedDoor> for super::BlockState {
        fn from(value : WarpedDoor) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:brown_shulker_box` block.
pub mod brown_shulker_box {
    /// `minecraft:brown_shulker_box` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct BrownShulkerBox {
        /// `facing` state.
        pub facing : Facing,
    }
    /// `facing` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Facing {
        /// `north` variant.
        North,
        /// `east` variant.
        East,
        /// `south` variant.
        South,
        /// `west` variant.
        West,
        /// `up` variant.
        Up,
        /// `down` variant.
        Down,
    }
    impl super::Block for BrownShulkerBox {
    }
    impl From<BrownShulkerBox> for super::BlockState {
        fn from(value : BrownShulkerBox) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:lime_stained_glass_pane` block.
pub mod lime_stained_glass_pane {
    /// `minecraft:lime_stained_glass_pane` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct LimeStainedGlassPane {
        /// `west` state.
        pub west : bool,
        /// `east` state.
        pub east : bool,
        /// `north` state.
        pub north : bool,
        /// `south` state.
        pub south : bool,
        /// `waterlogged` state.
        pub waterlogged : bool,
    }
    impl super::Block for LimeStainedGlassPane {
    }
    impl From<LimeStainedGlassPane> for super::BlockState {
        fn from(value : LimeStainedGlassPane) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:sandstone` block.
pub mod sandstone {
    /// `minecraft:sandstone` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct Sandstone;
    impl super::Block for Sandstone {
    }
    impl From<Sandstone> for super::BlockState {
        fn from(value : Sandstone) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:coal_block` block.
pub mod coal_block {
    /// `minecraft:coal_block` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct CoalBlock;
    impl super::Block for CoalBlock {
    }
    impl From<CoalBlock> for super::BlockState {
        fn from(value : CoalBlock) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:air` block.
pub mod air {
    /// `minecraft:air` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct Air;
    impl super::Block for Air {
    }
    impl From<Air> for super::BlockState {
        fn from(value : Air) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:pitcher_crop` block.
pub mod pitcher_crop {
    /// `minecraft:pitcher_crop` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct PitcherCrop {
        /// `half` state.
        pub half : Half,
        /// `age` state.
        pub age : Age,
    }
    /// `half` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Half {
        /// `upper` variant.
        Upper,
        /// `lower` variant.
        Lower,
    }
    /// `age` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Age {
        /// `0` variant.
        N0,
        /// `1` variant.
        N1,
        /// `2` variant.
        N2,
        /// `3` variant.
        N3,
        /// `4` variant.
        N4,
    }
    impl super::Block for PitcherCrop {
    }
    impl From<PitcherCrop> for super::BlockState {
        fn from(value : PitcherCrop) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:warped_wall_sign` block.
pub mod warped_wall_sign {
    /// `minecraft:warped_wall_sign` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct WarpedWallSign {
        /// `waterlogged` state.
        pub waterlogged : bool,
        /// `facing` state.
        pub facing : Facing,
    }
    /// `facing` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Facing {
        /// `north` variant.
        North,
        /// `south` variant.
        South,
        /// `west` variant.
        West,
        /// `east` variant.
        East,
    }
    impl super::Block for WarpedWallSign {
    }
    impl From<WarpedWallSign> for super::BlockState {
        fn from(value : WarpedWallSign) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:dead_bush` block.
pub mod dead_bush {
    /// `minecraft:dead_bush` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct DeadBush;
    impl super::Block for DeadBush {
    }
    impl From<DeadBush> for super::BlockState {
        fn from(value : DeadBush) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:cherry_stairs` block.
pub mod cherry_stairs {
    /// `minecraft:cherry_stairs` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct CherryStairs {
        /// `half` state.
        pub half : Half,
        /// `waterlogged` state.
        pub waterlogged : bool,
        /// `facing` state.
        pub facing : Facing,
        /// `shape` state.
        pub shape : Shape,
    }
    /// `half` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Half {
        /// `top` variant.
        Top,
        /// `bottom` variant.
        Bottom,
    }
    /// `facing` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Facing {
        /// `north` variant.
        North,
        /// `south` variant.
        South,
        /// `west` variant.
        West,
        /// `east` variant.
        East,
    }
    /// `shape` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Shape {
        /// `straight` variant.
        Straight,
        /// `inner_left` variant.
        InnerLeft,
        /// `inner_right` variant.
        InnerRight,
        /// `outer_left` variant.
        OuterLeft,
        /// `outer_right` variant.
        OuterRight,
    }
    impl super::Block for CherryStairs {
    }
    impl From<CherryStairs> for super::BlockState {
        fn from(value : CherryStairs) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:warped_wart_block` block.
pub mod warped_wart_block {
    /// `minecraft:warped_wart_block` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct WarpedWartBlock;
    impl super::Block for WarpedWartBlock {
    }
    impl From<WarpedWartBlock> for super::BlockState {
        fn from(value : WarpedWartBlock) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:chiseled_red_sandstone` block.
pub mod chiseled_red_sandstone {
    /// `minecraft:chiseled_red_sandstone` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct ChiseledRedSandstone;
    impl super::Block for ChiseledRedSandstone {
    }
    impl From<ChiseledRedSandstone> for super::BlockState {
        fn from(value : ChiseledRedSandstone) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:weathered_copper` block.
pub mod weathered_copper {
    /// `minecraft:weathered_copper` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct WeatheredCopper;
    impl super::Block for WeatheredCopper {
    }
    impl From<WeatheredCopper> for super::BlockState {
        fn from(value : WeatheredCopper) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:purple_carpet` block.
pub mod purple_carpet {
    /// `minecraft:purple_carpet` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct PurpleCarpet;
    impl super::Block for PurpleCarpet {
    }
    impl From<PurpleCarpet> for super::BlockState {
        fn from(value : PurpleCarpet) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:waxed_exposed_cut_copper_stairs` block.
pub mod waxed_exposed_cut_copper_stairs {
    /// `minecraft:waxed_exposed_cut_copper_stairs` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct WaxedExposedCutCopperStairs {
        /// `shape` state.
        pub shape : Shape,
        /// `half` state.
        pub half : Half,
        /// `facing` state.
        pub facing : Facing,
        /// `waterlogged` state.
        pub waterlogged : bool,
    }
    /// `shape` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Shape {
        /// `straight` variant.
        Straight,
        /// `inner_left` variant.
        InnerLeft,
        /// `inner_right` variant.
        InnerRight,
        /// `outer_left` variant.
        OuterLeft,
        /// `outer_right` variant.
        OuterRight,
    }
    /// `half` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Half {
        /// `top` variant.
        Top,
        /// `bottom` variant.
        Bottom,
    }
    /// `facing` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Facing {
        /// `north` variant.
        North,
        /// `south` variant.
        South,
        /// `west` variant.
        West,
        /// `east` variant.
        East,
    }
    impl super::Block for WaxedExposedCutCopperStairs {
    }
    impl From<WaxedExposedCutCopperStairs> for super::BlockState {
        fn from(value : WaxedExposedCutCopperStairs) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:dirt_path` block.
pub mod dirt_path {
    /// `minecraft:dirt_path` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct DirtPath;
    impl super::Block for DirtPath {
    }
    impl From<DirtPath> for super::BlockState {
        fn from(value : DirtPath) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:azalea` block.
pub mod azalea {
    /// `minecraft:azalea` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct Azalea;
    impl super::Block for Azalea {
    }
    impl From<Azalea> for super::BlockState {
        fn from(value : Azalea) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:emerald_block` block.
pub mod emerald_block {
    /// `minecraft:emerald_block` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct EmeraldBlock;
    impl super::Block for EmeraldBlock {
    }
    impl From<EmeraldBlock> for super::BlockState {
        fn from(value : EmeraldBlock) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:tripwire` block.
pub mod tripwire {
    /// `minecraft:tripwire` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct Tripwire {
        /// `powered` state.
        pub powered : bool,
        /// `north` state.
        pub north : bool,
        /// `south` state.
        pub south : bool,
        /// `west` state.
        pub west : bool,
        /// `attached` state.
        pub attached : bool,
        /// `disarmed` state.
        pub disarmed : bool,
        /// `east` state.
        pub east : bool,
    }
    impl super::Block for Tripwire {
    }
    impl From<Tripwire> for super::BlockState {
        fn from(value : Tripwire) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:wheat` block.
pub mod wheat {
    /// `minecraft:wheat` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct Wheat {
        /// `age` state.
        pub age : Age,
    }
    /// `age` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Age {
        /// `0` variant.
        N0,
        /// `1` variant.
        N1,
        /// `2` variant.
        N2,
        /// `3` variant.
        N3,
        /// `4` variant.
        N4,
        /// `5` variant.
        N5,
        /// `6` variant.
        N6,
        /// `7` variant.
        N7,
    }
    impl super::Block for Wheat {
    }
    impl From<Wheat> for super::BlockState {
        fn from(value : Wheat) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:water` block.
pub mod water {
    /// `minecraft:water` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct Water {
        /// `level` state.
        pub level : Level,
    }
    /// `level` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Level {
        /// `0` variant.
        N0,
        /// `1` variant.
        N1,
        /// `2` variant.
        N2,
        /// `3` variant.
        N3,
        /// `4` variant.
        N4,
        /// `5` variant.
        N5,
        /// `6` variant.
        N6,
        /// `7` variant.
        N7,
        /// `8` variant.
        N8,
        /// `9` variant.
        N9,
        /// `10` variant.
        N10,
        /// `11` variant.
        N11,
        /// `12` variant.
        N12,
        /// `13` variant.
        N13,
        /// `14` variant.
        N14,
        /// `15` variant.
        N15,
    }
    impl super::Block for Water {
    }
    impl From<Water> for super::BlockState {
        fn from(value : Water) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:light_gray_carpet` block.
pub mod light_gray_carpet {
    /// `minecraft:light_gray_carpet` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct LightGrayCarpet;
    impl super::Block for LightGrayCarpet {
    }
    impl From<LightGrayCarpet> for super::BlockState {
        fn from(value : LightGrayCarpet) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:jungle_stairs` block.
pub mod jungle_stairs {
    /// `minecraft:jungle_stairs` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct JungleStairs {
        /// `facing` state.
        pub facing : Facing,
        /// `half` state.
        pub half : Half,
        /// `shape` state.
        pub shape : Shape,
        /// `waterlogged` state.
        pub waterlogged : bool,
    }
    /// `facing` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Facing {
        /// `north` variant.
        North,
        /// `south` variant.
        South,
        /// `west` variant.
        West,
        /// `east` variant.
        East,
    }
    /// `half` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Half {
        /// `top` variant.
        Top,
        /// `bottom` variant.
        Bottom,
    }
    /// `shape` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Shape {
        /// `straight` variant.
        Straight,
        /// `inner_left` variant.
        InnerLeft,
        /// `inner_right` variant.
        InnerRight,
        /// `outer_left` variant.
        OuterLeft,
        /// `outer_right` variant.
        OuterRight,
    }
    impl super::Block for JungleStairs {
    }
    impl From<JungleStairs> for super::BlockState {
        fn from(value : JungleStairs) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:dark_oak_fence` block.
pub mod dark_oak_fence {
    /// `minecraft:dark_oak_fence` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct DarkOakFence {
        /// `waterlogged` state.
        pub waterlogged : bool,
        /// `west` state.
        pub west : bool,
        /// `north` state.
        pub north : bool,
        /// `east` state.
        pub east : bool,
        /// `south` state.
        pub south : bool,
    }
    impl super::Block for DarkOakFence {
    }
    impl From<DarkOakFence> for super::BlockState {
        fn from(value : DarkOakFence) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:crimson_slab` block.
pub mod crimson_slab {
    /// `minecraft:crimson_slab` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct CrimsonSlab {
        /// `type` state.
        pub kind : Kind,
        /// `waterlogged` state.
        pub waterlogged : bool,
    }
    /// `type` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Kind {
        /// `top` variant.
        Top,
        /// `bottom` variant.
        Bottom,
        /// `double` variant.
        Double,
    }
    impl super::Block for CrimsonSlab {
    }
    impl From<CrimsonSlab> for super::BlockState {
        fn from(value : CrimsonSlab) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:gray_wall_banner` block.
pub mod gray_wall_banner {
    /// `minecraft:gray_wall_banner` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct GrayWallBanner {
        /// `facing` state.
        pub facing : Facing,
    }
    /// `facing` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Facing {
        /// `north` variant.
        North,
        /// `south` variant.
        South,
        /// `west` variant.
        West,
        /// `east` variant.
        East,
    }
    impl super::Block for GrayWallBanner {
    }
    impl From<GrayWallBanner> for super::BlockState {
        fn from(value : GrayWallBanner) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:bamboo_mosaic_slab` block.
pub mod bamboo_mosaic_slab {
    /// `minecraft:bamboo_mosaic_slab` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct BambooMosaicSlab {
        /// `type` state.
        pub kind : Kind,
        /// `waterlogged` state.
        pub waterlogged : bool,
    }
    /// `type` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Kind {
        /// `top` variant.
        Top,
        /// `bottom` variant.
        Bottom,
        /// `double` variant.
        Double,
    }
    impl super::Block for BambooMosaicSlab {
    }
    impl From<BambooMosaicSlab> for super::BlockState {
        fn from(value : BambooMosaicSlab) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:potted_oak_sapling` block.
pub mod potted_oak_sapling {
    /// `minecraft:potted_oak_sapling` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct PottedOakSapling;
    impl super::Block for PottedOakSapling {
    }
    impl From<PottedOakSapling> for super::BlockState {
        fn from(value : PottedOakSapling) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:crimson_fence_gate` block.
pub mod crimson_fence_gate {
    /// `minecraft:crimson_fence_gate` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct CrimsonFenceGate {
        /// `facing` state.
        pub facing : Facing,
        /// `powered` state.
        pub powered : bool,
        /// `in_wall` state.
        pub in_wall : bool,
        /// `open` state.
        pub open : bool,
    }
    /// `facing` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Facing {
        /// `north` variant.
        North,
        /// `south` variant.
        South,
        /// `west` variant.
        West,
        /// `east` variant.
        East,
    }
    impl super::Block for CrimsonFenceGate {
    }
    impl From<CrimsonFenceGate> for super::BlockState {
        fn from(value : CrimsonFenceGate) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:end_stone_brick_wall` block.
pub mod end_stone_brick_wall {
    /// `minecraft:end_stone_brick_wall` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct EndStoneBrickWall {
        /// `east` state.
        pub east : East,
        /// `north` state.
        pub north : North,
        /// `up` state.
        pub up : bool,
        /// `waterlogged` state.
        pub waterlogged : bool,
        /// `west` state.
        pub west : West,
        /// `south` state.
        pub south : South,
    }
    /// `east` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum East {
        /// `none` variant.
        None,
        /// `low` variant.
        Low,
        /// `tall` variant.
        Tall,
    }
    /// `north` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum North {
        /// `none` variant.
        None,
        /// `low` variant.
        Low,
        /// `tall` variant.
        Tall,
    }
    /// `west` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum West {
        /// `none` variant.
        None,
        /// `low` variant.
        Low,
        /// `tall` variant.
        Tall,
    }
    /// `south` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum South {
        /// `none` variant.
        None,
        /// `low` variant.
        Low,
        /// `tall` variant.
        Tall,
    }
    impl super::Block for EndStoneBrickWall {
    }
    impl From<EndStoneBrickWall> for super::BlockState {
        fn from(value : EndStoneBrickWall) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:redstone_torch` block.
pub mod redstone_torch {
    /// `minecraft:redstone_torch` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct RedstoneTorch {
        /// `lit` state.
        pub lit : bool,
    }
    impl super::Block for RedstoneTorch {
    }
    impl From<RedstoneTorch> for super::BlockState {
        fn from(value : RedstoneTorch) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:fletching_table` block.
pub mod fletching_table {
    /// `minecraft:fletching_table` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct FletchingTable;
    impl super::Block for FletchingTable {
    }
    impl From<FletchingTable> for super::BlockState {
        fn from(value : FletchingTable) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:waxed_weathered_copper_trapdoor` block.
pub mod waxed_weathered_copper_trapdoor {
    /// `minecraft:waxed_weathered_copper_trapdoor` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct WaxedWeatheredCopperTrapdoor {
        /// `open` state.
        pub open : bool,
        /// `powered` state.
        pub powered : bool,
        /// `waterlogged` state.
        pub waterlogged : bool,
        /// `facing` state.
        pub facing : Facing,
        /// `half` state.
        pub half : Half,
    }
    /// `facing` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Facing {
        /// `north` variant.
        North,
        /// `south` variant.
        South,
        /// `west` variant.
        West,
        /// `east` variant.
        East,
    }
    /// `half` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Half {
        /// `top` variant.
        Top,
        /// `bottom` variant.
        Bottom,
    }
    impl super::Block for WaxedWeatheredCopperTrapdoor {
    }
    impl From<WaxedWeatheredCopperTrapdoor> for super::BlockState {
        fn from(value : WaxedWeatheredCopperTrapdoor) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:zombie_head` block.
pub mod zombie_head {
    /// `minecraft:zombie_head` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct ZombieHead {
        /// `powered` state.
        pub powered : bool,
        /// `rotation` state.
        pub rotation : Rotation,
    }
    /// `rotation` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Rotation {
        /// `0` variant.
        N0,
        /// `1` variant.
        N1,
        /// `2` variant.
        N2,
        /// `3` variant.
        N3,
        /// `4` variant.
        N4,
        /// `5` variant.
        N5,
        /// `6` variant.
        N6,
        /// `7` variant.
        N7,
        /// `8` variant.
        N8,
        /// `9` variant.
        N9,
        /// `10` variant.
        N10,
        /// `11` variant.
        N11,
        /// `12` variant.
        N12,
        /// `13` variant.
        N13,
        /// `14` variant.
        N14,
        /// `15` variant.
        N15,
    }
    impl super::Block for ZombieHead {
    }
    impl From<ZombieHead> for super::BlockState {
        fn from(value : ZombieHead) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:exposed_chiseled_copper` block.
pub mod exposed_chiseled_copper {
    /// `minecraft:exposed_chiseled_copper` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct ExposedChiseledCopper;
    impl super::Block for ExposedChiseledCopper {
    }
    impl From<ExposedChiseledCopper> for super::BlockState {
        fn from(value : ExposedChiseledCopper) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:prismarine_wall` block.
pub mod prismarine_wall {
    /// `minecraft:prismarine_wall` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct PrismarineWall {
        /// `north` state.
        pub north : North,
        /// `south` state.
        pub south : South,
        /// `up` state.
        pub up : bool,
        /// `waterlogged` state.
        pub waterlogged : bool,
        /// `east` state.
        pub east : East,
        /// `west` state.
        pub west : West,
    }
    /// `north` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum North {
        /// `none` variant.
        None,
        /// `low` variant.
        Low,
        /// `tall` variant.
        Tall,
    }
    /// `south` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum South {
        /// `none` variant.
        None,
        /// `low` variant.
        Low,
        /// `tall` variant.
        Tall,
    }
    /// `east` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum East {
        /// `none` variant.
        None,
        /// `low` variant.
        Low,
        /// `tall` variant.
        Tall,
    }
    /// `west` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum West {
        /// `none` variant.
        None,
        /// `low` variant.
        Low,
        /// `tall` variant.
        Tall,
    }
    impl super::Block for PrismarineWall {
    }
    impl From<PrismarineWall> for super::BlockState {
        fn from(value : PrismarineWall) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:sponge` block.
pub mod sponge {
    /// `minecraft:sponge` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct Sponge;
    impl super::Block for Sponge {
    }
    impl From<Sponge> for super::BlockState {
        fn from(value : Sponge) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:yellow_banner` block.
pub mod yellow_banner {
    /// `minecraft:yellow_banner` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct YellowBanner {
        /// `rotation` state.
        pub rotation : Rotation,
    }
    /// `rotation` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Rotation {
        /// `0` variant.
        N0,
        /// `1` variant.
        N1,
        /// `2` variant.
        N2,
        /// `3` variant.
        N3,
        /// `4` variant.
        N4,
        /// `5` variant.
        N5,
        /// `6` variant.
        N6,
        /// `7` variant.
        N7,
        /// `8` variant.
        N8,
        /// `9` variant.
        N9,
        /// `10` variant.
        N10,
        /// `11` variant.
        N11,
        /// `12` variant.
        N12,
        /// `13` variant.
        N13,
        /// `14` variant.
        N14,
        /// `15` variant.
        N15,
    }
    impl super::Block for YellowBanner {
    }
    impl From<YellowBanner> for super::BlockState {
        fn from(value : YellowBanner) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:gray_candle` block.
pub mod gray_candle {
    /// `minecraft:gray_candle` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct GrayCandle {
        /// `candles` state.
        pub candles : Candles,
        /// `lit` state.
        pub lit : bool,
        /// `waterlogged` state.
        pub waterlogged : bool,
    }
    /// `candles` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Candles {
        /// `1` variant.
        N1,
        /// `2` variant.
        N2,
        /// `3` variant.
        N3,
        /// `4` variant.
        N4,
    }
    impl super::Block for GrayCandle {
    }
    impl From<GrayCandle> for super::BlockState {
        fn from(value : GrayCandle) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:red_mushroom` block.
pub mod red_mushroom {
    /// `minecraft:red_mushroom` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct RedMushroom;
    impl super::Block for RedMushroom {
    }
    impl From<RedMushroom> for super::BlockState {
        fn from(value : RedMushroom) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:potted_fern` block.
pub mod potted_fern {
    /// `minecraft:potted_fern` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct PottedFern;
    impl super::Block for PottedFern {
    }
    impl From<PottedFern> for super::BlockState {
        fn from(value : PottedFern) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:resin_brick_wall` block.
pub mod resin_brick_wall {
    /// `minecraft:resin_brick_wall` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct ResinBrickWall {
        /// `north` state.
        pub north : North,
        /// `south` state.
        pub south : South,
        /// `up` state.
        pub up : bool,
        /// `west` state.
        pub west : West,
        /// `waterlogged` state.
        pub waterlogged : bool,
        /// `east` state.
        pub east : East,
    }
    /// `north` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum North {
        /// `none` variant.
        None,
        /// `low` variant.
        Low,
        /// `tall` variant.
        Tall,
    }
    /// `south` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum South {
        /// `none` variant.
        None,
        /// `low` variant.
        Low,
        /// `tall` variant.
        Tall,
    }
    /// `west` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum West {
        /// `none` variant.
        None,
        /// `low` variant.
        Low,
        /// `tall` variant.
        Tall,
    }
    /// `east` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum East {
        /// `none` variant.
        None,
        /// `low` variant.
        Low,
        /// `tall` variant.
        Tall,
    }
    impl super::Block for ResinBrickWall {
    }
    impl From<ResinBrickWall> for super::BlockState {
        fn from(value : ResinBrickWall) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:polished_blackstone_brick_wall` block.
pub mod polished_blackstone_brick_wall {
    /// `minecraft:polished_blackstone_brick_wall` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct PolishedBlackstoneBrickWall {
        /// `east` state.
        pub east : East,
        /// `waterlogged` state.
        pub waterlogged : bool,
        /// `south` state.
        pub south : South,
        /// `up` state.
        pub up : bool,
        /// `north` state.
        pub north : North,
        /// `west` state.
        pub west : West,
    }
    /// `east` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum East {
        /// `none` variant.
        None,
        /// `low` variant.
        Low,
        /// `tall` variant.
        Tall,
    }
    /// `south` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum South {
        /// `none` variant.
        None,
        /// `low` variant.
        Low,
        /// `tall` variant.
        Tall,
    }
    /// `north` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum North {
        /// `none` variant.
        None,
        /// `low` variant.
        Low,
        /// `tall` variant.
        Tall,
    }
    /// `west` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum West {
        /// `none` variant.
        None,
        /// `low` variant.
        Low,
        /// `tall` variant.
        Tall,
    }
    impl super::Block for PolishedBlackstoneBrickWall {
    }
    impl From<PolishedBlackstoneBrickWall> for super::BlockState {
        fn from(value : PolishedBlackstoneBrickWall) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:waxed_cut_copper_stairs` block.
pub mod waxed_cut_copper_stairs {
    /// `minecraft:waxed_cut_copper_stairs` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct WaxedCutCopperStairs {
        /// `facing` state.
        pub facing : Facing,
        /// `waterlogged` state.
        pub waterlogged : bool,
        /// `shape` state.
        pub shape : Shape,
        /// `half` state.
        pub half : Half,
    }
    /// `facing` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Facing {
        /// `north` variant.
        North,
        /// `south` variant.
        South,
        /// `west` variant.
        West,
        /// `east` variant.
        East,
    }
    /// `shape` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Shape {
        /// `straight` variant.
        Straight,
        /// `inner_left` variant.
        InnerLeft,
        /// `inner_right` variant.
        InnerRight,
        /// `outer_left` variant.
        OuterLeft,
        /// `outer_right` variant.
        OuterRight,
    }
    /// `half` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Half {
        /// `top` variant.
        Top,
        /// `bottom` variant.
        Bottom,
    }
    impl super::Block for WaxedCutCopperStairs {
    }
    impl From<WaxedCutCopperStairs> for super::BlockState {
        fn from(value : WaxedCutCopperStairs) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:conduit` block.
pub mod conduit {
    /// `minecraft:conduit` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct Conduit {
        /// `waterlogged` state.
        pub waterlogged : bool,
    }
    impl super::Block for Conduit {
    }
    impl From<Conduit> for super::BlockState {
        fn from(value : Conduit) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:potted_open_eyeblossom` block.
pub mod potted_open_eyeblossom {
    /// `minecraft:potted_open_eyeblossom` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct PottedOpenEyeblossom;
    impl super::Block for PottedOpenEyeblossom {
    }
    impl From<PottedOpenEyeblossom> for super::BlockState {
        fn from(value : PottedOpenEyeblossom) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:polished_blackstone_pressure_plate` block.
pub mod polished_blackstone_pressure_plate {
    /// `minecraft:polished_blackstone_pressure_plate` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct PolishedBlackstonePressurePlate {
        /// `powered` state.
        pub powered : bool,
    }
    impl super::Block for PolishedBlackstonePressurePlate {
    }
    impl From<PolishedBlackstonePressurePlate> for super::BlockState {
        fn from(value : PolishedBlackstonePressurePlate) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:prismarine_brick_stairs` block.
pub mod prismarine_brick_stairs {
    /// `minecraft:prismarine_brick_stairs` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct PrismarineBrickStairs {
        /// `facing` state.
        pub facing : Facing,
        /// `half` state.
        pub half : Half,
        /// `shape` state.
        pub shape : Shape,
        /// `waterlogged` state.
        pub waterlogged : bool,
    }
    /// `facing` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Facing {
        /// `north` variant.
        North,
        /// `south` variant.
        South,
        /// `west` variant.
        West,
        /// `east` variant.
        East,
    }
    /// `half` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Half {
        /// `top` variant.
        Top,
        /// `bottom` variant.
        Bottom,
    }
    /// `shape` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Shape {
        /// `straight` variant.
        Straight,
        /// `inner_left` variant.
        InnerLeft,
        /// `inner_right` variant.
        InnerRight,
        /// `outer_left` variant.
        OuterLeft,
        /// `outer_right` variant.
        OuterRight,
    }
    impl super::Block for PrismarineBrickStairs {
    }
    impl From<PrismarineBrickStairs> for super::BlockState {
        fn from(value : PrismarineBrickStairs) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:andesite_wall` block.
pub mod andesite_wall {
    /// `minecraft:andesite_wall` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct AndesiteWall {
        /// `up` state.
        pub up : bool,
        /// `west` state.
        pub west : West,
        /// `north` state.
        pub north : North,
        /// `east` state.
        pub east : East,
        /// `south` state.
        pub south : South,
        /// `waterlogged` state.
        pub waterlogged : bool,
    }
    /// `west` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum West {
        /// `none` variant.
        None,
        /// `low` variant.
        Low,
        /// `tall` variant.
        Tall,
    }
    /// `north` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum North {
        /// `none` variant.
        None,
        /// `low` variant.
        Low,
        /// `tall` variant.
        Tall,
    }
    /// `east` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum East {
        /// `none` variant.
        None,
        /// `low` variant.
        Low,
        /// `tall` variant.
        Tall,
    }
    /// `south` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum South {
        /// `none` variant.
        None,
        /// `low` variant.
        Low,
        /// `tall` variant.
        Tall,
    }
    impl super::Block for AndesiteWall {
    }
    impl From<AndesiteWall> for super::BlockState {
        fn from(value : AndesiteWall) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:carved_pumpkin` block.
pub mod carved_pumpkin {
    /// `minecraft:carved_pumpkin` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct CarvedPumpkin {
        /// `facing` state.
        pub facing : Facing,
    }
    /// `facing` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Facing {
        /// `north` variant.
        North,
        /// `south` variant.
        South,
        /// `west` variant.
        West,
        /// `east` variant.
        East,
    }
    impl super::Block for CarvedPumpkin {
    }
    impl From<CarvedPumpkin> for super::BlockState {
        fn from(value : CarvedPumpkin) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:pink_stained_glass_pane` block.
pub mod pink_stained_glass_pane {
    /// `minecraft:pink_stained_glass_pane` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct PinkStainedGlassPane {
        /// `east` state.
        pub east : bool,
        /// `north` state.
        pub north : bool,
        /// `west` state.
        pub west : bool,
        /// `south` state.
        pub south : bool,
        /// `waterlogged` state.
        pub waterlogged : bool,
    }
    impl super::Block for PinkStainedGlassPane {
    }
    impl From<PinkStainedGlassPane> for super::BlockState {
        fn from(value : PinkStainedGlassPane) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:cracked_nether_bricks` block.
pub mod cracked_nether_bricks {
    /// `minecraft:cracked_nether_bricks` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct CrackedNetherBricks;
    impl super::Block for CrackedNetherBricks {
    }
    impl From<CrackedNetherBricks> for super::BlockState {
        fn from(value : CrackedNetherBricks) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:green_stained_glass` block.
pub mod green_stained_glass {
    /// `minecraft:green_stained_glass` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct GreenStainedGlass;
    impl super::Block for GreenStainedGlass {
    }
    impl From<GreenStainedGlass> for super::BlockState {
        fn from(value : GreenStainedGlass) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:purple_glazed_terracotta` block.
pub mod purple_glazed_terracotta {
    /// `minecraft:purple_glazed_terracotta` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct PurpleGlazedTerracotta {
        /// `facing` state.
        pub facing : Facing,
    }
    /// `facing` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Facing {
        /// `north` variant.
        North,
        /// `south` variant.
        South,
        /// `west` variant.
        West,
        /// `east` variant.
        East,
    }
    impl super::Block for PurpleGlazedTerracotta {
    }
    impl From<PurpleGlazedTerracotta> for super::BlockState {
        fn from(value : PurpleGlazedTerracotta) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:polished_andesite_slab` block.
pub mod polished_andesite_slab {
    /// `minecraft:polished_andesite_slab` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct PolishedAndesiteSlab {
        /// `type` state.
        pub kind : Kind,
        /// `waterlogged` state.
        pub waterlogged : bool,
    }
    /// `type` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Kind {
        /// `top` variant.
        Top,
        /// `bottom` variant.
        Bottom,
        /// `double` variant.
        Double,
    }
    impl super::Block for PolishedAndesiteSlab {
    }
    impl From<PolishedAndesiteSlab> for super::BlockState {
        fn from(value : PolishedAndesiteSlab) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:yellow_concrete` block.
pub mod yellow_concrete {
    /// `minecraft:yellow_concrete` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct YellowConcrete;
    impl super::Block for YellowConcrete {
    }
    impl From<YellowConcrete> for super::BlockState {
        fn from(value : YellowConcrete) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:exposed_copper_grate` block.
pub mod exposed_copper_grate {
    /// `minecraft:exposed_copper_grate` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct ExposedCopperGrate {
        /// `waterlogged` state.
        pub waterlogged : bool,
    }
    impl super::Block for ExposedCopperGrate {
    }
    impl From<ExposedCopperGrate> for super::BlockState {
        fn from(value : ExposedCopperGrate) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:magenta_shulker_box` block.
pub mod magenta_shulker_box {
    /// `minecraft:magenta_shulker_box` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct MagentaShulkerBox {
        /// `facing` state.
        pub facing : Facing,
    }
    /// `facing` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Facing {
        /// `north` variant.
        North,
        /// `east` variant.
        East,
        /// `south` variant.
        South,
        /// `west` variant.
        West,
        /// `up` variant.
        Up,
        /// `down` variant.
        Down,
    }
    impl super::Block for MagentaShulkerBox {
    }
    impl From<MagentaShulkerBox> for super::BlockState {
        fn from(value : MagentaShulkerBox) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:orange_concrete` block.
pub mod orange_concrete {
    /// `minecraft:orange_concrete` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct OrangeConcrete;
    impl super::Block for OrangeConcrete {
    }
    impl From<OrangeConcrete> for super::BlockState {
        fn from(value : OrangeConcrete) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:birch_stairs` block.
pub mod birch_stairs {
    /// `minecraft:birch_stairs` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct BirchStairs {
        /// `waterlogged` state.
        pub waterlogged : bool,
        /// `shape` state.
        pub shape : Shape,
        /// `facing` state.
        pub facing : Facing,
        /// `half` state.
        pub half : Half,
    }
    /// `shape` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Shape {
        /// `straight` variant.
        Straight,
        /// `inner_left` variant.
        InnerLeft,
        /// `inner_right` variant.
        InnerRight,
        /// `outer_left` variant.
        OuterLeft,
        /// `outer_right` variant.
        OuterRight,
    }
    /// `facing` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Facing {
        /// `north` variant.
        North,
        /// `south` variant.
        South,
        /// `west` variant.
        West,
        /// `east` variant.
        East,
    }
    /// `half` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Half {
        /// `top` variant.
        Top,
        /// `bottom` variant.
        Bottom,
    }
    impl super::Block for BirchStairs {
    }
    impl From<BirchStairs> for super::BlockState {
        fn from(value : BirchStairs) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:pale_oak_door` block.
pub mod pale_oak_door {
    /// `minecraft:pale_oak_door` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct PaleOakDoor {
        /// `facing` state.
        pub facing : Facing,
        /// `open` state.
        pub open : bool,
        /// `half` state.
        pub half : Half,
        /// `powered` state.
        pub powered : bool,
        /// `hinge` state.
        pub hinge : Hinge,
    }
    /// `facing` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Facing {
        /// `north` variant.
        North,
        /// `south` variant.
        South,
        /// `west` variant.
        West,
        /// `east` variant.
        East,
    }
    /// `half` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Half {
        /// `upper` variant.
        Upper,
        /// `lower` variant.
        Lower,
    }
    /// `hinge` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Hinge {
        /// `left` variant.
        Left,
        /// `right` variant.
        Right,
    }
    impl super::Block for PaleOakDoor {
    }
    impl From<PaleOakDoor> for super::BlockState {
        fn from(value : PaleOakDoor) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:wither_skeleton_skull` block.
pub mod wither_skeleton_skull {
    /// `minecraft:wither_skeleton_skull` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct WitherSkeletonSkull {
        /// `rotation` state.
        pub rotation : Rotation,
        /// `powered` state.
        pub powered : bool,
    }
    /// `rotation` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Rotation {
        /// `0` variant.
        N0,
        /// `1` variant.
        N1,
        /// `2` variant.
        N2,
        /// `3` variant.
        N3,
        /// `4` variant.
        N4,
        /// `5` variant.
        N5,
        /// `6` variant.
        N6,
        /// `7` variant.
        N7,
        /// `8` variant.
        N8,
        /// `9` variant.
        N9,
        /// `10` variant.
        N10,
        /// `11` variant.
        N11,
        /// `12` variant.
        N12,
        /// `13` variant.
        N13,
        /// `14` variant.
        N14,
        /// `15` variant.
        N15,
    }
    impl super::Block for WitherSkeletonSkull {
    }
    impl From<WitherSkeletonSkull> for super::BlockState {
        fn from(value : WitherSkeletonSkull) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:purple_candle_cake` block.
pub mod purple_candle_cake {
    /// `minecraft:purple_candle_cake` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct PurpleCandleCake {
        /// `lit` state.
        pub lit : bool,
    }
    impl super::Block for PurpleCandleCake {
    }
    impl From<PurpleCandleCake> for super::BlockState {
        fn from(value : PurpleCandleCake) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:cut_copper_slab` block.
pub mod cut_copper_slab {
    /// `minecraft:cut_copper_slab` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct CutCopperSlab {
        /// `waterlogged` state.
        pub waterlogged : bool,
        /// `type` state.
        pub kind : Kind,
    }
    /// `type` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Kind {
        /// `top` variant.
        Top,
        /// `bottom` variant.
        Bottom,
        /// `double` variant.
        Double,
    }
    impl super::Block for CutCopperSlab {
    }
    impl From<CutCopperSlab> for super::BlockState {
        fn from(value : CutCopperSlab) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:sculk_shrieker` block.
pub mod sculk_shrieker {
    /// `minecraft:sculk_shrieker` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct SculkShrieker {
        /// `can_summon` state.
        pub can_summon : bool,
        /// `shrieking` state.
        pub shrieking : bool,
        /// `waterlogged` state.
        pub waterlogged : bool,
    }
    impl super::Block for SculkShrieker {
    }
    impl From<SculkShrieker> for super::BlockState {
        fn from(value : SculkShrieker) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:hanging_roots` block.
pub mod hanging_roots {
    /// `minecraft:hanging_roots` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct HangingRoots {
        /// `waterlogged` state.
        pub waterlogged : bool,
    }
    impl super::Block for HangingRoots {
    }
    impl From<HangingRoots> for super::BlockState {
        fn from(value : HangingRoots) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:cocoa` block.
pub mod cocoa {
    /// `minecraft:cocoa` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct Cocoa {
        /// `age` state.
        pub age : Age,
        /// `facing` state.
        pub facing : Facing,
    }
    /// `age` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Age {
        /// `0` variant.
        N0,
        /// `1` variant.
        N1,
        /// `2` variant.
        N2,
    }
    /// `facing` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Facing {
        /// `north` variant.
        North,
        /// `south` variant.
        South,
        /// `west` variant.
        West,
        /// `east` variant.
        East,
    }
    impl super::Block for Cocoa {
    }
    impl From<Cocoa> for super::BlockState {
        fn from(value : Cocoa) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:tall_dry_grass` block.
pub mod tall_dry_grass {
    /// `minecraft:tall_dry_grass` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct TallDryGrass;
    impl super::Block for TallDryGrass {
    }
    impl From<TallDryGrass> for super::BlockState {
        fn from(value : TallDryGrass) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:spruce_leaves` block.
pub mod spruce_leaves {
    /// `minecraft:spruce_leaves` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct SpruceLeaves {
        /// `waterlogged` state.
        pub waterlogged : bool,
        /// `distance` state.
        pub distance : Distance,
        /// `persistent` state.
        pub persistent : bool,
    }
    /// `distance` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Distance {
        /// `1` variant.
        N1,
        /// `2` variant.
        N2,
        /// `3` variant.
        N3,
        /// `4` variant.
        N4,
        /// `5` variant.
        N5,
        /// `6` variant.
        N6,
        /// `7` variant.
        N7,
    }
    impl super::Block for SpruceLeaves {
    }
    impl From<SpruceLeaves> for super::BlockState {
        fn from(value : SpruceLeaves) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:blackstone_wall` block.
pub mod blackstone_wall {
    /// `minecraft:blackstone_wall` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct BlackstoneWall {
        /// `south` state.
        pub south : South,
        /// `up` state.
        pub up : bool,
        /// `waterlogged` state.
        pub waterlogged : bool,
        /// `west` state.
        pub west : West,
        /// `north` state.
        pub north : North,
        /// `east` state.
        pub east : East,
    }
    /// `south` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum South {
        /// `none` variant.
        None,
        /// `low` variant.
        Low,
        /// `tall` variant.
        Tall,
    }
    /// `west` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum West {
        /// `none` variant.
        None,
        /// `low` variant.
        Low,
        /// `tall` variant.
        Tall,
    }
    /// `north` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum North {
        /// `none` variant.
        None,
        /// `low` variant.
        Low,
        /// `tall` variant.
        Tall,
    }
    /// `east` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum East {
        /// `none` variant.
        None,
        /// `low` variant.
        Low,
        /// `tall` variant.
        Tall,
    }
    impl super::Block for BlackstoneWall {
    }
    impl From<BlackstoneWall> for super::BlockState {
        fn from(value : BlackstoneWall) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:crimson_fence` block.
pub mod crimson_fence {
    /// `minecraft:crimson_fence` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct CrimsonFence {
        /// `east` state.
        pub east : bool,
        /// `waterlogged` state.
        pub waterlogged : bool,
        /// `south` state.
        pub south : bool,
        /// `north` state.
        pub north : bool,
        /// `west` state.
        pub west : bool,
    }
    impl super::Block for CrimsonFence {
    }
    impl From<CrimsonFence> for super::BlockState {
        fn from(value : CrimsonFence) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:stripped_jungle_wood` block.
pub mod stripped_jungle_wood {
    /// `minecraft:stripped_jungle_wood` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct StrippedJungleWood {
        /// `axis` state.
        pub axis : Axis,
    }
    /// `axis` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Axis {
        /// `x` variant.
        X,
        /// `y` variant.
        Y,
        /// `z` variant.
        Z,
    }
    impl super::Block for StrippedJungleWood {
    }
    impl From<StrippedJungleWood> for super::BlockState {
        fn from(value : StrippedJungleWood) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:smithing_table` block.
pub mod smithing_table {
    /// `minecraft:smithing_table` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct SmithingTable;
    impl super::Block for SmithingTable {
    }
    impl From<SmithingTable> for super::BlockState {
        fn from(value : SmithingTable) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:brown_wall_banner` block.
pub mod brown_wall_banner {
    /// `minecraft:brown_wall_banner` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct BrownWallBanner {
        /// `facing` state.
        pub facing : Facing,
    }
    /// `facing` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Facing {
        /// `north` variant.
        North,
        /// `south` variant.
        South,
        /// `west` variant.
        West,
        /// `east` variant.
        East,
    }
    impl super::Block for BrownWallBanner {
    }
    impl From<BrownWallBanner> for super::BlockState {
        fn from(value : BrownWallBanner) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:oxidized_copper` block.
pub mod oxidized_copper {
    /// `minecraft:oxidized_copper` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct OxidizedCopper;
    impl super::Block for OxidizedCopper {
    }
    impl From<OxidizedCopper> for super::BlockState {
        fn from(value : OxidizedCopper) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:yellow_wall_banner` block.
pub mod yellow_wall_banner {
    /// `minecraft:yellow_wall_banner` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct YellowWallBanner {
        /// `facing` state.
        pub facing : Facing,
    }
    /// `facing` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Facing {
        /// `north` variant.
        North,
        /// `south` variant.
        South,
        /// `west` variant.
        West,
        /// `east` variant.
        East,
    }
    impl super::Block for YellowWallBanner {
    }
    impl From<YellowWallBanner> for super::BlockState {
        fn from(value : YellowWallBanner) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:potted_azalea_bush` block.
pub mod potted_azalea_bush {
    /// `minecraft:potted_azalea_bush` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct PottedAzaleaBush;
    impl super::Block for PottedAzaleaBush {
    }
    impl From<PottedAzaleaBush> for super::BlockState {
        fn from(value : PottedAzaleaBush) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:pale_oak_hanging_sign` block.
pub mod pale_oak_hanging_sign {
    /// `minecraft:pale_oak_hanging_sign` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct PaleOakHangingSign {
        /// `waterlogged` state.
        pub waterlogged : bool,
        /// `attached` state.
        pub attached : bool,
        /// `rotation` state.
        pub rotation : Rotation,
    }
    /// `rotation` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Rotation {
        /// `0` variant.
        N0,
        /// `1` variant.
        N1,
        /// `2` variant.
        N2,
        /// `3` variant.
        N3,
        /// `4` variant.
        N4,
        /// `5` variant.
        N5,
        /// `6` variant.
        N6,
        /// `7` variant.
        N7,
        /// `8` variant.
        N8,
        /// `9` variant.
        N9,
        /// `10` variant.
        N10,
        /// `11` variant.
        N11,
        /// `12` variant.
        N12,
        /// `13` variant.
        N13,
        /// `14` variant.
        N14,
        /// `15` variant.
        N15,
    }
    impl super::Block for PaleOakHangingSign {
    }
    impl From<PaleOakHangingSign> for super::BlockState {
        fn from(value : PaleOakHangingSign) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:potted_jungle_sapling` block.
pub mod potted_jungle_sapling {
    /// `minecraft:potted_jungle_sapling` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct PottedJungleSapling;
    impl super::Block for PottedJungleSapling {
    }
    impl From<PottedJungleSapling> for super::BlockState {
        fn from(value : PottedJungleSapling) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:light_gray_stained_glass` block.
pub mod light_gray_stained_glass {
    /// `minecraft:light_gray_stained_glass` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct LightGrayStainedGlass;
    impl super::Block for LightGrayStainedGlass {
    }
    impl From<LightGrayStainedGlass> for super::BlockState {
        fn from(value : LightGrayStainedGlass) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:smooth_red_sandstone_slab` block.
pub mod smooth_red_sandstone_slab {
    /// `minecraft:smooth_red_sandstone_slab` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct SmoothRedSandstoneSlab {
        /// `type` state.
        pub kind : Kind,
        /// `waterlogged` state.
        pub waterlogged : bool,
    }
    /// `type` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Kind {
        /// `top` variant.
        Top,
        /// `bottom` variant.
        Bottom,
        /// `double` variant.
        Double,
    }
    impl super::Block for SmoothRedSandstoneSlab {
    }
    impl From<SmoothRedSandstoneSlab> for super::BlockState {
        fn from(value : SmoothRedSandstoneSlab) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:torchflower` block.
pub mod torchflower {
    /// `minecraft:torchflower` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct Torchflower;
    impl super::Block for Torchflower {
    }
    impl From<Torchflower> for super::BlockState {
        fn from(value : Torchflower) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:mangrove_hanging_sign` block.
pub mod mangrove_hanging_sign {
    /// `minecraft:mangrove_hanging_sign` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct MangroveHangingSign {
        /// `attached` state.
        pub attached : bool,
        /// `rotation` state.
        pub rotation : Rotation,
        /// `waterlogged` state.
        pub waterlogged : bool,
    }
    /// `rotation` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Rotation {
        /// `0` variant.
        N0,
        /// `1` variant.
        N1,
        /// `2` variant.
        N2,
        /// `3` variant.
        N3,
        /// `4` variant.
        N4,
        /// `5` variant.
        N5,
        /// `6` variant.
        N6,
        /// `7` variant.
        N7,
        /// `8` variant.
        N8,
        /// `9` variant.
        N9,
        /// `10` variant.
        N10,
        /// `11` variant.
        N11,
        /// `12` variant.
        N12,
        /// `13` variant.
        N13,
        /// `14` variant.
        N14,
        /// `15` variant.
        N15,
    }
    impl super::Block for MangroveHangingSign {
    }
    impl From<MangroveHangingSign> for super::BlockState {
        fn from(value : MangroveHangingSign) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:mangrove_leaves` block.
pub mod mangrove_leaves {
    /// `minecraft:mangrove_leaves` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct MangroveLeaves {
        /// `distance` state.
        pub distance : Distance,
        /// `persistent` state.
        pub persistent : bool,
        /// `waterlogged` state.
        pub waterlogged : bool,
    }
    /// `distance` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Distance {
        /// `1` variant.
        N1,
        /// `2` variant.
        N2,
        /// `3` variant.
        N3,
        /// `4` variant.
        N4,
        /// `5` variant.
        N5,
        /// `6` variant.
        N6,
        /// `7` variant.
        N7,
    }
    impl super::Block for MangroveLeaves {
    }
    impl From<MangroveLeaves> for super::BlockState {
        fn from(value : MangroveLeaves) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:orange_glazed_terracotta` block.
pub mod orange_glazed_terracotta {
    /// `minecraft:orange_glazed_terracotta` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct OrangeGlazedTerracotta {
        /// `facing` state.
        pub facing : Facing,
    }
    /// `facing` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Facing {
        /// `north` variant.
        North,
        /// `south` variant.
        South,
        /// `west` variant.
        West,
        /// `east` variant.
        East,
    }
    impl super::Block for OrangeGlazedTerracotta {
    }
    impl From<OrangeGlazedTerracotta> for super::BlockState {
        fn from(value : OrangeGlazedTerracotta) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:crimson_sign` block.
pub mod crimson_sign {
    /// `minecraft:crimson_sign` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct CrimsonSign {
        /// `rotation` state.
        pub rotation : Rotation,
        /// `waterlogged` state.
        pub waterlogged : bool,
    }
    /// `rotation` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Rotation {
        /// `0` variant.
        N0,
        /// `1` variant.
        N1,
        /// `2` variant.
        N2,
        /// `3` variant.
        N3,
        /// `4` variant.
        N4,
        /// `5` variant.
        N5,
        /// `6` variant.
        N6,
        /// `7` variant.
        N7,
        /// `8` variant.
        N8,
        /// `9` variant.
        N9,
        /// `10` variant.
        N10,
        /// `11` variant.
        N11,
        /// `12` variant.
        N12,
        /// `13` variant.
        N13,
        /// `14` variant.
        N14,
        /// `15` variant.
        N15,
    }
    impl super::Block for CrimsonSign {
    }
    impl From<CrimsonSign> for super::BlockState {
        fn from(value : CrimsonSign) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:white_bed` block.
pub mod white_bed {
    /// `minecraft:white_bed` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct WhiteBed {
        /// `facing` state.
        pub facing : Facing,
        /// `occupied` state.
        pub occupied : bool,
        /// `part` state.
        pub part : Part,
    }
    /// `facing` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Facing {
        /// `north` variant.
        North,
        /// `south` variant.
        South,
        /// `west` variant.
        West,
        /// `east` variant.
        East,
    }
    /// `part` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Part {
        /// `head` variant.
        Head,
        /// `foot` variant.
        Foot,
    }
    impl super::Block for WhiteBed {
    }
    impl From<WhiteBed> for super::BlockState {
        fn from(value : WhiteBed) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:pale_oak_sign` block.
pub mod pale_oak_sign {
    /// `minecraft:pale_oak_sign` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct PaleOakSign {
        /// `waterlogged` state.
        pub waterlogged : bool,
        /// `rotation` state.
        pub rotation : Rotation,
    }
    /// `rotation` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Rotation {
        /// `0` variant.
        N0,
        /// `1` variant.
        N1,
        /// `2` variant.
        N2,
        /// `3` variant.
        N3,
        /// `4` variant.
        N4,
        /// `5` variant.
        N5,
        /// `6` variant.
        N6,
        /// `7` variant.
        N7,
        /// `8` variant.
        N8,
        /// `9` variant.
        N9,
        /// `10` variant.
        N10,
        /// `11` variant.
        N11,
        /// `12` variant.
        N12,
        /// `13` variant.
        N13,
        /// `14` variant.
        N14,
        /// `15` variant.
        N15,
    }
    impl super::Block for PaleOakSign {
    }
    impl From<PaleOakSign> for super::BlockState {
        fn from(value : PaleOakSign) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:blue_stained_glass` block.
pub mod blue_stained_glass {
    /// `minecraft:blue_stained_glass` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct BlueStainedGlass;
    impl super::Block for BlueStainedGlass {
    }
    impl From<BlueStainedGlass> for super::BlockState {
        fn from(value : BlueStainedGlass) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:mangrove_fence_gate` block.
pub mod mangrove_fence_gate {
    /// `minecraft:mangrove_fence_gate` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct MangroveFenceGate {
        /// `in_wall` state.
        pub in_wall : bool,
        /// `powered` state.
        pub powered : bool,
        /// `facing` state.
        pub facing : Facing,
        /// `open` state.
        pub open : bool,
    }
    /// `facing` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Facing {
        /// `north` variant.
        North,
        /// `south` variant.
        South,
        /// `west` variant.
        West,
        /// `east` variant.
        East,
    }
    impl super::Block for MangroveFenceGate {
    }
    impl From<MangroveFenceGate> for super::BlockState {
        fn from(value : MangroveFenceGate) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:dead_horn_coral_block` block.
pub mod dead_horn_coral_block {
    /// `minecraft:dead_horn_coral_block` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct DeadHornCoralBlock;
    impl super::Block for DeadHornCoralBlock {
    }
    impl From<DeadHornCoralBlock> for super::BlockState {
        fn from(value : DeadHornCoralBlock) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:bamboo_pressure_plate` block.
pub mod bamboo_pressure_plate {
    /// `minecraft:bamboo_pressure_plate` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct BambooPressurePlate {
        /// `powered` state.
        pub powered : bool,
    }
    impl super::Block for BambooPressurePlate {
    }
    impl From<BambooPressurePlate> for super::BlockState {
        fn from(value : BambooPressurePlate) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:orange_bed` block.
pub mod orange_bed {
    /// `minecraft:orange_bed` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct OrangeBed {
        /// `occupied` state.
        pub occupied : bool,
        /// `facing` state.
        pub facing : Facing,
        /// `part` state.
        pub part : Part,
    }
    /// `facing` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Facing {
        /// `north` variant.
        North,
        /// `south` variant.
        South,
        /// `west` variant.
        West,
        /// `east` variant.
        East,
    }
    /// `part` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Part {
        /// `head` variant.
        Head,
        /// `foot` variant.
        Foot,
    }
    impl super::Block for OrangeBed {
    }
    impl From<OrangeBed> for super::BlockState {
        fn from(value : OrangeBed) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:mycelium` block.
pub mod mycelium {
    /// `minecraft:mycelium` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct Mycelium {
        /// `snowy` state.
        pub snowy : bool,
    }
    impl super::Block for Mycelium {
    }
    impl From<Mycelium> for super::BlockState {
        fn from(value : Mycelium) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:lime_stained_glass` block.
pub mod lime_stained_glass {
    /// `minecraft:lime_stained_glass` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct LimeStainedGlass;
    impl super::Block for LimeStainedGlass {
    }
    impl From<LimeStainedGlass> for super::BlockState {
        fn from(value : LimeStainedGlass) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:dark_oak_hanging_sign` block.
pub mod dark_oak_hanging_sign {
    /// `minecraft:dark_oak_hanging_sign` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct DarkOakHangingSign {
        /// `waterlogged` state.
        pub waterlogged : bool,
        /// `attached` state.
        pub attached : bool,
        /// `rotation` state.
        pub rotation : Rotation,
    }
    /// `rotation` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Rotation {
        /// `0` variant.
        N0,
        /// `1` variant.
        N1,
        /// `2` variant.
        N2,
        /// `3` variant.
        N3,
        /// `4` variant.
        N4,
        /// `5` variant.
        N5,
        /// `6` variant.
        N6,
        /// `7` variant.
        N7,
        /// `8` variant.
        N8,
        /// `9` variant.
        N9,
        /// `10` variant.
        N10,
        /// `11` variant.
        N11,
        /// `12` variant.
        N12,
        /// `13` variant.
        N13,
        /// `14` variant.
        N14,
        /// `15` variant.
        N15,
    }
    impl super::Block for DarkOakHangingSign {
    }
    impl From<DarkOakHangingSign> for super::BlockState {
        fn from(value : DarkOakHangingSign) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:weeping_vines` block.
pub mod weeping_vines {
    /// `minecraft:weeping_vines` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct WeepingVines {
        /// `age` state.
        pub age : Age,
    }
    /// `age` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Age {
        /// `0` variant.
        N0,
        /// `1` variant.
        N1,
        /// `2` variant.
        N2,
        /// `3` variant.
        N3,
        /// `4` variant.
        N4,
        /// `5` variant.
        N5,
        /// `6` variant.
        N6,
        /// `7` variant.
        N7,
        /// `8` variant.
        N8,
        /// `9` variant.
        N9,
        /// `10` variant.
        N10,
        /// `11` variant.
        N11,
        /// `12` variant.
        N12,
        /// `13` variant.
        N13,
        /// `14` variant.
        N14,
        /// `15` variant.
        N15,
        /// `16` variant.
        N16,
        /// `17` variant.
        N17,
        /// `18` variant.
        N18,
        /// `19` variant.
        N19,
        /// `20` variant.
        N20,
        /// `21` variant.
        N21,
        /// `22` variant.
        N22,
        /// `23` variant.
        N23,
        /// `24` variant.
        N24,
        /// `25` variant.
        N25,
    }
    impl super::Block for WeepingVines {
    }
    impl From<WeepingVines> for super::BlockState {
        fn from(value : WeepingVines) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:cracked_stone_bricks` block.
pub mod cracked_stone_bricks {
    /// `minecraft:cracked_stone_bricks` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct CrackedStoneBricks;
    impl super::Block for CrackedStoneBricks {
    }
    impl From<CrackedStoneBricks> for super::BlockState {
        fn from(value : CrackedStoneBricks) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:sculk_catalyst` block.
pub mod sculk_catalyst {
    /// `minecraft:sculk_catalyst` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct SculkCatalyst {
        /// `bloom` state.
        pub bloom : bool,
    }
    impl super::Block for SculkCatalyst {
    }
    impl From<SculkCatalyst> for super::BlockState {
        fn from(value : SculkCatalyst) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:tube_coral` block.
pub mod tube_coral {
    /// `minecraft:tube_coral` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct TubeCoral {
        /// `waterlogged` state.
        pub waterlogged : bool,
    }
    impl super::Block for TubeCoral {
    }
    impl From<TubeCoral> for super::BlockState {
        fn from(value : TubeCoral) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:waxed_weathered_cut_copper` block.
pub mod waxed_weathered_cut_copper {
    /// `minecraft:waxed_weathered_cut_copper` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct WaxedWeatheredCutCopper;
    impl super::Block for WaxedWeatheredCutCopper {
    }
    impl From<WaxedWeatheredCutCopper> for super::BlockState {
        fn from(value : WaxedWeatheredCutCopper) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:waxed_oxidized_copper_grate` block.
pub mod waxed_oxidized_copper_grate {
    /// `minecraft:waxed_oxidized_copper_grate` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct WaxedOxidizedCopperGrate {
        /// `waterlogged` state.
        pub waterlogged : bool,
    }
    impl super::Block for WaxedOxidizedCopperGrate {
    }
    impl From<WaxedOxidizedCopperGrate> for super::BlockState {
        fn from(value : WaxedOxidizedCopperGrate) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:creeper_head` block.
pub mod creeper_head {
    /// `minecraft:creeper_head` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct CreeperHead {
        /// `powered` state.
        pub powered : bool,
        /// `rotation` state.
        pub rotation : Rotation,
    }
    /// `rotation` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Rotation {
        /// `0` variant.
        N0,
        /// `1` variant.
        N1,
        /// `2` variant.
        N2,
        /// `3` variant.
        N3,
        /// `4` variant.
        N4,
        /// `5` variant.
        N5,
        /// `6` variant.
        N6,
        /// `7` variant.
        N7,
        /// `8` variant.
        N8,
        /// `9` variant.
        N9,
        /// `10` variant.
        N10,
        /// `11` variant.
        N11,
        /// `12` variant.
        N12,
        /// `13` variant.
        N13,
        /// `14` variant.
        N14,
        /// `15` variant.
        N15,
    }
    impl super::Block for CreeperHead {
    }
    impl From<CreeperHead> for super::BlockState {
        fn from(value : CreeperHead) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:dead_horn_coral_fan` block.
pub mod dead_horn_coral_fan {
    /// `minecraft:dead_horn_coral_fan` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct DeadHornCoralFan {
        /// `waterlogged` state.
        pub waterlogged : bool,
    }
    impl super::Block for DeadHornCoralFan {
    }
    impl From<DeadHornCoralFan> for super::BlockState {
        fn from(value : DeadHornCoralFan) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:cyan_concrete` block.
pub mod cyan_concrete {
    /// `minecraft:cyan_concrete` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct CyanConcrete;
    impl super::Block for CyanConcrete {
    }
    impl From<CyanConcrete> for super::BlockState {
        fn from(value : CyanConcrete) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:frosted_ice` block.
pub mod frosted_ice {
    /// `minecraft:frosted_ice` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct FrostedIce {
        /// `age` state.
        pub age : Age,
    }
    /// `age` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Age {
        /// `0` variant.
        N0,
        /// `1` variant.
        N1,
        /// `2` variant.
        N2,
        /// `3` variant.
        N3,
    }
    impl super::Block for FrostedIce {
    }
    impl From<FrostedIce> for super::BlockState {
        fn from(value : FrostedIce) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:jukebox` block.
pub mod jukebox {
    /// `minecraft:jukebox` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct Jukebox {
        /// `has_record` state.
        pub has_record : bool,
    }
    impl super::Block for Jukebox {
    }
    impl From<Jukebox> for super::BlockState {
        fn from(value : Jukebox) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:cave_vines` block.
pub mod cave_vines {
    /// `minecraft:cave_vines` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct CaveVines {
        /// `berries` state.
        pub berries : bool,
        /// `age` state.
        pub age : Age,
    }
    /// `age` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Age {
        /// `0` variant.
        N0,
        /// `1` variant.
        N1,
        /// `2` variant.
        N2,
        /// `3` variant.
        N3,
        /// `4` variant.
        N4,
        /// `5` variant.
        N5,
        /// `6` variant.
        N6,
        /// `7` variant.
        N7,
        /// `8` variant.
        N8,
        /// `9` variant.
        N9,
        /// `10` variant.
        N10,
        /// `11` variant.
        N11,
        /// `12` variant.
        N12,
        /// `13` variant.
        N13,
        /// `14` variant.
        N14,
        /// `15` variant.
        N15,
        /// `16` variant.
        N16,
        /// `17` variant.
        N17,
        /// `18` variant.
        N18,
        /// `19` variant.
        N19,
        /// `20` variant.
        N20,
        /// `21` variant.
        N21,
        /// `22` variant.
        N22,
        /// `23` variant.
        N23,
        /// `24` variant.
        N24,
        /// `25` variant.
        N25,
    }
    impl super::Block for CaveVines {
    }
    impl From<CaveVines> for super::BlockState {
        fn from(value : CaveVines) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:dead_tube_coral` block.
pub mod dead_tube_coral {
    /// `minecraft:dead_tube_coral` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct DeadTubeCoral {
        /// `waterlogged` state.
        pub waterlogged : bool,
    }
    impl super::Block for DeadTubeCoral {
    }
    impl From<DeadTubeCoral> for super::BlockState {
        fn from(value : DeadTubeCoral) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:polished_deepslate_wall` block.
pub mod polished_deepslate_wall {
    /// `minecraft:polished_deepslate_wall` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct PolishedDeepslateWall {
        /// `up` state.
        pub up : bool,
        /// `east` state.
        pub east : East,
        /// `north` state.
        pub north : North,
        /// `waterlogged` state.
        pub waterlogged : bool,
        /// `south` state.
        pub south : South,
        /// `west` state.
        pub west : West,
    }
    /// `east` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum East {
        /// `none` variant.
        None,
        /// `low` variant.
        Low,
        /// `tall` variant.
        Tall,
    }
    /// `north` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum North {
        /// `none` variant.
        None,
        /// `low` variant.
        Low,
        /// `tall` variant.
        Tall,
    }
    /// `south` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum South {
        /// `none` variant.
        None,
        /// `low` variant.
        Low,
        /// `tall` variant.
        Tall,
    }
    /// `west` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum West {
        /// `none` variant.
        None,
        /// `low` variant.
        Low,
        /// `tall` variant.
        Tall,
    }
    impl super::Block for PolishedDeepslateWall {
    }
    impl From<PolishedDeepslateWall> for super::BlockState {
        fn from(value : PolishedDeepslateWall) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:lime_shulker_box` block.
pub mod lime_shulker_box {
    /// `minecraft:lime_shulker_box` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct LimeShulkerBox {
        /// `facing` state.
        pub facing : Facing,
    }
    /// `facing` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Facing {
        /// `north` variant.
        North,
        /// `east` variant.
        East,
        /// `south` variant.
        South,
        /// `west` variant.
        West,
        /// `up` variant.
        Up,
        /// `down` variant.
        Down,
    }
    impl super::Block for LimeShulkerBox {
    }
    impl From<LimeShulkerBox> for super::BlockState {
        fn from(value : LimeShulkerBox) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:granite` block.
pub mod granite {
    /// `minecraft:granite` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct Granite;
    impl super::Block for Granite {
    }
    impl From<Granite> for super::BlockState {
        fn from(value : Granite) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:light_gray_bed` block.
pub mod light_gray_bed {
    /// `minecraft:light_gray_bed` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct LightGrayBed {
        /// `part` state.
        pub part : Part,
        /// `facing` state.
        pub facing : Facing,
        /// `occupied` state.
        pub occupied : bool,
    }
    /// `part` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Part {
        /// `head` variant.
        Head,
        /// `foot` variant.
        Foot,
    }
    /// `facing` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Facing {
        /// `north` variant.
        North,
        /// `south` variant.
        South,
        /// `west` variant.
        West,
        /// `east` variant.
        East,
    }
    impl super::Block for LightGrayBed {
    }
    impl From<LightGrayBed> for super::BlockState {
        fn from(value : LightGrayBed) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:fern` block.
pub mod fern {
    /// `minecraft:fern` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct Fern;
    impl super::Block for Fern {
    }
    impl From<Fern> for super::BlockState {
        fn from(value : Fern) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:lily_pad` block.
pub mod lily_pad {
    /// `minecraft:lily_pad` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct LilyPad;
    impl super::Block for LilyPad {
    }
    impl From<LilyPad> for super::BlockState {
        fn from(value : LilyPad) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:orange_candle` block.
pub mod orange_candle {
    /// `minecraft:orange_candle` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct OrangeCandle {
        /// `lit` state.
        pub lit : bool,
        /// `waterlogged` state.
        pub waterlogged : bool,
        /// `candles` state.
        pub candles : Candles,
    }
    /// `candles` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Candles {
        /// `1` variant.
        N1,
        /// `2` variant.
        N2,
        /// `3` variant.
        N3,
        /// `4` variant.
        N4,
    }
    impl super::Block for OrangeCandle {
    }
    impl From<OrangeCandle> for super::BlockState {
        fn from(value : OrangeCandle) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:wildflowers` block.
pub mod wildflowers {
    /// `minecraft:wildflowers` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct Wildflowers {
        /// `facing` state.
        pub facing : Facing,
        /// `flower_amount` state.
        pub flower_amount : FlowerAmount,
    }
    /// `facing` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Facing {
        /// `north` variant.
        North,
        /// `south` variant.
        South,
        /// `west` variant.
        West,
        /// `east` variant.
        East,
    }
    /// `flower_amount` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum FlowerAmount {
        /// `1` variant.
        N1,
        /// `2` variant.
        N2,
        /// `3` variant.
        N3,
        /// `4` variant.
        N4,
    }
    impl super::Block for Wildflowers {
    }
    impl From<Wildflowers> for super::BlockState {
        fn from(value : Wildflowers) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:acacia_stairs` block.
pub mod acacia_stairs {
    /// `minecraft:acacia_stairs` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct AcaciaStairs {
        /// `shape` state.
        pub shape : Shape,
        /// `facing` state.
        pub facing : Facing,
        /// `half` state.
        pub half : Half,
        /// `waterlogged` state.
        pub waterlogged : bool,
    }
    /// `shape` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Shape {
        /// `straight` variant.
        Straight,
        /// `inner_left` variant.
        InnerLeft,
        /// `inner_right` variant.
        InnerRight,
        /// `outer_left` variant.
        OuterLeft,
        /// `outer_right` variant.
        OuterRight,
    }
    /// `facing` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Facing {
        /// `north` variant.
        North,
        /// `south` variant.
        South,
        /// `west` variant.
        West,
        /// `east` variant.
        East,
    }
    /// `half` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Half {
        /// `top` variant.
        Top,
        /// `bottom` variant.
        Bottom,
    }
    impl super::Block for AcaciaStairs {
    }
    impl From<AcaciaStairs> for super::BlockState {
        fn from(value : AcaciaStairs) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:snow` block.
pub mod snow {
    /// `minecraft:snow` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct Snow {
        /// `layers` state.
        pub layers : Layers,
    }
    /// `layers` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Layers {
        /// `1` variant.
        N1,
        /// `2` variant.
        N2,
        /// `3` variant.
        N3,
        /// `4` variant.
        N4,
        /// `5` variant.
        N5,
        /// `6` variant.
        N6,
        /// `7` variant.
        N7,
        /// `8` variant.
        N8,
    }
    impl super::Block for Snow {
    }
    impl From<Snow> for super::BlockState {
        fn from(value : Snow) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:horn_coral` block.
pub mod horn_coral {
    /// `minecraft:horn_coral` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct HornCoral {
        /// `waterlogged` state.
        pub waterlogged : bool,
    }
    impl super::Block for HornCoral {
    }
    impl From<HornCoral> for super::BlockState {
        fn from(value : HornCoral) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:yellow_bed` block.
pub mod yellow_bed {
    /// `minecraft:yellow_bed` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct YellowBed {
        /// `facing` state.
        pub facing : Facing,
        /// `part` state.
        pub part : Part,
        /// `occupied` state.
        pub occupied : bool,
    }
    /// `facing` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Facing {
        /// `north` variant.
        North,
        /// `south` variant.
        South,
        /// `west` variant.
        West,
        /// `east` variant.
        East,
    }
    /// `part` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Part {
        /// `head` variant.
        Head,
        /// `foot` variant.
        Foot,
    }
    impl super::Block for YellowBed {
    }
    impl From<YellowBed> for super::BlockState {
        fn from(value : YellowBed) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:dirt` block.
pub mod dirt {
    /// `minecraft:dirt` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct Dirt;
    impl super::Block for Dirt {
    }
    impl From<Dirt> for super::BlockState {
        fn from(value : Dirt) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:magenta_banner` block.
pub mod magenta_banner {
    /// `minecraft:magenta_banner` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct MagentaBanner {
        /// `rotation` state.
        pub rotation : Rotation,
    }
    /// `rotation` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Rotation {
        /// `0` variant.
        N0,
        /// `1` variant.
        N1,
        /// `2` variant.
        N2,
        /// `3` variant.
        N3,
        /// `4` variant.
        N4,
        /// `5` variant.
        N5,
        /// `6` variant.
        N6,
        /// `7` variant.
        N7,
        /// `8` variant.
        N8,
        /// `9` variant.
        N9,
        /// `10` variant.
        N10,
        /// `11` variant.
        N11,
        /// `12` variant.
        N12,
        /// `13` variant.
        N13,
        /// `14` variant.
        N14,
        /// `15` variant.
        N15,
    }
    impl super::Block for MagentaBanner {
    }
    impl From<MagentaBanner> for super::BlockState {
        fn from(value : MagentaBanner) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:horn_coral_wall_fan` block.
pub mod horn_coral_wall_fan {
    /// `minecraft:horn_coral_wall_fan` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct HornCoralWallFan {
        /// `facing` state.
        pub facing : Facing,
        /// `waterlogged` state.
        pub waterlogged : bool,
    }
    /// `facing` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Facing {
        /// `north` variant.
        North,
        /// `south` variant.
        South,
        /// `west` variant.
        West,
        /// `east` variant.
        East,
    }
    impl super::Block for HornCoralWallFan {
    }
    impl From<HornCoralWallFan> for super::BlockState {
        fn from(value : HornCoralWallFan) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:resin_bricks` block.
pub mod resin_bricks {
    /// `minecraft:resin_bricks` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct ResinBricks;
    impl super::Block for ResinBricks {
    }
    impl From<ResinBricks> for super::BlockState {
        fn from(value : ResinBricks) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:gray_banner` block.
pub mod gray_banner {
    /// `minecraft:gray_banner` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct GrayBanner {
        /// `rotation` state.
        pub rotation : Rotation,
    }
    /// `rotation` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Rotation {
        /// `0` variant.
        N0,
        /// `1` variant.
        N1,
        /// `2` variant.
        N2,
        /// `3` variant.
        N3,
        /// `4` variant.
        N4,
        /// `5` variant.
        N5,
        /// `6` variant.
        N6,
        /// `7` variant.
        N7,
        /// `8` variant.
        N8,
        /// `9` variant.
        N9,
        /// `10` variant.
        N10,
        /// `11` variant.
        N11,
        /// `12` variant.
        N12,
        /// `13` variant.
        N13,
        /// `14` variant.
        N14,
        /// `15` variant.
        N15,
    }
    impl super::Block for GrayBanner {
    }
    impl From<GrayBanner> for super::BlockState {
        fn from(value : GrayBanner) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:stone_bricks` block.
pub mod stone_bricks {
    /// `minecraft:stone_bricks` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct StoneBricks;
    impl super::Block for StoneBricks {
    }
    impl From<StoneBricks> for super::BlockState {
        fn from(value : StoneBricks) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:brown_concrete` block.
pub mod brown_concrete {
    /// `minecraft:brown_concrete` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct BrownConcrete;
    impl super::Block for BrownConcrete {
    }
    impl From<BrownConcrete> for super::BlockState {
        fn from(value : BrownConcrete) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:cherry_fence_gate` block.
pub mod cherry_fence_gate {
    /// `minecraft:cherry_fence_gate` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct CherryFenceGate {
        /// `powered` state.
        pub powered : bool,
        /// `facing` state.
        pub facing : Facing,
        /// `open` state.
        pub open : bool,
        /// `in_wall` state.
        pub in_wall : bool,
    }
    /// `facing` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Facing {
        /// `north` variant.
        North,
        /// `south` variant.
        South,
        /// `west` variant.
        West,
        /// `east` variant.
        East,
    }
    impl super::Block for CherryFenceGate {
    }
    impl From<CherryFenceGate> for super::BlockState {
        fn from(value : CherryFenceGate) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:tall_seagrass` block.
pub mod tall_seagrass {
    /// `minecraft:tall_seagrass` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct TallSeagrass {
        /// `half` state.
        pub half : Half,
    }
    /// `half` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Half {
        /// `upper` variant.
        Upper,
        /// `lower` variant.
        Lower,
    }
    impl super::Block for TallSeagrass {
    }
    impl From<TallSeagrass> for super::BlockState {
        fn from(value : TallSeagrass) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:iron_bars` block.
pub mod iron_bars {
    /// `minecraft:iron_bars` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct IronBars {
        /// `east` state.
        pub east : bool,
        /// `waterlogged` state.
        pub waterlogged : bool,
        /// `west` state.
        pub west : bool,
        /// `south` state.
        pub south : bool,
        /// `north` state.
        pub north : bool,
    }
    impl super::Block for IronBars {
    }
    impl From<IronBars> for super::BlockState {
        fn from(value : IronBars) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:bell` block.
pub mod bell {
    /// `minecraft:bell` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct Bell {
        /// `facing` state.
        pub facing : Facing,
        /// `attachment` state.
        pub attachment : Attachment,
        /// `powered` state.
        pub powered : bool,
    }
    /// `facing` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Facing {
        /// `north` variant.
        North,
        /// `south` variant.
        South,
        /// `west` variant.
        West,
        /// `east` variant.
        East,
    }
    /// `attachment` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Attachment {
        /// `floor` variant.
        Floor,
        /// `ceiling` variant.
        Ceiling,
        /// `single_wall` variant.
        SingleWall,
        /// `double_wall` variant.
        DoubleWall,
    }
    impl super::Block for Bell {
    }
    impl From<Bell> for super::BlockState {
        fn from(value : Bell) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:cherry_sapling` block.
pub mod cherry_sapling {
    /// `minecraft:cherry_sapling` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct CherrySapling {
        /// `stage` state.
        pub stage : Stage,
    }
    /// `stage` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Stage {
        /// `0` variant.
        N0,
        /// `1` variant.
        N1,
    }
    impl super::Block for CherrySapling {
    }
    impl From<CherrySapling> for super::BlockState {
        fn from(value : CherrySapling) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:muddy_mangrove_roots` block.
pub mod muddy_mangrove_roots {
    /// `minecraft:muddy_mangrove_roots` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct MuddyMangroveRoots {
        /// `axis` state.
        pub axis : Axis,
    }
    /// `axis` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Axis {
        /// `x` variant.
        X,
        /// `y` variant.
        Y,
        /// `z` variant.
        Z,
    }
    impl super::Block for MuddyMangroveRoots {
    }
    impl From<MuddyMangroveRoots> for super::BlockState {
        fn from(value : MuddyMangroveRoots) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:smooth_stone` block.
pub mod smooth_stone {
    /// `minecraft:smooth_stone` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct SmoothStone;
    impl super::Block for SmoothStone {
    }
    impl From<SmoothStone> for super::BlockState {
        fn from(value : SmoothStone) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:mangrove_sign` block.
pub mod mangrove_sign {
    /// `minecraft:mangrove_sign` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct MangroveSign {
        /// `rotation` state.
        pub rotation : Rotation,
        /// `waterlogged` state.
        pub waterlogged : bool,
    }
    /// `rotation` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Rotation {
        /// `0` variant.
        N0,
        /// `1` variant.
        N1,
        /// `2` variant.
        N2,
        /// `3` variant.
        N3,
        /// `4` variant.
        N4,
        /// `5` variant.
        N5,
        /// `6` variant.
        N6,
        /// `7` variant.
        N7,
        /// `8` variant.
        N8,
        /// `9` variant.
        N9,
        /// `10` variant.
        N10,
        /// `11` variant.
        N11,
        /// `12` variant.
        N12,
        /// `13` variant.
        N13,
        /// `14` variant.
        N14,
        /// `15` variant.
        N15,
    }
    impl super::Block for MangroveSign {
    }
    impl From<MangroveSign> for super::BlockState {
        fn from(value : MangroveSign) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:stripped_spruce_log` block.
pub mod stripped_spruce_log {
    /// `minecraft:stripped_spruce_log` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct StrippedSpruceLog {
        /// `axis` state.
        pub axis : Axis,
    }
    /// `axis` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Axis {
        /// `x` variant.
        X,
        /// `y` variant.
        Y,
        /// `z` variant.
        Z,
    }
    impl super::Block for StrippedSpruceLog {
    }
    impl From<StrippedSpruceLog> for super::BlockState {
        fn from(value : StrippedSpruceLog) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:nether_gold_ore` block.
pub mod nether_gold_ore {
    /// `minecraft:nether_gold_ore` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct NetherGoldOre;
    impl super::Block for NetherGoldOre {
    }
    impl From<NetherGoldOre> for super::BlockState {
        fn from(value : NetherGoldOre) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:turtle_egg` block.
pub mod turtle_egg {
    /// `minecraft:turtle_egg` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct TurtleEgg {
        /// `eggs` state.
        pub eggs : Eggs,
        /// `hatch` state.
        pub hatch : Hatch,
    }
    /// `eggs` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Eggs {
        /// `1` variant.
        N1,
        /// `2` variant.
        N2,
        /// `3` variant.
        N3,
        /// `4` variant.
        N4,
    }
    /// `hatch` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Hatch {
        /// `0` variant.
        N0,
        /// `1` variant.
        N1,
        /// `2` variant.
        N2,
    }
    impl super::Block for TurtleEgg {
    }
    impl From<TurtleEgg> for super::BlockState {
        fn from(value : TurtleEgg) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:copper_block` block.
pub mod copper_block {
    /// `minecraft:copper_block` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct CopperBlock;
    impl super::Block for CopperBlock {
    }
    impl From<CopperBlock> for super::BlockState {
        fn from(value : CopperBlock) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:gray_wool` block.
pub mod gray_wool {
    /// `minecraft:gray_wool` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct GrayWool;
    impl super::Block for GrayWool {
    }
    impl From<GrayWool> for super::BlockState {
        fn from(value : GrayWool) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:hopper` block.
pub mod hopper {
    /// `minecraft:hopper` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct Hopper {
        /// `enabled` state.
        pub enabled : bool,
        /// `facing` state.
        pub facing : Facing,
    }
    /// `facing` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Facing {
        /// `down` variant.
        Down,
        /// `north` variant.
        North,
        /// `south` variant.
        South,
        /// `west` variant.
        West,
        /// `east` variant.
        East,
    }
    impl super::Block for Hopper {
    }
    impl From<Hopper> for super::BlockState {
        fn from(value : Hopper) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:light_blue_wool` block.
pub mod light_blue_wool {
    /// `minecraft:light_blue_wool` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct LightBlueWool;
    impl super::Block for LightBlueWool {
    }
    impl From<LightBlueWool> for super::BlockState {
        fn from(value : LightBlueWool) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:jigsaw` block.
pub mod jigsaw {
    /// `minecraft:jigsaw` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct Jigsaw {
        /// `orientation` state.
        pub orientation : Orientation,
    }
    /// `orientation` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Orientation {
        /// `down_east` variant.
        DownEast,
        /// `down_north` variant.
        DownNorth,
        /// `down_south` variant.
        DownSouth,
        /// `down_west` variant.
        DownWest,
        /// `up_east` variant.
        UpEast,
        /// `up_north` variant.
        UpNorth,
        /// `up_south` variant.
        UpSouth,
        /// `up_west` variant.
        UpWest,
        /// `west_up` variant.
        WestUp,
        /// `east_up` variant.
        EastUp,
        /// `north_up` variant.
        NorthUp,
        /// `south_up` variant.
        SouthUp,
    }
    impl super::Block for Jigsaw {
    }
    impl From<Jigsaw> for super::BlockState {
        fn from(value : Jigsaw) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:mossy_cobblestone` block.
pub mod mossy_cobblestone {
    /// `minecraft:mossy_cobblestone` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct MossyCobblestone;
    impl super::Block for MossyCobblestone {
    }
    impl From<MossyCobblestone> for super::BlockState {
        fn from(value : MossyCobblestone) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:pale_oak_fence_gate` block.
pub mod pale_oak_fence_gate {
    /// `minecraft:pale_oak_fence_gate` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct PaleOakFenceGate {
        /// `facing` state.
        pub facing : Facing,
        /// `in_wall` state.
        pub in_wall : bool,
        /// `powered` state.
        pub powered : bool,
        /// `open` state.
        pub open : bool,
    }
    /// `facing` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Facing {
        /// `north` variant.
        North,
        /// `south` variant.
        South,
        /// `west` variant.
        West,
        /// `east` variant.
        East,
    }
    impl super::Block for PaleOakFenceGate {
    }
    impl From<PaleOakFenceGate> for super::BlockState {
        fn from(value : PaleOakFenceGate) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:end_stone_brick_slab` block.
pub mod end_stone_brick_slab {
    /// `minecraft:end_stone_brick_slab` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct EndStoneBrickSlab {
        /// `type` state.
        pub kind : Kind,
        /// `waterlogged` state.
        pub waterlogged : bool,
    }
    /// `type` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Kind {
        /// `top` variant.
        Top,
        /// `bottom` variant.
        Bottom,
        /// `double` variant.
        Double,
    }
    impl super::Block for EndStoneBrickSlab {
    }
    impl From<EndStoneBrickSlab> for super::BlockState {
        fn from(value : EndStoneBrickSlab) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:rail` block.
pub mod rail {
    /// `minecraft:rail` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct Rail {
        /// `shape` state.
        pub shape : Shape,
        /// `waterlogged` state.
        pub waterlogged : bool,
    }
    /// `shape` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Shape {
        /// `north_south` variant.
        NorthSouth,
        /// `east_west` variant.
        EastWest,
        /// `ascending_east` variant.
        AscendingEast,
        /// `ascending_west` variant.
        AscendingWest,
        /// `ascending_north` variant.
        AscendingNorth,
        /// `ascending_south` variant.
        AscendingSouth,
        /// `south_east` variant.
        SouthEast,
        /// `south_west` variant.
        SouthWest,
        /// `north_west` variant.
        NorthWest,
        /// `north_east` variant.
        NorthEast,
    }
    impl super::Block for Rail {
    }
    impl From<Rail> for super::BlockState {
        fn from(value : Rail) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:brain_coral_block` block.
pub mod brain_coral_block {
    /// `minecraft:brain_coral_block` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct BrainCoralBlock;
    impl super::Block for BrainCoralBlock {
    }
    impl From<BrainCoralBlock> for super::BlockState {
        fn from(value : BrainCoralBlock) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:brown_stained_glass` block.
pub mod brown_stained_glass {
    /// `minecraft:brown_stained_glass` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct BrownStainedGlass;
    impl super::Block for BrownStainedGlass {
    }
    impl From<BrownStainedGlass> for super::BlockState {
        fn from(value : BrownStainedGlass) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:smooth_quartz_slab` block.
pub mod smooth_quartz_slab {
    /// `minecraft:smooth_quartz_slab` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct SmoothQuartzSlab {
        /// `type` state.
        pub kind : Kind,
        /// `waterlogged` state.
        pub waterlogged : bool,
    }
    /// `type` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Kind {
        /// `top` variant.
        Top,
        /// `bottom` variant.
        Bottom,
        /// `double` variant.
        Double,
    }
    impl super::Block for SmoothQuartzSlab {
    }
    impl From<SmoothQuartzSlab> for super::BlockState {
        fn from(value : SmoothQuartzSlab) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:white_candle` block.
pub mod white_candle {
    /// `minecraft:white_candle` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct WhiteCandle {
        /// `waterlogged` state.
        pub waterlogged : bool,
        /// `candles` state.
        pub candles : Candles,
        /// `lit` state.
        pub lit : bool,
    }
    /// `candles` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Candles {
        /// `1` variant.
        N1,
        /// `2` variant.
        N2,
        /// `3` variant.
        N3,
        /// `4` variant.
        N4,
    }
    impl super::Block for WhiteCandle {
    }
    impl From<WhiteCandle> for super::BlockState {
        fn from(value : WhiteCandle) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:azalea_leaves` block.
pub mod azalea_leaves {
    /// `minecraft:azalea_leaves` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct AzaleaLeaves {
        /// `persistent` state.
        pub persistent : bool,
        /// `distance` state.
        pub distance : Distance,
        /// `waterlogged` state.
        pub waterlogged : bool,
    }
    /// `distance` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Distance {
        /// `1` variant.
        N1,
        /// `2` variant.
        N2,
        /// `3` variant.
        N3,
        /// `4` variant.
        N4,
        /// `5` variant.
        N5,
        /// `6` variant.
        N6,
        /// `7` variant.
        N7,
    }
    impl super::Block for AzaleaLeaves {
    }
    impl From<AzaleaLeaves> for super::BlockState {
        fn from(value : AzaleaLeaves) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:diamond_block` block.
pub mod diamond_block {
    /// `minecraft:diamond_block` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct DiamondBlock;
    impl super::Block for DiamondBlock {
    }
    impl From<DiamondBlock> for super::BlockState {
        fn from(value : DiamondBlock) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:pale_oak_button` block.
pub mod pale_oak_button {
    /// `minecraft:pale_oak_button` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct PaleOakButton {
        /// `facing` state.
        pub facing : Facing,
        /// `powered` state.
        pub powered : bool,
        /// `face` state.
        pub face : Face,
    }
    /// `facing` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Facing {
        /// `north` variant.
        North,
        /// `south` variant.
        South,
        /// `west` variant.
        West,
        /// `east` variant.
        East,
    }
    /// `face` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Face {
        /// `floor` variant.
        Floor,
        /// `wall` variant.
        Wall,
        /// `ceiling` variant.
        Ceiling,
    }
    impl super::Block for PaleOakButton {
    }
    impl From<PaleOakButton> for super::BlockState {
        fn from(value : PaleOakButton) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:andesite_stairs` block.
pub mod andesite_stairs {
    /// `minecraft:andesite_stairs` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct AndesiteStairs {
        /// `waterlogged` state.
        pub waterlogged : bool,
        /// `shape` state.
        pub shape : Shape,
        /// `facing` state.
        pub facing : Facing,
        /// `half` state.
        pub half : Half,
    }
    /// `shape` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Shape {
        /// `straight` variant.
        Straight,
        /// `inner_left` variant.
        InnerLeft,
        /// `inner_right` variant.
        InnerRight,
        /// `outer_left` variant.
        OuterLeft,
        /// `outer_right` variant.
        OuterRight,
    }
    /// `facing` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Facing {
        /// `north` variant.
        North,
        /// `south` variant.
        South,
        /// `west` variant.
        West,
        /// `east` variant.
        East,
    }
    /// `half` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Half {
        /// `top` variant.
        Top,
        /// `bottom` variant.
        Bottom,
    }
    impl super::Block for AndesiteStairs {
    }
    impl From<AndesiteStairs> for super::BlockState {
        fn from(value : AndesiteStairs) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:potted_flowering_azalea_bush` block.
pub mod potted_flowering_azalea_bush {
    /// `minecraft:potted_flowering_azalea_bush` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct PottedFloweringAzaleaBush;
    impl super::Block for PottedFloweringAzaleaBush {
    }
    impl From<PottedFloweringAzaleaBush> for super::BlockState {
        fn from(value : PottedFloweringAzaleaBush) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:tall_grass` block.
pub mod tall_grass {
    /// `minecraft:tall_grass` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct TallGrass {
        /// `half` state.
        pub half : Half,
    }
    /// `half` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Half {
        /// `upper` variant.
        Upper,
        /// `lower` variant.
        Lower,
    }
    impl super::Block for TallGrass {
    }
    impl From<TallGrass> for super::BlockState {
        fn from(value : TallGrass) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:waxed_copper_door` block.
pub mod waxed_copper_door {
    /// `minecraft:waxed_copper_door` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct WaxedCopperDoor {
        /// `half` state.
        pub half : Half,
        /// `open` state.
        pub open : bool,
        /// `hinge` state.
        pub hinge : Hinge,
        /// `powered` state.
        pub powered : bool,
        /// `facing` state.
        pub facing : Facing,
    }
    /// `half` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Half {
        /// `upper` variant.
        Upper,
        /// `lower` variant.
        Lower,
    }
    /// `hinge` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Hinge {
        /// `left` variant.
        Left,
        /// `right` variant.
        Right,
    }
    /// `facing` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Facing {
        /// `north` variant.
        North,
        /// `south` variant.
        South,
        /// `west` variant.
        West,
        /// `east` variant.
        East,
    }
    impl super::Block for WaxedCopperDoor {
    }
    impl From<WaxedCopperDoor> for super::BlockState {
        fn from(value : WaxedCopperDoor) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:yellow_stained_glass_pane` block.
pub mod yellow_stained_glass_pane {
    /// `minecraft:yellow_stained_glass_pane` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct YellowStainedGlassPane {
        /// `west` state.
        pub west : bool,
        /// `east` state.
        pub east : bool,
        /// `south` state.
        pub south : bool,
        /// `waterlogged` state.
        pub waterlogged : bool,
        /// `north` state.
        pub north : bool,
    }
    impl super::Block for YellowStainedGlassPane {
    }
    impl From<YellowStainedGlassPane> for super::BlockState {
        fn from(value : YellowStainedGlassPane) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:soul_fire` block.
pub mod soul_fire {
    /// `minecraft:soul_fire` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct SoulFire;
    impl super::Block for SoulFire {
    }
    impl From<SoulFire> for super::BlockState {
        fn from(value : SoulFire) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:stripped_birch_log` block.
pub mod stripped_birch_log {
    /// `minecraft:stripped_birch_log` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct StrippedBirchLog {
        /// `axis` state.
        pub axis : Axis,
    }
    /// `axis` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Axis {
        /// `x` variant.
        X,
        /// `y` variant.
        Y,
        /// `z` variant.
        Z,
    }
    impl super::Block for StrippedBirchLog {
    }
    impl From<StrippedBirchLog> for super::BlockState {
        fn from(value : StrippedBirchLog) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:deepslate_emerald_ore` block.
pub mod deepslate_emerald_ore {
    /// `minecraft:deepslate_emerald_ore` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct DeepslateEmeraldOre;
    impl super::Block for DeepslateEmeraldOre {
    }
    impl From<DeepslateEmeraldOre> for super::BlockState {
        fn from(value : DeepslateEmeraldOre) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:jack_o_lantern` block.
pub mod jack_o_lantern {
    /// `minecraft:jack_o_lantern` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct JackOLantern {
        /// `facing` state.
        pub facing : Facing,
    }
    /// `facing` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Facing {
        /// `north` variant.
        North,
        /// `south` variant.
        South,
        /// `west` variant.
        West,
        /// `east` variant.
        East,
    }
    impl super::Block for JackOLantern {
    }
    impl From<JackOLantern> for super::BlockState {
        fn from(value : JackOLantern) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:warped_planks` block.
pub mod warped_planks {
    /// `minecraft:warped_planks` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct WarpedPlanks;
    impl super::Block for WarpedPlanks {
    }
    impl From<WarpedPlanks> for super::BlockState {
        fn from(value : WarpedPlanks) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:black_stained_glass` block.
pub mod black_stained_glass {
    /// `minecraft:black_stained_glass` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct BlackStainedGlass;
    impl super::Block for BlackStainedGlass {
    }
    impl From<BlackStainedGlass> for super::BlockState {
        fn from(value : BlackStainedGlass) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:infested_deepslate` block.
pub mod infested_deepslate {
    /// `minecraft:infested_deepslate` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct InfestedDeepslate {
        /// `axis` state.
        pub axis : Axis,
    }
    /// `axis` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Axis {
        /// `x` variant.
        X,
        /// `y` variant.
        Y,
        /// `z` variant.
        Z,
    }
    impl super::Block for InfestedDeepslate {
    }
    impl From<InfestedDeepslate> for super::BlockState {
        fn from(value : InfestedDeepslate) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:melon` block.
pub mod melon {
    /// `minecraft:melon` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct Melon;
    impl super::Block for Melon {
    }
    impl From<Melon> for super::BlockState {
        fn from(value : Melon) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:smooth_basalt` block.
pub mod smooth_basalt {
    /// `minecraft:smooth_basalt` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct SmoothBasalt;
    impl super::Block for SmoothBasalt {
    }
    impl From<SmoothBasalt> for super::BlockState {
        fn from(value : SmoothBasalt) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:diorite_stairs` block.
pub mod diorite_stairs {
    /// `minecraft:diorite_stairs` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct DioriteStairs {
        /// `shape` state.
        pub shape : Shape,
        /// `facing` state.
        pub facing : Facing,
        /// `waterlogged` state.
        pub waterlogged : bool,
        /// `half` state.
        pub half : Half,
    }
    /// `shape` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Shape {
        /// `straight` variant.
        Straight,
        /// `inner_left` variant.
        InnerLeft,
        /// `inner_right` variant.
        InnerRight,
        /// `outer_left` variant.
        OuterLeft,
        /// `outer_right` variant.
        OuterRight,
    }
    /// `facing` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Facing {
        /// `north` variant.
        North,
        /// `south` variant.
        South,
        /// `west` variant.
        West,
        /// `east` variant.
        East,
    }
    /// `half` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Half {
        /// `top` variant.
        Top,
        /// `bottom` variant.
        Bottom,
    }
    impl super::Block for DioriteStairs {
    }
    impl From<DioriteStairs> for super::BlockState {
        fn from(value : DioriteStairs) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:chipped_anvil` block.
pub mod chipped_anvil {
    /// `minecraft:chipped_anvil` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct ChippedAnvil {
        /// `facing` state.
        pub facing : Facing,
    }
    /// `facing` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Facing {
        /// `north` variant.
        North,
        /// `south` variant.
        South,
        /// `west` variant.
        West,
        /// `east` variant.
        East,
    }
    impl super::Block for ChippedAnvil {
    }
    impl From<ChippedAnvil> for super::BlockState {
        fn from(value : ChippedAnvil) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:deepslate_copper_ore` block.
pub mod deepslate_copper_ore {
    /// `minecraft:deepslate_copper_ore` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct DeepslateCopperOre;
    impl super::Block for DeepslateCopperOre {
    }
    impl From<DeepslateCopperOre> for super::BlockState {
        fn from(value : DeepslateCopperOre) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:weathered_copper_door` block.
pub mod weathered_copper_door {
    /// `minecraft:weathered_copper_door` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct WeatheredCopperDoor {
        /// `half` state.
        pub half : Half,
        /// `open` state.
        pub open : bool,
        /// `powered` state.
        pub powered : bool,
        /// `facing` state.
        pub facing : Facing,
        /// `hinge` state.
        pub hinge : Hinge,
    }
    /// `half` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Half {
        /// `upper` variant.
        Upper,
        /// `lower` variant.
        Lower,
    }
    /// `facing` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Facing {
        /// `north` variant.
        North,
        /// `south` variant.
        South,
        /// `west` variant.
        West,
        /// `east` variant.
        East,
    }
    /// `hinge` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Hinge {
        /// `left` variant.
        Left,
        /// `right` variant.
        Right,
    }
    impl super::Block for WeatheredCopperDoor {
    }
    impl From<WeatheredCopperDoor> for super::BlockState {
        fn from(value : WeatheredCopperDoor) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:light_gray_candle_cake` block.
pub mod light_gray_candle_cake {
    /// `minecraft:light_gray_candle_cake` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct LightGrayCandleCake {
        /// `lit` state.
        pub lit : bool,
    }
    impl super::Block for LightGrayCandleCake {
    }
    impl From<LightGrayCandleCake> for super::BlockState {
        fn from(value : LightGrayCandleCake) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:magenta_terracotta` block.
pub mod magenta_terracotta {
    /// `minecraft:magenta_terracotta` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct MagentaTerracotta;
    impl super::Block for MagentaTerracotta {
    }
    impl From<MagentaTerracotta> for super::BlockState {
        fn from(value : MagentaTerracotta) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:budding_amethyst` block.
pub mod budding_amethyst {
    /// `minecraft:budding_amethyst` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct BuddingAmethyst;
    impl super::Block for BuddingAmethyst {
    }
    impl From<BuddingAmethyst> for super::BlockState {
        fn from(value : BuddingAmethyst) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:bubble_coral` block.
pub mod bubble_coral {
    /// `minecraft:bubble_coral` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct BubbleCoral {
        /// `waterlogged` state.
        pub waterlogged : bool,
    }
    impl super::Block for BubbleCoral {
    }
    impl From<BubbleCoral> for super::BlockState {
        fn from(value : BubbleCoral) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:wall_torch` block.
pub mod wall_torch {
    /// `minecraft:wall_torch` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct WallTorch {
        /// `facing` state.
        pub facing : Facing,
    }
    /// `facing` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Facing {
        /// `north` variant.
        North,
        /// `south` variant.
        South,
        /// `west` variant.
        West,
        /// `east` variant.
        East,
    }
    impl super::Block for WallTorch {
    }
    impl From<WallTorch> for super::BlockState {
        fn from(value : WallTorch) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:jungle_button` block.
pub mod jungle_button {
    /// `minecraft:jungle_button` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct JungleButton {
        /// `face` state.
        pub face : Face,
        /// `facing` state.
        pub facing : Facing,
        /// `powered` state.
        pub powered : bool,
    }
    /// `face` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Face {
        /// `floor` variant.
        Floor,
        /// `wall` variant.
        Wall,
        /// `ceiling` variant.
        Ceiling,
    }
    /// `facing` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Facing {
        /// `north` variant.
        North,
        /// `south` variant.
        South,
        /// `west` variant.
        West,
        /// `east` variant.
        East,
    }
    impl super::Block for JungleButton {
    }
    impl From<JungleButton> for super::BlockState {
        fn from(value : JungleButton) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:powder_snow` block.
pub mod powder_snow {
    /// `minecraft:powder_snow` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct PowderSnow;
    impl super::Block for PowderSnow {
    }
    impl From<PowderSnow> for super::BlockState {
        fn from(value : PowderSnow) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:copper_ore` block.
pub mod copper_ore {
    /// `minecraft:copper_ore` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct CopperOre;
    impl super::Block for CopperOre {
    }
    impl From<CopperOre> for super::BlockState {
        fn from(value : CopperOre) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:cherry_wall_hanging_sign` block.
pub mod cherry_wall_hanging_sign {
    /// `minecraft:cherry_wall_hanging_sign` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct CherryWallHangingSign {
        /// `facing` state.
        pub facing : Facing,
        /// `waterlogged` state.
        pub waterlogged : bool,
    }
    /// `facing` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Facing {
        /// `north` variant.
        North,
        /// `south` variant.
        South,
        /// `west` variant.
        West,
        /// `east` variant.
        East,
    }
    impl super::Block for CherryWallHangingSign {
    }
    impl From<CherryWallHangingSign> for super::BlockState {
        fn from(value : CherryWallHangingSign) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:deepslate` block.
pub mod deepslate {
    /// `minecraft:deepslate` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct Deepslate {
        /// `axis` state.
        pub axis : Axis,
    }
    /// `axis` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Axis {
        /// `x` variant.
        X,
        /// `y` variant.
        Y,
        /// `z` variant.
        Z,
    }
    impl super::Block for Deepslate {
    }
    impl From<Deepslate> for super::BlockState {
        fn from(value : Deepslate) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:green_shulker_box` block.
pub mod green_shulker_box {
    /// `minecraft:green_shulker_box` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct GreenShulkerBox {
        /// `facing` state.
        pub facing : Facing,
    }
    /// `facing` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Facing {
        /// `north` variant.
        North,
        /// `east` variant.
        East,
        /// `south` variant.
        South,
        /// `west` variant.
        West,
        /// `up` variant.
        Up,
        /// `down` variant.
        Down,
    }
    impl super::Block for GreenShulkerBox {
    }
    impl From<GreenShulkerBox> for super::BlockState {
        fn from(value : GreenShulkerBox) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:dark_oak_button` block.
pub mod dark_oak_button {
    /// `minecraft:dark_oak_button` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct DarkOakButton {
        /// `facing` state.
        pub facing : Facing,
        /// `powered` state.
        pub powered : bool,
        /// `face` state.
        pub face : Face,
    }
    /// `facing` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Facing {
        /// `north` variant.
        North,
        /// `south` variant.
        South,
        /// `west` variant.
        West,
        /// `east` variant.
        East,
    }
    /// `face` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Face {
        /// `floor` variant.
        Floor,
        /// `wall` variant.
        Wall,
        /// `ceiling` variant.
        Ceiling,
    }
    impl super::Block for DarkOakButton {
    }
    impl From<DarkOakButton> for super::BlockState {
        fn from(value : DarkOakButton) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:cyan_terracotta` block.
pub mod cyan_terracotta {
    /// `minecraft:cyan_terracotta` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct CyanTerracotta;
    impl super::Block for CyanTerracotta {
    }
    impl From<CyanTerracotta> for super::BlockState {
        fn from(value : CyanTerracotta) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:purple_wool` block.
pub mod purple_wool {
    /// `minecraft:purple_wool` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct PurpleWool;
    impl super::Block for PurpleWool {
    }
    impl From<PurpleWool> for super::BlockState {
        fn from(value : PurpleWool) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:red_nether_brick_wall` block.
pub mod red_nether_brick_wall {
    /// `minecraft:red_nether_brick_wall` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct RedNetherBrickWall {
        /// `west` state.
        pub west : West,
        /// `up` state.
        pub up : bool,
        /// `south` state.
        pub south : South,
        /// `east` state.
        pub east : East,
        /// `north` state.
        pub north : North,
        /// `waterlogged` state.
        pub waterlogged : bool,
    }
    /// `west` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum West {
        /// `none` variant.
        None,
        /// `low` variant.
        Low,
        /// `tall` variant.
        Tall,
    }
    /// `south` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum South {
        /// `none` variant.
        None,
        /// `low` variant.
        Low,
        /// `tall` variant.
        Tall,
    }
    /// `east` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum East {
        /// `none` variant.
        None,
        /// `low` variant.
        Low,
        /// `tall` variant.
        Tall,
    }
    /// `north` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum North {
        /// `none` variant.
        None,
        /// `low` variant.
        Low,
        /// `tall` variant.
        Tall,
    }
    impl super::Block for RedNetherBrickWall {
    }
    impl From<RedNetherBrickWall> for super::BlockState {
        fn from(value : RedNetherBrickWall) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:redstone_wire` block.
pub mod redstone_wire {
    /// `minecraft:redstone_wire` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct RedstoneWire {
        /// `south` state.
        pub south : South,
        /// `north` state.
        pub north : North,
        /// `power` state.
        pub power : Power,
        /// `west` state.
        pub west : West,
        /// `east` state.
        pub east : East,
    }
    /// `south` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum South {
        /// `up` variant.
        Up,
        /// `side` variant.
        Side,
        /// `none` variant.
        None,
    }
    /// `north` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum North {
        /// `up` variant.
        Up,
        /// `side` variant.
        Side,
        /// `none` variant.
        None,
    }
    /// `power` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Power {
        /// `0` variant.
        N0,
        /// `1` variant.
        N1,
        /// `2` variant.
        N2,
        /// `3` variant.
        N3,
        /// `4` variant.
        N4,
        /// `5` variant.
        N5,
        /// `6` variant.
        N6,
        /// `7` variant.
        N7,
        /// `8` variant.
        N8,
        /// `9` variant.
        N9,
        /// `10` variant.
        N10,
        /// `11` variant.
        N11,
        /// `12` variant.
        N12,
        /// `13` variant.
        N13,
        /// `14` variant.
        N14,
        /// `15` variant.
        N15,
    }
    /// `west` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum West {
        /// `up` variant.
        Up,
        /// `side` variant.
        Side,
        /// `none` variant.
        None,
    }
    /// `east` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum East {
        /// `up` variant.
        Up,
        /// `side` variant.
        Side,
        /// `none` variant.
        None,
    }
    impl super::Block for RedstoneWire {
    }
    impl From<RedstoneWire> for super::BlockState {
        fn from(value : RedstoneWire) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:waxed_weathered_cut_copper_stairs` block.
pub mod waxed_weathered_cut_copper_stairs {
    /// `minecraft:waxed_weathered_cut_copper_stairs` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct WaxedWeatheredCutCopperStairs {
        /// `facing` state.
        pub facing : Facing,
        /// `half` state.
        pub half : Half,
        /// `waterlogged` state.
        pub waterlogged : bool,
        /// `shape` state.
        pub shape : Shape,
    }
    /// `facing` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Facing {
        /// `north` variant.
        North,
        /// `south` variant.
        South,
        /// `west` variant.
        West,
        /// `east` variant.
        East,
    }
    /// `half` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Half {
        /// `top` variant.
        Top,
        /// `bottom` variant.
        Bottom,
    }
    /// `shape` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Shape {
        /// `straight` variant.
        Straight,
        /// `inner_left` variant.
        InnerLeft,
        /// `inner_right` variant.
        InnerRight,
        /// `outer_left` variant.
        OuterLeft,
        /// `outer_right` variant.
        OuterRight,
    }
    impl super::Block for WaxedWeatheredCutCopperStairs {
    }
    impl From<WaxedWeatheredCutCopperStairs> for super::BlockState {
        fn from(value : WaxedWeatheredCutCopperStairs) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:yellow_candle_cake` block.
pub mod yellow_candle_cake {
    /// `minecraft:yellow_candle_cake` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct YellowCandleCake {
        /// `lit` state.
        pub lit : bool,
    }
    impl super::Block for YellowCandleCake {
    }
    impl From<YellowCandleCake> for super::BlockState {
        fn from(value : YellowCandleCake) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:cobbled_deepslate_wall` block.
pub mod cobbled_deepslate_wall {
    /// `minecraft:cobbled_deepslate_wall` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct CobbledDeepslateWall {
        /// `waterlogged` state.
        pub waterlogged : bool,
        /// `up` state.
        pub up : bool,
        /// `west` state.
        pub west : West,
        /// `north` state.
        pub north : North,
        /// `east` state.
        pub east : East,
        /// `south` state.
        pub south : South,
    }
    /// `west` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum West {
        /// `none` variant.
        None,
        /// `low` variant.
        Low,
        /// `tall` variant.
        Tall,
    }
    /// `north` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum North {
        /// `none` variant.
        None,
        /// `low` variant.
        Low,
        /// `tall` variant.
        Tall,
    }
    /// `east` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum East {
        /// `none` variant.
        None,
        /// `low` variant.
        Low,
        /// `tall` variant.
        Tall,
    }
    /// `south` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum South {
        /// `none` variant.
        None,
        /// `low` variant.
        Low,
        /// `tall` variant.
        Tall,
    }
    impl super::Block for CobbledDeepslateWall {
    }
    impl From<CobbledDeepslateWall> for super::BlockState {
        fn from(value : CobbledDeepslateWall) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:potted_warped_roots` block.
pub mod potted_warped_roots {
    /// `minecraft:potted_warped_roots` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct PottedWarpedRoots;
    impl super::Block for PottedWarpedRoots {
    }
    impl From<PottedWarpedRoots> for super::BlockState {
        fn from(value : PottedWarpedRoots) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:brown_mushroom` block.
pub mod brown_mushroom {
    /// `minecraft:brown_mushroom` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct BrownMushroom;
    impl super::Block for BrownMushroom {
    }
    impl From<BrownMushroom> for super::BlockState {
        fn from(value : BrownMushroom) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:magenta_stained_glass` block.
pub mod magenta_stained_glass {
    /// `minecraft:magenta_stained_glass` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct MagentaStainedGlass;
    impl super::Block for MagentaStainedGlass {
    }
    impl From<MagentaStainedGlass> for super::BlockState {
        fn from(value : MagentaStainedGlass) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:lodestone` block.
pub mod lodestone {
    /// `minecraft:lodestone` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct Lodestone;
    impl super::Block for Lodestone {
    }
    impl From<Lodestone> for super::BlockState {
        fn from(value : Lodestone) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:ancient_debris` block.
pub mod ancient_debris {
    /// `minecraft:ancient_debris` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct AncientDebris;
    impl super::Block for AncientDebris {
    }
    impl From<AncientDebris> for super::BlockState {
        fn from(value : AncientDebris) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:stripped_mangrove_wood` block.
pub mod stripped_mangrove_wood {
    /// `minecraft:stripped_mangrove_wood` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct StrippedMangroveWood {
        /// `axis` state.
        pub axis : Axis,
    }
    /// `axis` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Axis {
        /// `x` variant.
        X,
        /// `y` variant.
        Y,
        /// `z` variant.
        Z,
    }
    impl super::Block for StrippedMangroveWood {
    }
    impl From<StrippedMangroveWood> for super::BlockState {
        fn from(value : StrippedMangroveWood) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:white_stained_glass` block.
pub mod white_stained_glass {
    /// `minecraft:white_stained_glass` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct WhiteStainedGlass;
    impl super::Block for WhiteStainedGlass {
    }
    impl From<WhiteStainedGlass> for super::BlockState {
        fn from(value : WhiteStainedGlass) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:brick_wall` block.
pub mod brick_wall {
    /// `minecraft:brick_wall` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct BrickWall {
        /// `waterlogged` state.
        pub waterlogged : bool,
        /// `up` state.
        pub up : bool,
        /// `east` state.
        pub east : East,
        /// `north` state.
        pub north : North,
        /// `west` state.
        pub west : West,
        /// `south` state.
        pub south : South,
    }
    /// `east` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum East {
        /// `none` variant.
        None,
        /// `low` variant.
        Low,
        /// `tall` variant.
        Tall,
    }
    /// `north` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum North {
        /// `none` variant.
        None,
        /// `low` variant.
        Low,
        /// `tall` variant.
        Tall,
    }
    /// `west` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum West {
        /// `none` variant.
        None,
        /// `low` variant.
        Low,
        /// `tall` variant.
        Tall,
    }
    /// `south` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum South {
        /// `none` variant.
        None,
        /// `low` variant.
        Low,
        /// `tall` variant.
        Tall,
    }
    impl super::Block for BrickWall {
    }
    impl From<BrickWall> for super::BlockState {
        fn from(value : BrickWall) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:red_concrete_powder` block.
pub mod red_concrete_powder {
    /// `minecraft:red_concrete_powder` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct RedConcretePowder;
    impl super::Block for RedConcretePowder {
    }
    impl From<RedConcretePowder> for super::BlockState {
        fn from(value : RedConcretePowder) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:anvil` block.
pub mod anvil {
    /// `minecraft:anvil` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct Anvil {
        /// `facing` state.
        pub facing : Facing,
    }
    /// `facing` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Facing {
        /// `north` variant.
        North,
        /// `south` variant.
        South,
        /// `west` variant.
        West,
        /// `east` variant.
        East,
    }
    impl super::Block for Anvil {
    }
    impl From<Anvil> for super::BlockState {
        fn from(value : Anvil) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:waxed_oxidized_copper_door` block.
pub mod waxed_oxidized_copper_door {
    /// `minecraft:waxed_oxidized_copper_door` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct WaxedOxidizedCopperDoor {
        /// `half` state.
        pub half : Half,
        /// `powered` state.
        pub powered : bool,
        /// `open` state.
        pub open : bool,
        /// `facing` state.
        pub facing : Facing,
        /// `hinge` state.
        pub hinge : Hinge,
    }
    /// `half` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Half {
        /// `upper` variant.
        Upper,
        /// `lower` variant.
        Lower,
    }
    /// `facing` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Facing {
        /// `north` variant.
        North,
        /// `south` variant.
        South,
        /// `west` variant.
        West,
        /// `east` variant.
        East,
    }
    /// `hinge` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Hinge {
        /// `left` variant.
        Left,
        /// `right` variant.
        Right,
    }
    impl super::Block for WaxedOxidizedCopperDoor {
    }
    impl From<WaxedOxidizedCopperDoor> for super::BlockState {
        fn from(value : WaxedOxidizedCopperDoor) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:mossy_cobblestone_wall` block.
pub mod mossy_cobblestone_wall {
    /// `minecraft:mossy_cobblestone_wall` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct MossyCobblestoneWall {
        /// `north` state.
        pub north : North,
        /// `east` state.
        pub east : East,
        /// `up` state.
        pub up : bool,
        /// `west` state.
        pub west : West,
        /// `south` state.
        pub south : South,
        /// `waterlogged` state.
        pub waterlogged : bool,
    }
    /// `north` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum North {
        /// `none` variant.
        None,
        /// `low` variant.
        Low,
        /// `tall` variant.
        Tall,
    }
    /// `east` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum East {
        /// `none` variant.
        None,
        /// `low` variant.
        Low,
        /// `tall` variant.
        Tall,
    }
    /// `west` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum West {
        /// `none` variant.
        None,
        /// `low` variant.
        Low,
        /// `tall` variant.
        Tall,
    }
    /// `south` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum South {
        /// `none` variant.
        None,
        /// `low` variant.
        Low,
        /// `tall` variant.
        Tall,
    }
    impl super::Block for MossyCobblestoneWall {
    }
    impl From<MossyCobblestoneWall> for super::BlockState {
        fn from(value : MossyCobblestoneWall) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:waxed_exposed_chiseled_copper` block.
pub mod waxed_exposed_chiseled_copper {
    /// `minecraft:waxed_exposed_chiseled_copper` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct WaxedExposedChiseledCopper;
    impl super::Block for WaxedExposedChiseledCopper {
    }
    impl From<WaxedExposedChiseledCopper> for super::BlockState {
        fn from(value : WaxedExposedChiseledCopper) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:crimson_nylium` block.
pub mod crimson_nylium {
    /// `minecraft:crimson_nylium` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct CrimsonNylium;
    impl super::Block for CrimsonNylium {
    }
    impl From<CrimsonNylium> for super::BlockState {
        fn from(value : CrimsonNylium) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:nether_wart` block.
pub mod nether_wart {
    /// `minecraft:nether_wart` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct NetherWart {
        /// `age` state.
        pub age : Age,
    }
    /// `age` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Age {
        /// `0` variant.
        N0,
        /// `1` variant.
        N1,
        /// `2` variant.
        N2,
        /// `3` variant.
        N3,
    }
    impl super::Block for NetherWart {
    }
    impl From<NetherWart> for super::BlockState {
        fn from(value : NetherWart) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:pale_oak_sapling` block.
pub mod pale_oak_sapling {
    /// `minecraft:pale_oak_sapling` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct PaleOakSapling {
        /// `stage` state.
        pub stage : Stage,
    }
    /// `stage` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Stage {
        /// `0` variant.
        N0,
        /// `1` variant.
        N1,
    }
    impl super::Block for PaleOakSapling {
    }
    impl From<PaleOakSapling> for super::BlockState {
        fn from(value : PaleOakSapling) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:pale_oak_wall_sign` block.
pub mod pale_oak_wall_sign {
    /// `minecraft:pale_oak_wall_sign` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct PaleOakWallSign {
        /// `waterlogged` state.
        pub waterlogged : bool,
        /// `facing` state.
        pub facing : Facing,
    }
    /// `facing` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Facing {
        /// `north` variant.
        North,
        /// `south` variant.
        South,
        /// `west` variant.
        West,
        /// `east` variant.
        East,
    }
    impl super::Block for PaleOakWallSign {
    }
    impl From<PaleOakWallSign> for super::BlockState {
        fn from(value : PaleOakWallSign) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:sandstone_slab` block.
pub mod sandstone_slab {
    /// `minecraft:sandstone_slab` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct SandstoneSlab {
        /// `waterlogged` state.
        pub waterlogged : bool,
        /// `type` state.
        pub kind : Kind,
    }
    /// `type` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Kind {
        /// `top` variant.
        Top,
        /// `bottom` variant.
        Bottom,
        /// `double` variant.
        Double,
    }
    impl super::Block for SandstoneSlab {
    }
    impl From<SandstoneSlab> for super::BlockState {
        fn from(value : SandstoneSlab) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:warped_slab` block.
pub mod warped_slab {
    /// `minecraft:warped_slab` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct WarpedSlab {
        /// `waterlogged` state.
        pub waterlogged : bool,
        /// `type` state.
        pub kind : Kind,
    }
    /// `type` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Kind {
        /// `top` variant.
        Top,
        /// `bottom` variant.
        Bottom,
        /// `double` variant.
        Double,
    }
    impl super::Block for WarpedSlab {
    }
    impl From<WarpedSlab> for super::BlockState {
        fn from(value : WarpedSlab) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:waxed_weathered_copper_bulb` block.
pub mod waxed_weathered_copper_bulb {
    /// `minecraft:waxed_weathered_copper_bulb` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct WaxedWeatheredCopperBulb {
        /// `lit` state.
        pub lit : bool,
        /// `powered` state.
        pub powered : bool,
    }
    impl super::Block for WaxedWeatheredCopperBulb {
    }
    impl From<WaxedWeatheredCopperBulb> for super::BlockState {
        fn from(value : WaxedWeatheredCopperBulb) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:acacia_wood` block.
pub mod acacia_wood {
    /// `minecraft:acacia_wood` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct AcaciaWood {
        /// `axis` state.
        pub axis : Axis,
    }
    /// `axis` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Axis {
        /// `x` variant.
        X,
        /// `y` variant.
        Y,
        /// `z` variant.
        Z,
    }
    impl super::Block for AcaciaWood {
    }
    impl From<AcaciaWood> for super::BlockState {
        fn from(value : AcaciaWood) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:infested_mossy_stone_bricks` block.
pub mod infested_mossy_stone_bricks {
    /// `minecraft:infested_mossy_stone_bricks` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct InfestedMossyStoneBricks;
    impl super::Block for InfestedMossyStoneBricks {
    }
    impl From<InfestedMossyStoneBricks> for super::BlockState {
        fn from(value : InfestedMossyStoneBricks) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:acacia_wall_hanging_sign` block.
pub mod acacia_wall_hanging_sign {
    /// `minecraft:acacia_wall_hanging_sign` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct AcaciaWallHangingSign {
        /// `facing` state.
        pub facing : Facing,
        /// `waterlogged` state.
        pub waterlogged : bool,
    }
    /// `facing` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Facing {
        /// `north` variant.
        North,
        /// `south` variant.
        South,
        /// `west` variant.
        West,
        /// `east` variant.
        East,
    }
    impl super::Block for AcaciaWallHangingSign {
    }
    impl From<AcaciaWallHangingSign> for super::BlockState {
        fn from(value : AcaciaWallHangingSign) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:lime_carpet` block.
pub mod lime_carpet {
    /// `minecraft:lime_carpet` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct LimeCarpet;
    impl super::Block for LimeCarpet {
    }
    impl From<LimeCarpet> for super::BlockState {
        fn from(value : LimeCarpet) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:flowering_azalea_leaves` block.
pub mod flowering_azalea_leaves {
    /// `minecraft:flowering_azalea_leaves` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct FloweringAzaleaLeaves {
        /// `distance` state.
        pub distance : Distance,
        /// `persistent` state.
        pub persistent : bool,
        /// `waterlogged` state.
        pub waterlogged : bool,
    }
    /// `distance` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Distance {
        /// `1` variant.
        N1,
        /// `2` variant.
        N2,
        /// `3` variant.
        N3,
        /// `4` variant.
        N4,
        /// `5` variant.
        N5,
        /// `6` variant.
        N6,
        /// `7` variant.
        N7,
    }
    impl super::Block for FloweringAzaleaLeaves {
    }
    impl From<FloweringAzaleaLeaves> for super::BlockState {
        fn from(value : FloweringAzaleaLeaves) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:polished_granite_stairs` block.
pub mod polished_granite_stairs {
    /// `minecraft:polished_granite_stairs` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct PolishedGraniteStairs {
        /// `half` state.
        pub half : Half,
        /// `facing` state.
        pub facing : Facing,
        /// `waterlogged` state.
        pub waterlogged : bool,
        /// `shape` state.
        pub shape : Shape,
    }
    /// `half` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Half {
        /// `top` variant.
        Top,
        /// `bottom` variant.
        Bottom,
    }
    /// `facing` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Facing {
        /// `north` variant.
        North,
        /// `south` variant.
        South,
        /// `west` variant.
        West,
        /// `east` variant.
        East,
    }
    /// `shape` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Shape {
        /// `straight` variant.
        Straight,
        /// `inner_left` variant.
        InnerLeft,
        /// `inner_right` variant.
        InnerRight,
        /// `outer_left` variant.
        OuterLeft,
        /// `outer_right` variant.
        OuterRight,
    }
    impl super::Block for PolishedGraniteStairs {
    }
    impl From<PolishedGraniteStairs> for super::BlockState {
        fn from(value : PolishedGraniteStairs) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:orange_wool` block.
pub mod orange_wool {
    /// `minecraft:orange_wool` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct OrangeWool;
    impl super::Block for OrangeWool {
    }
    impl From<OrangeWool> for super::BlockState {
        fn from(value : OrangeWool) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:cherry_hanging_sign` block.
pub mod cherry_hanging_sign {
    /// `minecraft:cherry_hanging_sign` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct CherryHangingSign {
        /// `attached` state.
        pub attached : bool,
        /// `waterlogged` state.
        pub waterlogged : bool,
        /// `rotation` state.
        pub rotation : Rotation,
    }
    /// `rotation` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Rotation {
        /// `0` variant.
        N0,
        /// `1` variant.
        N1,
        /// `2` variant.
        N2,
        /// `3` variant.
        N3,
        /// `4` variant.
        N4,
        /// `5` variant.
        N5,
        /// `6` variant.
        N6,
        /// `7` variant.
        N7,
        /// `8` variant.
        N8,
        /// `9` variant.
        N9,
        /// `10` variant.
        N10,
        /// `11` variant.
        N11,
        /// `12` variant.
        N12,
        /// `13` variant.
        N13,
        /// `14` variant.
        N14,
        /// `15` variant.
        N15,
    }
    impl super::Block for CherryHangingSign {
    }
    impl From<CherryHangingSign> for super::BlockState {
        fn from(value : CherryHangingSign) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:red_wool` block.
pub mod red_wool {
    /// `minecraft:red_wool` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct RedWool;
    impl super::Block for RedWool {
    }
    impl From<RedWool> for super::BlockState {
        fn from(value : RedWool) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:suspicious_sand` block.
pub mod suspicious_sand {
    /// `minecraft:suspicious_sand` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct SuspiciousSand {
        /// `dusted` state.
        pub dusted : Dusted,
    }
    /// `dusted` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Dusted {
        /// `0` variant.
        N0,
        /// `1` variant.
        N1,
        /// `2` variant.
        N2,
        /// `3` variant.
        N3,
    }
    impl super::Block for SuspiciousSand {
    }
    impl From<SuspiciousSand> for super::BlockState {
        fn from(value : SuspiciousSand) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:waxed_oxidized_copper_bulb` block.
pub mod waxed_oxidized_copper_bulb {
    /// `minecraft:waxed_oxidized_copper_bulb` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct WaxedOxidizedCopperBulb {
        /// `powered` state.
        pub powered : bool,
        /// `lit` state.
        pub lit : bool,
    }
    impl super::Block for WaxedOxidizedCopperBulb {
    }
    impl From<WaxedOxidizedCopperBulb> for super::BlockState {
        fn from(value : WaxedOxidizedCopperBulb) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:birch_fence` block.
pub mod birch_fence {
    /// `minecraft:birch_fence` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct BirchFence {
        /// `north` state.
        pub north : bool,
        /// `waterlogged` state.
        pub waterlogged : bool,
        /// `east` state.
        pub east : bool,
        /// `west` state.
        pub west : bool,
        /// `south` state.
        pub south : bool,
    }
    impl super::Block for BirchFence {
    }
    impl From<BirchFence> for super::BlockState {
        fn from(value : BirchFence) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:command_block` block.
pub mod command_block {
    /// `minecraft:command_block` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct CommandBlock {
        /// `conditional` state.
        pub conditional : bool,
        /// `facing` state.
        pub facing : Facing,
    }
    /// `facing` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Facing {
        /// `north` variant.
        North,
        /// `east` variant.
        East,
        /// `south` variant.
        South,
        /// `west` variant.
        West,
        /// `up` variant.
        Up,
        /// `down` variant.
        Down,
    }
    impl super::Block for CommandBlock {
    }
    impl From<CommandBlock> for super::BlockState {
        fn from(value : CommandBlock) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:dead_tube_coral_fan` block.
pub mod dead_tube_coral_fan {
    /// `minecraft:dead_tube_coral_fan` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct DeadTubeCoralFan {
        /// `waterlogged` state.
        pub waterlogged : bool,
    }
    impl super::Block for DeadTubeCoralFan {
    }
    impl From<DeadTubeCoralFan> for super::BlockState {
        fn from(value : DeadTubeCoralFan) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:birch_leaves` block.
pub mod birch_leaves {
    /// `minecraft:birch_leaves` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct BirchLeaves {
        /// `persistent` state.
        pub persistent : bool,
        /// `waterlogged` state.
        pub waterlogged : bool,
        /// `distance` state.
        pub distance : Distance,
    }
    /// `distance` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Distance {
        /// `1` variant.
        N1,
        /// `2` variant.
        N2,
        /// `3` variant.
        N3,
        /// `4` variant.
        N4,
        /// `5` variant.
        N5,
        /// `6` variant.
        N6,
        /// `7` variant.
        N7,
    }
    impl super::Block for BirchLeaves {
    }
    impl From<BirchLeaves> for super::BlockState {
        fn from(value : BirchLeaves) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:potted_bamboo` block.
pub mod potted_bamboo {
    /// `minecraft:potted_bamboo` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct PottedBamboo;
    impl super::Block for PottedBamboo {
    }
    impl From<PottedBamboo> for super::BlockState {
        fn from(value : PottedBamboo) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:resin_clump` block.
pub mod resin_clump {
    /// `minecraft:resin_clump` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct ResinClump {
        /// `north` state.
        pub north : bool,
        /// `up` state.
        pub up : bool,
        /// `east` state.
        pub east : bool,
        /// `west` state.
        pub west : bool,
        /// `waterlogged` state.
        pub waterlogged : bool,
        /// `down` state.
        pub down : bool,
        /// `south` state.
        pub south : bool,
    }
    impl super::Block for ResinClump {
    }
    impl From<ResinClump> for super::BlockState {
        fn from(value : ResinClump) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:pale_oak_wall_hanging_sign` block.
pub mod pale_oak_wall_hanging_sign {
    /// `minecraft:pale_oak_wall_hanging_sign` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct PaleOakWallHangingSign {
        /// `waterlogged` state.
        pub waterlogged : bool,
        /// `facing` state.
        pub facing : Facing,
    }
    /// `facing` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Facing {
        /// `north` variant.
        North,
        /// `south` variant.
        South,
        /// `west` variant.
        West,
        /// `east` variant.
        East,
    }
    impl super::Block for PaleOakWallHangingSign {
    }
    impl From<PaleOakWallHangingSign> for super::BlockState {
        fn from(value : PaleOakWallHangingSign) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:potted_closed_eyeblossom` block.
pub mod potted_closed_eyeblossom {
    /// `minecraft:potted_closed_eyeblossom` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct PottedClosedEyeblossom;
    impl super::Block for PottedClosedEyeblossom {
    }
    impl From<PottedClosedEyeblossom> for super::BlockState {
        fn from(value : PottedClosedEyeblossom) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:soul_soil` block.
pub mod soul_soil {
    /// `minecraft:soul_soil` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct SoulSoil;
    impl super::Block for SoulSoil {
    }
    impl From<SoulSoil> for super::BlockState {
        fn from(value : SoulSoil) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:lightning_rod` block.
pub mod lightning_rod {
    /// `minecraft:lightning_rod` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct LightningRod {
        /// `facing` state.
        pub facing : Facing,
        /// `waterlogged` state.
        pub waterlogged : bool,
        /// `powered` state.
        pub powered : bool,
    }
    /// `facing` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Facing {
        /// `north` variant.
        North,
        /// `east` variant.
        East,
        /// `south` variant.
        South,
        /// `west` variant.
        West,
        /// `up` variant.
        Up,
        /// `down` variant.
        Down,
    }
    impl super::Block for LightningRod {
    }
    impl From<LightningRod> for super::BlockState {
        fn from(value : LightningRod) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:acacia_wall_sign` block.
pub mod acacia_wall_sign {
    /// `minecraft:acacia_wall_sign` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct AcaciaWallSign {
        /// `facing` state.
        pub facing : Facing,
        /// `waterlogged` state.
        pub waterlogged : bool,
    }
    /// `facing` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Facing {
        /// `north` variant.
        North,
        /// `south` variant.
        South,
        /// `west` variant.
        West,
        /// `east` variant.
        East,
    }
    impl super::Block for AcaciaWallSign {
    }
    impl From<AcaciaWallSign> for super::BlockState {
        fn from(value : AcaciaWallSign) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:jungle_log` block.
pub mod jungle_log {
    /// `minecraft:jungle_log` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct JungleLog {
        /// `axis` state.
        pub axis : Axis,
    }
    /// `axis` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Axis {
        /// `x` variant.
        X,
        /// `y` variant.
        Y,
        /// `z` variant.
        Z,
    }
    impl super::Block for JungleLog {
    }
    impl From<JungleLog> for super::BlockState {
        fn from(value : JungleLog) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:chain` block.
pub mod chain {
    /// `minecraft:chain` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct Chain {
        /// `axis` state.
        pub axis : Axis,
        /// `waterlogged` state.
        pub waterlogged : bool,
    }
    /// `axis` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Axis {
        /// `x` variant.
        X,
        /// `y` variant.
        Y,
        /// `z` variant.
        Z,
    }
    impl super::Block for Chain {
    }
    impl From<Chain> for super::BlockState {
        fn from(value : Chain) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:pink_glazed_terracotta` block.
pub mod pink_glazed_terracotta {
    /// `minecraft:pink_glazed_terracotta` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct PinkGlazedTerracotta {
        /// `facing` state.
        pub facing : Facing,
    }
    /// `facing` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Facing {
        /// `north` variant.
        North,
        /// `south` variant.
        South,
        /// `west` variant.
        West,
        /// `east` variant.
        East,
    }
    impl super::Block for PinkGlazedTerracotta {
    }
    impl From<PinkGlazedTerracotta> for super::BlockState {
        fn from(value : PinkGlazedTerracotta) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:cut_sandstone_slab` block.
pub mod cut_sandstone_slab {
    /// `minecraft:cut_sandstone_slab` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct CutSandstoneSlab {
        /// `waterlogged` state.
        pub waterlogged : bool,
        /// `type` state.
        pub kind : Kind,
    }
    /// `type` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Kind {
        /// `top` variant.
        Top,
        /// `bottom` variant.
        Bottom,
        /// `double` variant.
        Double,
    }
    impl super::Block for CutSandstoneSlab {
    }
    impl From<CutSandstoneSlab> for super::BlockState {
        fn from(value : CutSandstoneSlab) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:short_dry_grass` block.
pub mod short_dry_grass {
    /// `minecraft:short_dry_grass` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct ShortDryGrass;
    impl super::Block for ShortDryGrass {
    }
    impl From<ShortDryGrass> for super::BlockState {
        fn from(value : ShortDryGrass) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:farmland` block.
pub mod farmland {
    /// `minecraft:farmland` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct Farmland {
        /// `moisture` state.
        pub moisture : Moisture,
    }
    /// `moisture` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Moisture {
        /// `0` variant.
        N0,
        /// `1` variant.
        N1,
        /// `2` variant.
        N2,
        /// `3` variant.
        N3,
        /// `4` variant.
        N4,
        /// `5` variant.
        N5,
        /// `6` variant.
        N6,
        /// `7` variant.
        N7,
    }
    impl super::Block for Farmland {
    }
    impl From<Farmland> for super::BlockState {
        fn from(value : Farmland) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:prismarine_stairs` block.
pub mod prismarine_stairs {
    /// `minecraft:prismarine_stairs` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct PrismarineStairs {
        /// `waterlogged` state.
        pub waterlogged : bool,
        /// `half` state.
        pub half : Half,
        /// `facing` state.
        pub facing : Facing,
        /// `shape` state.
        pub shape : Shape,
    }
    /// `half` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Half {
        /// `top` variant.
        Top,
        /// `bottom` variant.
        Bottom,
    }
    /// `facing` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Facing {
        /// `north` variant.
        North,
        /// `south` variant.
        South,
        /// `west` variant.
        West,
        /// `east` variant.
        East,
    }
    /// `shape` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Shape {
        /// `straight` variant.
        Straight,
        /// `inner_left` variant.
        InnerLeft,
        /// `inner_right` variant.
        InnerRight,
        /// `outer_left` variant.
        OuterLeft,
        /// `outer_right` variant.
        OuterRight,
    }
    impl super::Block for PrismarineStairs {
    }
    impl From<PrismarineStairs> for super::BlockState {
        fn from(value : PrismarineStairs) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:stripped_cherry_log` block.
pub mod stripped_cherry_log {
    /// `minecraft:stripped_cherry_log` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct StrippedCherryLog {
        /// `axis` state.
        pub axis : Axis,
    }
    /// `axis` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Axis {
        /// `x` variant.
        X,
        /// `y` variant.
        Y,
        /// `z` variant.
        Z,
    }
    impl super::Block for StrippedCherryLog {
    }
    impl From<StrippedCherryLog> for super::BlockState {
        fn from(value : StrippedCherryLog) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:potted_orange_tulip` block.
pub mod potted_orange_tulip {
    /// `minecraft:potted_orange_tulip` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct PottedOrangeTulip;
    impl super::Block for PottedOrangeTulip {
    }
    impl From<PottedOrangeTulip> for super::BlockState {
        fn from(value : PottedOrangeTulip) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:stripped_bamboo_block` block.
pub mod stripped_bamboo_block {
    /// `minecraft:stripped_bamboo_block` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct StrippedBambooBlock {
        /// `axis` state.
        pub axis : Axis,
    }
    /// `axis` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Axis {
        /// `x` variant.
        X,
        /// `y` variant.
        Y,
        /// `z` variant.
        Z,
    }
    impl super::Block for StrippedBambooBlock {
    }
    impl From<StrippedBambooBlock> for super::BlockState {
        fn from(value : StrippedBambooBlock) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:test_instance_block` block.
pub mod test_instance_block {
    /// `minecraft:test_instance_block` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct TestInstanceBlock;
    impl super::Block for TestInstanceBlock {
    }
    impl From<TestInstanceBlock> for super::BlockState {
        fn from(value : TestInstanceBlock) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:dark_oak_sign` block.
pub mod dark_oak_sign {
    /// `minecraft:dark_oak_sign` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct DarkOakSign {
        /// `rotation` state.
        pub rotation : Rotation,
        /// `waterlogged` state.
        pub waterlogged : bool,
    }
    /// `rotation` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Rotation {
        /// `0` variant.
        N0,
        /// `1` variant.
        N1,
        /// `2` variant.
        N2,
        /// `3` variant.
        N3,
        /// `4` variant.
        N4,
        /// `5` variant.
        N5,
        /// `6` variant.
        N6,
        /// `7` variant.
        N7,
        /// `8` variant.
        N8,
        /// `9` variant.
        N9,
        /// `10` variant.
        N10,
        /// `11` variant.
        N11,
        /// `12` variant.
        N12,
        /// `13` variant.
        N13,
        /// `14` variant.
        N14,
        /// `15` variant.
        N15,
    }
    impl super::Block for DarkOakSign {
    }
    impl From<DarkOakSign> for super::BlockState {
        fn from(value : DarkOakSign) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:sand` block.
pub mod sand {
    /// `minecraft:sand` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct Sand;
    impl super::Block for Sand {
    }
    impl From<Sand> for super::BlockState {
        fn from(value : Sand) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:tuff_slab` block.
pub mod tuff_slab {
    /// `minecraft:tuff_slab` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct TuffSlab {
        /// `type` state.
        pub kind : Kind,
        /// `waterlogged` state.
        pub waterlogged : bool,
    }
    /// `type` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Kind {
        /// `top` variant.
        Top,
        /// `bottom` variant.
        Bottom,
        /// `double` variant.
        Double,
    }
    impl super::Block for TuffSlab {
    }
    impl From<TuffSlab> for super::BlockState {
        fn from(value : TuffSlab) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:waxed_oxidized_chiseled_copper` block.
pub mod waxed_oxidized_chiseled_copper {
    /// `minecraft:waxed_oxidized_chiseled_copper` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct WaxedOxidizedChiseledCopper;
    impl super::Block for WaxedOxidizedChiseledCopper {
    }
    impl From<WaxedOxidizedChiseledCopper> for super::BlockState {
        fn from(value : WaxedOxidizedChiseledCopper) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:spruce_log` block.
pub mod spruce_log {
    /// `minecraft:spruce_log` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct SpruceLog {
        /// `axis` state.
        pub axis : Axis,
    }
    /// `axis` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Axis {
        /// `x` variant.
        X,
        /// `y` variant.
        Y,
        /// `z` variant.
        Z,
    }
    impl super::Block for SpruceLog {
    }
    impl From<SpruceLog> for super::BlockState {
        fn from(value : SpruceLog) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:daylight_detector` block.
pub mod daylight_detector {
    /// `minecraft:daylight_detector` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct DaylightDetector {
        /// `power` state.
        pub power : Power,
        /// `inverted` state.
        pub inverted : bool,
    }
    /// `power` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Power {
        /// `0` variant.
        N0,
        /// `1` variant.
        N1,
        /// `2` variant.
        N2,
        /// `3` variant.
        N3,
        /// `4` variant.
        N4,
        /// `5` variant.
        N5,
        /// `6` variant.
        N6,
        /// `7` variant.
        N7,
        /// `8` variant.
        N8,
        /// `9` variant.
        N9,
        /// `10` variant.
        N10,
        /// `11` variant.
        N11,
        /// `12` variant.
        N12,
        /// `13` variant.
        N13,
        /// `14` variant.
        N14,
        /// `15` variant.
        N15,
    }
    impl super::Block for DaylightDetector {
    }
    impl From<DaylightDetector> for super::BlockState {
        fn from(value : DaylightDetector) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:oak_log` block.
pub mod oak_log {
    /// `minecraft:oak_log` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct OakLog {
        /// `axis` state.
        pub axis : Axis,
    }
    /// `axis` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Axis {
        /// `x` variant.
        X,
        /// `y` variant.
        Y,
        /// `z` variant.
        Z,
    }
    impl super::Block for OakLog {
    }
    impl From<OakLog> for super::BlockState {
        fn from(value : OakLog) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:waxed_exposed_cut_copper_slab` block.
pub mod waxed_exposed_cut_copper_slab {
    /// `minecraft:waxed_exposed_cut_copper_slab` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct WaxedExposedCutCopperSlab {
        /// `waterlogged` state.
        pub waterlogged : bool,
        /// `type` state.
        pub kind : Kind,
    }
    /// `type` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Kind {
        /// `top` variant.
        Top,
        /// `bottom` variant.
        Bottom,
        /// `double` variant.
        Double,
    }
    impl super::Block for WaxedExposedCutCopperSlab {
    }
    impl From<WaxedExposedCutCopperSlab> for super::BlockState {
        fn from(value : WaxedExposedCutCopperSlab) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:waxed_oxidized_copper_trapdoor` block.
pub mod waxed_oxidized_copper_trapdoor {
    /// `minecraft:waxed_oxidized_copper_trapdoor` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct WaxedOxidizedCopperTrapdoor {
        /// `open` state.
        pub open : bool,
        /// `waterlogged` state.
        pub waterlogged : bool,
        /// `facing` state.
        pub facing : Facing,
        /// `powered` state.
        pub powered : bool,
        /// `half` state.
        pub half : Half,
    }
    /// `facing` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Facing {
        /// `north` variant.
        North,
        /// `south` variant.
        South,
        /// `west` variant.
        West,
        /// `east` variant.
        East,
    }
    /// `half` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Half {
        /// `top` variant.
        Top,
        /// `bottom` variant.
        Bottom,
    }
    impl super::Block for WaxedOxidizedCopperTrapdoor {
    }
    impl From<WaxedOxidizedCopperTrapdoor> for super::BlockState {
        fn from(value : WaxedOxidizedCopperTrapdoor) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:resin_block` block.
pub mod resin_block {
    /// `minecraft:resin_block` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct ResinBlock;
    impl super::Block for ResinBlock {
    }
    impl From<ResinBlock> for super::BlockState {
        fn from(value : ResinBlock) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:dead_brain_coral_block` block.
pub mod dead_brain_coral_block {
    /// `minecraft:dead_brain_coral_block` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct DeadBrainCoralBlock;
    impl super::Block for DeadBrainCoralBlock {
    }
    impl From<DeadBrainCoralBlock> for super::BlockState {
        fn from(value : DeadBrainCoralBlock) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:rooted_dirt` block.
pub mod rooted_dirt {
    /// `minecraft:rooted_dirt` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct RootedDirt;
    impl super::Block for RootedDirt {
    }
    impl From<RootedDirt> for super::BlockState {
        fn from(value : RootedDirt) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:bamboo_stairs` block.
pub mod bamboo_stairs {
    /// `minecraft:bamboo_stairs` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct BambooStairs {
        /// `half` state.
        pub half : Half,
        /// `shape` state.
        pub shape : Shape,
        /// `facing` state.
        pub facing : Facing,
        /// `waterlogged` state.
        pub waterlogged : bool,
    }
    /// `half` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Half {
        /// `top` variant.
        Top,
        /// `bottom` variant.
        Bottom,
    }
    /// `shape` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Shape {
        /// `straight` variant.
        Straight,
        /// `inner_left` variant.
        InnerLeft,
        /// `inner_right` variant.
        InnerRight,
        /// `outer_left` variant.
        OuterLeft,
        /// `outer_right` variant.
        OuterRight,
    }
    /// `facing` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Facing {
        /// `north` variant.
        North,
        /// `south` variant.
        South,
        /// `west` variant.
        West,
        /// `east` variant.
        East,
    }
    impl super::Block for BambooStairs {
    }
    impl From<BambooStairs> for super::BlockState {
        fn from(value : BambooStairs) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:piglin_wall_head` block.
pub mod piglin_wall_head {
    /// `minecraft:piglin_wall_head` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct PiglinWallHead {
        /// `facing` state.
        pub facing : Facing,
        /// `powered` state.
        pub powered : bool,
    }
    /// `facing` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Facing {
        /// `north` variant.
        North,
        /// `south` variant.
        South,
        /// `west` variant.
        West,
        /// `east` variant.
        East,
    }
    impl super::Block for PiglinWallHead {
    }
    impl From<PiglinWallHead> for super::BlockState {
        fn from(value : PiglinWallHead) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:pumpkin_stem` block.
pub mod pumpkin_stem {
    /// `minecraft:pumpkin_stem` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct PumpkinStem {
        /// `age` state.
        pub age : Age,
    }
    /// `age` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Age {
        /// `0` variant.
        N0,
        /// `1` variant.
        N1,
        /// `2` variant.
        N2,
        /// `3` variant.
        N3,
        /// `4` variant.
        N4,
        /// `5` variant.
        N5,
        /// `6` variant.
        N6,
        /// `7` variant.
        N7,
    }
    impl super::Block for PumpkinStem {
    }
    impl From<PumpkinStem> for super::BlockState {
        fn from(value : PumpkinStem) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:dispenser` block.
pub mod dispenser {
    /// `minecraft:dispenser` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct Dispenser {
        /// `facing` state.
        pub facing : Facing,
        /// `triggered` state.
        pub triggered : bool,
    }
    /// `facing` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Facing {
        /// `north` variant.
        North,
        /// `east` variant.
        East,
        /// `south` variant.
        South,
        /// `west` variant.
        West,
        /// `up` variant.
        Up,
        /// `down` variant.
        Down,
    }
    impl super::Block for Dispenser {
    }
    impl From<Dispenser> for super::BlockState {
        fn from(value : Dispenser) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:jungle_pressure_plate` block.
pub mod jungle_pressure_plate {
    /// `minecraft:jungle_pressure_plate` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct JunglePressurePlate {
        /// `powered` state.
        pub powered : bool,
    }
    impl super::Block for JunglePressurePlate {
    }
    impl From<JunglePressurePlate> for super::BlockState {
        fn from(value : JunglePressurePlate) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:fire` block.
pub mod fire {
    /// `minecraft:fire` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct Fire {
        /// `west` state.
        pub west : bool,
        /// `east` state.
        pub east : bool,
        /// `age` state.
        pub age : Age,
        /// `south` state.
        pub south : bool,
        /// `up` state.
        pub up : bool,
        /// `north` state.
        pub north : bool,
    }
    /// `age` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Age {
        /// `0` variant.
        N0,
        /// `1` variant.
        N1,
        /// `2` variant.
        N2,
        /// `3` variant.
        N3,
        /// `4` variant.
        N4,
        /// `5` variant.
        N5,
        /// `6` variant.
        N6,
        /// `7` variant.
        N7,
        /// `8` variant.
        N8,
        /// `9` variant.
        N9,
        /// `10` variant.
        N10,
        /// `11` variant.
        N11,
        /// `12` variant.
        N12,
        /// `13` variant.
        N13,
        /// `14` variant.
        N14,
        /// `15` variant.
        N15,
    }
    impl super::Block for Fire {
    }
    impl From<Fire> for super::BlockState {
        fn from(value : Fire) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:magenta_carpet` block.
pub mod magenta_carpet {
    /// `minecraft:magenta_carpet` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct MagentaCarpet;
    impl super::Block for MagentaCarpet {
    }
    impl From<MagentaCarpet> for super::BlockState {
        fn from(value : MagentaCarpet) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:mud_brick_stairs` block.
pub mod mud_brick_stairs {
    /// `minecraft:mud_brick_stairs` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct MudBrickStairs {
        /// `waterlogged` state.
        pub waterlogged : bool,
        /// `shape` state.
        pub shape : Shape,
        /// `half` state.
        pub half : Half,
        /// `facing` state.
        pub facing : Facing,
    }
    /// `shape` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Shape {
        /// `straight` variant.
        Straight,
        /// `inner_left` variant.
        InnerLeft,
        /// `inner_right` variant.
        InnerRight,
        /// `outer_left` variant.
        OuterLeft,
        /// `outer_right` variant.
        OuterRight,
    }
    /// `half` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Half {
        /// `top` variant.
        Top,
        /// `bottom` variant.
        Bottom,
    }
    /// `facing` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Facing {
        /// `north` variant.
        North,
        /// `south` variant.
        South,
        /// `west` variant.
        West,
        /// `east` variant.
        East,
    }
    impl super::Block for MudBrickStairs {
    }
    impl From<MudBrickStairs> for super::BlockState {
        fn from(value : MudBrickStairs) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:dripstone_block` block.
pub mod dripstone_block {
    /// `minecraft:dripstone_block` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct DripstoneBlock;
    impl super::Block for DripstoneBlock {
    }
    impl From<DripstoneBlock> for super::BlockState {
        fn from(value : DripstoneBlock) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:black_concrete_powder` block.
pub mod black_concrete_powder {
    /// `minecraft:black_concrete_powder` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct BlackConcretePowder;
    impl super::Block for BlackConcretePowder {
    }
    impl From<BlackConcretePowder> for super::BlockState {
        fn from(value : BlackConcretePowder) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:oxidized_copper_door` block.
pub mod oxidized_copper_door {
    /// `minecraft:oxidized_copper_door` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct OxidizedCopperDoor {
        /// `hinge` state.
        pub hinge : Hinge,
        /// `facing` state.
        pub facing : Facing,
        /// `open` state.
        pub open : bool,
        /// `half` state.
        pub half : Half,
        /// `powered` state.
        pub powered : bool,
    }
    /// `hinge` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Hinge {
        /// `left` variant.
        Left,
        /// `right` variant.
        Right,
    }
    /// `facing` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Facing {
        /// `north` variant.
        North,
        /// `south` variant.
        South,
        /// `west` variant.
        West,
        /// `east` variant.
        East,
    }
    /// `half` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Half {
        /// `upper` variant.
        Upper,
        /// `lower` variant.
        Lower,
    }
    impl super::Block for OxidizedCopperDoor {
    }
    impl From<OxidizedCopperDoor> for super::BlockState {
        fn from(value : OxidizedCopperDoor) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:potted_blue_orchid` block.
pub mod potted_blue_orchid {
    /// `minecraft:potted_blue_orchid` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct PottedBlueOrchid;
    impl super::Block for PottedBlueOrchid {
    }
    impl From<PottedBlueOrchid> for super::BlockState {
        fn from(value : PottedBlueOrchid) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:polished_deepslate` block.
pub mod polished_deepslate {
    /// `minecraft:polished_deepslate` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct PolishedDeepslate;
    impl super::Block for PolishedDeepslate {
    }
    impl From<PolishedDeepslate> for super::BlockState {
        fn from(value : PolishedDeepslate) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:barrel` block.
pub mod barrel {
    /// `minecraft:barrel` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct Barrel {
        /// `facing` state.
        pub facing : Facing,
        /// `open` state.
        pub open : bool,
    }
    /// `facing` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Facing {
        /// `north` variant.
        North,
        /// `east` variant.
        East,
        /// `south` variant.
        South,
        /// `west` variant.
        West,
        /// `up` variant.
        Up,
        /// `down` variant.
        Down,
    }
    impl super::Block for Barrel {
    }
    impl From<Barrel> for super::BlockState {
        fn from(value : Barrel) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:dead_bubble_coral_block` block.
pub mod dead_bubble_coral_block {
    /// `minecraft:dead_bubble_coral_block` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct DeadBubbleCoralBlock;
    impl super::Block for DeadBubbleCoralBlock {
    }
    impl From<DeadBubbleCoralBlock> for super::BlockState {
        fn from(value : DeadBubbleCoralBlock) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:deepslate_lapis_ore` block.
pub mod deepslate_lapis_ore {
    /// `minecraft:deepslate_lapis_ore` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct DeepslateLapisOre;
    impl super::Block for DeepslateLapisOre {
    }
    impl From<DeepslateLapisOre> for super::BlockState {
        fn from(value : DeepslateLapisOre) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:moss_carpet` block.
pub mod moss_carpet {
    /// `minecraft:moss_carpet` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct MossCarpet;
    impl super::Block for MossCarpet {
    }
    impl From<MossCarpet> for super::BlockState {
        fn from(value : MossCarpet) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:pink_concrete` block.
pub mod pink_concrete {
    /// `minecraft:pink_concrete` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct PinkConcrete;
    impl super::Block for PinkConcrete {
    }
    impl From<PinkConcrete> for super::BlockState {
        fn from(value : PinkConcrete) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:birch_door` block.
pub mod birch_door {
    /// `minecraft:birch_door` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct BirchDoor {
        /// `hinge` state.
        pub hinge : Hinge,
        /// `facing` state.
        pub facing : Facing,
        /// `half` state.
        pub half : Half,
        /// `open` state.
        pub open : bool,
        /// `powered` state.
        pub powered : bool,
    }
    /// `hinge` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Hinge {
        /// `left` variant.
        Left,
        /// `right` variant.
        Right,
    }
    /// `facing` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Facing {
        /// `north` variant.
        North,
        /// `south` variant.
        South,
        /// `west` variant.
        West,
        /// `east` variant.
        East,
    }
    /// `half` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Half {
        /// `upper` variant.
        Upper,
        /// `lower` variant.
        Lower,
    }
    impl super::Block for BirchDoor {
    }
    impl From<BirchDoor> for super::BlockState {
        fn from(value : BirchDoor) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:black_candle_cake` block.
pub mod black_candle_cake {
    /// `minecraft:black_candle_cake` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct BlackCandleCake {
        /// `lit` state.
        pub lit : bool,
    }
    impl super::Block for BlackCandleCake {
    }
    impl From<BlackCandleCake> for super::BlockState {
        fn from(value : BlackCandleCake) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:brown_banner` block.
pub mod brown_banner {
    /// `minecraft:brown_banner` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct BrownBanner {
        /// `rotation` state.
        pub rotation : Rotation,
    }
    /// `rotation` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Rotation {
        /// `0` variant.
        N0,
        /// `1` variant.
        N1,
        /// `2` variant.
        N2,
        /// `3` variant.
        N3,
        /// `4` variant.
        N4,
        /// `5` variant.
        N5,
        /// `6` variant.
        N6,
        /// `7` variant.
        N7,
        /// `8` variant.
        N8,
        /// `9` variant.
        N9,
        /// `10` variant.
        N10,
        /// `11` variant.
        N11,
        /// `12` variant.
        N12,
        /// `13` variant.
        N13,
        /// `14` variant.
        N14,
        /// `15` variant.
        N15,
    }
    impl super::Block for BrownBanner {
    }
    impl From<BrownBanner> for super::BlockState {
        fn from(value : BrownBanner) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:mossy_stone_brick_wall` block.
pub mod mossy_stone_brick_wall {
    /// `minecraft:mossy_stone_brick_wall` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct MossyStoneBrickWall {
        /// `up` state.
        pub up : bool,
        /// `west` state.
        pub west : West,
        /// `waterlogged` state.
        pub waterlogged : bool,
        /// `east` state.
        pub east : East,
        /// `north` state.
        pub north : North,
        /// `south` state.
        pub south : South,
    }
    /// `west` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum West {
        /// `none` variant.
        None,
        /// `low` variant.
        Low,
        /// `tall` variant.
        Tall,
    }
    /// `east` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum East {
        /// `none` variant.
        None,
        /// `low` variant.
        Low,
        /// `tall` variant.
        Tall,
    }
    /// `north` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum North {
        /// `none` variant.
        None,
        /// `low` variant.
        Low,
        /// `tall` variant.
        Tall,
    }
    /// `south` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum South {
        /// `none` variant.
        None,
        /// `low` variant.
        Low,
        /// `tall` variant.
        Tall,
    }
    impl super::Block for MossyStoneBrickWall {
    }
    impl From<MossyStoneBrickWall> for super::BlockState {
        fn from(value : MossyStoneBrickWall) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:pink_candle_cake` block.
pub mod pink_candle_cake {
    /// `minecraft:pink_candle_cake` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct PinkCandleCake {
        /// `lit` state.
        pub lit : bool,
    }
    impl super::Block for PinkCandleCake {
    }
    impl From<PinkCandleCake> for super::BlockState {
        fn from(value : PinkCandleCake) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:polished_blackstone_bricks` block.
pub mod polished_blackstone_bricks {
    /// `minecraft:polished_blackstone_bricks` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct PolishedBlackstoneBricks;
    impl super::Block for PolishedBlackstoneBricks {
    }
    impl From<PolishedBlackstoneBricks> for super::BlockState {
        fn from(value : PolishedBlackstoneBricks) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:acacia_planks` block.
pub mod acacia_planks {
    /// `minecraft:acacia_planks` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct AcaciaPlanks;
    impl super::Block for AcaciaPlanks {
    }
    impl From<AcaciaPlanks> for super::BlockState {
        fn from(value : AcaciaPlanks) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:infested_cracked_stone_bricks` block.
pub mod infested_cracked_stone_bricks {
    /// `minecraft:infested_cracked_stone_bricks` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct InfestedCrackedStoneBricks;
    impl super::Block for InfestedCrackedStoneBricks {
    }
    impl From<InfestedCrackedStoneBricks> for super::BlockState {
        fn from(value : InfestedCrackedStoneBricks) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:polished_tuff_slab` block.
pub mod polished_tuff_slab {
    /// `minecraft:polished_tuff_slab` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct PolishedTuffSlab {
        /// `waterlogged` state.
        pub waterlogged : bool,
        /// `type` state.
        pub kind : Kind,
    }
    /// `type` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Kind {
        /// `top` variant.
        Top,
        /// `bottom` variant.
        Bottom,
        /// `double` variant.
        Double,
    }
    impl super::Block for PolishedTuffSlab {
    }
    impl From<PolishedTuffSlab> for super::BlockState {
        fn from(value : PolishedTuffSlab) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:spruce_sign` block.
pub mod spruce_sign {
    /// `minecraft:spruce_sign` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct SpruceSign {
        /// `rotation` state.
        pub rotation : Rotation,
        /// `waterlogged` state.
        pub waterlogged : bool,
    }
    /// `rotation` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Rotation {
        /// `0` variant.
        N0,
        /// `1` variant.
        N1,
        /// `2` variant.
        N2,
        /// `3` variant.
        N3,
        /// `4` variant.
        N4,
        /// `5` variant.
        N5,
        /// `6` variant.
        N6,
        /// `7` variant.
        N7,
        /// `8` variant.
        N8,
        /// `9` variant.
        N9,
        /// `10` variant.
        N10,
        /// `11` variant.
        N11,
        /// `12` variant.
        N12,
        /// `13` variant.
        N13,
        /// `14` variant.
        N14,
        /// `15` variant.
        N15,
    }
    impl super::Block for SpruceSign {
    }
    impl From<SpruceSign> for super::BlockState {
        fn from(value : SpruceSign) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:end_stone` block.
pub mod end_stone {
    /// `minecraft:end_stone` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct EndStone;
    impl super::Block for EndStone {
    }
    impl From<EndStone> for super::BlockState {
        fn from(value : EndStone) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:white_shulker_box` block.
pub mod white_shulker_box {
    /// `minecraft:white_shulker_box` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct WhiteShulkerBox {
        /// `facing` state.
        pub facing : Facing,
    }
    /// `facing` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Facing {
        /// `north` variant.
        North,
        /// `east` variant.
        East,
        /// `south` variant.
        South,
        /// `west` variant.
        West,
        /// `up` variant.
        Up,
        /// `down` variant.
        Down,
    }
    impl super::Block for WhiteShulkerBox {
    }
    impl From<WhiteShulkerBox> for super::BlockState {
        fn from(value : WhiteShulkerBox) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:dead_brain_coral_fan` block.
pub mod dead_brain_coral_fan {
    /// `minecraft:dead_brain_coral_fan` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct DeadBrainCoralFan {
        /// `waterlogged` state.
        pub waterlogged : bool,
    }
    impl super::Block for DeadBrainCoralFan {
    }
    impl From<DeadBrainCoralFan> for super::BlockState {
        fn from(value : DeadBrainCoralFan) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:spawner` block.
pub mod spawner {
    /// `minecraft:spawner` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct Spawner;
    impl super::Block for Spawner {
    }
    impl From<Spawner> for super::BlockState {
        fn from(value : Spawner) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:quartz_stairs` block.
pub mod quartz_stairs {
    /// `minecraft:quartz_stairs` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct QuartzStairs {
        /// `facing` state.
        pub facing : Facing,
        /// `shape` state.
        pub shape : Shape,
        /// `half` state.
        pub half : Half,
        /// `waterlogged` state.
        pub waterlogged : bool,
    }
    /// `facing` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Facing {
        /// `north` variant.
        North,
        /// `south` variant.
        South,
        /// `west` variant.
        West,
        /// `east` variant.
        East,
    }
    /// `shape` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Shape {
        /// `straight` variant.
        Straight,
        /// `inner_left` variant.
        InnerLeft,
        /// `inner_right` variant.
        InnerRight,
        /// `outer_left` variant.
        OuterLeft,
        /// `outer_right` variant.
        OuterRight,
    }
    /// `half` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Half {
        /// `top` variant.
        Top,
        /// `bottom` variant.
        Bottom,
    }
    impl super::Block for QuartzStairs {
    }
    impl From<QuartzStairs> for super::BlockState {
        fn from(value : QuartzStairs) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:stripped_jungle_log` block.
pub mod stripped_jungle_log {
    /// `minecraft:stripped_jungle_log` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct StrippedJungleLog {
        /// `axis` state.
        pub axis : Axis,
    }
    /// `axis` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Axis {
        /// `x` variant.
        X,
        /// `y` variant.
        Y,
        /// `z` variant.
        Z,
    }
    impl super::Block for StrippedJungleLog {
    }
    impl From<StrippedJungleLog> for super::BlockState {
        fn from(value : StrippedJungleLog) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:hay_block` block.
pub mod hay_block {
    /// `minecraft:hay_block` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct HayBlock {
        /// `axis` state.
        pub axis : Axis,
    }
    /// `axis` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Axis {
        /// `x` variant.
        X,
        /// `y` variant.
        Y,
        /// `z` variant.
        Z,
    }
    impl super::Block for HayBlock {
    }
    impl From<HayBlock> for super::BlockState {
        fn from(value : HayBlock) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:end_portal` block.
pub mod end_portal {
    /// `minecraft:end_portal` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct EndPortal;
    impl super::Block for EndPortal {
    }
    impl From<EndPortal> for super::BlockState {
        fn from(value : EndPortal) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:dead_fire_coral` block.
pub mod dead_fire_coral {
    /// `minecraft:dead_fire_coral` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct DeadFireCoral {
        /// `waterlogged` state.
        pub waterlogged : bool,
    }
    impl super::Block for DeadFireCoral {
    }
    impl From<DeadFireCoral> for super::BlockState {
        fn from(value : DeadFireCoral) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:brown_terracotta` block.
pub mod brown_terracotta {
    /// `minecraft:brown_terracotta` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct BrownTerracotta;
    impl super::Block for BrownTerracotta {
    }
    impl From<BrownTerracotta> for super::BlockState {
        fn from(value : BrownTerracotta) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:andesite_slab` block.
pub mod andesite_slab {
    /// `minecraft:andesite_slab` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct AndesiteSlab {
        /// `waterlogged` state.
        pub waterlogged : bool,
        /// `type` state.
        pub kind : Kind,
    }
    /// `type` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Kind {
        /// `top` variant.
        Top,
        /// `bottom` variant.
        Bottom,
        /// `double` variant.
        Double,
    }
    impl super::Block for AndesiteSlab {
    }
    impl From<AndesiteSlab> for super::BlockState {
        fn from(value : AndesiteSlab) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:raw_iron_block` block.
pub mod raw_iron_block {
    /// `minecraft:raw_iron_block` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct RawIronBlock;
    impl super::Block for RawIronBlock {
    }
    impl From<RawIronBlock> for super::BlockState {
        fn from(value : RawIronBlock) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:warped_hyphae` block.
pub mod warped_hyphae {
    /// `minecraft:warped_hyphae` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct WarpedHyphae {
        /// `axis` state.
        pub axis : Axis,
    }
    /// `axis` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Axis {
        /// `x` variant.
        X,
        /// `y` variant.
        Y,
        /// `z` variant.
        Z,
    }
    impl super::Block for WarpedHyphae {
    }
    impl From<WarpedHyphae> for super::BlockState {
        fn from(value : WarpedHyphae) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:dropper` block.
pub mod dropper {
    /// `minecraft:dropper` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct Dropper {
        /// `triggered` state.
        pub triggered : bool,
        /// `facing` state.
        pub facing : Facing,
    }
    /// `facing` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Facing {
        /// `north` variant.
        North,
        /// `east` variant.
        East,
        /// `south` variant.
        South,
        /// `west` variant.
        West,
        /// `up` variant.
        Up,
        /// `down` variant.
        Down,
    }
    impl super::Block for Dropper {
    }
    impl From<Dropper> for super::BlockState {
        fn from(value : Dropper) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:beehive` block.
pub mod beehive {
    /// `minecraft:beehive` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct Beehive {
        /// `facing` state.
        pub facing : Facing,
        /// `honey_level` state.
        pub honey_level : HoneyLevel,
    }
    /// `facing` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Facing {
        /// `north` variant.
        North,
        /// `south` variant.
        South,
        /// `west` variant.
        West,
        /// `east` variant.
        East,
    }
    /// `honey_level` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum HoneyLevel {
        /// `0` variant.
        N0,
        /// `1` variant.
        N1,
        /// `2` variant.
        N2,
        /// `3` variant.
        N3,
        /// `4` variant.
        N4,
        /// `5` variant.
        N5,
    }
    impl super::Block for Beehive {
    }
    impl From<Beehive> for super::BlockState {
        fn from(value : Beehive) -> super::BlockState {
            todo!();
        }
    }
}

/// `minecraft:glow_lichen` block.
pub mod glow_lichen {
    /// `minecraft:glow_lichen` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct GlowLichen {
        /// `east` state.
        pub east : bool,
        /// `south` state.
        pub south : bool,
        /// `west` state.
        pub west : bool,
        /// `up` state.
        pub up : bool,
        /// `north` state.
        pub north : bool,
        /// `down` state.
        pub down : bool,
        /// `waterlogged` state.
        pub waterlogged : bool,
    }
    impl super::Block for GlowLichen {
    }
    impl From<GlowLichen> for super::BlockState {
        fn from(value : GlowLichen) -> super::BlockState {
            todo!();
        }
    }
}

