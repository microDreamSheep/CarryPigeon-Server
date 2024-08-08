/*!
 数据库持久化相关操作，repository是对dao层的一层封装，持久化操作通过对dao层的调用实现
 直接与service/manager层进行交互，service/manager不应该越过repository层与dao层进行交互
 */

/**
 帐号相关的数据库操作
 */
pub mod account;
/**
 消息相关数据库操作
 */
pub mod message;
