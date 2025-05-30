use super::{ ConnPeerState, ConnPeerComms, ConnPeerResult, ConnPeerError };
use octetmc_protocol::packet::login::c2s::hello::HelloC2SLoginPacket;
use octetmc_protocol::packet::login::s2c::login_compression::LoginCompressionS2CLoginPacket;
use rand::TryRngCore;
use core::time::Duration;
use rand::{ self, rngs::ThreadRng, RngCore };
use rsa::rand_core;
use rsa::{ RsaPrivateKey, RsaPublicKey };


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

    let mut rng = rand::rng();
    let (private_key, public_key,) = generate_key_pair(&mut rng, 2048);
    let verify_token               = generate_verify_token(&mut rng);

    todo!()
}


fn generate_key_pair(rng : &mut ThreadRng, bit_count : usize) -> (RsaPrivateKey, RsaPublicKey,) {
    let private_key = RsaPrivateKey::new(&mut GenKeyPairRng(rng), bit_count).unwrap();
    let public_key  = RsaPublicKey::from(&private_key);
    (private_key, public_key,)
}
struct GenKeyPairRng<'l>(&'l mut ThreadRng);
impl rand_core::CryptoRng for GenKeyPairRng<'_> {}
impl rand_core::RngCore for GenKeyPairRng<'_> {

    fn next_u32(&mut self) -> u32 { self.0.next_u32() }

    fn next_u64(&mut self) -> u64 { self.0.next_u64() }

    fn fill_bytes(&mut self, dest : &mut [u8]) { self.0.fill_bytes(dest); }

    fn try_fill_bytes(&mut self, dest : &mut [u8]) -> Result<(), rand_core::Error> {
        self.0.try_fill_bytes(dest).map_err(|_| unreachable!())
    }

}

fn generate_verify_token(rng : &mut ThreadRng) -> [u8; 4] {
    let mut buf = [0u8; 4];
    rng.fill_bytes(&mut buf);
    buf
}
