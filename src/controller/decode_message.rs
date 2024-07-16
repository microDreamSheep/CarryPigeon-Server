use aes_gcm::aes::Aes256;

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
pub async fn decode_message(info: Form<DecodeUserInfo>) -> Option<String> {
    let raw_decoded_message = decode_message_impl(info).await;

    match raw_decoded_message {
        Some(v) => {
            if v.len() == 1 {
                return Some(v[0].clone());
            }

            let mut result = String::new();
            for i in v {
                let temp = format!("{}\n", &i);
                result.clone_from(&temp);
            }
            None
        }
        None => None,
    }
}

pub async fn decode_message_impl(info: Form<DecodeUserInfo>) -> Option<Vec<String>> {
    if info.message_type == 0 {
        // 只需解密一条信息
        if info.message_id_from == info.message_id_to {
            // 鉴权
            let result_auth = Box::new(group_authentication(info.uuid, info.to).await);
            if *result_auth {
                return Some(group_message::decode_message(info.to, info.message_id_from).await);
            }
        } else {
            // 需要解密多条信息
            let result_auth = Box::new(group_authentication(info.uuid, info.to).await);
            if *result_auth {
                return Some(
                    group_message::decode_messages_vec(
                        info.to,
                        info.message_id_from,
                        info.message_id_to,
                    )
                    .await,
                );
            }
        }
    } else if info.message_type == 1 {
        //鉴权
        if (info.uuid == info.from || info.uuid == info.to)
            && info.message_id_from == info.message_id_to
        {
            return Some(
                private_message::decode_message(info.from, info.to, info.message_id_from).await,
            );
        } else if (info.uuid == info.from || info.uuid == info.to)
            && info.message_id_from != info.message_id_to
        {
            return Some(
                private_message::decode_messages_vec(
                    info.from,
                    info.to,
                    info.message_id_from,
                    info.message_id_to,
                )
                .await,
            );
        }
        return None;
    }
    None
}
