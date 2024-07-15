use aes_gcm::aes::{cipher::{block_padding::Pkcs7, BlockDecryptMut, KeyIvInit}, Aes256};

use rocket::{post, FromForm, form::Form};

#[derive(Debug, FromForm)]
pub struct DecodeUserInfo{
   uuid: i64, 
   message_type: u8,
   from: i64,
   to: i64,
   message_id_from: i64,
   message_id_to: i64,
}

pub type Aes256CbcEnc = cbc::Encryptor<Aes256>;
pub type Aes256CbcDec = cbc::Decryptor<Aes256>;

#[post("/decode_message", data = "<info>")]
pub async fn decode_message(info:Form<DecodeUserInfo>) -> String{
    if info.message_type == 0 {
        if info.message_id_from == info.message_id_to{
            use crate::dao::group_message::get_message;

            // 鉴权

            //获取信息
            let message = Box::new(get_message(info.to, info.message_id_from).await.unwrap());
            let cipher = Box::new(Aes256CbcDec::new_from_slices(message.aes_key.as_bytes(), message.aes_iv.as_bytes()).unwrap());
            let decoded_message = cipher.decrypt_padded_vec_mut::<Pkcs7>(message.text.as_bytes()).unwrap();
            return String::from_utf8(decoded_message).unwrap();
        }
    } else if info.message_type == 1 {
        return String::new();
    }
    String::new()
}