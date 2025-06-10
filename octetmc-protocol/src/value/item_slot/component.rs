use super::{ ItemSlot, ItemComponentType };
use crate::value::ident::Ident;
use crate::value::text::Text;
use crate::value::dim_block_pos::DimBlockPos;
use crate::value::rgb::Rgb;
use crate::value::sound_event::SoundEvent;
use std::borrow::Cow;


/// Unimplemented item components.
#[derive(Debug, Clone)]
pub enum TODO {}


/// Components that can be attached to items.
#[derive(Debug, Clone)]
pub enum ItemComponent<'l> {

    /// Customizable data that doesn't fit any specific component.
    CustomData(TODO),

    /// Maximum stack size for the item.
    MaxStackSize(u32),

    /// The maximum damage the item can take before breaking.
    MaxDamage(u32),

    /// The current damage of the item.
    Damage(u32),

    /// Marks the item as unbreakable.
    Unbreakable,

    /// Item's custom name.
    ///  Normally shown in italic, and changeable at an anvil.
    CustomName(Text<'l, 'l>),

    /// Override for the item's default name.
    ///  Shown when the item has no custom name.
    ItemName(Text<'l, 'l>),

    /// Item's model.
    ItemModel(Ident<'l>),

    /// Item's lore.
    Lore(Cow<'l, [Text<'l, 'l>]>),

    /// Item's rarity.
    ///  This affects the default color of the item's name.
    Rarity(ItemRarity),

    /// The enchantments of the item.
    Enchantments(TODO),

    /// List of blocks this block can be placed on when in adventure mode.
    CanPlaceOn(TODO),

    /// List of blocks this item can break when in adventure mode.
    CanBreak(TODO),

    /// The attribute modifiers of the item.
    AttributeModifiers(TODO),

    /// Value for the item predicate when using custom item models.
    ///  More info can be found here.
    CustomModelData(TODO),

    /// Allows you to hide all or parts of the item tooltip.
    TooltipDisplay {
        /// Whether to hide the tooltip entirely.
        hide_tooltip      : bool,
        /// The IDs of data components to hide.
        hidden_components : Cow<'l, [ItemComponentType]>
    },

    /// Accumulated anvil usage cost. The client displays "Too Expensive"
    ///  if the value is greater than 40 and the player is not in creative
    ///  mode (more specifically, if they don't have the insta-build flag
    ///  enabled).
    /// This behavior can be overridden by setting the level with the Set
    ///  Container Property packet.
    RepairCost(u32),

    /// Marks the item as non-interactive on the creative inventory (the
    ///  first 5 rows of items).
    /// This is used internally by the client on the paper icon in the
    ///  saved hot-bars tab.
    CreativeSlotLock,

    /// Overrides the item glint resulted from enchantments
    EnchantmentGlintOverride(bool),

    /// Marks the projectile as intangible (cannot be picked-up).
    IntangibleProjectile,

    /// Makes the item restore the player's hunger bar when consumed.
    Food(TODO),

    /// Makes the item consumable.
    Consumable(TODO),

    /// This specifies the item produced after using the current item. In
    ///  the Notchian server, this is used for stews, which turn into bowls.
    UseRemainder(ItemSlot<'l>),

    /// Cooldown to apply on use of the item.
    UseCooldown {
        /// How long the cooldown lasts.
        seconds : f32,
        /// Group of items to apply the cooldown to.
        ///  Defaults to the item's ID.
        group   : Option<Ident<'l>>
    },

    /// Marks this item as damage resistant.
    ///  The client won't render the item as being on-fire if this component
    ///  is present.
    DamageResistant {
        /// Tag specifying damage types the item is immune to. Not prefixed by `#`!.
        damage_type_tags : Ident<'l>
    },

    /// Alters the speed at which this item breaks certain blocks
    Tool(TODO),

    /// Item treated as a weapon
    Weapon {
        /// How much damage to deal
        damage_per_attack    : u32,
        /// How long to disable shields, in seconds
        disable_blocking_for : f32
    },

    /// Allows the item to be enchanted by an enchanting table.
    Enchantable(u32),

    /// Allows the item to be equipped by the player.
    Equippable(TODO),

    /// Items that can be combined with this item in an anvil to repair it.
    Repairable(TODO),

    /// Makes the item function like elytra.
    Glider,

    /// Custom textures for the item tooltip.
    TooltipStyle(Ident<'l>),

    /// Makes the item function like a totem of undying.
    DeathProtection {
        /// Effects to apply on consumption.
        effects : Cow<'l, [ConsumeEffect<'l>]>
    },

    /// Makes the item act like a shield.
    BlockAttacks(TODO),

    /// The enchantments stored in this enchanted book.
    StoredEnchantments(TODO),

    /// Colour of dyed leather armor.
    DyedColour(Rgb),

    /// Colour of the markings on the map item model.
    MapColour(Rgb),

    /// The ID of the map.
    MapId(u32),

    /// Icons present on a map.
    MapDecorations(TODO),

    /// Used internally by the client when expanding or locking a map. Display
    ///  extra information on the item's tooltip when the component is present.
    MapPostProcessing(MapPostProcessing),

    /// Projectiles loaded into a charged crossbow.
    ChargedProjectiles(Cow<'l, [ItemSlot<'l>]>),

    /// Contents of a bundle.
    BundleContents(Cow<'l, [ItemSlot<'l>]>),

    /// Visual and effects of a potion item.
    PotionContents(TODO),

    /// A duration multiplier for potion items.
    PotionDurationScale(f32),

    /// Effects granted by a suspicious stew.
    SuspiciousStewEffects(TODO),

    /// Content of a writable book.
    WritableBookContent(TODO),

    /// Content of a written and signed book.
    WrittenBookContent(TODO),

    /// Armor's trim pattern and color
    Trim(TODO),

    /// State of the debug stick
    DebugStickState(TODO),

    /// Data for the entity to be created from this item.
    EntityData(TODO),

    /// Data of the entity contained in this bucket.
    BucketEntityData(TODO),

    /// Data of the block entity to be created from this item.
    BlockEntityData(TODO),

    /// The sound played when using a goat horn.
    Instrument(TODO),

    /// Used to make an item into a valid armor trim material.
    ProvidesTrimMaterial(TODO),

    /// Amplifier for the effect of an ominous bottle.
    OminousBottleAmplifier(u8),

    /// The song this item will play when inserted into a jukebox.
    JukeboxPlayable(TODO),

    /// Used to make an item into a valid banner pattern material.
    ProvidesBannerPatterns(TODO),

    /// The recipes this knowledge book unlocks.
    Recipes(TODO),

    /// The lodestone this compass points to.
    LodestoneTracker {
        /// The location the compass points to.
        target_location : Option<DimBlockPos<'l>>,
        /// Whether the component is removed when the associated lodestone is broken.
        tracked         : bool
    },

    /// Properties of a firework star.
    FireworkExplosion(TODO),

    /// Properties of a firework.
    Fireworks(TODO),

    /// Game profile of a player's head.
    Profile(TODO),

    /// Sound played by a note block when this player's head is placed on top of it.
    NoteBlockSound(Ident<'l>),

    /// Patterns of a banner or banner applied to a shield.
    BannerPatterns(TODO),

    /// Base colour of the banner applied to a shield.
    BaseColour(TODO),

    /// Decorations on the four sides of a pot.
    PotDecorations(TODO),

    /// Items inside a container of any type.
    Container(Cow<'l, [ItemSlot<'l>]>),

    /// State of a block.
    BlockState(TODO),

    /// Bees inside a hive.
    Bees(TODO),

    /// Name of the necessary key to open this container.
    Lock(TODO),

    /// Loot table for an unopened container.
    ContainerLoot(TODO),

    /// Changes the sound that plays when the item breaks.
    BreakSound(SoundEvent<'l>),

    /// The biome variant of a villager.
    VillagerVariant(TODO),

    /// The variant of a wolf.
    WolfVariant(TODO),

    /// The type of sounds that a wolf makes.
    WolfSoundVariant(TODO),

    /// The dye colour of the wolf's collar.
    WolfCollar(TODO),

    /// The variant of a fox.
    FoxVariant(TODO),

    /// The size of a salmon.
    SalmonSize(TODO),

    /// The variant of a parrot.
    ParrotVariant(TODO),

    /// The pattern of a tropical fish.
    TropicalFishPattern(TODO),

    /// The base colour of a tropical fish.
    TropicalFishBaseColour(TODO),

    /// The pattern colour of a tropical fish.
    TropicalFishPatternColour(TODO),

    /// The variant of a mooshroom.
    MooshroomVariant(TODO),

    /// The variant of a rabbit.
    RabbitVariant(TODO),

    /// The variant of a pig.
    PigVariant(TODO),

    /// The variant of a cow.
    CowVariant(TODO),

    /// The variant of a chicken.
    ChickenVariant(TODO),

    /// The variant of a frog.
    FrogVariant(TODO),

    /// The variant of a horse.
    HorseVariant(TODO),

    /// The variant of a painting.
    PaintingVariant(TODO),

    /// The variant of a llama.
    LlamaVariant(TODO),

    /// The variant of an axolotl.
    AxolotlVariant(TODO),

    /// The variant of a cat.
    CatVariant(TODO),

    /// The dye colour of the cat's collar.
    CatCollar(TODO),

    /// The colour of a sheep.
    SheepColour(TODO),

    /// The colour of a shulker.
    ShulkerColour(TODO)

}


#[expect(unused_variables)]
impl ItemComponent<'_> {

    /// Convert the inner parts of this `ItemComponent` to their owned counterparts, or
    ///  take ownership if they are already owned. Returns the newly created
    ///  `ItemComponent<'static>`.
    #[inline]
    pub fn into_static_owned(self) -> ItemComponent<'static> { match (self) {
        Self::CustomData(todo) => todo!(),
        Self::MaxStackSize(_) => todo!(),
        Self::MaxDamage(_) => todo!(),
        Self::Damage(_) => todo!(),
        Self::Unbreakable => todo!(),
        Self::CustomName(text) => todo!(),
        Self::ItemName(text) => todo!(),
        Self::ItemModel(ident) => todo!(),
        Self::Lore(cow) => todo!(),
        Self::Rarity(item_rarity) => todo!(),
        Self::Enchantments(todo) => todo!(),
        Self::CanPlaceOn(todo) => todo!(),
        Self::CanBreak(todo) => todo!(),
        Self::AttributeModifiers(todo) => todo!(),
        Self::CustomModelData(todo) => todo!(),
        Self::TooltipDisplay { hide_tooltip, hidden_components } => todo!(),
        Self::RepairCost(_) => todo!(),
        Self::CreativeSlotLock => todo!(),
        Self::EnchantmentGlintOverride(_) => todo!(),
        Self::IntangibleProjectile => todo!(),
        Self::Food(todo) => todo!(),
        Self::Consumable(todo) => todo!(),
        Self::UseRemainder(item_slot) => todo!(),
        Self::UseCooldown { seconds, group } => todo!(),
        Self::DamageResistant { damage_type_tags } => todo!(),
        Self::Tool(todo) => todo!(),
        Self::Weapon { damage_per_attack, disable_blocking_for } => todo!(),
        Self::Enchantable(_) => todo!(),
        Self::Equippable(todo) => todo!(),
        Self::Repairable(todo) => todo!(),
        Self::Glider => todo!(),
        Self::TooltipStyle(ident) => todo!(),
        Self::DeathProtection { effects } => todo!(),
        Self::BlockAttacks(todo) => todo!(),
        Self::StoredEnchantments(todo) => todo!(),
        Self::DyedColour(rgb) => todo!(),
        Self::MapColour(rgb) => todo!(),
        Self::MapId(_) => todo!(),
        Self::MapDecorations(todo) => todo!(),
        Self::MapPostProcessing(map_post_processing) => todo!(),
        Self::ChargedProjectiles(cow) => todo!(),
        Self::BundleContents(cow) => todo!(),
        Self::PotionContents(todo) => todo!(),
        Self::PotionDurationScale(_) => todo!(),
        Self::SuspiciousStewEffects(todo) => todo!(),
        Self::WritableBookContent(todo) => todo!(),
        Self::WrittenBookContent(todo) => todo!(),
        Self::Trim(todo) => todo!(),
        Self::DebugStickState(todo) => todo!(),
        Self::EntityData(todo) => todo!(),
        Self::BucketEntityData(todo) => todo!(),
        Self::BlockEntityData(todo) => todo!(),
        Self::Instrument(todo) => todo!(),
        Self::ProvidesTrimMaterial(todo) => todo!(),
        Self::OminousBottleAmplifier(_) => todo!(),
        Self::JukeboxPlayable(todo) => todo!(),
        Self::ProvidesBannerPatterns(todo) => todo!(),
        Self::Recipes(todo) => todo!(),
        Self::LodestoneTracker { target_location, tracked } => todo!(),
        Self::FireworkExplosion(todo) => todo!(),
        Self::Fireworks(todo) => todo!(),
        Self::Profile(todo) => todo!(),
        Self::NoteBlockSound(ident) => todo!(),
        Self::BannerPatterns(todo) => todo!(),
        Self::BaseColour(todo) => todo!(),
        Self::PotDecorations(todo) => todo!(),
        Self::Container(cow) => todo!(),
        Self::BlockState(todo) => todo!(),
        Self::Bees(todo) => todo!(),
        Self::Lock(todo) => todo!(),
        Self::ContainerLoot(todo) => todo!(),
        Self::BreakSound(sound_event) => todo!(),
        Self::VillagerVariant(todo) => todo!(),
        Self::WolfVariant(todo) => todo!(),
        Self::WolfSoundVariant(todo) => todo!(),
        Self::WolfCollar(todo) => todo!(),
        Self::FoxVariant(todo) => todo!(),
        Self::SalmonSize(todo) => todo!(),
        Self::ParrotVariant(todo) => todo!(),
        Self::TropicalFishPattern(todo) => todo!(),
        Self::TropicalFishBaseColour(todo) => todo!(),
        Self::TropicalFishPatternColour(todo) => todo!(),
        Self::MooshroomVariant(todo) => todo!(),
        Self::RabbitVariant(todo) => todo!(),
        Self::PigVariant(todo) => todo!(),
        Self::CowVariant(todo) => todo!(),
        Self::ChickenVariant(todo) => todo!(),
        Self::FrogVariant(todo) => todo!(),
        Self::HorseVariant(todo) => todo!(),
        Self::PaintingVariant(todo) => todo!(),
        Self::LlamaVariant(todo) => todo!(),
        Self::AxolotlVariant(todo) => todo!(),
        Self::CatVariant(todo) => todo!(),
        Self::CatCollar(todo) => todo!(),
        Self::SheepColour(todo) => todo!(),
        Self::ShulkerColour(todo) => todo!(),
    } }

    /// Convert the inner parts of this `ItemComponent` to their owned counterparts.
    ///  Returns the newly created `ItemComponent<'static>`.
    #[inline]
    pub fn to_static_owned(&self) -> ItemComponent<'static> { match (self) {
        Self::CustomData(todo) => todo!(),
        Self::MaxStackSize(_) => todo!(),
        Self::MaxDamage(_) => todo!(),
        Self::Damage(_) => todo!(),
        Self::Unbreakable => todo!(),
        Self::CustomName(text) => todo!(),
        Self::ItemName(text) => todo!(),
        Self::ItemModel(ident) => todo!(),
        Self::Lore(cow) => todo!(),
        Self::Rarity(item_rarity) => todo!(),
        Self::Enchantments(todo) => todo!(),
        Self::CanPlaceOn(todo) => todo!(),
        Self::CanBreak(todo) => todo!(),
        Self::AttributeModifiers(todo) => todo!(),
        Self::CustomModelData(todo) => todo!(),
        Self::TooltipDisplay { hide_tooltip, hidden_components } => todo!(),
        Self::RepairCost(_) => todo!(),
        Self::CreativeSlotLock => todo!(),
        Self::EnchantmentGlintOverride(_) => todo!(),
        Self::IntangibleProjectile => todo!(),
        Self::Food(todo) => todo!(),
        Self::Consumable(todo) => todo!(),
        Self::UseRemainder(item_slot) => todo!(),
        Self::UseCooldown { seconds, group } => todo!(),
        Self::DamageResistant { damage_type_tags } => todo!(),
        Self::Tool(todo) => todo!(),
        Self::Weapon { damage_per_attack, disable_blocking_for } => todo!(),
        Self::Enchantable(_) => todo!(),
        Self::Equippable(todo) => todo!(),
        Self::Repairable(todo) => todo!(),
        Self::Glider => todo!(),
        Self::TooltipStyle(ident) => todo!(),
        Self::DeathProtection { effects } => todo!(),
        Self::BlockAttacks(todo) => todo!(),
        Self::StoredEnchantments(todo) => todo!(),
        Self::DyedColour(rgb) => todo!(),
        Self::MapColour(rgb) => todo!(),
        Self::MapId(_) => todo!(),
        Self::MapDecorations(todo) => todo!(),
        Self::MapPostProcessing(map_post_processing) => todo!(),
        Self::ChargedProjectiles(cow) => todo!(),
        Self::BundleContents(cow) => todo!(),
        Self::PotionContents(todo) => todo!(),
        Self::PotionDurationScale(_) => todo!(),
        Self::SuspiciousStewEffects(todo) => todo!(),
        Self::WritableBookContent(todo) => todo!(),
        Self::WrittenBookContent(todo) => todo!(),
        Self::Trim(todo) => todo!(),
        Self::DebugStickState(todo) => todo!(),
        Self::EntityData(todo) => todo!(),
        Self::BucketEntityData(todo) => todo!(),
        Self::BlockEntityData(todo) => todo!(),
        Self::Instrument(todo) => todo!(),
        Self::ProvidesTrimMaterial(todo) => todo!(),
        Self::OminousBottleAmplifier(_) => todo!(),
        Self::JukeboxPlayable(todo) => todo!(),
        Self::ProvidesBannerPatterns(todo) => todo!(),
        Self::Recipes(todo) => todo!(),
        Self::LodestoneTracker { target_location, tracked } => todo!(),
        Self::FireworkExplosion(todo) => todo!(),
        Self::Fireworks(todo) => todo!(),
        Self::Profile(todo) => todo!(),
        Self::NoteBlockSound(ident) => todo!(),
        Self::BannerPatterns(todo) => todo!(),
        Self::BaseColour(todo) => todo!(),
        Self::PotDecorations(todo) => todo!(),
        Self::Container(cow) => todo!(),
        Self::BlockState(todo) => todo!(),
        Self::Bees(todo) => todo!(),
        Self::Lock(todo) => todo!(),
        Self::ContainerLoot(todo) => todo!(),
        Self::BreakSound(sound_event) => todo!(),
        Self::VillagerVariant(todo) => todo!(),
        Self::WolfVariant(todo) => todo!(),
        Self::WolfSoundVariant(todo) => todo!(),
        Self::WolfCollar(todo) => todo!(),
        Self::FoxVariant(todo) => todo!(),
        Self::SalmonSize(todo) => todo!(),
        Self::ParrotVariant(todo) => todo!(),
        Self::TropicalFishPattern(todo) => todo!(),
        Self::TropicalFishBaseColour(todo) => todo!(),
        Self::TropicalFishPatternColour(todo) => todo!(),
        Self::MooshroomVariant(todo) => todo!(),
        Self::RabbitVariant(todo) => todo!(),
        Self::PigVariant(todo) => todo!(),
        Self::CowVariant(todo) => todo!(),
        Self::ChickenVariant(todo) => todo!(),
        Self::FrogVariant(todo) => todo!(),
        Self::HorseVariant(todo) => todo!(),
        Self::PaintingVariant(todo) => todo!(),
        Self::LlamaVariant(todo) => todo!(),
        Self::AxolotlVariant(todo) => todo!(),
        Self::CatVariant(todo) => todo!(),
        Self::CatCollar(todo) => todo!(),
        Self::SheepColour(todo) => todo!(),
        Self::ShulkerColour(todo) => todo!(),
    } }

}



/// Item's rarity.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub enum ItemRarity {
    /// Common (white)
    Common,
    /// Uncommon (yellow)
    Uncommon,
    /// Rare (aqua)
    Rare,
    /// Epic (pink)
    Epic
}


/// Item's consumable animation.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub enum ConsumableAnimation {
    /// None
    None,
    /// Eat
    Eat,
    /// Drink
    Drink,
    /// Block
    Block,
    /// Bow
    Bow,
    /// Spear
    Spear,
    /// Crossbow
    Crossbow,
    /// Spyglass
    Spyglass,
    /// Toot Horn
    TootHorn,
    /// Brush
    Brush
}


/// Effects to apply on consuming an item.
#[derive(Debug, Clone)]
pub enum ConsumeEffect<'l> {

    /// Apply effects.
    ApplyEffects {
        /// Effects to add.
        effects : TODO
    },

    /// Remove effects.
    RemoveEffects {
        /// Effects to remove.
        effects : TODO
    },

    /// Clear all effects.
    ClearAllEffects,

    /// Teleport randomly.
    TeleportRandomly {
        /// Maximum diameter to teleport the entity within.
        diameter : f32
    },

    /// Play sound
    PlaySound {
        /// The sound to play.
        sound : SoundEvent<'l>
    }

}


/// Used internally by the client when expanding or locking a map. Display
///  extra information on the item's tooltip when the component is present.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub enum MapPostProcessing {
    /// Lock
    Lock,
    /// Scale
    Scale
}
