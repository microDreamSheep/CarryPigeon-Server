#[macro_use] extern crate rocket;

use std::sync::Arc;
use sqlx::postgres::PgPoolOptions;
use tracing_appender::{non_blocking, rolling};
use tracing_subscriber::{
    filter::EnvFilter, fmt, layer::SubscriberExt, util::SubscriberInitExt, Registry,
};
#[rocket::main]
async fn main() -> Result<(),rocket::Error>{

    // 处理tracing输出和调用
    let env_filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));
    // 输出到控制台中
    let formatting_layer = fmt::layer();
    
    // 输出到文件中
    let file_appender = rolling::never("logs", "log.txt");
    let (non_blocking_appender, _guard) = non_blocking(file_appender);
    let file_layer = fmt::layer()
        .with_ansi(false)
        .with_writer(non_blocking_appender);
        
    // 注册
    Registry::default()
        .with(env_filter)
        .with(formatting_layer)
        .with(file_layer)
        .init();

    //
    //
    // rocket
    use carrypigeon_server::controller::authenticator::post_authenticator;

    let _rocket = rocket::build()
        .mount("/",routes![post_authenticator])
        .launch()
        .await?;
    Ok(())
}