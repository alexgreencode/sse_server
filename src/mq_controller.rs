use serde_json;
use actix_web::web::Data;
use std::sync::Mutex;

use crate::broadcaster::Broadcaster;
use crate::types::{
    SseMessage,
    ActionType,
};
use crate::sse_service::{broadcast};

pub fn action_handler(msg_str: &str, broadcaster: &Data<Mutex<Broadcaster>>) -> Result<(), &'static str> {
    match serde_json::from_str::<SseMessage<String>>(msg_str) {
        Ok(msg) => {
            match msg.action {
                ActionType::PushMetric => {
                    println!("Let's send a message -------->");
                    match broadcast(&msg.sub, &msg.event, &msg.payload, broadcaster) {
                        Ok(_) => { return Ok(()); },
                        Err(_) => { return Err("error"); },
                    };
                },
                _ => {
                    println!("Action is not supported: {:?}", msg.action);
                    return Err("Action is not supported");
                },
            }
        },
        Err(e) => {
            println!("Bad Request -> {}", e);
            Err("Action is not supported")
        }
    }
}