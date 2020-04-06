use lapin::{
    options::*, types::FieldTable, Connection,
    ConnectionProperties
};
use actix_web::web::Data;
use std::sync::Mutex;
use std::{thread, time};

use crate::broadcaster::Broadcaster;
use crate::mq_controller::action_handler;

fn get_mq_url() -> String {
    let host = std::env::var("MQ_HOST").unwrap_or("localhost:5672".to_string());
    let username = std::env::var("MQ_USERNAME").unwrap_or("pg".to_string());
    let password = std::env::var("MQ_PASSWORD").unwrap_or("pg".to_string());
    
    format!("amqp://{}:{}@{}/%2f", username, password, host)
}

pub async fn run_mq(broadcaster: &Data<Mutex<Broadcaster>>) {
    println!("Waiting for starting server.....");
    thread::sleep(time::Duration::from_millis(10000));
    let url = get_mq_url(); 
    println!("Connecting to {} ...", url);
    let conn = match Connection::connect(&url, ConnectionProperties::default())
	.await {
        Ok(c) => c,
        Err(e) => {
            println!("Couldn't connect to RabbitMQ. - {}", e);
            println!("Waiting for more....");
            thread::sleep(time::Duration::from_millis(20000));

            Connection::connect(&url, ConnectionProperties::default()).await.expect("Couldn't connect to RabbitMQ. - {}")
        },
    };

    println!("=== Connected === ");

    //receive channel
    let channel = conn.create_channel().wait().expect("create_channel");
    println!("[{}] state: {:?}", line!(), conn.status().state());

    let queue = channel
        .queue_declare(
            "pg_sse",
            QueueDeclareOptions::default(),
            FieldTable::default(),
        )
        .await
        .expect("queue_declare");
    println!("[{}] state: {:?}", line!(), conn.status().state());

    println!("will consume ->");
    let consumer = channel
        .basic_consume(
            &queue,
            "pg_sse", 
            BasicConsumeOptions::default(),
            FieldTable::default(),
        )
        .await
        .expect("basic_consume");
    println!("[{}] state: {:?}", line!(), conn.status().state());

    //let th = thread::spawn(move || async {
    //    println!("Consuming...");

    //    consumer
    //        .for_each(move |delivery| {
    //            thread::sleep(time::Duration::from_millis(100));
//
    //            let delivery = delivery.expect("Couldn't receive delivery from RabbitMQ.");
    //            let msg = std::str::from_utf8(&delivery.data).unwrap_or_default();
    //            println!("msg: {:?}", msg);
    //            action_handler(msg, bro/'\adcaster).unwrap_or_default();
    //            println!("Sending acknowledge back");
    //            channel
    //            .basic_ack(delivery.delivery_tag, BasicAckOptions::default())
    //            .map(|_| ())
    //        })
    //        .await;
    //});

    //let _ = th.join();

    for delivery in consumer {
        thread::sleep(time::Duration::from_millis(100));
        if let Ok(delivery) = delivery {
            let msg = std::str::from_utf8(&delivery.data).unwrap_or_default();
            println!("msg: {:?}", msg);
            match action_handler(msg, broadcaster) {
                Ok(_) => {
                    println!("Sending acknowledge back");
                    channel
                    .basic_ack(delivery.delivery_tag, BasicAckOptions::default())
                    .await
                    .expect("basic_ack");
                },
                Err(e) => {
                    println!("Error in action handling - {}", e);
                    channel
                    .basic_ack(delivery.delivery_tag, BasicAckOptions::default())
                    .await
                    .expect("basic_ack");
                },
            };
        } else {
            println!("Couldn't receive delivery from RabbitMQ.");
        }
    }

    println!("========================= THE END ========================");
}