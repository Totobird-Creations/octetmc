//! Packets sent by the server to the client in the config state.


use crate::packet::StateConfig;
use crate::packet::encode::packet_encode_group;


// TODO: cookie_request

pub mod custom_payload;

// TODO: disconnect

pub mod finish_configuration;

// TODO: keep_alive

// TODO: ping

// TODO: reset_chat

pub mod registry_data;

// TODO: resource_pack_pop

// TODO: resource_pack_push

// TODO: store_cookie

// TODO: transfer

// TODO: update_enabled_features

// TODO: update_tags

pub mod select_known_packs;

// TODO: custom_report_details

// TODO: server_links

// TODO: clear_dialog

// TODO: show_dialog


packet_encode_group!{
    type State = StateConfig;
    /// `S2CConfig`-type packets.
    pub enum S2CConfigPackets<'l> {
        /// `CustomPayloadS2CConfigPacket`
        CustomPayload(custom_payload::CustomPayloadS2CConfigPacket<'l>),
        /// `FinishConfigurationS2CConfigPacket`
        FinishConfiguration(finish_configuration::FinishConfigurationS2CConfigPacket),
        /// `RegistryDataS2CConfigPacket`
        RegistryData(registry_data::RegistryDataS2CConfigPacket<'l>),
        /// `SelectKnownPacksS2CConfigPacket`
        SelectKnownPacks(select_known_packs::SelectKnownPacksS2CConfigPacket<'l>)
    }
}
