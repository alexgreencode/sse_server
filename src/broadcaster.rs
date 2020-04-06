use std::pin::Pin;
use std::sync::Mutex;
use std::task::{Context, Poll};
use tokio::sync::mpsc::{channel, Receiver, Sender};
use tokio::time::{interval_at, Instant};
use std::time::{Duration};
use futures::{Stream, StreamExt};
use actix_web::web::{Bytes, Data};
use actix_web::{Error};

pub struct ClientItem {
    sub: String,
    tx: Sender<Bytes>,
}

pub struct Broadcaster {
    clients: Vec<ClientItem>,
}

impl Broadcaster {
    pub fn create() -> Data<Mutex<Self>> {
        // Data ≃ Arc
        let me = Data::new(Mutex::new(Broadcaster::new()));

        // ping clients every 10 seconds to see if they are alive
        Broadcaster::spawn_ping(me.clone());

        me
    }

    fn new() -> Self {
        Broadcaster {
            clients: Vec::new(),
        }
    }

    fn spawn_ping(me: Data<Mutex<Self>>) {
        actix_rt::spawn(async move {
            let mut task = interval_at(Instant::now(), Duration::from_secs(30));
            while let Some(_) = task.next().await {
                me.lock().unwrap().remove_stale_clients();
            }
        })
    }

    fn remove_stale_clients(&mut self) {
        let mut ok_clients: Vec<ClientItem>  = Vec::new();
        for client in self.clients.iter() {
            let result = client.tx.clone().try_send(Bytes::from("data: ping\n\n"));

            if let Ok(()) = result {
                ok_clients.push(
                    ClientItem {
                        sub: client.sub.clone(),
                        tx: client.tx.clone(),
                    }
                );
            }
        }
        self.clients = ok_clients;
    }

    pub fn new_client(&mut self, sub: &str) -> Client {
        let (tx, rx) = channel(100);

        tx.clone()
            .try_send(Bytes::from("data: connected\r\n\r\n"))
            .unwrap();

        self.clients.push(
            ClientItem {
                sub: sub.to_string(),
                tx,
            }
        );
        Client(rx)
    }

    pub fn send_to_all(&self, event: &str, msg: &str) {
        let msg = Bytes::from(format!("event: {}\r\ndata: {}\r\n\r\n", event, msg));

        for client in self.clients.iter() {
            println!("sending message!!!");
            client.tx.clone().try_send(msg.clone()).unwrap_or(());
        }
    }
    pub fn send(&self, sub: &str, event: &str, msg: &str) -> Result<(), ()> {
        let msg = Bytes::from(format!("event: {}\r\ndata: {}\r\n\r\n", event, msg));
        let mut is_success = false;
        self.clients.iter()
            .filter(|item| item.sub == sub)
            .for_each(|client| {
                println!("sending message event: {} to sub: {}", event, sub);
                if let Err(e) = client.tx.clone().try_send(msg.clone()) {
                    println!("Error when sending sse message: {}", e);
                } else {
                    // The message sent to at least one client is success
                    is_success = true;
                }
            });

        if is_success {
            return Ok(());
        }

        Err(())
    }
}

// wrap Receiver in own type, with correct error type
pub struct Client(Receiver<Bytes>);

impl Stream for Client {
    type Item = Result<Bytes, Error>;

    fn poll_next(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
    ) -> Poll<Option<Self::Item>> {
        match Pin::new(&mut self.0).poll_next(cx) {
            Poll::Ready(Some(v)) => Poll::Ready(Some(Ok(v))),
            Poll::Ready(None) => Poll::Ready(None),
            Poll::Pending => Poll::Pending,
        }
    }
}
