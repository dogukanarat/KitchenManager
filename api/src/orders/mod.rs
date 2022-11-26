pub mod collection;
pub mod model;
pub mod service;
pub mod stream;

use actix_web::{web, App, HttpServer, Scope};

pub fn config(config: &mut web::ServiceConfig)
{
    config.service(
        web::scope("/orders")
        .service(
            web::resource("")
            .route(web::post().to(service::create))
            .route(web::get().to(service::list))
        )
        .service(
            web::resource("/events/create")
            .route(web::get().to(stream::order_create))
        )
        // .service(
        //     web::resource("/{id}")
        //     .route(web::get().to(service::get))
        //     .route(web::put().to(service::update))
        //     .route(web::delete().to(service::delete))
        // )
        
    );
}
