use jsonwebtoken::{encode, Algorithm, EncodingKey, Header};
use minori_jwt::claims::{Claims, RequiredClaims};

pub async fn encrypt(
    aud: String,
    exp: usize,
    iat: usize,
    iss: String,
    nbf: usize,
    sub: String,
) -> String {
    let user_claims = Claims {
        aud,
        exp,
        iat,
        iss,
        nbf,
        sub: sub.clone(),
    };
    match encode(
        &Header::default(),
        &user_claims,
        &EncodingKey::from_secret(sub.as_bytes()),
    ) {
        Ok(v) => v,
        Err(e) => {
            tracing::error!("{}", e);
            String::new()
        }
    }
}

pub async fn authenticator_encrypt(sub: String, iat: i64, exp: i64) -> String {
    let user_claims = RequiredClaims {
        sub: sub.clone(),
        iat,
        exp,
    };
    match encode(
        &Header::new(Algorithm::HS512),
        &user_claims,
        &EncodingKey::from_secret(sub.as_bytes()),
    ) {
        Ok(v) => v,
        Err(e) => {
            tracing::error!("{}", e);
            String::from("null")
        }
    }
}
