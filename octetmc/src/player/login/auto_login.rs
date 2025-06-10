use super::PlayerLoggingInEvent;
use crate::player::Player;
use crate::world::dimension::Dimension;
use octetmc_protocol::value::ident::Ident;
use octetmc_protocol::value::game_mode::GameMode;
use octetmc_protocol::registry::{
    cat_variant::CatVariant,
    chicken_variant::ChickenVariant,
    cow_variant::CowVariant,
    frog_variant::FrogVariant,
    painting_variant::PaintingVariant,
    pig_variant::PigVariant,
    wolf_sound_variant::WolfSoundVariant,
    wolf_variant::WolfVariant,
    damage_type::DamageType,
    worldgen::biome::Biome
};
use std::borrow::Cow;
use bevy_app::{ App, Plugin, Update };
use bevy_ecs::system::{ Query, Res };
use bevy_ecs::resource::Resource;
use bevy_ecs::event::EventReader;


/// Bevy [`Plugin`] which automatically logs players in.
#[derive(Clone)]
pub struct PlayerAutoLoginPlugin {

    /// Whether to log the player in hardcode mode.
    pub is_hardcore                 : bool,

    /// Whether to default to reduced debug info.
    pub reduced_debug_info          : bool,

    /// Whether to default to respawn screens.
    pub respawn_screens             : bool,

    /// The default game mode to log in as.
    pub game_mode                   : GameMode,

    /// The default cat variant registry.
    pub cat_variant_registry        : Cow<'static, [CatVariant<'static>]>,

    /// The default chicken variant registry.
    pub chicken_variant_registry    : Cow<'static, [ChickenVariant<'static>]>,

    /// The default cow variant registry.
    pub cow_variant_registry        : Cow<'static, [CowVariant<'static>]>,

    /// The default damage type registry.
    pub damage_type_registry        : Cow<'static, [DamageType<'static>]>,

    /// The default dimensions.
    ///
    /// The player will be spawned in the first dimension.
    pub dimension_registry          : Cow<'static, [Dimension<'static>]>,

    /// The default frog variant registry.
    pub frog_variant_registry       : Cow<'static, [FrogVariant<'static>]>,

    /// The default painting variant registry.
    pub painting_variant_registry   : Cow<'static, [PaintingVariant<'static>]>,

    /// The default pig variant registry.
    pub pig_variant_registry        : Cow<'static, [PigVariant<'static>]>,

    /// The default wolf sound variant registry.
    pub wolf_sound_variant_registry : Cow<'static, [WolfSoundVariant<'static>]>,

    /// The default wolf variant registry.
    pub wolf_variant_registry       : Cow<'static, [WolfVariant<'static>]>,

    /// The default biome registry.
    pub biome_registry              : Cow<'static, [Biome<'static>]>

}

impl Default for PlayerAutoLoginPlugin {
    fn default() -> Self { Self {
        is_hardcore                 : false,
        reduced_debug_info          : false,
        respawn_screens             : true,
        game_mode                   : GameMode::Adventure,
        cat_variant_registry        : Cow::Borrowed(CatVariant::VANILLA_CAT_VARIANTS),
        chicken_variant_registry    : Cow::Borrowed(ChickenVariant::VANILLA_CHICKEN_VARIANTS),
        cow_variant_registry        : Cow::Borrowed(CowVariant::VANILLA_COW_VARIANTS),
        damage_type_registry        : Cow::Borrowed(DamageType::VANILLA_DAMAGE_TYPES),
        dimension_registry          : Cow::Borrowed(Dimension::VANILLA_SUPERFLAT_DIMENSIONS),
        frog_variant_registry       : Cow::Borrowed(FrogVariant::VANILLA_FROG_VARIANTS),
        painting_variant_registry   : Cow::Borrowed(PaintingVariant::VANILLA_PAINTING_VARIANTS),
        pig_variant_registry        : Cow::Borrowed(PigVariant::VANILLA_PIG_VARIANTS),
        wolf_sound_variant_registry : Cow::Borrowed(WolfSoundVariant::VANILLA_WOLF_SOUND_VARIANTS),
        wolf_variant_registry       : Cow::Borrowed(WolfVariant::VANILLA_WOLF_VARIANTS),
        biome_registry              : Cow::Borrowed(Biome::VANILLA_BIOMES)
    } }
}

impl Plugin for PlayerAutoLoginPlugin {
    fn build(&self, app : &mut App) {
        assert!(! self.dimension_registry.is_empty());
        app .insert_resource(LoginPlugin(self.clone()))
            .add_systems(Update, login_players);
    }
}


#[derive(Resource)]
struct LoginPlugin(PlayerAutoLoginPlugin);

fn login_players(
    mut er_login  : EventReader<PlayerLoggingInEvent>,
        q_players : Query<(&Player,)>,
        r_plugin  : Res<LoginPlugin>
) {
    let plugin = &r_plugin.0;
    er_login.par_read().for_each(|login| {
        if let Ok((player,)) = q_players.get(*login.player_id) {
            player.set_registry(Ident::vanilla_str("cat_variant"),        plugin.cat_variant_registry        .iter().map(|entry| entry.to_registry_entry().into_static_owned()).collect());
            player.set_registry(Ident::vanilla_str("chicken_variant"),    plugin.chicken_variant_registry    .iter().map(|entry| entry.to_registry_entry().into_static_owned()).collect());
            player.set_registry(Ident::vanilla_str("cow_variant"),        plugin.cow_variant_registry        .iter().map(|entry| entry.to_registry_entry().into_static_owned()).collect());
            player.set_registry(Ident::vanilla_str("frog_variant"),       plugin.frog_variant_registry       .iter().map(|entry| entry.to_registry_entry().into_static_owned()).collect());
            player.set_registry(Ident::vanilla_str("painting_variant"),   plugin.painting_variant_registry   .iter().map(|entry| entry.to_registry_entry().into_static_owned()).collect());
            player.set_registry(Ident::vanilla_str("pig_variant"),        plugin.pig_variant_registry        .iter().map(|entry| entry.to_registry_entry().into_static_owned()).collect());
            player.set_registry(Ident::vanilla_str("wolf_sound_variant"), plugin.wolf_sound_variant_registry .iter().map(|entry| entry.to_registry_entry().into_static_owned()).collect());
            player.set_registry(Ident::vanilla_str("wolf_variant"),       plugin.wolf_variant_registry       .iter().map(|entry| entry.to_registry_entry().into_static_owned()).collect());
            player.set_registry(Ident::vanilla_str("damage_type"),        plugin.damage_type_registry        .iter().map(|entry| entry.to_registry_entry().into_static_owned()).collect());
            player.set_registry(Ident::vanilla_str("dimension_type"),     plugin.dimension_registry          .iter().map(|entry| entry.kind.to_registry_entry().into_static_owned()).collect());
            player.set_registry(Ident::vanilla_str("worldgen/biome"),     plugin.biome_registry              .iter().map(|entry| entry.to_registry_entry().into_static_owned()).collect());
            player.login(
                plugin.is_hardcore,
                plugin.dimension_registry[0].clone(),
                plugin.reduced_debug_info,
                plugin.respawn_screens,
                plugin.game_mode
            );
        }
    });
}
