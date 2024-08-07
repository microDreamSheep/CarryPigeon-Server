/*!
服务层，用于提供具体的服务

方法定义规范:
 1. 内部方法必须为异步方法
 2. 内部方法必须以_service结尾
 3. 内部方法接收的参数必须为基本的数据类型或者DTO层的结构体
 */

/**
 账户相关内容
 */
pub mod account;
/**
 tree hole 相关服务
 */
pub mod tree_hole;