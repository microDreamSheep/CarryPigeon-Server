/*!
controller层，用于处理前端的请求

前端与后端交互的数据格式必须为基本的数据格式，复合格式统一使用VO层进行封，只允许以json形式提交数据

详细请求响应规范请见model::protocol::{http,ws}
 */

/**
 账户相关模块，用于账户相关操作
 */
pub mod account;
/**
 树洞功能相关的controller
 */
pub mod tree_hole;
/**
 具体聊天服务相关的controller
 */
pub mod chat;
