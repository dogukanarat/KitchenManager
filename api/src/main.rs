#![allow(non_snake_case)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(dead_code)]

use actix_cors::Cors;
use actix_files::{Files, NamedFile};
use actix_web::{
    get, guard, http::header, middleware, post, web, App, HttpResponse, HttpServer, Responder, http
};

use dotenv::dotenv;
use env_logger::{Builder, Env};
use std::{
    collections::HashMap,
    env, io,
    sync::{Arc, Mutex},
};

#[macro_use]
extern crate log;

mod broadcast;
mod common_model;
mod console;
mod constants;
mod database;
mod orders;

#[actix_rt::main]
async fn main() -> io::Result<()> {
    // load .env file
    dotenv().ok();

    // get required environment variables
    env::set_var("RUST_LOG", "debug");

    info!("Initializing environment variables...");
    // start logger
    env_logger::init();

    // create managers
    let database_manager = database::Database::init().await;
    let broadcast_manager = broadcast::Broadcaster::create();

    // info message for listing server address and port
    info!("Listening on {}...", constants::SERVER_PORT);

    // start http server
    HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::default())
            .wrap(Cors::permissive())
            .app_data(web::Data::new(database_manager.clone()))
            .app_data(web::Data::from(Arc::clone(&broadcast_manager)))
            .service(
                web::scope("/v1")
                    .configure(orders::config),
            )
            .route("/", web::get().to(index))
    })
    .bind(("0.0.0.0", constants::SERVER_PORT))?
    .run()
    .await
}

async fn index() -> impl Responder {
    let content = r#"<html lang="en">
    <head>
        <meta charset="UTF-8">
        <meta name="viewport" content="width=device-width, initial-scale=1.0">
        <meta http-equiv="X-UA-Compatible" content="ie=edge">
        <title>Server-sent events</title>
        <style>
            p {
                margin-top: 0.5em;
                margin-bottom: 0.5em;
            }
        </style>
    </head>
    <body>
        <div>
        Order Activities:
        <span id="root"></span>
        </div>
        <script>
            let root = document.getElementById("root");

            let events = new EventSource("/v1/orders/events/create");
            
            events.addEventListener("order_create", (event) => {
                let data = document.createElement("p");
                root.appendChild(data);
                data.innerText = event.data;
            });
        </script>
    </body>
    </html>"#;

    HttpResponse::Ok()
        .append_header(header::ContentType("text/html".parse().unwrap()))
        .body(content)
}
