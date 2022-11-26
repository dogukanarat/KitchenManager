extern crate dotenv;
use dotenv::dotenv;
use futures::StreamExt;
use log;
use mongodb::{
    bson::{doc, extjson::de::Error},
    results::{DeleteResult, InsertOneResult, UpdateResult},
    Client, Collection,
};
use std::env;

use crate::console;
use crate::orders::{self};

#[derive(Clone)]
pub struct Database {
    collection_orders: orders::collection::OrderCollection,
}

impl Database {
    pub async fn init() -> Self {
        dotenv().ok();

        let uri: Option<String> = match env::var("MONGO_URI") {
            Ok(key) => {
                info!("[Database] Database Url: {}", key);
                Some(key.to_string())
            },
            Err(_) => {
                format!("[Database] Error loading env variable");
                None
            },
        };

        if uri.is_none() {
            panic!("[Database] Database Url is not set");
        } else {
            let client = Client::with_uri_str(uri.unwrap()).await.unwrap();

            let database = client.database("KitchenManager");

            let collection_orders =
                orders::collection::OrderCollection::init(database.clone()).await;

            Database { collection_orders }
        }
    }

    pub async fn orders(&self) -> &orders::collection::OrderCollection {
        &self.collection_orders
    }
}
