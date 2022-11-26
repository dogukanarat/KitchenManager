use std::{sync::Mutex, collections::HashMap};

use actix_web::{web, Responder, HttpResponse, http::header};

use crate::{broadcast};

pub async fn order_create(
    broadcaster: web::Data<Mutex<broadcast::Broadcaster>>,
) -> impl Responder {

    let mut new_hashmap = HashMap::<String, String>::new();

    new_hashmap.insert("order_create".to_string(), "".to_string());

    let rx = broadcaster.lock().unwrap().new_client(new_hashmap);

    HttpResponse::Ok()
        .append_header(header::ContentType("text/event-stream".parse().unwrap()))
        .no_chunking(1024 * 5)
        .streaming(rx)
}