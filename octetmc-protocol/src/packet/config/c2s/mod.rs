//! Packets sent by the client to the server in the config state.


use crate::value::ident::IdentDecodeError;
use crate::packet::StateConfig;
use crate::packet::decode::packet_decode_group;


pub mod custom_payload;

pub mod finish_configuration;


packet_decode_group!{
    type State     = StateConfig;
    type Error<'l> = IdentDecodeError;
    /// `C2SConfig`-type packets.
    pub enum C2SConfigPackets<'l> {
        /// `CustomPayloadC2SConfigPacket`
        CustomPayload(custom_payload::CustomPayloadC2SConfigPacket<'l>),
        /// `FinishConfigurationC2SConfigPacket`
        FinishConfiguration(finish_configuration::FinishConfigurationC2SConfigPacket)
    }
}
