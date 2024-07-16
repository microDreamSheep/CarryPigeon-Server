use super::PG_POOL;
use crate::dao::row::Group;

pub async fn get_member(group_id: i64) -> Vec<i64> {
    let rows_temp = Box::new(
        sqlx::query_as::<_, Group>(r#"SELECT member FROM "group"."group" WHERE id = $1"#)
            .bind(group_id)
            .fetch_one(PG_POOL.get().unwrap())
            .await,
    );

    match *rows_temp {
        Ok(v) => v.member,
        Err(e) => {
            tracing::error!("Missing group_id:{} or other error.Info:{}", group_id, e);
            // 表示查询失败
            vec![]
        }
    }
}

pub async fn get_admin(group_id: i64) -> Vec<i64> {
    let rows_temp = Box::new(
        sqlx::query_as::<_, Group>(r#"SELECT admin FROM "group"."group" WHERE id = $1"#)
            .bind(group_id)
            .fetch_one(PG_POOL.get().unwrap())
            .await,
    );

    match *rows_temp {
        Ok(v) => v.admin,
        Err(e) => {
            tracing::error!("Missing group_id:{} or other error.Info:{}", group_id, e);
            // 表示查询失败
            vec![]
        }
    }
}

pub async fn get_owner(group_id: i64) -> i64 {
    let rows_temp = Box::new(
        sqlx::query_as::<_, Group>(r#"SELECT owner FROM "group"."group" WHERE id = $1"#)
            .bind(group_id)
            .fetch_one(PG_POOL.get().unwrap())
            .await,
    );

    match *rows_temp {
        Ok(v) => v.owner,
        Err(e) => {
            tracing::error!("Missing group_id:{} or other error.Info:{}", group_id, e);
            // 表示查询失败
            -1
        }
    }
}

pub async fn get_all_member(group_id: i64) -> Vec<i64> {
    let mut result = vec![];
    let member = get_member(group_id).await;
    let admin = get_admin(group_id).await;
    let owner = get_owner(group_id).await;
    result.copy_from_slice(&member);
    result.copy_from_slice(&admin);
    result.push(owner);

    // return
    result
}

pub async fn push_member(group_id: i64, member_id: i64) {
    let rows_temp = Box::new(
        sqlx::query_as::<_, Group>(r#"SELECT member FROM "group"."group" WHERE id = $1"#)
            .bind(group_id)
            .fetch_one(PG_POOL.get().unwrap())
            .await,
    );

    let mut member_value = match *rows_temp {
        Ok(v) => v.member,
        Err(e) => {
            tracing::error!("Missing group_id:{} or other error.Info:{}", group_id, e);
            // 表示查询失败
            vec![]
        }
    };
    member_value.push(member_id);
    let rows_temp = Box::new(
        sqlx::query(r#"UPDATE "group"."group" SET member = $1 WHERE id = $2"#)
            .bind(member_id)
            .bind(group_id)
            .execute(PG_POOL.get().unwrap())
            .await,
    );
    match *rows_temp {
        Ok(_) => {}
        Err(e) => tracing::error!("{}", e),
    }
}

pub async fn push_admin(group_id: i64, admin_id: i64) {
    let rows_temp = Box::new(
        sqlx::query_as::<_, Group>(r#"SELECT admin FROM "group"."group" WHERE id = $1"#)
            .bind(group_id)
            .fetch_one(PG_POOL.get().unwrap())
            .await,
    );

    let mut member_value = match *rows_temp {
        Ok(v) => v.member,
        Err(e) => {
            tracing::error!("Missing group_id:{} or other error.Info:{}", group_id, e);
            // 表示查询失败
            vec![]
        }
    };
    member_value.push(admin_id);
    let rows_temp = Box::new(
        sqlx::query(r#"UPDATE "group"."group" SET admin = $1 WHERE id = $2"#)
            .bind(admin_id)
            .bind(group_id)
            .execute(PG_POOL.get().unwrap())
            .await,
    );
    match *rows_temp {
        Ok(_) => {}
        Err(e) => tracing::error!("{}", e),
    }
}

pub async fn push_new_group(group: &Group) -> i64 {
    let group_id = Box::new(match get_latest_group_id().await {
        Some(v) => v,
        None => return -1,
    });
    let rows_temp = Box::new(sqlx::query(r#"INSERT INTO "group"."group" (id, name, owner, admin, member) VALUES($1 , $2, $3, $4, $5)"#)
        .bind(group.id)
        .bind(&group.name)
        .bind(group.owner)
        .bind(&group.admin)
        .bind(&group.member)
        .execute(PG_POOL.get().unwrap())
        .await);

    match *rows_temp {
        Ok(_) => {}
        Err(e) => {
            tracing::error!("{}", e);
        }
    };

    // 创建表
    let sql = format!(
        r#"create table "group".group_message_{}
(
    "from"     bigint,
    text       text,
    file_path  text,
    json       json,
    timestamp  varchar,
    message_id bigint,
    aes_key    text,
    aes_iv     text
);
"#,
        group_id
    );
    let rows_temp = Box::new(sqlx::query(&sql).execute(PG_POOL.get().unwrap()).await);
    match *rows_temp {
        Ok(_) => *group_id,
        Err(e) => {
            tracing::error!("{}", e);
            -1
        }
    }
}

pub async fn owner_move(group_id: i64, owner: i64) {
    let rows_temp = Box::new(
        sqlx::query(r#"UPDATE "group"."group" SET owner = $1 WHERE id = $2"#)
            .bind(group_id)
            .bind(owner)
            .execute(PG_POOL.get().unwrap())
            .await,
    );

    match *rows_temp {
        Ok(_) => {}
        Err(e) => {
            tracing::error!("{}", e);
        }
    };
}

async fn get_latest_group_id() -> Option<i64> {
    let sql = r#"SELECT MAX(id) id FROM "group"."group""#.to_string();
    let rows_temp = Box::new(
        sqlx::query_as::<_, Group>(&sql)
            .fetch_one(PG_POOL.get().unwrap())
            .await,
    );
    match *rows_temp {
        Ok(v) => Some(v.id),
        Err(e) => {
            tracing::error!("{}", e);
            // 表示查询失败
            None
        }
    }
}

pub async fn group_authentication(uuid: i64, group_id: i64) -> bool {
    let member = get_all_member(group_id).await;
    for i in member {
        if i == uuid {
            return true;
        }
    }
    false
}
