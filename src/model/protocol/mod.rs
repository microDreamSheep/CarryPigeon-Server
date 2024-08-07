/*!
此包用于规范通信中请求和响应的json结构
 */
/**
http请求的相关规范
 */
pub mod http;
/**
通过websocket进行通信的相关规范
 */
pub mod ws;


/*相关常量*/
pub const SUCCESS_CODE:u32 = 200;
pub const ERROR_CODE:u32 = 100;