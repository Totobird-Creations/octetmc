//! `0x4B` `respawn`


use crate::mapping_check;
use crate::value::varint::VarInt;
use crate::value::ident::Ident;
use crate::value::dim_block_pos::DimBlockPos;
use crate::value::game_mode::GameMode;
use crate::packet::{ Packet, StatePlay };
use crate::packet::encode::{ EncodeBuf, PacketEncode, PacketPartEncode };
use crate::packet::prefix_check::prefix_check_play_s2c;
use std::borrow::Cow;


mapping_check!("net.minecraft.network.protocol.game.ClientboundRespawnPacket", "ea4b98f6cb80f0c9fb704db1e5adeb3a46d44921be38cdb445de0c6896b8fb73");


/// TODO: Address
#[derive(Debug, Clone)]
pub struct RespawnS2CPlayPacket<'l> {

    /// The `minecraft:dimension_type `registry ID of the dimension the player is in.
    pub dimension_type     : u32,

    /// The name of the dimension the player is in.
    pub dimension          : Ident<'l>,

    /// Used client side for biome noise.
    ///
    /// Vanilla servers use the first 8 bytes of the SHA-256 hash of the world seed.
    pub hashed_seed        : u64,

    /// The game mode to spawn the player in.
    pub game_mode          : GameMode,

    /// The game mode the player used to be in.
    ///
    /// Clients use this for the debug (F3 + N & F3 + F4) game mode switch.
    pub previous_game_mode : Option<GameMode>,

    /// Whether the world is a [debug world](https://minecraft.wiki/w/Debug_mode).
    ///
    /// Debug mode worlds cannot be modified and have predefined blocks.
    pub is_debug           : bool,

    /// Whether the world is a [superflat world](https://minecraft.wiki/w/Superflat).
    ///
    /// Superflat worlds have different void fog, and a horizon at y=0 instead of y=63.
    pub is_superflat       : bool,

    /// The last death location of the player.
    pub death_location     : Option<DimBlockPos<'l>>,

    /// The number of ticks until the player can use the last used portal again.
    pub portal_cooldown    : u32,

    /// The world's sea level.
    ///
    /// Default is 64, unless superflat.
    pub sea_level          : i32,

    /// `true` for exiting the end, and other dimension changes.
    pub keep_attributes    : bool,

    /// `true` for other dimension changes.
    pub keep_metadata      : bool

}




impl crate::Sealed for RespawnS2CPlayPacket<'_> { }

impl Packet for RespawnS2CPlayPacket<'_> { }


impl RespawnS2CPlayPacket<'_> {

    /// Convert the inner parts of this packet to their owned counterparts, or
    ///  take ownership if they are already owned. Returns the newly created
    ///  `RespawnS2CPlayPacket<'static>`.
    pub fn into_static_owned(self) -> RespawnS2CPlayPacket<'static> {
        RespawnS2CPlayPacket {
            dimension_type       : self.dimension_type,
            dimension            : self.dimension.into_static_owned(),
            hashed_seed          : self.hashed_seed,
            game_mode            : self.game_mode,
            previous_game_mode   : self.previous_game_mode,
            is_debug             : self.is_debug,
            is_superflat         : self.is_superflat,
            death_location       : self.death_location.map(|death_location| death_location.into_static_owned()),
            portal_cooldown      : self.portal_cooldown,
            sea_level            : self.sea_level,
            keep_attributes      : self.keep_attributes,
            keep_metadata        : self.keep_metadata
        }
    }

    /// Convert the inner parts of this packet to their owned counterparts.
    ///  Returns the newly created `RespawnS2CPlayPacket<'static>`.
    pub fn to_static_owned(&self) -> RespawnS2CPlayPacket<'static> {
        RespawnS2CPlayPacket {
            dimension_type       : self.dimension_type,
            dimension            : self.dimension.to_static_owned(),
            hashed_seed          : self.hashed_seed,
            game_mode            : self.game_mode,
            previous_game_mode   : self.previous_game_mode,
            is_debug             : self.is_debug,
            is_superflat         : self.is_superflat,
            death_location       : self.death_location.as_ref().map(|death_location| death_location.to_static_owned()),
            portal_cooldown      : self.portal_cooldown,
            sea_level            : self.sea_level,
            keep_attributes      : self.keep_attributes,
            keep_metadata        : self.keep_metadata
        }
    }

}


impl PacketEncode for RespawnS2CPlayPacket<'_> {
    type State = StatePlay;

    const PREFIX : u8 = prefix_check_play_s2c!(respawn, 0x4B);

    fn predict_size(&self) -> usize {
        todo!()
    }

    fn encode(&self, buf : &mut EncodeBuf) {
        todo!()
    }

}
