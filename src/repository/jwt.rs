use jsonwebtoken::{encode, Algorithm, EncodingKey, Header};
use crate::repository::claims::{Claims, RequiredClaims};
use rand::rngs::ThreadRng;
use rsa::{
    pkcs8::{EncodePrivateKey, EncodePublicKey, LineEnding},
    RsaPrivateKey, RsaPublicKey,
};
use tokio::task::spawn;

use crate::dao::user_token::push_token;

const BITS: usize = 2048;

pub async fn generate_key() -> (RsaPrivateKey, RsaPublicKey, Box<ThreadRng>) {
    let mut rng_temp: Box<ThreadRng> = Box::new(rand::thread_rng());
    let private_key = RsaPrivateKey::new(&mut rng_temp, BITS).expect("failed to generate key");
    let public_key = private_key.to_public_key();
    (private_key, public_key, rng_temp)
}

pub async fn encrypt(
    aud: String,
    exp: usize,
    iat: usize,
    iss: String,
    nbf: usize,
    sub: String,
) -> String {
    let (private_key, _public_key, mut _rng) = generate_key().await;
    let user_claims = Claims {
        aud,
        exp,
        iat,
        iss,
        nbf,
        sub: sub.clone(),
    };
    match encode(
        &Header::new(Algorithm::RS256),
        &user_claims,
        &EncodingKey::from_rsa_pem(private_key.to_pkcs8_pem(LineEnding::LF).unwrap().as_bytes())
            .unwrap(),
    ) {
        Ok(v) => v,
        Err(e) => {
            tracing::error!("{}", e);
            String::new()
        }
    }
}

pub async fn authenticator_encrypt(sub: i64, iat: i64, exp: i64) -> String {
    let (private_key, public_key, mut _rng) = generate_key().await;
    let user_claims = RequiredClaims {
        sub,
        iat,
        exp,
    };
    match encode(
        &Header::new(Algorithm::RS256),
        &user_claims,
        &EncodingKey::from_rsa_pem(private_key.to_pkcs8_pem(LineEnding::LF).unwrap().as_bytes())
            .unwrap(),
    ) {
        Ok(v) => {
            spawn(async move {
                push_token(sub, public_key.to_public_key_pem(LineEnding::CR).unwrap()).await;
            });
            v
        }
        Err(e) => {
            tracing::error!("{}", e);
            String::from("null")
        }
    }
}
