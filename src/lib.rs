#[cfg(test)]
mod test;

/**
 controller模块，用于处理相关请求
 */
pub mod controller;
/**
service模块，用于处理业务逻辑
 */
pub mod service;
/**
 dao模块，用于存储相关数据结构
 */
pub mod dao;
/**
 repository模块，用于数据持久话
 */
pub mod repository;
/**
 工具包，包含的相关的工具函数
 */
pub mod utils;
/**
 模型包，包含各种实体结构体和模型
 */
pub mod model;
/**
 webSocket包，用于自定义webSocket协议
 */
pub mod ws;
pub mod manager;