use crypto::aes;
use crypto::aes::KeySize::KeySize256;
use crypto::blockmodes::PkcsPadding;
use crypto::buffer::{ReadBuffer, RefReadBuffer, RefWriteBuffer, WriteBuffer};
use crypto::symmetriccipher::SymmetricCipherError;
use openssl::rsa::{Padding, Rsa};
use rocket::yansi::Paint;

const IV:[u8;16] = [5u8;16];

/// aes加密
pub fn aes256_cbc_encrypt(
    data: &str,
    key: &str,
) -> Result<String, SymmetricCipherError> {
    let mut encryptor = aes::cbc_encryptor(
        KeySize256,
        key.as_bytes(), &IV,
        PkcsPadding,
    );
    let mut buffer = [0; 4096];
    let mut write_buffer = RefWriteBuffer::new(&mut buffer);
    let mut read_buffer = RefReadBuffer::new(data.as_bytes());
    let mut final_result = Vec::new();

    loop {
        let result = encryptor.encrypt(&mut read_buffer, &mut write_buffer, true)?;
        final_result.extend(write_buffer.take_read_buffer().take_remaining().iter().map(|&i| i));
        match result {
            _ => break,
        }
    }
    Ok(base64::encode(&final_result))
}


/// aes解密
pub fn aes256_cbc_decrypt(
    data: &str,
    key: &str
) -> Result<String, SymmetricCipherError> {
    let mut decryptor = aes::cbc_decryptor(
        KeySize256,
        key.as_bytes(), &IV,
        PkcsPadding,
    );

    let mut buffer = [0; 4096];
    let mut write_buffer = RefWriteBuffer::new(&mut buffer);
    let x = base64::decode(data).unwrap();
    let mut read_buffer = RefReadBuffer::new(x.as_slice());
    let mut final_result = Vec::new();

    loop {
        let result = decryptor.decrypt(&mut read_buffer, &mut write_buffer, true)?;
        final_result.extend(write_buffer.take_read_buffer().take_remaining().iter().map(|&i| i));
        match result {
            _ => break,
        }
    }
    Ok(String::from_utf8_lossy(final_result.as_slice()).parse().unwrap())
}


// RSA加密
pub fn rsa_encrypt(public_key: &str, data: &str) -> String {
    let rsa = Rsa::public_key_from_der(base64::decode(public_key).unwrap().as_slice()).map_err(|e| e.to_string()).unwrap();
    let mut result = vec![0; rsa.size() as usize];
    let size = rsa.public_encrypt(data.as_bytes(), &mut result, Padding::PKCS1).map_err(|e| e.to_string()).unwrap();
    result.truncate(size);
    base64::encode(result)
}

// RSA解密
pub fn rsa_decrypt(private_key: &str, data: &str) -> String {
    let rsa = Rsa::private_key_from_der(base64::decode(private_key).unwrap().as_slice()).map_err(|e| e.to_string()).unwrap();
    let mut result = vec![0; rsa.size() as usize];
    let size = rsa.private_decrypt(base64::decode(data).unwrap().as_slice(), &mut result, Padding::PKCS1).map_err(|e| e.to_string()).unwrap();
    result.truncate(size);
    String::from_utf8_lossy(result.as_slice()).parse().unwrap()
}

/// 生成RSA密钥对
pub fn generate_rsa_key() -> (String,String) {
    let rsa = Rsa::generate(2048).map_err(|e| e.to_string()).unwrap();
    let public_key = rsa.public_key_to_der().map_err(|e| e.to_string()).unwrap();
    let private_key = rsa.private_key_to_der().map_err(|e| e.to_string()).unwrap();
    (base64::encode(public_key),base64::encode(private_key))
}
