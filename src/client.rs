use tokio::sync::mpsc::{error::SendError, UnboundedSender};
use uuid::Uuid;
use warp::ws::Message;

#[derive(Debug, Clone)]
pub struct Client {
    pub id: Uuid,
    pub sender: UnboundedSender<Message>,
}

impl Client {
    pub fn new(id: Uuid, sender: UnboundedSender<Message>) -> Self {
        Client { id, sender }
    }

    pub fn send(self, message: Message) -> Result<(), SendError<Message>> {
        self.sender.send(message)?;
        Ok(())
    }
}
