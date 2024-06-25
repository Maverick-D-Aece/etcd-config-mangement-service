use etcd_client::Client;
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Clone)]
pub struct AppState {
    pub etcd_client: Arc<Mutex<Client>>,
}
