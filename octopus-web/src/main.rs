#![allow(dead_code)]

mod accounting;
mod core;
mod filters;
mod handlers;
mod trading_platform;

use std::sync::Arc;
use tokio::sync::Mutex;
use trading_platform::TradingPlatform;

const SERVER_ADDR: &str = "127.0.0.1:8080";

#[tokio::main]
async fn main() {
    pretty_env_logger::init();

    let trading_platform = Arc::new(Mutex::new(TradingPlatform::new()));
    let routes = filters::routes(trading_platform);

    warp::serve(routes).run(([127, 0, 0, 1], 8080)).await;
    // warp::serve(routes).run(SERVER_ADDR).await;
}
