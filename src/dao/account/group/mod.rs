use rbatis::{crud, impl_select};
use rbatis::rbdc::DateTime;
use serde::{Deserialize, Serialize};

/**
群聊
 */
#[derive(Clone, Default, Debug,Deserialize,Serialize)]
pub struct Group {
    /// 群聊唯一id
    pub id:Option<i64>,
    /// 群聊名
    pub name:Option<String>,
    /// 群聊拥有者id
    pub own_user_id:Option<i64>,
    /// 群聊其他相关数据
    pub data:Option<String>,
    /// 群聊创建时间
    pub create_time:Option<DateTime>
}

crud!(Group{});

/**
群聊用户
 */
#[derive(Clone, Default, Debug,Deserialize,Serialize)]
pub struct GroupMember {
    /// 唯一id
    pub id:Option<i64>,
    /// 群组id
    pub group_id:Option<i64>,
    /// 用户id
    pub user_id:Option<i64>,
    /// 权限 1：群主 2：管理员 3：普通用户
    pub permission:Option<i32>,
    /// 用户状态：1：待处理 2：已同意 3：已拒绝
    pub state: Option<i32>,
    /// 申请发送时间
    pub application_time:Option<DateTime>

}

crud!(GroupMember{});

impl_select!(GroupMember{select_all_member(group_id:&i64) => "`where group_id=#{group_id} and state = 2`"});