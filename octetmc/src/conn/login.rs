use super::{ ConnPeerState, ConnPeerComms, ConnPeerResult, ConnPeerError };
use octetmc_protocol::packet::login::c2s::hello::HelloC2SLoginPacket;
use octetmc_protocol::packet::login::c2s::key::KeyC2SLoginPacket;
use octetmc_protocol::packet::login::s2c::hello::HelloS2CLoginPacket;
use octetmc_protocol::packet::login::s2c::login_compression::LoginCompressionS2CLoginPacket;
use core::time::Duration;
use core::ptr;
use rand::{ self, rngs::ThreadRng, RngCore };
use openssl::pkey::{ PKey, Private as PrivateKey, Public as PublicKey };
use openssl::rsa::{ Padding as RsaPadding, Rsa };
use openssl::encrypt::Decrypter;
use openssl::symm::{ Crypter, Cipher, Mode as CrypterMode };
use openssl::sha::Sha1;


const LOGIN_TIMEOUT : Duration = Duration::from_millis(250);

const SERVER_ID     : &str     = "octectmc";

const MOJAUTH_URL_PREFIX   : &str = "https://sessionserver.mojang.com/session/minecraft/hasJoined?username=";
const MOJAUTH_URL_SERVERID : &str = "&serverId=";


pub(super) async fn handle_login_process(
    comms              : &mut ConnPeerComms,
    compress_threshold : Option<u32>,
    mojauth_enabled    : bool
) -> ConnPeerResult {
    comms.set_state(ConnPeerState::Login);

    // Wait for hello.
    let hello = comms.read_packet_timeout::<HelloC2SLoginPacket>(LOGIN_TIMEOUT).await?;
    if (hello.username.len() > 16) {
        return Err(ConnPeerError::UsernameTooLong);
    }

    // TODO: Set compression threshold.
    // if let Some(threshold) = compress_threshold {
    //     comms.send_packet(&LoginCompressionS2CLoginPacket { threshold }).await?;
    //     comms.set_compress_threshold(threshold);
    // }

    // Generate and send public key.
    let (private_key, public_key,) = generate_key_pair(2048);
    let verify_token               = generate_verify_token::<4>(&mut rand::rng());
    let public_key_der             = public_key.public_key_to_der().unwrap();
    comms.send_packet(&HelloS2CLoginPacket {
        server_id       : SERVER_ID,
        public_key      : &public_key_der,
        verify_token    : &verify_token,
        mojauth_enabled,
    }).await?;

    // Wait for key response.
    let key = comms.read_packet_timeout::<KeyC2SLoginPacket>(LOGIN_TIMEOUT).await?;

    // Create new pkey decrypter.
    let mut decrypter = Decrypter::new(&private_key).unwrap();
    let _ = decrypter.set_rsa_padding(RsaPadding::PKCS1);

    // Decrypt and compare verify token.
    decrypt!(&decrypter, key.verify_token => decrypted_verify_token);
    if (decrypted_verify_token != verify_token) {
        return Err(ConnPeerError::KeyExchangeFailed);
    }

    // Decrypt secret key and create ciphers.
    decrypt!(&decrypter, key.secret_key => secret_key);
    let cipher    = Cipher::aes_128_cfb8();
    let encrypter = Crypter::new(cipher, CrypterMode::Encrypt, secret_key, Some(secret_key)).map_err(|_| ConnPeerError::KeyExchangeFailed)?;
    let decrypter = Crypter::new(cipher, CrypterMode::Decrypt, secret_key, Some(secret_key)).map_err(|_| ConnPeerError::KeyExchangeFailed)?;
    comms.set_crypters(encrypter, decrypter);

    // Build the server ID.
    let mut sha = Sha1::new();
    sha.update(SERVER_ID.as_bytes());
    sha.update(secret_key);
    sha.update(&public_key_der);
    let mut sha_buf = [0u8; 40];
    let _ = hex::encode_to_slice(sha.finish(), &mut sha_buf);

    // Build the mojauth URL.
    let mut url_buf = [0u8; MOJAUTH_URL_PREFIX.len() + 16 + MOJAUTH_URL_SERVERID.len() + 40];
    let mut url_ptr = 0;
    // SAFETY: url_buf has enough space for `MOJAUTH_URL_PREFIX`, `hello.username`, `MOJAUTH_URL_SERVERID`, and `sha_buf`.
    //         None are written to overlap each other.
    //         hello.username can not be longer than 16 bytes (checked above).
    {
        unsafe { ptr::copy_nonoverlapping(MOJAUTH_URL_PREFIX.as_ptr(), url_buf.as_mut_ptr().byte_add(url_ptr), MOJAUTH_URL_PREFIX.len()); }
        url_ptr += MOJAUTH_URL_PREFIX.len();
        unsafe { ptr::copy_nonoverlapping(hello.username.as_ptr(), url_buf.as_mut_ptr().byte_add(url_ptr), hello.username.len()); }
        url_ptr += hello.username.len();
        unsafe { ptr::copy_nonoverlapping(MOJAUTH_URL_SERVERID.as_ptr(), url_buf.as_mut_ptr().byte_add(url_ptr), MOJAUTH_URL_SERVERID.len()); }
        url_ptr += MOJAUTH_URL_SERVERID.len();
        unsafe { ptr::copy_nonoverlapping(sha_buf.as_ptr(), url_buf.as_mut_ptr().byte_add(url_ptr), sha_buf.len()); }
        url_ptr += sha_buf.len();
    }
    let url = unsafe { str::from_utf8_unchecked(url_buf.get_unchecked(0..url_ptr)) };

    println!("{}", comms.addr());
    println!("{:?}", url);
    //surf::get(&);

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

macro_rules! decrypt { ( $decrypter:expr , $cipherdata:expr $(,)? => $target:ident ) => {
    #[allow(non_snake_case)]
    let mut __DECRYPT_PRIVATE_DECRYPTER  = $decrypter;
    #[allow(non_snake_case)]
    let mut __DECRYPT_PRIVATE_CIPHERDATA = $cipherdata;
    #[allow(non_snake_case)]
    // SAFETY: u8 has no Drop. The uninitialised part of the [u8] is inaccessible from $target.
    let mut __DECRYPT_PRIVATE_BUF     = unsafe { Box::<[u8]>::new_uninit_slice(__DECRYPT_PRIVATE_DECRYPTER.decrypt_len(__DECRYPT_PRIVATE_CIPHERDATA).map_err(|_| ConnPeerError::KeyExchangeFailed)?).assume_init() };
    #[allow(non_snake_case)]
    let mut __DECRYPT_PRIVATE_WRITTEN = __DECRYPT_PRIVATE_DECRYPTER.decrypt(__DECRYPT_PRIVATE_CIPHERDATA, &mut __DECRYPT_PRIVATE_BUF).map_err(|_| ConnPeerError::KeyExchangeFailed)?;
    let $target = &__DECRYPT_PRIVATE_BUF[0..__DECRYPT_PRIVATE_WRITTEN];
} }
use decrypt;
