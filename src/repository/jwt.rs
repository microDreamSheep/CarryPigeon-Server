use jsonwebtoken::{encode, errors::Error, EncodingKey, Header};
use once_cell::sync::OnceCell;
use rand::rngs::ThreadRng;
use rsa::{traits::PublicKeyParts, RsaPrivateKey, RsaPublicKey};

use minori_jwt::claims::Claims;

const BITS: usize = 2048;
pub static PRIVATE_KEY: OnceCell<RsaPrivateKey> = OnceCell::new();
pub static PUBLIC_KEY: OnceCell<RsaPublicKey> = OnceCell::new();
const RNG: OnceCell<ThreadRng> = OnceCell::new();

#[allow(const_item_mutation)]
pub fn generate_key() {
    RNG.set(rand::thread_rng()).unwrap();
    PRIVATE_KEY
        .set(
            RsaPrivateKey::new(RNG.get_mut().unwrap(), BITS)
                .expect("failed to generate private key"),
        )
        .unwrap();
    PUBLIC_KEY
        .set(PRIVATE_KEY.get().unwrap().to_public_key())
        .unwrap();
}

#[allow(const_item_mutation)]
pub fn encrypt(
    aud: String,
    exp: usize,
    iat: usize,
    iss: String,
    nbf: usize,
    sub: String,
) -> Result<String, Error> {
    let user_claims = Claims {
        aud,
        exp,
        iat,
        iss,
        nbf,
        sub,
    };
    let result = encode(
        &Header::default(),
        &user_claims,
        &EncodingKey::from_rsa_der(PUBLIC_KEY.get().unwrap().n().to_string().as_bytes()),
    );
    result
}
