extern crate dotenv;
use dotenv::dotenv;
use futures::StreamExt;
use mongodb::{
    bson::{doc, extjson::de::Error, oid::ObjectId, self},
    results::{DeleteResult, InsertOneResult, UpdateResult},
    Client, Collection,
};
use std::{env, str::FromStr};

use super::model::*;
use crate::console;

#[derive(Clone)]
pub struct OrderCollection
{
    collection_order: Collection<Order>,
}

#[derive(Debug)]
pub enum OrderCollectionError
{
    OrderNotFound,
    CustomError(String),
}

impl OrderCollection
{
    /// Creates a new instance of the OrderCollection
    ///
    /// # Arguments
    ///
    /// * `database` - The database to use
    ///
    ///
    pub async fn init(database: mongodb::Database) -> Self
    {
        let collection_order: Collection<Order> = database.collection("Orders");

        OrderCollection { collection_order }
    }

    /// Create new order
    ///
    /// # Arguments
    ///
    /// * `content` - OrderCreateRequest
    /// * `products` - ProductCollection
    ///
    /// # Examples
    ///
    /// ```
    /// // TODO
    ///
    /// ```
    pub async fn create(
        &self,
        content: OrderCreateRequest,
    ) -> Result<InsertOneResult, OrderCollectionError>
    {
        self.info("Order create requested").await;

        let new_order = Order {
            id: None,
            order_id: content.id,
            order_shelf: content.shelf,
            created_at: chrono::Utc::now(),
        };

        let inser_one_result = self.collection_order.insert_one(
            new_order,
            None
        ).await;

        match inser_one_result
        {
            Ok(result) => Ok(result),
            Err(error) => Err(OrderCollectionError::CustomError(error.to_string())),
        }
    }

    /// Get all orders
    ///
    ///
    pub async fn list(&self, offset: u64, limit: i64) -> Result<Vec<Order>, OrderCollectionError>
    {
        self.info("List orders requested").await;

        let find_options = mongodb::options::FindOptions::builder()
            .skip(offset)
            .limit(limit)
            .build();

        let mut cursor = self.collection_order.find(None, find_options).await.unwrap();

        let mut orders: Vec<Order> = Vec::new();

        while let Some(result) = cursor.next().await
        {
            match result
            {
                Ok(document) =>
                {
                    orders.push(document);
                },
                Err(error) => return Err(OrderCollectionError::CustomError(error.to_string())),
            }
        }

        Ok(orders)
    }

    /// Get a single order
    ///
    /// # Arguments
    ///
    /// * `id` - ObjectId
    /// * `order_id` - String
    ///
    /// # Examples
    ///
    /// ```
    /// // TODO
    ///
    /// ```
    pub async fn get(&self, req_id: String) -> Result<Order, OrderCollectionError>
    {
        self.info("Single order get requested").await;

        let id = ObjectId::from_str(&req_id).unwrap();

        let filter = doc! { "_id": id };

        let result = self.collection_order.find_one(filter, None).await;

        match result
        {
            Ok(result) => match result
            {
                Some(order) => Ok(order),
                None => Err(OrderCollectionError::OrderNotFound),
            },
            Err(error) => Err(OrderCollectionError::CustomError(error.to_string())),
        }
    }

    /// Delete a single order
    /// 
    /// # Arguments
    /// 
    /// * `req_id` - ObjectId
    /// 
    /// ```
    /// # Examples
    /// 
    /// ```
    pub async fn delete(&self, req_id: String) -> Result<DeleteResult, OrderCollectionError>
    {
        self.info("Single order delete requested").await;

        let id = ObjectId::from_str(&req_id).unwrap();

        let filter = doc! { "_id": id };

        let delete_one_result = self.collection_order.delete_one(filter, None).await;

        match delete_one_result
        {
            Ok(result) => match result.deleted_count
            {
                1 => Ok(result),
                _ => Err(OrderCollectionError::OrderNotFound),
            },
            Err(error) => Err(OrderCollectionError::CustomError(error.to_string())),
        }
    }

    pub async fn info(&self, msg: &str)
    {
        info!("[OrderCollection] {}", msg);
    }
}
