use crate::trading_platform::TradingPlatform;
use octopus_common::{
    core::types::{AccountBalanceRequest, AccountUpdateRequest, Order, SendRequest},
    errors::OctopusError,
};
use std::{convert::Infallible, sync::Arc};
use tokio::sync::Mutex;

pub async fn deposit(
    deposit: AccountUpdateRequest,
    platform: Arc<Mutex<TradingPlatform>>,
) -> Result<impl warp::Reply, warp::Rejection> {
    let mut platform = platform.lock().await;

    match platform.deposit(&deposit.signer, deposit.amount) {
        Ok(tx) => Ok(warp::reply::json(&tx)),
        Err(err) => Err(warp::reject::custom(OctopusError(err))),
    }
}

pub async fn withdraw(
    withdraw: AccountUpdateRequest,
    platform: Arc<Mutex<TradingPlatform>>,
) -> Result<impl warp::Reply, warp::Rejection> {
    let mut platform = platform.lock().await;

    match platform.withdraw(&withdraw.signer, withdraw.amount) {
        Ok(tx) => Ok(warp::reply::json(&tx)),
        Err(err) => Err(warp::reject::custom(OctopusError(err))),
    }
}

pub async fn send(
    send: SendRequest,
    platform: Arc<Mutex<TradingPlatform>>,
) -> Result<impl warp::Reply, warp::Rejection> {
    let mut platform = platform.lock().await;

    match platform.send(&send.sender, &send.recipient, send.amount) {
        Ok(txs) => Ok(warp::reply::json(&txs)),
        Err(err) => Err(warp::reject::custom(OctopusError(err))),
    }
}

pub async fn order(
    order: Order,
    ledger: Arc<Mutex<TradingPlatform>>,
) -> Result<impl warp::Reply, warp::Rejection> {
    let mut platform = ledger.lock().await;

    match platform.order(order) {
        Ok(receipt) => Ok(warp::reply::json(&receipt)),
        Err(err) => Err(warp::reject::custom(OctopusError(err))),
    }
}

pub async fn account(
    params: AccountBalanceRequest,
    platform: Arc<Mutex<TradingPlatform>>,
) -> Result<impl warp::Reply, warp::Rejection> {
    let platform: tokio::sync::MutexGuard<'_, TradingPlatform> = platform.lock().await;

    match platform.balance_of(&params.signer) {
        Ok(balance) => Ok(warp::reply::json(balance)),
        Err(err) => Err(warp::reject::custom(OctopusError(err))),
    }
}

pub async fn orderbook(
    platform: Arc<Mutex<TradingPlatform>>,
) -> Result<impl warp::Reply, Infallible> {
    let platform = platform.lock().await;

    let orderbook = platform.orderbook();
    Ok(warp::reply::json(&orderbook))
}

pub async fn txlog(platform: Arc<Mutex<TradingPlatform>>) -> Result<impl warp::Reply, Infallible> {
    let platform = platform.lock().await;

    let txlog = platform.txlog();
    Ok(warp::reply::json(&txlog))
}

pub async fn accounts(
    platform: Arc<Mutex<TradingPlatform>>,
) -> Result<impl warp::Reply, Infallible> {
    let platform = platform.lock().await;

    let accounts = platform.accounts();
    Ok(warp::reply::json(&accounts))
}
