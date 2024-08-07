use crate::model::dto::tree_hole::TreeHoleSendDTO;
use crate::repository::message::push_tree_hole_message_repository;
use crate::service::account::user::user_authority_check_service;

pub async fn tree_hole_send_service(token: String, data: TreeHoleSendDTO) ->Result<(),String>{
    // 校验用户权限
    if !user_authority_check_service(&data.user_id,token).await {
        return Err("authority check error".to_string());
    }
    // 调用服务接口发送数据
    return match push_tree_hole_message_repository((&data).user_id,(data).data).await {
        Ok(_)=> {Ok(())}
        Err(e)=> {Err(e.to_string())}
    }
}