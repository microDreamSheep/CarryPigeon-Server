use tracing::instrument;

use super::PG_POOL;
use crate::dao::row::Group;

#[instrument]
pub async fn get_member(group_id: i64) -> Vec<i64> {
    let rows_temp =
        sqlx::query_as::<_, Group>(r#"SELECT member FROM public."group" WHERE id = $1"#)
            .bind(group_id)
            .fetch_one(PG_POOL.get().unwrap())
            .await;

    match rows_temp {
        Ok(v) => v.member,
        Err(e) => {
            tracing::error!("Missing group_id:{} or other error.Info:{}", group_id, e);
            // 表示查询失败
            vec![]
        }
    }
}

#[instrument]
pub async fn get_admin(group_id: i64) -> Vec<i64> {
    let rows_temp = sqlx::query_as::<_, Group>(r#"SELECT admin FROM public."group" WHERE id = $1"#)
        .bind(group_id)
        .fetch_one(PG_POOL.get().unwrap())
        .await;

    match rows_temp {
        Ok(v) => v.admin,
        Err(e) => {
            tracing::error!("Missing group_id:{} or other error.Info:{}", group_id, e);
            // 表示查询失败
            vec![]
        }
    }
}

#[instrument]
pub async fn get_owner(group_id: i64) -> i64 {
    let rows_temp = sqlx::query_as::<_, Group>(r#"SELECT owner FROM public."group" WHERE id = $1"#)
        .bind(group_id)
        .fetch_one(PG_POOL.get().unwrap())
        .await;

    match rows_temp {
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

#[instrument]
pub async fn push_member(group_id: i64, member_id: i64) {
    let rows_temp =
        sqlx::query_as::<_, Group>(r#"SELECT member FROM public."group" WHERE id = $1"#)
            .bind(group_id)
            .fetch_one(PG_POOL.get().unwrap())
            .await;

    let mut member_value = match rows_temp {
        Ok(v) => v.member,
        Err(e) => {
            tracing::error!("Missing group_id:{} or other error.Info:{}", group_id, e);
            // 表示查询失败
            vec![]
        }
    };
    member_value.push(member_id);
    let rows_temp = sqlx::query(r#"UPDATE public."group" SET member = $1 WHERE id = $2"#)
        .bind(member_id)
        .bind(group_id)
        .execute(PG_POOL.get().unwrap())
        .await;
    match rows_temp {
        Ok(_) => {}
        Err(e) => tracing::error!("{}", e),
    }
}

#[instrument]
pub async fn push_admin(group_id: i64, admin_id: i64) {
    let rows_temp = sqlx::query_as::<_, Group>(r#"SELECT admin FROM public."group" WHERE id = $1"#)
        .bind(group_id)
        .fetch_one(PG_POOL.get().unwrap())
        .await;

    let mut member_value = match rows_temp {
        Ok(v) => v.member,
        Err(e) => {
            tracing::error!("Missing group_id:{} or other error.Info:{}", group_id, e);
            // 表示查询失败
            vec![]
        }
    };
    member_value.push(admin_id);
    let rows_temp = sqlx::query(r#"UPDATE public."group" SET admin = $1 WHERE id = $2"#)
        .bind(admin_id)
        .bind(group_id)
        .execute(PG_POOL.get().unwrap())
        .await;
    match rows_temp {
        Ok(_) => {}
        Err(e) => tracing::error!("{}", e),
    }
}

#[instrument]
pub async fn push_new_group<'a>(group: &'a Group) {
    let rows_temp = sqlx::query(r#"INSERT INTO public."group" (id, name, owner, admin, member) VALUES($1 , $2, $3, $4, $5)"#)
        .bind(group.id)
        .bind(&group.name)
        .bind(group.owner)
        .bind(&group.admin)
        .bind(&group.member)
        .execute(PG_POOL.get().unwrap())
        .await;

    match rows_temp {
        Ok(_) => {},
        Err(e) => {
            tracing::error!("{}", e);
        }
    };
}
