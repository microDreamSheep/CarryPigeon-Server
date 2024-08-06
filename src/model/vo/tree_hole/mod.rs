use rocket::serde::{Deserialize, Serialize};
use rocket_json_response::serialize_to_json;
use crate::model::dto::tree_hole::TreeHoleSendDTO;

#[derive(Clone,Debug,Deserialize,Serialize)]
pub struct TreeHoleSendVO {
    pub user_id:i64,
    pub data:String,
    pub token:String
}

impl TreeHoleSendVO {
    pub fn to_dto(self)->TreeHoleSendDTO{
        TreeHoleSendDTO{
            user_id: self.user_id,
            data: self.data,
        }
    }
}

#[derive(Clone,Debug,Deserialize,Serialize)]
pub struct TreeHoleSendResponseVO {
    pub msg:String
}
serialize_to_json!(TreeHoleSendResponseVO);

impl TreeHoleSendResponseVO {
    pub fn success()-> TreeHoleSendResponseVO {
        TreeHoleSendResponseVO {
            msg: "tree hole post success".to_string(),
        }
    }
    pub fn error(error_msg:String)-> TreeHoleSendResponseVO {
        TreeHoleSendResponseVO {
            msg: error_msg,
        }
    }
}
