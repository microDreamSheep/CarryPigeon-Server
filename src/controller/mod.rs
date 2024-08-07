/*!
controller层，用于分发前端发送过来的数据
前端与后端交互的数据格式必须为基本的数据格式，复合格式统一使用VO层进行封，只允许以json形式提交数据
详细请求响应规范请见model::protocol::{http,ws}
 */

/**
 账户相关模块，用于账户相关操作
 */
pub mod account;
pub mod tree_hole;
pub mod chat;
