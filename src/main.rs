use actix_web::{web, App, HttpServer};
use etcd_client::Client;
use std::sync::Arc;
use tokio::sync::Mutex;

mod handlers;
mod state;

use state::AppState;
use crate::handlers::{put_key, get_key, delete_key};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    // Connect to the ETCD client
    let etcd_client = Client::connect(["http://127.0.0.1:2379"], None)
        .await
        .expect("Failed to connect to ETCD");

    // Wrap the ETCD client in shared state
    let app_state = web::Data::new(AppState {
        etcd_client: Arc::new(Mutex::new(etcd_client)),
    });

    // Start the HTTP server
    HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            .route("/put", web::post().to(put_key))
            .route("/get/{key}", web::get().to(get_key))
            .route("/delete/{key}", web::delete().to(delete_key))
    })
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
