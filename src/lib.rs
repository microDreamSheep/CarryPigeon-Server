#![feature(fn_traits)]

#[cfg(test)]
mod test;

/**
 controller模块，用于处理相关请求，包括websocket请求分发和http请求格式分发
 */
pub mod controller;
/**
service模块，用于具体处理业务逻辑
 */
pub mod service;
/**
 dao模块，用于提供基础的数据库接口操作，通过rbatis进行生成接口
 */
pub mod dao;
/**
 repository模块，用于数据持久化，是对dao接口的一层封装
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
 webSocket包，用于自定义webSocket响应，包括websocket请求的响应请求
 */
pub mod ws;
/**
 manager，全局的管理器，用于管理存储在内存中的数据，例如在线的用户及其发送通道和用于验证的token
 */
pub mod manager;