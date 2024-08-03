use crypto::aes;
use crypto::aes::KeySize::KeySize256;
use crypto::blockmodes::PkcsPadding;
use crypto::buffer::{ReadBuffer, RefReadBuffer, RefWriteBuffer, WriteBuffer};
use crypto::symmetriccipher::SymmetricCipherError;
use rsa::{Pkcs1v15Encrypt, RsaPrivateKey, RsaPublicKey};
use rsa::pkcs1::DecodeRsaPrivateKey;

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
            _BufferUnderflow => break,
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
            _BufferUnderflow => break,
        }
    }
    Ok(String::from_utf8_lossy(final_result.as_slice()).parse().unwrap())
}