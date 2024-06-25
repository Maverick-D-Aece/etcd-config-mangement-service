use actix_web::{HttpResponse, Responder, web};
use etcd_client::PutOptions;
use serde::{Deserialize, Serialize};

use crate::state::AppState;

#[derive(Serialize, Deserialize)]
pub struct KeyValue {
    pub key: String,
    pub value: String,
}

// Handler for putting a key-value pair into ETCD
pub async fn put_key(data: web::Data<AppState>, key_value: web::Json<KeyValue>) -> impl Responder {
    let mut client = data.etcd_client.lock().await;
    match client
        .put(
            key_value.key.clone(),
            key_value.value.clone(),
            Some(PutOptions::default()),
        )
        .await
    {
        Ok(_) => HttpResponse::Ok().json("Key inserted successfully"),
        Err(e) => HttpResponse::InternalServerError().json(format!("Error: {}", e)),
    }
}

// Handler for getting a key-value pair from ETCD
pub async fn get_key(data: web::Data<AppState>, path: web::Path<String>) -> impl Responder {
    let key = path.into_inner();
    let mut client = data.etcd_client.lock().await;
    match client.get(key, None).await {
        Ok(response) => {
            if let Some(kv) = response.kvs().first() {
                HttpResponse::Ok().json(KeyValue {
                    key: kv.key_str().unwrap().to_string(),
                    value: kv.value_str().unwrap().to_string(),
                })
            } else {
                HttpResponse::NotFound().json("Key not found")
            }
        }
        Err(e) => HttpResponse::InternalServerError().json(format!("Error: {}", e)),
    }
}

// Handler for deleting a key-value pair from ETCD
pub async fn delete_key(data: web::Data<AppState>, path: web::Path<String>) -> impl Responder {
    let key = path.into_inner();
    let mut client = data.etcd_client.lock().await;
    match client.delete(key, None).await {
        Ok(_) => HttpResponse::Ok().json("Key deleted successfully"),
        Err(e) => HttpResponse::InternalServerError().json(format!("Error: {}", e)),
    }
}
