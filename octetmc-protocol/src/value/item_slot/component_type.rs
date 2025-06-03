/// Types of components that can be attached to items.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum ItemComponentType {

    /// Customizable data that doesn't fit any specific component.
    CustomData,

    /// Maximum stack size for the item.
    MaxStackSize,

    /// The maximum damage the item can take before breaking.
    MaxDamage,

    /// The current damage of the item.
    Damage,

    /// Marks the item as unbreakable.
    Unbreakable,

    /// Item's custom name.
    ///  Normally shown in italic, and changeable at an anvil.
    CustomName,

    /// Override for the item's default name.
    ///  Shown when the item has no custom name.
    ItemName,

    /// Item's model.
    ItemModel,

    /// Item's lore.
    Lore,

    /// Item's rarity.
    ///  This affects the default color of the item's name.
    Rarity,

    /// The enchantments of the item.
    Enchantments,

    /// List of blocks this block can be placed on when in adventure mode.
    CanPlaceOn,

    /// List of blocks this item can break when in adventure mode.
    CanBreak,

    /// The attribute modifiers of the item.
    AttributeModifiers,

    /// Value for the item predicate when using custom item models.
    ///  More info can be found here.
    CustomModelData,

    /// Allows you to hide all or parts of the item tooltip.
    TooltipDisplay,

    /// Accumulated anvil usage cost. The client displays "Too Expensive"
    ///  if the value is greater than 40 and the player is not in creative
    ///  mode (more specifically, if they don't have the insta-build flag
    ///  enabled).
    /// This behavior can be overridden by setting the level with the Set
    ///  Container Property packet.
    RepairCost,

    /// Marks the item as non-interactive on the creative inventory (the
    ///  first 5 rows of items).
    /// This is used internally by the client on the paper icon in the
    ///  saved hot-bars tab.
    CreativeSlotLock,

    /// Overrides the item glint resulted from enchantments
    EnchantmentGlintOverride,

    /// Marks the projectile as intangible .
    IntangibleProjectile,

    /// Makes the item restore the player's hunger bar when consumed.
    Food,

    /// Makes the item consumable.
    Consumable,

    /// This specifies the item produced after using the current item. In
    ///  the Notchian server, this is used for stews, which turn into bowls.
    UseRemainder,

    /// Cooldown to apply on use of the item.
    UseCooldown,

    /// Marks this item as damage resistant.
    ///  The client won't render the item as being on-fire if this component
    ///  is present.
    DamageResistant,

    /// Alters the speed at which this item breaks certain blocks
    Tool,

    /// Item treated as a weapon
    Weapon,

    /// Allows the item to be enchanted by an enchanting table.
    Enchantable,

    /// Allows the item to be equipped by the player.
    Equippable,

    /// Items that can be combined with this item in an anvil to repair it.
    Repairable,

    /// Makes the item function like elytra.
    Glider,

    /// Custom textures for the item tooltip.
    TooltipStyle,

    /// Makes the item function like a totem of undying.
    DeathProtection,

    /// Makes the item act like a shield.
    BlockAttacks,

    /// The enchantments stored in this enchanted book.
    StoredEnchantments,

    /// Colour of dyed leather armor.
    DyedColour,

    /// Colour of the markings on the map item model.
    MapColour,

    /// The ID of the map.
    MapId,

    /// Icons present on a map.
    MapDecorations,

    /// Used internally by the client when expanding or locking a map. Display
    ///  extra information on the item's tooltip when the component is present.
    MapPostProcessing,

    /// Projectiles loaded into a charged crossbow.
    ChargedProjectiles,

    /// Contents of a bundle.
    BundleContents,

    /// Visual and effects of a potion item.
    PotionContents,

    /// A duration multiplier for potion items.
    PotionDurationScale,

    /// Effects granted by a suspicious stew.
    SuspiciousStewEffects,

    /// Content of a writable book.
    WritableBookContent,

    /// Content of a written and signed book.
    WrittenBookContent,

    /// Armor's trim pattern and color
    Trim,

    /// State of the debug stick
    DebugStickState,

    /// Data for the entity to be created from this item.
    EntityData,

    /// Data of the entity contained in this bucket.
    BucketEntityData,

    /// Data of the block entity to be created from this item.
    BlockEntityData,

    /// The sound played when using a goat horn.
    Instrument,

    /// Used to make an item into a valid armor trim material.
    ProvidesTrimMaterial,

    /// Amplifier for the effect of an ominous bottle.
    OminousBottleAmplifier,

    /// The song this item will play when inserted into a jukebox.
    JukeboxPlayable,

    /// Used to make an item into a valid banner pattern material.
    ProvidesBannerPatterns,

    /// The recipes this knowledge book unlocks.
    Recipes,

    /// The lodestone this compass points to.
    LodestoneTracker,

    /// Properties of a firework star.
    FireworkExplosion,

    /// Properties of a firework.
    Fireworks,

    /// Game profile of a player's head.
    Profile,

    /// Sound played by a note block when this player's head is placed on top of it.
    NoteBlockSound,

    /// Patterns of a banner or banner applied to a shield.
    BannerPatterns,

    /// Base colour of the banner applied to a shield.
    BaseColour,

    /// Decorations on the four sides of a pot.
    PotDecorations,

    /// Items inside a container of any type.
    Container,

    /// State of a block.
    BlockState,

    /// Bees inside a hive.
    Bees,

    /// Name of the necessary key to open this container.
    Lock,

    /// Loot table for an unopened container.
    ContainerLoot,

    /// Changes the sound that plays when the item breaks.
    BreakSound,

    /// The biome variant of a villager.
    VillagerVariant,

    /// The variant of a wolf.
    WolfVariant,

    /// The type of sounds that a wolf makes.
    WolfSoundVariant,

    /// The dye colour of the wolf's collar.
    WolfCollar,

    /// The variant of a fox.
    FoxVariant,

    /// The size of a salmon.
    SalmonSize,

    /// The variant of a parrot.
    ParrotVariant,

    /// The pattern of a tropical fish.
    TropicalFishPattern,

    /// The base colour of a tropical fish.
    TropicalFishBaseColour,

    /// The pattern colour of a tropical fish.
    TropicalFishPatternColour,

    /// The variant of a mooshroom.
    MooshroomVariant,

    /// The variant of a rabbit.
    RabbitVariant,

    /// The variant of a pig.
    PigVariant,

    /// The variant of a cow.
    CowVariant,

    /// The variant of a chicken.
    ChickenVariant,

    /// The variant of a frog.
    FrogVariant,

    /// The variant of a horse.
    HorseVariant,

    /// The variant of a painting.
    PaintingVariant,

    /// The variant of a llama.
    LlamaVariant,

    /// The variant of an axolotl.
    AxolotlVariant,

    /// The variant of a cat.
    CatVariant,

    /// The dye colour of the cat's collar.
    CatCollar,

    /// The colour of a sheep.
    SheepColour,

    /// The colour of a shulker.
    ShulkerColour

}
