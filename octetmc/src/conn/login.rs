use super::error::{ ConnPeerResult, ConnPeerError };
use super::state::{ ConnPeerState, ConfigPlay };
use super::comms::ConnPeerComms;
use crate::player::{ Player, PlayerId };
use crate::player::login::PlayerLoginEvent;
use crate::util::future::timeout;
use octetmc_protocol::value::profile::{ PlayerProfile, PlayerProfileSkin };
use octetmc_protocol::packet::login::c2s::hello::HelloC2SLoginPacket;
use octetmc_protocol::packet::login::c2s::key::KeyC2SLoginPacket;
use octetmc_protocol::packet::login::c2s::login_acknowledged::LoginAcknowledgedC2SLoginPacket;
use octetmc_protocol::packet::login::s2c::hello::HelloS2CLoginPacket;
use octetmc_protocol::packet::login::s2c::login_finished::LoginFinishedS2CLoginPacket;
use octetmc_protocol::packet::login::s2c::login_compression::LoginCompressionS2CLoginPacket;
use core::time::Duration;
use core::ptr;
use std::borrow::Cow;
use rand::{ self, rngs::ThreadRng, RngCore };
use openssl::pkey::{ PKey, Private as PrivateKey, Public as PublicKey };
use openssl::rsa::{ Padding as RsaPadding, Rsa };
use openssl::encrypt::Decrypter;
use openssl::symm::{ Crypter, Cipher, Mode as CrypterMode };
use openssl::sha::Sha1;
use ethnum::i256;
use surf::StatusCode;
use serde::Deserialize as Deser;
use uuid::Uuid;
use bevy_defer::AsyncWorld;


const LOGIN_TIMEOUT   : Duration = Duration::from_millis(250);
const MOJAUTH_TIMEOUT : Duration = Duration::from_millis(2500);

const SERVER_ID              : &str = "octectmc";
const OFFLINE_UUID_NAMESPACE : Uuid = Uuid::from_bytes(*b"OCTECTMC_PROFILE");

const MOJAUTH_URL_PREFIX   : &str = "https://sessionserver.mojang.com/session/minecraft/hasJoined?username=";
const MOJAUTH_URL_SERVERID : &str = "&serverId=";


pub(super) async fn handle_login_process(
    comms              : &mut ConnPeerComms,
    compress_threshold : Option<u32>,
    mojauth_enabled    : bool
) -> ConnPeerResult {
    comms.set_state(ConnPeerState::Login);

    // Wait for hello.
    let hello = comms.read_packet_boxed_timeout::<HelloC2SLoginPacket>(LOGIN_TIMEOUT).await?;
    if (hello.get().username.len() > 16) {
        return Err(ConnPeerError::UsernameTooLong);
    }

    // Set compression threshold.
    if let Some(threshold) = compress_threshold {
        comms.send_packet(&LoginCompressionS2CLoginPacket { threshold }).await?;
        comms.set_compress_threshold(threshold);
    }

    // Generate and send public key.
    let (private_key, public_key,) = generate_key_pair(2048);
    let verify_token               = generate_verify_token::<4>(&mut rand::rng());
    let public_key_der             = public_key.public_key_to_der().unwrap();
    comms.send_packet(&HelloS2CLoginPacket {
        server_id       : Cow::Borrowed(SERVER_ID),
        public_key      : Cow::Borrowed(&public_key_der),
        verify_token    : Cow::Borrowed(&verify_token),
        mojauth_enabled,
    }).await?;

    // Wait for key response.
    let key = comms.read_packet_timeout::<KeyC2SLoginPacket>(LOGIN_TIMEOUT).await?;

    // Create new pkey decrypter.
    let mut decrypter = Decrypter::new(&private_key).unwrap();
    _ = decrypter.set_rsa_padding(RsaPadding::PKCS1);

    // Decrypt and compare verify token.
    decrypt!(&decrypter, key.verify_token => decrypted_verify_token);
    if (decrypted_verify_token != verify_token) {
        return Err(ConnPeerError::KeyExchangeFailed);
    }

    // Decrypt secret key and create ciphers.
    decrypt!(&decrypter, key.secret_key => secret_key);
    let cipher = Cipher::aes_128_cfb8();
    let Ok(encrypter) = Crypter::new(cipher, CrypterMode::Encrypt, secret_key, Some(secret_key))
        else { return Err(ConnPeerError::KeyExchangeFailed) };
    let Ok(decrypter) = Crypter::new(cipher, CrypterMode::Decrypt, secret_key, Some(secret_key))
        else { return Err(ConnPeerError::KeyExchangeFailed) };
    comms.set_crypters(encrypter, decrypter, cipher.block_size());

    let profile = if (mojauth_enabled) {

        // Build the server ID.
        let mut sha = Sha1::new();
        sha.update(SERVER_ID.as_bytes());
        sha.update(secret_key);
        sha.update(&public_key_der);
        let sha_in_20 = sha.finish();

        let mut sha_in_32 = [0u8; 32];
        unsafe { ptr::copy_nonoverlapping(sha_in_20.as_ptr(), sha_in_32.as_mut_ptr(), 20); }
        let sha_in_i256 = i256::from_be_bytes(sha_in_32);
        let mut sha_buf = [0u8; 40];
        if (sha_in_i256 >= 0) {
            // SAFETY: sha_buf has room for 40 items.
            _ = hex::encode_to_slice(sha_in_20, &mut sha_buf).unwrap();
        } else {
            let neg_sha_in_32 = (-sha_in_i256).to_be_bytes();
            // SAFETY: sha_in_32 bytes has 32 items.
            //         sha_buf has room for 40 items.
            _ = hex::encode_to_slice(unsafe { neg_sha_in_32.get_unchecked(0..20) }, unsafe { sha_buf.get_unchecked_mut(0..40) });
        }
        // SAFETY: sha_buf has 40 items.
        let sha_buf = unsafe { sha_buf.get_unchecked((sha_buf.iter().position(|&x| x != b'0').unwrap_or(39))..40) };


        // Build the mojauth URL.
        let mut url_buf = [0u8; MOJAUTH_URL_PREFIX.len() + 16 + MOJAUTH_URL_SERVERID.len() + 41];
        let mut url_ptr = 0;
        // SAFETY: url_buf has enough space for `MOJAUTH_URL_PREFIX`, `hello.username`, `MOJAUTH_URL_SERVERID`, and `sha_buf`.
        //         None are written to overlap each other.
        //         hello.username can not be longer than 16 bytes (checked above).
        {
            unsafe { ptr::copy_nonoverlapping(MOJAUTH_URL_PREFIX.as_ptr(), url_buf.as_mut_ptr().byte_add(url_ptr), MOJAUTH_URL_PREFIX.len()); }
            url_ptr += MOJAUTH_URL_PREFIX.len();
            unsafe { ptr::copy_nonoverlapping(hello.get().username.as_ptr(), url_buf.as_mut_ptr().byte_add(url_ptr), hello.get().username.len()); }
            url_ptr += hello.get().username.len();
            unsafe { ptr::copy_nonoverlapping(MOJAUTH_URL_SERVERID.as_ptr(), url_buf.as_mut_ptr().byte_add(url_ptr), MOJAUTH_URL_SERVERID.len()); }
            url_ptr += MOJAUTH_URL_SERVERID.len();
            if (sha_in_i256 < 0) {
                unsafe { url_buf.as_mut_ptr().byte_add(url_ptr).write(b'-'); }
                url_ptr += 1;
            }
            unsafe { ptr::copy_nonoverlapping(sha_buf.as_ptr(), url_buf.as_mut_ptr().byte_add(url_ptr), sha_buf.len()); }
            url_ptr += sha_buf.len();
        }
        let url = unsafe { str::from_utf8_unchecked(url_buf.get_unchecked(0..url_ptr)) };

        let Ok(Ok(mut resp)) = timeout(MOJAUTH_TIMEOUT, surf::get(url).send()).await
            else { return Err(ConnPeerError::AuthServerUnreachable); };
        match (resp.status()) {
            StatusCode::NoContent => {
                return Err(ConnPeerError::AuthFailed);
            },
            StatusCode::Ok => {
                let Ok(Ok(profile)) = timeout(MOJAUTH_TIMEOUT, resp.body_json::<MojauthProfile>()).await
                    else { return Err(ConnPeerError::BadAuthServer); };
                let MojauthProfileProp::Textures { sig : texture_sig, value : texture_value } = &profile.props[0];
                PlayerProfile {
                    uuid : profile.uuid,
                    name : Cow::Owned(profile.name.clone()),
                    skin : Some(PlayerProfileSkin {
                        sig   : texture_sig.as_ref().map(|v| Cow::Owned(v.clone())),
                        value : Cow::Owned(texture_value.clone())
                    })
                }
            },
            _ => { return Err(ConnPeerError::BadAuthServer); }
        }

    } else { PlayerProfile {
        uuid : Uuid::new_v3(&OFFLINE_UUID_NAMESPACE, hello.get().username.as_bytes()),
        name : Cow::Owned(hello.get().username.to_string()),
        skin : None
    } };

    // Send login success and await confirmation.
    comms.send_packet(&LoginFinishedS2CLoginPacket {
        profile : PlayerProfile {
            uuid : profile.uuid,
            name : Cow::Borrowed(&*profile.name),
            skin : profile.skin.as_ref().map(|skin| PlayerProfileSkin {
                sig   : skin.sig.as_ref().map(|sig| Cow::Borrowed(&**sig)),
                value : Cow::Borrowed(&*skin.value),
            }),
        },
    }).await?;
    let _ = comms.read_packet_timeout::<LoginAcknowledgedC2SLoginPacket>(LOGIN_TIMEOUT).await?;
    comms.set_state(ConnPeerState::ConfigPlay(ConfigPlay::Config { active_ticks : 0 }));

    // Add a player to the ECS world.
    let player = AsyncWorld.spawn_bundle((Player {
        profile,
        // SAFETY: take_conn_sender_unchecked has not been called before.
        conn_sender : unsafe { comms.take_conn_sender_unchecked() }
    }));
    _ = AsyncWorld.send_event(PlayerLoginEvent { player_id : PlayerId::from(player.id()) });

    // Continue to config_play.
    Ok(())
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
    let mut __DECRYPT_PRIVATE_CIPHERDATA = &*$cipherdata;
    #[allow(non_snake_case)]
    // SAFETY: u8 has no Drop. The uninitialised part of the [u8] is inaccessible from $target.
    let mut __DECRYPT_PRIVATE_BUF     = unsafe { Box::<[u8]>::new_uninit_slice(__DECRYPT_PRIVATE_DECRYPTER.decrypt_len(__DECRYPT_PRIVATE_CIPHERDATA).map_err(|_| ConnPeerError::KeyExchangeFailed)?).assume_init() };
    #[allow(non_snake_case)]
    let mut __DECRYPT_PRIVATE_WRITTEN = __DECRYPT_PRIVATE_DECRYPTER.decrypt(__DECRYPT_PRIVATE_CIPHERDATA, &mut __DECRYPT_PRIVATE_BUF).map_err(|_| ConnPeerError::KeyExchangeFailed)?;
    let $target = &__DECRYPT_PRIVATE_BUF[0..__DECRYPT_PRIVATE_WRITTEN];
} }
use decrypt;


#[derive(Deser)]
struct MojauthProfile {
    #[serde(rename = "id")]
    uuid    : Uuid,
    name    : String,
    #[serde(default, rename = "legacy")]
    _legacy : bool,
    #[serde(rename = "properties")]
    props   : [MojauthProfileProp; 1]
}

#[derive(Deser)]
#[serde(tag = "name")]
enum MojauthProfileProp {
    #[serde(rename = "textures")]
    Textures {
        #[serde(rename = "signature")]
        sig   : Option<String>,
        value : String
    }

}
