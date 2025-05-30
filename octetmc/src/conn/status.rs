use super::{ ConnPeerComms, ConnPeerResult, ConnPeerError };
use super::comms::{ ConnPeerState };
use crate::server::{ ServerBrand, ServerMotd, ServerFavicon };
use crate::player::PlayerCount;
use crate::util::dirty::Dirtyable;
use octetmc_protocol::{ LATEST_GAME_VERSION, PROTOCOL_VERSION };
use octetmc_protocol::value::text::Text;
use octetmc_protocol::packet::status::c2s::C2SStatusPackets;
use octetmc_protocol::packet::status::c2s::ping_request::PingRequestC2SStatusPacket;
use octetmc_protocol::packet::status::s2c::status_response::*;
use core::time::Duration;
use std::borrow::Cow;
use std::sync::LazyLock;
use bevy_defer::{ AsyncWorld, AsyncAccess };
use smol::lock::Mutex;


const REQUEST_TIMEOUT : Duration = Duration::from_millis(250);


static CACHE_LATEST_GAME_VERSION_STRING : LazyLock<String>      = LazyLock::new(|| LATEST_GAME_VERSION.to_string());
static CACHE_SERVER_BRAND               : Mutex<Option<String>> = Mutex::new(None);

static CACHE_SERVER_MOTD                : Mutex<Option<Text>>   = Mutex::new(None);

static CACHE_SERVER_FAVICON             : Mutex<Option<String>> = Mutex::new(None);


pub(super) async fn handle_requests(comms : &mut ConnPeerComms) -> ConnPeerResult<()> {
    comms.set_state(ConnPeerState::Status);

    match (*comms.read_packet_timeout::<C2SStatusPackets>(REQUEST_TIMEOUT).await?) {

        C2SStatusPackets::StatusRequest(_) => {

            let (mut cache_brand, mut cache_motd, mut cache_favicon,) = futures::join!(
                CACHE_SERVER_BRAND.lock(),
                CACHE_SERVER_MOTD.lock(),
                CACHE_SERVER_FAVICON.lock()
            );

            let _ = AsyncWorld.resource::<ServerBrand>().get_mut(|r| {
                if (r.take_dirty()) { let _ = cache_brand.insert(r.to_string()); }
            });
            let brand = cache_brand.as_ref().unwrap_or(&*CACHE_LATEST_GAME_VERSION_STRING);

            let _ = AsyncWorld.resource::<ServerMotd>().get_mut(|r| {
                if (r.take_dirty()) { let _ = cache_motd.insert((**r).clone()); }
            });
            let motd = cache_motd.as_ref();

            let _ = AsyncWorld.resource::<ServerFavicon>().get_mut(|r| {
                if (r.take_dirty()) { let _ = cache_favicon.insert(format!("{FAVICON_PREFIX}{}", r.as_b64_png())); }
            });
            let favicon = cache_favicon.as_ref().map(|s| Cow::Borrowed(s.as_str()));

            let players = AsyncWorld.resource::<PlayerCount>().get(|r| r.0).ok();

            let packet = StatusResponseS2CStatusPacket {
                version : StatusVersion {
                    name     : Cow::Borrowed(brand),
                    protocol : PROTOCOL_VERSION
                },
                players : players.map(|count| StatusPlayers {
                    online : count as u32,
                    max    : MAX_PLAYERS,
                    sample : Cow::Borrowed(&[])
                }),
                motd,
                favicon,
                enforce_secure_chat : false,
                block_chat_reports  : true
            };

            Ok(())
        },

        C2SStatusPackets::PingRequest(PingRequestC2SStatusPacket { timestamp }) => {
            Err(ConnPeerError::OperationCompleted)
        }

    }
}
