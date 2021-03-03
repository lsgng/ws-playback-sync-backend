use tokio::sync::mpsc;
use uuid::Uuid;
use warp::ws::Message;

#[derive(Debug, Clone)]
pub struct Client {
    pub id: Uuid,
    pub sender: Option<mpsc::UnboundedSender<Message>>,
}

impl Client {
    pub fn new(id: Uuid) -> Self {
        Client { id, sender: None }
    }
}
