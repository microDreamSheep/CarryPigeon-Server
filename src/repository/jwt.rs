use jsonwebtoken::{encode, errors::Error, EncodingKey, Header};
use minori_jwt::claims::Claims;
use once_cell::sync::OnceCell;
use rsa::{traits::PublicKeyParts, RsaPrivateKey, RsaPublicKey};

const BITS: usize = 2048;
pub static PRIVATE_KEY: OnceCell<RsaPrivateKey> = OnceCell::new();
pub static PUBLIC_KEY: OnceCell<RsaPublicKey> = OnceCell::new();

pub async fn generate_key() {
    tokio::task::spawn_blocking(move || {
        let rng_temp = &mut rand::thread_rng();
        PRIVATE_KEY
            .set(RsaPrivateKey::new(rng_temp, BITS).expect("failed to generate key"))
            .unwrap();
        PUBLIC_KEY
            .set(PRIVATE_KEY.get().unwrap().to_public_key())
            .unwrap();
    });
}

#[allow(const_item_mutation)]
pub async fn encrypt(
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
