use aes_gcm::aes::{
    cipher::{block_padding::Pkcs7, BlockDecryptMut, KeyIvInit},
    Aes256,
};

use rocket::{form::Form, post, FromForm};

use crate::dao::{group::group_authentication, group_message, private_message};

#[derive(Debug, FromForm)]
pub struct DecodeUserInfo {
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
pub async fn decode_message(info: Form<DecodeUserInfo>) -> String {
    let raw_decoded_message = decode_message_impl(info).await;

    if raw_decoded_message.len() == 1 {
        return raw_decoded_message[0].clone();
    }

    let mut result = String::new();
    for i in raw_decoded_message {
        let temp = format!("{}\n", &i);
        result.clone_from(&temp);
    }
    String::new()
}

pub async fn decode_message_impl(info: Form<DecodeUserInfo>) -> Vec<String> {
    if info.message_type == 0 {
        // 只需解密一条信息
        if info.message_id_from == info.message_id_to {
            // 鉴权
            let result_auth = Box::new(group_authentication(info.uuid, info.to).await);
            if *result_auth {
                let mut result = vec![];
                result.push(group_message::decode_message(info.to, info.message_id_from).await);
                return result;
            }
        } else {
            // 需要解密多条信息
            let result_auth = Box::new(group_authentication(info.uuid, info.to).await);
            if *result_auth {
                return group_message::decode_messages_vec(
                    info.to,
                    info.message_id_from,
                    info.message_id_to,
                )
                .await;
            }
        }
    } else if info.message_type == 1 {
        //鉴权
        if (info.uuid == info.from || info.uuid == info.to)
            && info.message_id_from == info.message_id_to
        {
            let mut result = vec![];
            let message = Box::new(
                private_message::get_message(info.from, info.to, info.message_id_from).await,
            );
            let cipher = Box::new(
                Aes256CbcDec::new_from_slices(
                    message.aes_key.as_bytes(),
                    message.aes_iv.as_bytes(),
                )
                .unwrap(),
            );
            let decoded_message = cipher
                .decrypt_padded_vec_mut::<Pkcs7>(message.text.as_bytes())
                .unwrap();
            result.push(String::from_utf8(decoded_message).unwrap());
            return result;
        }
        return vec![String::new()];
    }
    vec![String::new()]
}
