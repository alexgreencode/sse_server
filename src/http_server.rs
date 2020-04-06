use actix_cors::Cors;
use actix_web::{middleware, web, App, HttpResponse, HttpServer};
use actix_web::web::Data;
use std::sync::Mutex;

use std::env;
use std::io::BufReader;
use std::fs::File;

use rustls::internal::pemfile::{certs, rsa_private_keys};
use rustls::{NoClientAuth, ServerConfig};

const DEFAULT_ALLOWED_HOSTNAME: &'static str = "http://localhost:3004";
const DEFAULT_PORT: &'static str = "8005";
const PUBLIC_PATHS: [&'static str; 1] = [
    "/test",
];

use crate::broadcaster::Broadcaster;
use crate::sse_service::new_client;
use crate::auth_middleware::Auth;


pub async fn run_server(broadcaster: Data<Mutex<Broadcaster>>) -> std::io::Result<()> {
    let mut config = ServerConfig::new(NoClientAuth::new());
    let cert_file = &mut BufReader::new(File::open("cert/pagespeed_green/STAR_pagespeed_green.crt").unwrap());
    let key_file = &mut BufReader::new(File::open("cert/pagespeed_green/STAR_pg.key").unwrap());
    let cert_chain = certs(cert_file).unwrap();
    let mut keys = rsa_private_keys(key_file).unwrap();
    config.set_single_cert(cert_chain, keys.remove(0)).unwrap();

    let allowed_hostnames = env::var("ALLOWED_HOSTNAME").unwrap_or(DEFAULT_ALLOWED_HOSTNAME.to_string());
    let port = env::var("PORT").unwrap_or(DEFAULT_PORT.to_string());

    println!("allowed_hostnames: {}", allowed_hostnames);
    std::env::set_var("RUST_LOG", "actix_web=info");

    HttpServer::new(move || {
        App::new()
        .wrap(middleware::Logger::default())
        //.wrap(middleware::Compress::default())
        .wrap(Auth::init(PUBLIC_PATHS.to_vec()))
        .wrap(
            Cors::new()
                .allowed_origin(&allowed_hostnames)
                //.allowed_headers(vec![header::ORIGIN, header::AUTHORIZATION, header::ACCEPT, header::CONTENT_TYPE])
                .max_age(864000)
                .supports_credentials()
                .finish()
        )
        .service(web::resource("/test").route(web::get().to(|| {
            HttpResponse::Ok()
                .content_type("text/html; charset=utf-8")
                .body("Ok")
        })))
        .app_data(broadcaster.clone())
        .service(web::resource("/events").route(web::get().to(new_client)))
            //.route("/broadcast/{event}/{msg}", web::get().to(broadcast_all))
    })
    .bind_rustls(format!("0.0.0.0:{}", port), config).expect(&format!("Can not bind to 0.0.0.0:{}", port))
    //.bind(format!("0.0.0.0:{}", port)).expect(&format!("Can not bind to 0.0.0.0:{}", port))
    .shutdown_timeout(600)
    .run()
    .await
}