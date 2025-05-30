use super::{ ConnPeerState, ConnPeerComms, ConnPeerResult, ConnPeerError };
use octetmc_protocol::packet::login::c2s::hello::HelloC2SLoginPacket;
use octetmc_protocol::packet::login::s2c::hello::HelloS2CLoginPacket;
use octetmc_protocol::packet::login::s2c::login_compression::LoginCompressionS2CLoginPacket;
use core::time::Duration;
use std::borrow::Cow;
use rand::{ self, rngs::ThreadRng, RngCore };
use openssl::pkey::{ PKey, Private as PrivateKey, Public as PublicKey };
use openssl::rsa::{ Rsa, Padding as RsaPadding };


const LOGIN_TIMEOUT : Duration = Duration::from_millis(250);

const SERVER_ID     : &str     = "octectmc";


pub(super) async fn handle_login_process(
    comms              : &mut ConnPeerComms,
    compress_threshold : Option<u32>,
    mojauth_enabled    : bool
) -> ConnPeerResult {
    comms.set_state(ConnPeerState::Login);

    let hello = comms.read_packet_timeout::<HelloC2SLoginPacket>(LOGIN_TIMEOUT).await?;

    if let Some(threshold) = compress_threshold {
        comms.send_packet(&LoginCompressionS2CLoginPacket { threshold }).await?;
        comms.set_compress_threshold(threshold);
    }

    let (private_key, public_key,) = generate_key_pair(2048);
    let verify_token               = generate_verify_token::<4>(&mut rand::rng());
    comms.send_packet(&HelloS2CLoginPacket {
        server_id       : SERVER_ID,
        public_key      : &public_key.public_key_to_der().unwrap(),
        verify_token    : &verify_token,
        mojauth_enabled,
    }).await?;

    todo!()
}


#[inline(always)]
fn generate_key_pair(bits : u32) -> (PKey<PrivateKey>, PKey<PublicKey>,) {
    let private = PKey::from_rsa(Rsa::generate(bits).unwrap()).unwrap();
    let public  = PKey::from_rsa(Rsa::public_key_from_der(&private.public_key_to_der().unwrap()).unwrap()).unwrap();
    (private, public,)
}

#[inline(always)]
fn generate_verify_token<const N : usize>(rng : &mut ThreadRng) -> [u8; N] {
    let mut buf = [0u8; N];
    rng.fill_bytes(&mut buf);
    buf
}
