//! `0x22` `game_event`


use crate::mapping_check;
use crate::value::game_mode::GameMode;
use crate::packet::{ Packet, StatePlay };
use crate::packet::encode::{ PacketEncode, EncodeBuf };
use crate::packet::prefix_check::prefix_check_play_s2c;


mapping_check!("net.minecraft.network.protocol.game.ClientboundGameEventPacket", "885fb51bea327f9df0a11a1ad797d9cfb957c48a2162b1e7444ef76014fbc645");
mapping_check!("net.minecraft.network.protocol.game.ClientboundGameEventPacket$Type", "87181ce0703194f202652578e96e0f74b00b0e1de399cc1e46f96060f8a33353");


/// <https://minecraft.wiki/w/Java_Edition_protocol/Packets#Game_Event>
#[derive(Debug, Clone, Copy)]
pub enum GameEventS2CPlayPacket {

    /// Displays `block.minecraft.spawn.not_valid` to the player
    ///  (`You have no home bed or charged respawn anchor, or it was obstructed`).
    NoRespawnBlock,

    /// Start raining.
    StartRain,

    /// Stop raining.
    StopRain,

    /// Changes the player's game mode.
    ChangeGameMode {
        /// The game mode to switch to.
        game_mode : GameMode
    },

    /// The player entered an end portal in the end.
    WinGame {
        /// Whether to roll credits.
        ///
        /// If `false`, the player will just be respawned.
        roll_credits : bool
    },

    /// Show a demo message.
    DemoEvent(DemoEvent),

    /// Sent when any player is struck by an arrow.
    ArrowHitPlayer,

    /// Set the sky colour and lighting.
    ///
    /// Ranges from `0.0` to `1.0`.
    SetRainLevel(f32),

    /// Set the sky colour and lighting.
    ///
    /// Ranges from `0.0` to `1.0`.
    SetThunderLevel(f32),

    /// Plays the pufferfish sting sound.
    PufferfishStingSound,

    /// Plays the elder guardian effect and sound.
    ElderGuardianEffect,

    /// Sets whether respawn screens are enabled.
    SetRespawnScreens {
        /// Whether a respawn screen is shown on death.
        enabled : bool
    },

    /// Sets whether crafting is restricted to the recipe book.
    SetLimitedCrafting {
        /// Whether recipes must be unlocked.
        enabled : bool
    },

    /// Instructs the client to begin the waiting process for the level chunks.
    ///
    /// Sent by the server after the level is cleared on the client and is being
    ///  re-sent (either during the first, or subsequent reconfigurations).
    WaitForChunks

}


/// Show a demo message.
#[derive(Debug, Clone, Copy)]
pub enum DemoEvent {

    /// Show welcome to demo screen.
    Welcome,

    /// Tell movement controls.
    MovementControls,

    /// Tell jump control.
    JumpControl,

    /// Tell inventory control.
    InventoryControl,

    /// Show demo over and tell screenshot control.
    DemoOver

}


impl crate::Sealed for GameEventS2CPlayPacket { }

impl Packet for GameEventS2CPlayPacket { }


impl PacketEncode for GameEventS2CPlayPacket {
    type State = StatePlay;

    const PREFIX : u8 = prefix_check_play_s2c!(game_event, 0x22);

    #[inline(always)]
    fn predict_size(&self) -> usize {
        1 + size_of::<f32>()
    }

    fn encode(&self, buf : &mut EncodeBuf) {
        let (event, value,) = { match (self) {
            Self::NoRespawnBlock                        => (0u8, None,),
            Self::StartRain                             => (1, None,),
            Self::StopRain                              => (2, None,),
            Self::ChangeGameMode       { game_mode }    => (3, Some(*game_mode as u8 as f32)),
            Self::WinGame              { roll_credits } => (4, Some(if (*roll_credits) { 1.0 } else { 0.0 })),
            Self::DemoEvent            (demo_event)     => (5, Some(match (demo_event) {
                DemoEvent::Welcome          => 0.0,
                DemoEvent::MovementControls => 101.0,
                DemoEvent::JumpControl      => 102.0,
                DemoEvent::InventoryControl => 103.0,
                DemoEvent::DemoOver         => 104.0
            },)),
            Self::ArrowHitPlayer                        => (6, None,),
            Self::SetRainLevel         (v)              => (7, Some(*v),),
            Self::SetThunderLevel      (v)              => (8, Some(*v),),
            Self::PufferfishStingSound                  => (9, None,),
            Self::ElderGuardianEffect                   => (10, None,),
            Self::SetRespawnScreens    { enabled }      => (11, Some(if (*enabled) { 0.0 } else { 1.0 })),
            Self::SetLimitedCrafting   { enabled }      => (12, Some(if (*enabled) { 1.0 } else { 0.0 })),
            Self::WaitForChunks                         => (13, None,),
        } };
        buf.write(event);
        buf.encode_write(value.unwrap_or(0.0f32));
    }
}
