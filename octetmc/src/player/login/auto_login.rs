use super::PlayerLoggingInEvent;
use crate::player::Player;
use crate::world::dimension::Dimension;
use crate::registry::{ SimpleVariant, PaintingVariant, WolfSoundVariant, WolfVariant, DamageType };
use octetmc_protocol::value::ident::Ident;
use octetmc_protocol::value::game_mode::GameMode;
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

    /// Default dimension to spawn the player in.
    pub dimension                   : Dimension<'static>,

    /// Whether to default to reduced debug info.
    pub reduced_debug_info          : bool,

    /// Whether to default to respawn screens.
    pub respawn_screens             : bool,

    /// The default game mode to log in as.
    pub game_mode                   : GameMode,

    /// The default cat variant registry.
    pub cat_variant_registry        : Cow<'static, [SimpleVariant<'static>]>,

    /// The default chicken variant registry.
    pub chicken_variant_registry    : Cow<'static, [SimpleVariant<'static>]>,

    /// The default cow variant registry.
    pub cow_variant_registry        : Cow<'static, [SimpleVariant<'static>]>,

    /// The default frog variant registry.
    pub frog_variant_registry       : Cow<'static, [SimpleVariant<'static>]>,

    /// The default painting variant registry.
    pub painting_variant_registry   : Cow<'static, [PaintingVariant<'static>]>,

    /// The default pig variant registry.
    pub pig_variant_registry        : Cow<'static, [SimpleVariant<'static>]>,

    /// The default wolf sound variant registry.
    pub wolf_sound_variant_registry : Cow<'static, [WolfSoundVariant<'static>]>,

    /// The default wolf variant registry.
    pub wolf_variant_registry       : Cow<'static, [WolfVariant<'static>]>,

    /// The default damage type registry.
    pub damage_type_registry        : Cow<'static, [DamageType<'static>]>

}

impl Default for PlayerAutoLoginPlugin {
    fn default() -> Self { Self {
        is_hardcore                 : false,
        dimension                   : Dimension::OVERWORLD_THE_VOID,
        reduced_debug_info          : false,
        respawn_screens             : true,
        game_mode                   : GameMode::Adventure,
        cat_variant_registry        : Cow::Borrowed(const { &[SimpleVariant::MINIMAL] }),
        chicken_variant_registry    : Cow::Borrowed(const { &[SimpleVariant::MINIMAL] }),
        cow_variant_registry        : Cow::Borrowed(const { &[SimpleVariant::MINIMAL] }),
        frog_variant_registry       : Cow::Borrowed(const { &[SimpleVariant::MINIMAL] }),
        painting_variant_registry   : Cow::Borrowed(const { &[PaintingVariant::MINIMAL] }),
        pig_variant_registry        : Cow::Borrowed(const { &[SimpleVariant::MINIMAL] }),
        wolf_sound_variant_registry : Cow::Borrowed(const { &[WolfSoundVariant::MINIMAL] }),
        wolf_variant_registry       : Cow::Borrowed(const { &[WolfVariant::MINIMAL ] }),
        damage_type_registry        : Cow::Borrowed(DamageType::VANILLA_DAMAGE_TYPES)
    } }
}

impl Plugin for PlayerAutoLoginPlugin {
    fn build(&self, app : &mut App) {
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
    for login in er_login.read() {
        if let Ok((player,)) = q_players.get(*login.player_id) {
            player.set_registry(Ident::new_vanilla("cat_variant"),        plugin.cat_variant_registry        .iter().map(|entry| entry.to_registry_entry().into_static_owned()).collect());
            player.set_registry(Ident::new_vanilla("chicken_variant"),    plugin.chicken_variant_registry    .iter().map(|entry| entry.to_registry_entry().into_static_owned()).collect());
            player.set_registry(Ident::new_vanilla("cow_variant"),        plugin.cow_variant_registry        .iter().map(|entry| entry.to_registry_entry().into_static_owned()).collect());
            player.set_registry(Ident::new_vanilla("frog_variant"),       plugin.frog_variant_registry       .iter().map(|entry| entry.to_registry_entry().into_static_owned()).collect());
            player.set_registry(Ident::new_vanilla("painting_variant"),   plugin.painting_variant_registry   .iter().map(|entry| entry.to_registry_entry().into_static_owned()).collect());
            player.set_registry(Ident::new_vanilla("pig_variant"),        plugin.pig_variant_registry        .iter().map(|entry| entry.to_registry_entry().into_static_owned()).collect());
            player.set_registry(Ident::new_vanilla("wolf_sound_variant"), plugin.wolf_sound_variant_registry .iter().map(|entry| entry.to_registry_entry().into_static_owned()).collect());
            player.set_registry(Ident::new_vanilla("wolf_variant"),       plugin.wolf_variant_registry       .iter().map(|entry| entry.to_registry_entry().into_static_owned()).collect());
            player.set_registry(Ident::new_vanilla("damage_type"),        plugin.damage_type_registry        .iter().map(|entry| entry.to_registry_entry().into_static_owned()).collect());
            player.login(
                plugin.is_hardcore,
                plugin.dimension.clone(),
                plugin.reduced_debug_info,
                plugin.respawn_screens,
                plugin.game_mode
            );
        }
    }
}
