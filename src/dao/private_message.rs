use aes_gcm::aes::cipher::{block_padding::Pkcs7, BlockDecryptMut, KeyIvInit};

use super::{row::GlobalMessage, PG_POOL};

/// !Undefined
pub async fn get_offline_message(_uuid: i64) -> Vec<GlobalMessage> {
    vec![]
}

pub async fn get_latest_message_id(from: i64, to: i64) -> Option<i64> {
    let sql = format!(
        r#"SELECT MAX(message_id) message_id FROM private_message.private_message_{} WHERE "from" = $1 "to" = $2"#,
        to
    );
    let rows_temp = Box::new(
        sqlx::query_as::<_, GlobalMessage>(&sql)
            .bind(from)
            .bind(to)
            .fetch_one(PG_POOL.get().unwrap())
            .await,
    );
    let temp_from_to = match *rows_temp {
        Ok(v) => Box::from(v.message_id),
        Err(e) => {
            tracing::error!("Missing from:{} to:{} or other error.Info:{}", from, to, e);
            // 表示查询失败
            Box::new(-1)
        }
    };
    let rows_temp = Box::new(
        sqlx::query_as::<_, GlobalMessage>(&sql)
            .bind(to)
            .bind(from)
            .fetch_one(PG_POOL.get().unwrap())
            .await,
    );
    let temp_to_from = match *rows_temp {
        Ok(v) => Box::from(v.message_id),
        Err(e) => {
            tracing::error!("Missing from:{} to:{} or other error.Info:{}", from, to, e);
            // 表示查询失败
            Box::new(-1)
        }
    };
    if temp_from_to >= temp_to_from {
        Some(*temp_from_to)
    } else {
        Some(*temp_to_from)
    }
}

pub async fn push_private_message(message: &GlobalMessage) {
    let sql = format!(
        r#"INSERT INTO private_message.private_message_{} ("from", "to" ,text, file_path, json, timestamp, message_id) VALUES($1, $2, $3, $4, $5, $6, $7)"#,
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
    let rows_temp = Box::new(
        sqlx::query(&sql)
            .bind(message.to)
            .bind(message.from)
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

pub async fn delete_private_message(id: i64, from: i64, to: i64, message_id: i64) {
    let sql = format!(
        r#"DELETE private_message.private_message_{} WHERE "from" = $1 "to" = $2 message_id = $3"#,
        id
    );
    let rows_temp = Box::new(
        sqlx::query(&sql)
            .bind(from)
            .bind(to)
            .bind(message_id)
            .execute(PG_POOL.get().unwrap())
            .await,
    );
    match *rows_temp {
        Ok(_) => {}
        Err(e) => tracing::error!("{}", e),
    }
}

pub async fn get_message(from: i64, to: i64, message_id: i64) -> Option<GlobalMessage> {
    let sql = format!(
        r#"SELECT * FROM private_message.private_message_{} WHERE "from" = $1 "to" = $2 message_id = $3"#,
        from
    );
    let rows_temp = Box::new(
        sqlx::query_as::<_, GlobalMessage>(&sql)
            .bind(from)
            .bind(to)
            .bind(message_id)
            .fetch_one(PG_POOL.get().unwrap())
            .await,
    );
    match *rows_temp {
        Ok(v) => Some(v),
        Err(e) => {
            tracing::error!("{}", e);
            None
        }
    }
}

pub async fn get_messages_vec(
    from: i64,
    to: i64,
    id_from: i64,
    id_to: i64,
) -> Option<Vec<GlobalMessage>> {
    let sql = format!(
        r#"SELECT * FROM private_message.private_message_{} WHERE from = $1 to = $2 message_id = $3"#,
        from
    );
    // 初始化数组，使用with_capacity可以提高性能
    let mut rows_temp_vec: Vec<GlobalMessage> = Vec::with_capacity((id_to - id_from) as usize);
    for i in id_from..id_to {
        let rows_temp = Box::new(
            sqlx::query_as::<_, GlobalMessage>(&sql)
                .bind(from)
                .bind(to)
                .bind(i)
                .fetch_one(PG_POOL.get().unwrap())
                .await,
        );
        match *rows_temp {
            Ok(v) => rows_temp_vec.push(v),
            Err(e) => {
                tracing::error!("{}", e);
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

pub async fn decode_message(from: i64, to: i64, message_id: i64) -> Vec<String> {
    let mut result = vec![];
    let message = Box::new(get_message(from, to, message_id).await);
    match *message {
        Some(v) => {
            result
        }
        None => result,
    }
}

pub async fn decode_messages_vec(
    from: i64,
    to: i64,
    message_id_from: i64,
    message_id_to: i64,
) -> Vec<String> {
    let mut result: Vec<String> = vec![];
    //获取信息
    let message = get_messages_vec(from, to, message_id_from, message_id_to)
        .await
        .unwrap();
    result
}
