use super::error::ConnPeerResult;
use super::state::ConnPeerState;
use super::comms::ConnPeerComms;
use crate::server::{ ServerBrand, ServerMotd, ServerFavicon };
use crate::player::{ PlayerCount, MaxPlayerCount };
use crate::util::dirty::Dirtyable;
use octetmc_protocol::{ LATEST_GAME_VERSION, PROTOCOL_VERSION };
use octetmc_protocol::value::rgb::Rgb;
use octetmc_protocol::value::text::{ Text, TextComponent, TextContent, TextStyle, TextInteract, TextColour };
use octetmc_protocol::packet::status::c2s::C2SStatusPackets;
use octetmc_protocol::packet::status::c2s::ping_request::PingRequestC2SStatusPacket;
use octetmc_protocol::packet::status::s2c::status_response::{ StatusResponseS2CStatusPacket, StatusVersion, StatusPlayers, FAVICON_PREFIX, MAX_PLAYERS };
use octetmc_protocol::packet::status::s2c::pong_response::PongResponseS2CStatusPacket;
use core::time::Duration;
use std::borrow::Cow;
use std::sync::LazyLock;
use bevy_defer::{ AsyncWorld, AsyncAccess };
use smol::lock::Mutex;


const REQUEST_TIMEOUT : Duration = Duration::from_millis(250);

const DEFAULT_MOTD    : Text     = Text { components : Cow::Borrowed(&[
    TextComponent {
        content  : TextContent::Literal { value : Cow::Borrowed("An ") },
        style    : TextStyle::NONE,
        interact : TextInteract::NONE,
        extra    : Cow::Borrowed(&[])
    },
    TextComponent {
        content  : TextContent::Literal { value : Cow::Borrowed("OctectMC") },
        style    : TextStyle {
            colour : Some(TextColour::Rgb(Rgb { r : 124, g : 68, b : 255 })),
            ..TextStyle::NONE
        },
        interact : TextInteract::NONE,
        extra    : Cow::Borrowed(&[])
    },
    TextComponent {
        content  : TextContent::Literal { value : Cow::Borrowed(" Server") },
        style    : TextStyle::NONE,
        interact : TextInteract::NONE,
        extra    : Cow::Borrowed(&[])
    }
]) };

const DEFAULT_FAVICON : &str     = "data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAMAAACdt4HsAAAABGdBTUEAALGPC/xhBQAAAAFzUkdCAK7OHOkAAAA8UExURUdwTP748/////////////////////////+/Af8BAVMTaQgADDsPSSQFL/jSvtajp/9NTf/GTIV+iXJXgEo/9IwAAAAHdFJOUwD80BaaZz7Q6pnNAAAC0klEQVRYw91X2XKDMAysANscvoD//9dKvm1IGtOXTtWG6TTserWSbPj6+uMxYvwGPoEQMD3HD8LF8FQE4s15aiEeamCIXzGMEOxRAiD06kI/S2IScHqC45EEFGDWNUl44kAUsEqUwB+UQK9rltBdiFGkDCQVAsb+DNL6cj2h28acgaTf7hxSBtJ9JObwOANHcfbWYaoywIDOHIZYg4CXnb2EFhzJQEfQWchkQRTQa0K0IOHJBPbIghRdJmQLIlyprk7gQpwlXimpcCLfuMgmdudhhquLCRUEV6y+9YNQwOlTjwNumEVZJtq7Lx5WcKl0c0u5WQ8Cqj5BBeeaxPtL7SJueFAQIqDqE6SHI8JdbKp2EZPWNUE1LHSkaVXHhi6WSYOpCXTd61gHGZaOP6ooA50ZDcFe1QFPRVCbg+UoCJDfNgRzaSNVxWwxgoIiBRKwtAR7URYkM2orojERHbAXghld4InAJOQWmTKAk4ClrAIyzvMO6QxFB2S1Pkbe1MggSwRT6flMEiInLy0IFHlLQrxZlqWcDQTsnmFK9xwVw3ak79BhTGCxZeuRq0iAlQh3jS3DkfIL+MU0zU85kA1iGoNPYJIPuKUG/Eh4SwT1bDGfg9Pg7+Q0D+ZADnmY9F967PJ4W28AMQfnA7AxrIUBAHR1ukYGQT8JqLd5LOQeGGg5R86n+Jjn02JOlMfb9qTKEpwRSOElcwz/F8GD/BsBztwgwYugdVm4Z2Reiw7L3wjwW8w8ZwqI6mMekOF3Alwh9DzfcDgntbFLib87poaGAd3YKRnElmDXQ7enFHVfw+D6Qi9NGHjx5Eo26J8JLLw86fmV4UqA68PLg55D68OFgDqVv3/FgP01gdU/vX64lxS93xNYDR+8vtDAQKIoCBwcPnhMcW9akSMSWKPTUH5KQf3nG0n74fgUHmYvN7Lv5oF1Pq+PHGdwoGHCC+O/ef389/ENv2s5bHHprKEAAAAASUVORK5CYII=";


static CACHE_LATEST_GAME_VERSION_STRING : LazyLock<String>      = LazyLock::new(|| LATEST_GAME_VERSION.to_string());
static CACHE_SERVER_BRAND               : Mutex<Option<String>> = Mutex::new(None);

static CACHE_SERVER_MOTD                : Mutex<Option<Text>>   = Mutex::new(None);

static CACHE_SERVER_FAVICON             : Mutex<Option<String>> = Mutex::new(None);


pub(super) async fn handle_requests(comms : &mut ConnPeerComms) -> ConnPeerResult {
    comms.set_state(ConnPeerState::Status);

    match (comms.read_packet_timeout::<C2SStatusPackets>(REQUEST_TIMEOUT).await?) {

        C2SStatusPackets::StatusRequest(_) => {

            let (mut cache_brand, mut cache_motd, mut cache_favicon,) = futures::join!(
                CACHE_SERVER_BRAND.lock(),
                CACHE_SERVER_MOTD.lock(),
                CACHE_SERVER_FAVICON.lock()
            );

            _ = AsyncWorld.resource::<ServerBrand>().get_mut(|r| {
                if (r.take_dirty()) { _ = cache_brand.insert(r.to_string()); }
            });
            let brand = cache_brand.as_ref().unwrap_or(&*CACHE_LATEST_GAME_VERSION_STRING);

            _ = AsyncWorld.resource::<ServerMotd>().get_mut(|r| {
                if (r.take_dirty()) { _ = cache_motd.insert((**r).clone()); }
            });
            let motd = Text { components : Cow::Borrowed(&cache_motd.as_ref().unwrap_or(&DEFAULT_MOTD).components) };

            _ = AsyncWorld.resource::<ServerFavicon>().get_mut(|r| {
                if (r.take_dirty()) { _ = cache_favicon.insert(format!("{FAVICON_PREFIX}{}", r.as_b64_png())); }
            });
            let favicon = cache_favicon.as_ref().map_or(DEFAULT_FAVICON, |s| s);

            let players = AsyncWorld.resource::<PlayerCount>().get(|r| r.0).ok();

            let max_players = AsyncWorld.resource::<MaxPlayerCount>().get_mut(|r| **r).unwrap_or(MAX_PLAYERS);

            comms.send_packet(&StatusResponseS2CStatusPacket {
                version : StatusVersion {
                    name     : Cow::Borrowed(brand),
                    protocol : PROTOCOL_VERSION
                },
                players : players.map(|online| StatusPlayers {
                    online,
                    max    : max_players,
                    sample : Cow::Borrowed(&[])
                }),
                motd                : Some(motd),
                favicon             : Some(Cow::Borrowed(favicon)),
                enforce_secure_chat : false,
                block_chat_reports  : true
            }).await?;

            let ping_request = comms.read_packet_timeout::<PingRequestC2SStatusPacket>(REQUEST_TIMEOUT).await?;
            comms.send_packet(&PongResponseS2CStatusPacket { timestamp : ping_request.timestamp }).await?;

            Ok(())
        },

        C2SStatusPackets::PingRequest(PingRequestC2SStatusPacket { timestamp }) => {
            comms.send_packet(&PongResponseS2CStatusPacket { timestamp }).await?;

            Ok(())
        }

    }
}
