//! `0x02` `login_finished`


use crate::mapping_check;
use crate::value::profile::PlayerProfile;
use crate::packet::{ Packet, StateLogin};
use crate::packet::encode::{ EncodeBuf, PacketEncode, PacketPartEncode };
use crate::packet::prefix_check::prefix_check_login_s2c;


mapping_check!("net.minecraft.network.protocol.login.ClientboundLoginFinishedPacket", "6dafe5ded47752e5b830ff574f6e751712a3cfebeb37e3778e55ad024d3a881c");


/// <https://minecraft.wiki/w/Java_Edition_protocol/Packets#Login_Success>
#[derive(Debug, Clone)]
pub struct LoginFinishedS2CLoginPacket<'l> {

    /// The player's profile.
    ///
    /// This does not have to match the profile sent in `HelloC2SLoginPacket`.
    pub profile : PlayerProfile<'l>

}


impl crate::Sealed for LoginFinishedS2CLoginPacket<'_> { }

impl Packet for LoginFinishedS2CLoginPacket<'_> { }


impl LoginFinishedS2CLoginPacket<'_> {

    /// Convert the inner parts of this packet to their owned counterparts, or
    ///  take ownership if they are already owned. Returns the newly created
    ///  `LoginFinishedS2CLoginPacket<'static>`.
    #[inline]
    pub fn into_static_owned(self) -> LoginFinishedS2CLoginPacket<'static> {
        LoginFinishedS2CLoginPacket { profile : self.profile.into_static_owned() }
    }

    /// Convert the inner parts of this packet to their owned counterparts.
    ///  Returns the newly created `LoginFinishedS2CLoginPacket<'static>`.
    #[inline]
    pub fn to_static_owned(&self) -> LoginFinishedS2CLoginPacket<'static> {
        LoginFinishedS2CLoginPacket { profile : self.profile.to_static_owned() }
    }

}


impl PacketEncode for LoginFinishedS2CLoginPacket<'_> {
    type State = StateLogin;

    const PREFIX : u8 = prefix_check_login_s2c!(login_finished, 0x02);

    #[inline]
    fn predict_size(&self) -> usize {
        self.profile.predict_size()
    }

    #[inline(always)]
    fn encode(&self, buf : &mut EncodeBuf) {
        buf.encode_write(&self.profile);
    }
}
