use super::collection::*;
use super::model::*;
use super::stream;
use crate::broadcast;
use crate::common_model::CommonResponse;
use crate::console;
use crate::database::Database;
use actix_web::Responder;
use actix_web::{web, HttpResponse};
use env_logger::{Builder, Env};
use log;
use serde::{Deserialize, Serialize};

pub async fn create(
    database_data: web::Data<Database>,
    broadcaster_data: web::Data<std::sync::Mutex<broadcast::Broadcaster>>,
    content: web::Json<OrderCreateRequest>,
) -> impl Responder {
    info!("[OrderService] Order create requested");

    let collection = database_data.orders().await;

    let create_result = collection.create(content.clone()).await;

    match create_result {
        Ok(result) => {
            let inserted_data = result.inserted_id;

            let response = CommonResponse::<OrderCreateRequest> {
                message: format!("New order created."),
                data: Some(content.clone()),
            };

            let broadcast_message = serde_json::to_string(&response.clone()).unwrap();

            broadcast::broadcast(
                "order_create".to_string(),
                broadcast_message,
                broadcaster_data,
            );

            HttpResponse::Ok().json(inserted_data)
        },
        Err(error) => match error {
            OrderCollectionError::CustomError(message) => {
                let response = CommonResponse::<Order> {
                    message,
                    data: None,
                };

                HttpResponse::BadRequest().json(response)
            },
            _ => {
                let response = CommonResponse::<Order> {
                    message: "Unknown error.".to_string(),
                    data: None,
                };
                HttpResponse::BadRequest().json(response)
            },
        },
    }
}

pub async fn list(
    database_data: web::Data<Database>,
    query: web::Query<OrderListQuery>,
) -> impl Responder {
    info!("[OrderService] Order list requested");

    let offset = query.offset.unwrap_or(0);
    let limit = query.limit.unwrap_or(0);

    let collection = database_data.orders().await;

    let result = collection.list(offset, limit).await;

    match result {
        Ok(orders) => HttpResponse::Ok().json(orders),
        Err(error) => {
            error!("[Service] Failed to get order. Error: {:?}", error);
            HttpResponse::InternalServerError().finish()
        },
    }
}
