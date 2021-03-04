use std::collections::HashMap;
use std::error::Error;
use std::io::{Error as IOError, ErrorKind};
use std::sync::Arc;
use tokio::sync::{mpsc::UnboundedSender, RwLock};
use uuid::Uuid;
use warp::ws::Message;

use crate::client::Client;
use crate::protocol::Output;

#[derive(Debug, Clone)]
pub struct ClientPool(Arc<RwLock<HashMap<Uuid, Client>>>);

impl ClientPool {
    pub fn new() -> Self {
        ClientPool(Arc::new(RwLock::new(HashMap::new())))
    }

    pub async fn register_client(&self, client_id: Uuid, sender: UnboundedSender<Message>) {
        self.0
            .write()
            .await
            .insert(client_id, Client::new(client_id, sender));
    }

    pub async fn send(self, output: Output, client_id: Uuid) -> Result<(), Box<dyn Error>> {
        let client_pool = self.0.read().await;
        let client = client_pool.get(&client_id).ok_or_else(|| {
            IOError::new(
                ErrorKind::Other,
                format!("Failed to read client with ID {}", &client_id),
            )
        })?;
        let message = output.to_message()?;
        client.clone().send(message)?;
        Ok(())
    }
}
