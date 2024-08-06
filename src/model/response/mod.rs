use lazy_static::lazy_static;
use rocket::serde::json::serde_json::json;
use rocket::serde::json::Value;
use rocket_json_response::{JSONResponse, ToJSON};
use serde::{Deserialize, Serialize};

const SUCCESS_CODE:u32 = 200;
const ERROR_CODE:u32 = 100;

/**
 response 用于前端进行数据交换
 */
pub struct Response;

impl Response{
    pub fn success<'a,T: ToJSON>(data:T)->JSONResponse<'a,T>{
        JSONResponse::err(SUCCESS_CODE,data)
    }

    pub fn error<'a,T: ToJSON>(data:T)->JSONResponse<'a,T>{
        JSONResponse::err(ERROR_CODE,data)
    }

    pub fn response<'a,T: ToJSON>(code:u32,data:T) ->JSONResponse<'a,T>{
        JSONResponse::err(code,data)
    }
}

/**
 用于websocket用于数据返回
 */
#[derive(Debug,Deserialize,Serialize,Clone)]
pub struct WebSocketResponse{
    /// 状态码，用于表示请求成功或者失败
    pub code:u32,
    /// 请求id，用于本地客户端使用用于鉴别返回值，如果为-1则为服务端主动向客户端发送消息，例如其他用户发送了消息发到本地
    pub id:i64,
    /// 返回值携带的数据
    pub data:Option<Value>
}

impl WebSocketResponse {
    pub fn success(data:Value)->WebSocketResponse{
        WebSocketResponse{
            code: SUCCESS_CODE,
            id: -1,
            data: Some(data)
        }
    }

    pub fn error(data:Value)->WebSocketResponse{
        WebSocketResponse{
            code: ERROR_CODE,
            id: -1,
            data: Some(data)
        }
    }

    pub fn to_json(self)->String{
        json!(self).to_json()
    }
}

/*标准的响应*/

lazy_static!(
    /**
    异常的route，用于分配路径失败使用
     */
    pub static ref WEBSOCKET_RESPONSE_ROUTE_ERROR: WebSocketResponse = WebSocketResponse {
        code: ERROR_CODE,
        id: -1,
        data: Some(json!("no such route")),
    };
);


lazy_static!(
    /**
    异常的参数，用于参数分析失败时使用
    */
    pub static ref WEBSOCKET_RESPONSE_CONTENT_STRUCTURE_ERROR:WebSocketResponse = WebSocketResponse{
    code: ERROR_CODE,
    id: -1,
    data: Some(json!("the analyse of the json meet some wrong")),
};
);

lazy_static!(
    /**
    用于测试使用
     */
    pub static ref WEBSOCKET_RESPONSE_ERROR:WebSocketResponse = WebSocketResponse{
    code: SUCCESS_CODE,
    id: -1,
    data: None
};
);
