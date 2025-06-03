use crate::player::{ Player, PlayerId };
use crate::player::info::{ PlayerInfoUpdated, PlayerChannelDataReceived };
use crate::util::CratePrivateNew;
use octetmc_protocol::value::client_info::ClientInfo;
use octetmc_protocol::value::channel_data::ChannelData;
use bevy_ecs::entity::Entity;
use bevy_ecs::system::Query;
use bevy_ecs::event::EventWriter;


pub(crate) enum ConnPeerInMessage {

    SetClientInfo(ClientInfo<'static>),

    RecieveChannelData(ChannelData<'static>)

}


pub(crate) fn handle_in_messages(
    mut q_players       : Query<(Entity, &mut Player,)>,
    mut ew_update_info  : EventWriter<PlayerInfoUpdated>,
    mut ew_channel_data : EventWriter<PlayerChannelDataReceived>
) {
    for (player_id, mut player,) in &mut q_players {
        while let Ok(message) = player.try_read_in_message() { match (message) {


            ConnPeerInMessage::SetClientInfo(info) => {
                player.set_info(info);
                ew_update_info.write(PlayerInfoUpdated::crate_private_new(PlayerId::from(player_id)));
            },


            ConnPeerInMessage::RecieveChannelData(data) => {
                match (data) {

                    ChannelData::Brand  { brand } => {
                        player.set_brand(brand.into_owned());
                        ew_update_info.write(PlayerInfoUpdated::crate_private_new(PlayerId::from(player_id)));
                    },

                    ChannelData::Custom { channel, data } => {
                        ew_channel_data.write(PlayerChannelDataReceived { player_id : PlayerId::from(player_id), channel, data : data.into_owned() });
                    }

                }
            }


        } }
    }
}
