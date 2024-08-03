use rocket_json_response::JSONResponse;

const SUCCESS_CODE:u32 = 200;
const ERROR_CODE:u32 = 100;

pub struct Response;

impl Response{
    pub fn success<'a,T: rocket_json_response::ToJSON>(data:T)->JSONResponse<'a,T>{
        JSONResponse::err(SUCCESS_CODE,data)
    }

    pub fn error<'a,T: rocket_json_response::ToJSON>(data:T)->JSONResponse<'a,T>{
        JSONResponse::err(ERROR_CODE,data)
    }

    pub fn response<'a,T: rocket_json_response::ToJSON>(code:u32,data:T) ->JSONResponse<'a,T>{
        JSONResponse::err(code,data)
    }
}