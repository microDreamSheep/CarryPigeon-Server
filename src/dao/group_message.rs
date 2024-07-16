use super::{row::GlobalMessage, PG_POOL};

use aes_gcm::aes::{
    cipher::{block_padding::Pkcs7, BlockDecryptMut, KeyIvInit},
    Aes256,
};

pub type Aes256CbcEnc = cbc::Encryptor<Aes256>;
pub type Aes256CbcDec = cbc::Decryptor<Aes256>;

pub async fn get_latest_message_id(group_id: i64) -> Option<i64> {
    let sql = format!(
        r#"SELECT MAX(message_id) message_id FROM "group".group_{}"#,
        group_id
    );
    let rows_temp = Box::new(
        sqlx::query_as::<_, GlobalMessage>(&sql)
            .bind(group_id)
            .fetch_one(PG_POOL.get().unwrap())
            .await,
    );
    match *rows_temp {
        Ok(v) => Some(v.message_id),
        Err(e) => {
            tracing::error!("Missing group_id:{} or other error.Info:{}", group_id, e);
            // 表示查询失败
            None
        }
    }
}

pub async fn get_message(group_id: i64, message_id: i64) -> Option<GlobalMessage> {
    let sql = format!(
        r#"SELECT * FROM "group".group_{} WHERE message_id = $1"#,
        group_id
    );
    let rows_temp = Box::new(
        sqlx::query_as::<_, GlobalMessage>(&sql)
            .bind(message_id)
            .fetch_one(PG_POOL.get().unwrap())
            .await,
    );
    match *rows_temp {
        Ok(v) => Some(v),
        Err(e) => {
            tracing::error!("Missing group_id:{} or other error.Info:{}", group_id, e);
            // 表示查询失败
            None
        }
    }
}

pub async fn get_message_vec(
    group_id: i64,
    id_from: i64,
    id_to: i64,
) -> Option<Vec<GlobalMessage>> {
    let sql = format!(
        r#"SELECT * FROM "group".group_{} WHERE message_id = $1"#,
        group_id
    );
    // 初始化数组，使用with_capacity可以提高性能
    let mut rows_temp_vec: Vec<GlobalMessage> = Vec::with_capacity((id_to - id_from) as usize);
    for i in id_from..id_to {
        let rows_temp = Box::new(
            sqlx::query_as::<_, GlobalMessage>(&sql)
                .bind(i)
                .fetch_one(PG_POOL.get().unwrap())
                .await,
        );
        match *rows_temp {
            Ok(v) => rows_temp_vec.push(v),
            Err(e) => {
                tracing::error!("Missing group_id:{} or other error.Info:{}", group_id, e);
                // 表示查询失败
                return None;
            }
        }
    }
    if rows_temp_vec.is_empty() {
        return None;
    }
    Some(rows_temp_vec)
}

pub async fn decode_message(group_id: i64, message_id: i64) -> String {
    //获取信息
    let message = Box::new(get_message(group_id, message_id).await.unwrap());
    let cipher = Box::new(
        Aes256CbcDec::new_from_slices(message.aes_key.as_bytes(), message.aes_iv.as_bytes())
            .unwrap(),
    );
    let decoded_message = cipher
        .decrypt_padded_vec_mut::<Pkcs7>(message.text.as_bytes())
        .unwrap();
    String::from_utf8(decoded_message).unwrap()
}

pub async fn decode_messages_vec(
    group_id: i64,
    message_id_from: i64,
    message_id_to: i64,
) -> Vec<String> {
    let mut result: Vec<String> = vec![];
    //获取信息
    let message = get_message_vec(group_id, message_id_from, message_id_to)
        .await
        .unwrap();
    for i in message {
        let cipher = Box::new(
            Aes256CbcDec::new_from_slices(i.aes_key.as_bytes(), i.aes_iv.as_bytes()).unwrap(),
        );
        let decoded_message = cipher
            .decrypt_padded_vec_mut::<Pkcs7>(i.text.as_bytes())
            .unwrap();
        result.push(String::from_utf8(decoded_message).unwrap());
    }
    result
}

pub async fn push_group_message(message: &GlobalMessage) {
    let sql = format!(
        r#"INSERT INTO "group".group_{} ("from", group_id, text, file_path, json, timestamp, message_id) VALUES($1, $2, $3, $4, $5, $6, $7)"#,
        message.to
    );
    let rows_temp = Box::new(
        sqlx::query(&sql)
            .bind(message.from)
            .bind(message.to)
            .bind(message.text.clone())
            .bind(message.file.clone())
            .bind(message.json.clone())
            .bind(message.timestamp.clone())
            .bind(message.message_id)
            .execute(PG_POOL.get().unwrap())
            .await,
    );
    match *rows_temp {
        Ok(_) => {}
        Err(e) => tracing::error!("{}", e),
    }
}

pub async fn delete_group_message(group_id: i64, message_id: i64) {
    let sql = format!(r#"DELETE "group".group_{} WHERE message = $1"#, group_id);
    let _ = sqlx::query(&sql)
        .bind(message_id)
        .execute(PG_POOL.get().unwrap())
        .await;
}
