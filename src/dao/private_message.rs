use super::{row::GlobalMessage, PG_POOL};

pub async fn get_offline_message(uuid: i64) -> Vec<GlobalMessage> {
    vec![]
}

pub async fn get_latest_message_id(from: i64, to: i64) -> Option<i64> {
    let sql = format!(
        r#"SELECT MAX(message_id) message_id FROM private_message.private_message_{} WHERE "from" = $1 "to" = $2"#,
        to
    );
    let rows_temp = sqlx::query_as::<_, GlobalMessage>(&sql)
        .bind(from)
        .bind(to)
        .fetch_one(PG_POOL.get().unwrap())
        .await;
    let temp_from_to = match rows_temp {
        Ok(v) => Box::from(v.message_id),
        Err(e) => {
            tracing::error!("Missing from:{} to:{} or other error.Info:{}", from, to, e);
            // 表示查询失败
            Box::new(-1)
        }
    };
    let rows_temp = sqlx::query_as::<_, GlobalMessage>(&sql)
        .bind(to)
        .bind(from)
        .fetch_one(PG_POOL.get().unwrap())
        .await;
    let temp_to_from = match rows_temp {
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
    let rows_temp = sqlx::query(&sql)
        .bind(message.from)
        .bind(message.to)
        .bind(message.text.clone())
        .bind(message.file.clone())
        .bind(message.json.clone())
        .bind(message.timestamp.clone())
        .bind(message.message_id)
        .execute(PG_POOL.get().unwrap())
        .await;
    match rows_temp {
        Ok(_) => {}
        Err(e) => tracing::error!("{}", e),
    }
}

pub async fn delete_private_message(id: i64, from: i64, to: i64, message_id: i64) {
    let sql = format!(
        r#"DELETE private_message.private_message_{} WHERE "from" = $1 "to" = $2 message_id = $3"#,
        id
    );
    let _rows_temp = sqlx::query(&sql)
        .bind(from)
        .bind(to)
        .bind(message_id)
        .execute(PG_POOL.get().unwrap())
        .await;
}
