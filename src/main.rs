#[macro_use]
extern crate serde_derive;

use futures;
use env_logger;
use std::env;
extern crate jsonwebtoken as jwt;

mod broadcaster;
mod sse_service;
mod mq_server;
mod helpers;
mod endpoint_service;
mod http_server;
mod types;
mod mq_controller;
mod auth_middleware;
mod token;

use http_server::run_server;
use mq_server::run_mq;
use crate::broadcaster::Broadcaster;

#[actix_rt::main]
async fn main() {
    env::set_var("RUST_BACKTRACE", "1"); // Temporary for debugging
    env_logger::init();
    let data = Broadcaster::create();
    // run_server(data.clone()).await;
    // run_mq(&data).await;
    let (s, _) = futures::join!(run_server(data.clone()), run_mq(&data));

    if let Err(e) = s {
        println!("Error running web server! {}", e);
    }
}
