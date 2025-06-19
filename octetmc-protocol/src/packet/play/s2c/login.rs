//! `0x2B` `login`


use crate::mapping_check;
use crate::value::varint::VarInt;
use crate::value::ident::Ident;
use crate::value::dim_block_pos::DimBlockPos;
use crate::value::game_mode::GameMode;
use crate::packet::{ Packet, StatePlay };
use crate::packet::encode::{ EncodeBuf, PacketEncode, PacketPartEncode };
use crate::packet::prefix_check::prefix_check_play_s2c;
use std::borrow::Cow;


mapping_check!("net.minecraft.network.protocol.game.ClientboundLoginPacket", "6ddb0e240b7eb82986b73c49ccbbd27b96d8880422d7b9ad9e36e7b09750d1f0");


/// <https://minecraft.wiki/w/Java_Edition_protocol/Packets#Login_(play)>
#[derive(Debug, Clone)]
pub struct LoginS2CPlayPacket<'l> {

    /// The player's entity ID.
    pub entity_id            : u32,

    /// Whether the server is in hardcore mode.
    pub is_hardcore          : bool,

    /// Identifiers for all dimensions on the server.
    pub dimensions           : Cow<'l, [Ident<'l>]>,

    /// Unused by the vanilla client.
    pub max_players          : u32,

    /// Maximum render distance (2~32).
    pub view_distance        : u8,

    /// Maximum distance for certain processes such as entity ticking.
    pub sim_distance         : u8,

    /// Whether to display reduced information in the F3 debug menu.
    pub reduced_debug_info   : bool,

    /// Whether death will show the respawn screen.
    ///
    /// If `false`, the server should skip the death screen and
    ///  immediately respawn the player
    pub respawn_screens      : bool,

    /// Unused by the vanilla client.
    pub limited_crafting     : bool,

    /// The `minecraft:dimension_type `registry ID of the dimension the player is in.
    pub dimension_type       : u32,

    /// The name of the dimension the player is in.
    pub dimension            : Ident<'l>,

    /// Used client side for biome noise.
    ///
    /// Vanilla servers use the first 8 bytes of the SHA-256 hash of the world seed.
    pub hashed_seed          : u64,

    /// The game mode to spawn the player in.
    pub game_mode            : GameMode,

    /// The game mode the player used to be in.
    ///
    /// Clients use this for the debug (F3 + N & F3 + F4) game mode switch.
    pub previous_game_mode   : Option<GameMode>,

    /// Whether the world is a [debug world](https://minecraft.wiki/w/Debug_mode).
    ///
    /// Debug mode worlds cannot be modified and have predefined blocks.
    pub is_debug             : bool,

    /// Whether the world is a [superflat world](https://minecraft.wiki/w/Superflat).
    ///
    /// Superflat worlds have different void fog, and a horizon at y=0 instead of y=63.
    pub is_superflat         : bool,

    /// The last death location of the player.
    pub death_location       : Option<DimBlockPos<'l>>,

    /// The number of ticks until the player can use the last used portal again.
    pub portal_cooldown      : u32,

    /// The world's sea level.
    ///
    /// Default is 64, unless superflat.
    pub sea_level            : i32,

    /// Whether this server enforces 'secure chat'.
    ///
    /// octectmc-protocol does not support 'secure chat'.
    pub enforces_secure_chat : bool

}


impl crate::Sealed for LoginS2CPlayPacket<'_> { }

impl Packet for LoginS2CPlayPacket<'_> { }


impl LoginS2CPlayPacket<'_> {

    /// Convert the inner parts of this packet to their owned counterparts, or
    ///  take ownership if they are already owned. Returns the newly created
    ///  `LoginS2CPlayPacket<'static>`.
    pub fn into_static_owned(self) -> LoginS2CPlayPacket<'static> {
        LoginS2CPlayPacket {
            entity_id            : self.entity_id,
            is_hardcore          : self.is_hardcore,
            dimensions           : Cow::Owned(match (self.dimensions) {
                Cow::Borrowed(dimensions) => dimensions.iter().map(|dimension| dimension.to_static_owned()).collect(),
                Cow::Owned(dimensions)    => dimensions.into_iter().map(|dimension| dimension.into_static_owned()).collect()
            }),
            max_players          : self.max_players,
            view_distance        : self.view_distance,
            sim_distance         : self.sim_distance,
            reduced_debug_info   : self.reduced_debug_info,
            respawn_screens      : self.respawn_screens,
            limited_crafting     : self.limited_crafting,
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
            enforces_secure_chat : self.enforces_secure_chat
        }
    }

    /// Convert the inner parts of this packet to their owned counterparts.
    ///  Returns the newly created `LoginS2CPlayPacket<'static>`.
    pub fn to_static_owned(&self) -> LoginS2CPlayPacket<'static> {
        LoginS2CPlayPacket {
            entity_id            : self.entity_id,
            is_hardcore          : self.is_hardcore,
            dimensions           : Cow::Owned(self.dimensions.iter().map(|dimension| dimension.to_static_owned()).collect()),
            max_players          : self.max_players,
            view_distance        : self.view_distance,
            sim_distance         : self.sim_distance,
            reduced_debug_info   : self.reduced_debug_info,
            respawn_screens      : self.respawn_screens,
            limited_crafting     : self.limited_crafting,
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
            enforces_secure_chat : self.enforces_secure_chat
        }
    }

}


impl PacketEncode for LoginS2CPlayPacket<'_> {
    type State = StatePlay;

    const PREFIX : u8 = prefix_check_play_s2c!(login, 0x2B);

    fn predict_size(&self) -> usize {
        self.entity_id.predict_size()
        + 1
        + VarInt::<u32>::MAX_BYTES
        + self.dimensions.iter().map(|dimension| dimension.predict_size()).sum::<usize>()
        + VarInt::<u32>::MAX_BYTES
        + VarInt::<u32>::MAX_BYTES
        + VarInt::<u32>::MAX_BYTES
        + 1
        + 1
        + 1
        + VarInt::<u32>::MAX_BYTES
        + self.dimension.predict_size()
        + self.hashed_seed.predict_size()
        + 1
        + 1
        + 1
        + 1
        + self.death_location.predict_size()
        + VarInt::<u32>::MAX_BYTES
        + VarInt::<i32>::MAX_BYTES
        + 1
    }

    fn encode(&self, buf : &mut EncodeBuf) {
        buf.encode_write(self.entity_id);
        buf.write(self.is_hardcore);
        buf.encode_write(VarInt::<u32>::from(self.dimensions.len() as u32));
        for dimension in &*self.dimensions { buf.encode_write(dimension); }
        buf.encode_write(VarInt::<u32>::from(self.max_players));
        buf.encode_write(VarInt::<u32>::from(self.view_distance as u32));
        buf.encode_write(VarInt::<u32>::from(self.sim_distance as u32));
        buf.write(self.reduced_debug_info);
        buf.write(self.respawn_screens);
        buf.write(self.limited_crafting);
        buf.encode_write(VarInt::<u32>::from(self.dimension_type));
        buf.encode_write(&self.dimension);
        buf.encode_write(self.hashed_seed);
        buf.write(self.game_mode as u8);
        buf.write(self.previous_game_mode.map_or(u8::MAX, |gamemode| gamemode as u8));
        buf.write(self.is_debug);
        buf.write(self.is_superflat);
        buf.encode_write(&self.death_location);
        buf.encode_write(VarInt::<u32>::from(self.portal_cooldown));
        buf.encode_write(VarInt::<i32>::from(self.sea_level));
        buf.write(self.enforces_secure_chat);
    }

}
