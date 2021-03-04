use serde::{Deserialize, Serialize};
use serde_json::error::Error as SerdeError;
use std::convert::TryFrom;
use std::error::Error;
use uuid::Uuid;
use warp::ws::Message;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", content = "payload", rename_all = "camelCase")]
pub enum Input {
    Register,
    Play(PlayPayload),
    Stop(StopPayload),
}

impl TryFrom<Message> for Input {
    type Error = Box<dyn Error>;

    fn try_from(message: Message) -> Result<Self, <Self as TryFrom<Message>>::Error> {
        let message_string = match message.to_str() {
            Ok(message_string) => message_string,
            Err(()) => return Err("Failed to parse message".into()),
        };
        match serde_json::from_str::<Input>(&message_string) {
            Ok(input) => Ok(input),
            Err(error) => {
                return Err(error.into());
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

impl TryFrom<Output> for Message {
    type Error = SerdeError;

    fn try_from(output: Output) -> Result<Self, Self::Error> {
        match serde_json::to_string(&output) {
            Ok(output_string) => Ok(Message::text(output_string)),
            Err(error) => {
                return Err(error);
            }
        }
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
