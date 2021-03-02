use futures::SinkExt;
use futures_util::stream::SplitSink;
use serde::{Deserialize, Serialize};
use serde_json::error::Error as SerdeError;
use std::error::Error;
use uuid::Uuid;
use warp::ws::{Message, WebSocket};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", content = "payload", rename_all = "camelCase")]
pub enum Input {
    #[serde(rename = "register")]
    Register,
    Play(PlayPayload),
    Stop(StopPayload),
}

impl Input {
    pub fn from_message(message: Message) -> Result<Self, ()> {
        let message_string = message.to_str()?;
        match serde_json::from_str::<Input>(&message_string) {
            Ok(input) => Ok(input),
            Err(err) => {
                eprintln!("{}", err);
                return Err(());
            }
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type", content = "payload", rename_all = "camelCase")]
pub enum Output {
    #[serde(rename = "registered")]
    Registered(RegisteredPayload),
    Play(PlayPayload),
    Stop(StopPayload),
}

impl Output {
    pub fn to_message(self) -> Result<Message, SerdeError> {
        let serialized = serde_json::to_string(&self)?;
        Ok(Message::text(serialized))
    }

    pub async fn send(
        self,
        sink: &mut SplitSink<WebSocket, Message>,
    ) -> Result<(), Box<dyn Error>> {
        let output_message = self.to_message()?;
        sink.send(output_message).await?;
        Ok(())
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RegisteredPayload {
    user_id: Uuid,
}

impl RegisteredPayload {
    pub fn new(user_id: Uuid) -> Self {
        RegisteredPayload { user_id }
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlayPayload {
    pub deck: u32,
}

impl PlayPayload {
    pub fn new(deck: u32) -> Self {
        PlayPayload { deck }
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StopPayload {
    pub deck: u32,
}

impl StopPayload {
    pub fn new(deck: u32) -> Self {
        StopPayload { deck }
    }
}
