use std::pin::Pin;
use std::sync::Mutex;
use std::task::{Context, Poll};
use std::time::Duration;
use std::collections::HashMap;

use actix_web::web;
use actix_web::{Error};
use futures::{Stream, StreamExt};
use tokio::sync::mpsc::{channel, Receiver, Sender};
use tokio::time::{interval_at, Instant};

pub fn broadcast(
    event: String,
    msg: String,
    broadcaster: web::Data<Mutex<Broadcaster>>,
) -> () {
    broadcaster.lock().unwrap().send(&event, &msg);
}


pub struct Broadcaster {
    clients: Vec<Sender<web::Bytes>>,
}

impl Broadcaster {
    pub fn create() -> web::Data<Mutex<Self>> {
        // Data ≃ Arc
        let me = web::Data::new(Mutex::new(Broadcaster::new()));

        // ping clients every 10 seconds to see if they are alive
        Broadcaster::spawn_ping(me.clone());

        me
    }

    pub fn new() -> Self {
        Broadcaster {
            clients: Vec::new()
        }
    }

    pub fn spawn_ping(me: web::Data<Mutex<Self>>) {
        actix_rt::spawn(async move {
            let mut task = interval_at(Instant::now(), Duration::from_secs(10));
            loop {
                task.tick().await;
                me.lock().unwrap().remove_stale_clients();
            }
        });
    }

    pub fn remove_stale_clients(&mut self) {
        let mut ok_clients = Vec::new();
        for client in self.clients.iter() {
            let result = client.clone().try_send(web::Bytes::from("event: internal_status\ndata: ping\n\n"));

            if let Ok(()) = result {
                ok_clients.push(client.clone());
            }
        }
        self.clients = ok_clients;
    }

    pub fn new_client(&mut self, collection: HashMap<String, String>) -> Client 
    {
        let (tx, rx) = channel(100);

        let tx_clone = tx.clone();

        let mut new_collection : HashMap<String, String> = HashMap::new();

        if collection.is_empty() {
            new_collection.insert("internal_status".to_owned(), "connected".to_owned());
        } else {
            new_collection = collection;
        }

        for (evt, msg) in new_collection {
            let msg = web::Bytes::from(["event: ", &evt, "\ndata: ", &msg, "\n\n"].concat());

            tx_clone.clone()
                .try_send(msg)
                .unwrap();

            self.clients.push(tx_clone.clone());
        }

        Client(rx)
    }

    pub fn send(&self, event: &str, message: &str) 
    {
        let data = ["event: ", event, "\n", "data: ", message, "\n\n"].concat();

        let bytes = web::Bytes::from(data);

        for client in self.clients.iter() {
            client.clone().try_send(bytes.clone()).unwrap_or(());
        }
    }
}

// wrap Receiver in own type, with correct error type
pub struct Client(Receiver<web::Bytes>);

impl Stream for Client {
    type Item = Result<web::Bytes, Error>;

    fn poll_next(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
    ) -> Poll<Option<Self::Item>> {

        let mut new_pin_data : Pin::<&mut Receiver<web::Bytes>> = Pin::new(&mut self.0);

        match new_pin_data.poll_recv(cx) {
            Poll::Ready(Some(v)) => Poll::Ready(Some(Ok(v))),
            Poll::Ready(None) => Poll::Ready(None),
            Poll::Pending => Poll::Pending,
        }
    }
}