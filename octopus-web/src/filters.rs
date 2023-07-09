use octopus_common::core::types::Order;
use std::sync::Arc;
use tokio::sync::Mutex;
use warp::Filter;

use crate::{handlers, trading_platform::TradingPlatform};

pub fn routes(
    trading_platform: Arc<Mutex<TradingPlatform>>,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    let deposit = warp::post()
        .and(warp::path!("account" / "deposit"))
        .and(warp::body::json())
        .and(with_platform(trading_platform.clone()))
        .and_then(handlers::deposit);

    let withdraw = warp::post()
        .and(warp::path!("account" / "withdraw"))
        .and(warp::body::json())
        .and(with_platform(trading_platform.clone()))
        .and_then(handlers::withdraw);

    let send = warp::post()
        .and(warp::path!("account" / "send"))
        .and(warp::body::json())
        .and(with_platform(trading_platform.clone()))
        .and_then(handlers::send);

    let order = warp::post()
        .and(warp::path!("order"))
        .and(json_body())
        .and(with_platform(trading_platform.clone()))
        .and_then(handlers::order);

    let balance = warp::post()
        .and(warp::path!("balance"))
        .and(warp::body::json())
        .and(with_platform(trading_platform.clone()))
        .and_then(handlers::account);

    let orderbook = warp::get()
        .and(warp::path!("orderbook"))
        .and(with_platform(trading_platform.clone()))
        .and_then(handlers::orderbook);

    let txlog = warp::get()
        .and(warp::path!("txlog"))
        .and(with_platform(trading_platform.clone()))
        .and_then(handlers::txlog);

    let accounts = warp::get()
        .and(warp::path!("accounts"))
        .and(with_platform(trading_platform))
        .and_then(handlers::accounts);

    deposit
        .or(withdraw)
        .or(send)
        .or(order)
        .or(balance)
        .or(orderbook)
        .or(txlog)
        .or(accounts)
}

fn json_body() -> impl Filter<Extract = (Order,), Error = warp::Rejection> + Clone {
    // When accepting a body, we want a JSON body
    // (and to reject huge payloads)...
    warp::body::content_length_limit(1024 * 16).and(warp::body::json())
}

fn with_platform(
    platform: Arc<Mutex<TradingPlatform>>,
) -> impl Filter<Extract = (Arc<Mutex<TradingPlatform>>,), Error = std::convert::Infallible> + Clone
{
    warp::any().map(move || platform.clone())
}
