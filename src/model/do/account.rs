use chrono::DateTime;

/**
 用户
 */
#[derive(sqlx::FromRow, Clone, Default, Debug)]
pub struct UserDo {
 /// 用户唯一id
 pub id:i64,
 /// 用户名
 pub username:String,
 /// 用户密码
 pub password:String,
 /// 用户其他相关数据
 pub data:String,
 /// 用户注册时间
 pub register_time:DateTime<chrono::Utc>
}

/**
 群聊
 */
#[derive(sqlx::FromRow, Clone, Default, Debug)]
pub struct GroupDo{
 /// 群聊唯一id
 pub id:i64,
 /// 群聊名
 pub name:String,
 /// 群聊拥有者id
 pub own_user_id:i64,
 /// 群聊其他相关数据
 pub data:String,
 /// 群聊创建时间
 pub create_time:DateTime<chrono::Utc>
}

/**
 好友关系
 */
#[derive(sqlx::FromRow, Clone, Default, Debug)]
pub struct FriendDo{
 /// 唯一id
 pub id:i64,
 /// 发出申请的用户id
 pub person_1:i64,
 /// 接受申请的用户id
 pub person_2:i64,
 /// 申请状态
 pub state:i32,
 /// 发出申请的时间
 pub application_time:DateTime<chrono::Utc>
}