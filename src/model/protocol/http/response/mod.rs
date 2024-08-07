/*!
http请求返回值标准格式，包含状态码和data

```json
{
    "code":,
    "data":
}
```

携带数据则需传入data,data的结构体需使用 #[derive(Serialize)] ，返回JSONResponse<'a,T: ToJSON>

不携带数据则直接调用success_without_data()等方法，返回JSONResponseWithoutData
 */

use rocket_json_response::{JSONResponse, JSONResponseWithoutData, ToJSON};
use crate::model::protocol::{ERROR_CODE, SUCCESS_CODE};

pub struct HttpResponse;

impl HttpResponse {
    pub fn success<'a,T: ToJSON>(data:T)->JSONResponse<'a,T>{
        JSONResponse::err(SUCCESS_CODE,data)
    }

    pub fn error<'a,T: ToJSON>(data:T)->JSONResponse<'a,T>{
        JSONResponse::err(ERROR_CODE,data)
    }

    pub fn response<'a,T: ToJSON>(code:u32,data:T) ->JSONResponse<'a,T>{
        JSONResponse::err(code,data)
    }
    pub fn success_without_data() -> JSONResponseWithoutData {
        JSONResponseWithoutData::ok()
    }
    pub fn error_without_data() -> JSONResponseWithoutData{
        JSONResponseWithoutData::err(ERROR_CODE)
    }
}
