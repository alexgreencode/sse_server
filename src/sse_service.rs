use actix_web::web::{Data, Path};
use actix_web::{HttpResponse, HttpRequest, Responder};
use std::sync::Mutex;

use crate::broadcaster::{Broadcaster};
use crate::helpers::get_sub_from_query;

pub async fn new_client(req: HttpRequest, broadcaster: Data<Mutex<Broadcaster>>) -> impl Responder {
    let sub = get_sub_from_query(req.query_string());
    //println!("Sub here ========> {}", sub);
    let rx = broadcaster.lock().unwrap().new_client(&sub);

    HttpResponse::Ok()
        .header("content-type", "text/event-stream")
        .no_chunking()
        .streaming(rx)
}

pub async fn broadcast_all(
    msg: Path<String>,
    broadcaster: Data<Mutex<Broadcaster>>,
) -> impl Responder {
    broadcaster.lock().unwrap().send_to_all("test_result", &msg.into_inner());

    HttpResponse::Ok().body("msg sent")
}

pub fn broadcast(
    sub: &str,
    event: &str,
    msg: &str,
    broadcaster: &Data<Mutex<Broadcaster>>,
) -> Result<(), ()> {
    broadcaster.lock().unwrap().send(sub, event, msg)
}