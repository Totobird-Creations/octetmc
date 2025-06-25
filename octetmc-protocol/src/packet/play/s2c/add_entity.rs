//! `0x01` `add_entity`


use crate::mapping_check;
use crate::value::varint::VarInt;
use crate::value::character_id::CharacterId;
use crate::value::character_pos::CharacterPos;
use crate::value::character_vel::CharacterVel;
use crate::value::entity_type::EntityType;
use crate::packet::{ Packet, StatePlay };
use crate::packet::encode::{ EncodeBuf, PacketEncode, PacketPartEncode };
use crate::packet::prefix_check::prefix_check_play_s2c;
use core::f64::consts::TAU;
use uuid::Uuid;


mapping_check!("net.minecraft.network.protocol.game.ClientboundAddEntityPacket", "be18ba5946684f25c0d4bafe5b19b54b904e613bd1ea03a916f6f1443f418d01");


/// TODO: Address
#[derive(Debug, Clone, Copy)]
pub struct AddEntityS2CPlayPacket {

    /// The networked character ID of this integer.
    ///
    /// This ID is used again later to update information about this entity.
    ///  The ID must be unique for the peer.
    pub eid      : CharacterId,

    /// The UUID of the entity.
    pub uuid     : Uuid,

    /// The type of entity.
    pub kind     : EntityType,

    /// The starting position of the entity.
    pub pos      : CharacterPos,

    /// The starting velocity of the entity.
    pub vel      : CharacterVel,

    /// The starting body yaw of the entity.
    ///
    /// Rotation is in radians.
    /// `0.0` is positive z (south),
    /// `PI/2.0` is negative x (west).
    /// `PI` is negative z (north).
    /// `-PI/2.0` is positive x (east).
    pub body_yaw : f64,

    /// TODO: Docs
    pub data     : u32

}


impl crate::Sealed for AddEntityS2CPlayPacket { }

impl Packet for AddEntityS2CPlayPacket { }


impl PacketEncode for AddEntityS2CPlayPacket {
    type State = StatePlay;

    const PREFIX : u8 = prefix_check_play_s2c!(add_entity, 0x01);

    #[inline(always)]
    fn predict_size(&self) -> usize {
        VarInt::<u32>::MAX_BYTES
        + self.uuid.predict_size()
        + VarInt::<u32>::MAX_BYTES
        + (size_of::<f64>() * 3)
        + 1
        + 1
        + 1
        + VarInt::<u32>::MAX_BYTES
        + (size_of::<i16>() * 3)
    }

    #[inline(always)]
    fn encode(&self, buf : &mut EncodeBuf) {
        buf.encode_write(VarInt::<u32>::from(*self.eid));
        buf.encode_write(self.uuid);
        buf.encode_write(VarInt::<u32>::from(self.kind.id()));
        buf.encode_write(&self.pos.x);
        buf.encode_write(&self.pos.y);
        buf.encode_write(&self.pos.z);
        buf.encode_write((&self.pos.pitch.rem_euclid(TAU) * (256.0 / TAU)) as u8);
        buf.encode_write((&self.body_yaw.rem_euclid(TAU) * (256.0 / TAU)) as u8);
        buf.encode_write((&self.pos.yaw.rem_euclid(TAU) * (256.0 / TAU)) as u8);
        buf.encode_write(VarInt::<u32>::from(self.data));
        todo!("encode velocity as i16");
    }
}
