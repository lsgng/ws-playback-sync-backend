use futures::{future::OptionFuture, SinkExt};
use futures_util::stream::SplitSink;
use serde::{Deserialize, Serialize};
use serde_json::error::Error as SerdeError;
use std::error::Error;
use tokio::sync::mpsc;
use uuid::Uuid;
use warp::ws::{Message, WebSocket};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", content = "payload", rename_all = "camelCase")]
pub enum Input {
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
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RegisteredPayload {
    client_id: Uuid,
}

impl RegisteredPayload {
    pub fn new(client_id: Uuid) -> Self {
        RegisteredPayload { client_id }
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlayPayload {
    pub client_id: Option<Uuid>,
    pub deck: u32,
}

impl PlayPayload {
    pub fn new(deck: u32, client_id: Option<Uuid>) -> Self {
        PlayPayload { client_id, deck }
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StopPayload {
    pub client_id: Option<Uuid>,
    pub deck: u32,
}

impl StopPayload {
    pub fn new(deck: u32, client_id: Option<Uuid>) -> Self {
        StopPayload { client_id, deck }
    }
}
