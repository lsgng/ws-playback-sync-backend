use crate::payload::{PlayPayload, StopPayload};
use serde::{Deserialize, Serialize};
use std::convert::TryFrom;
use std::error::Error;
use warp::ws::Message;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", content = "payload", rename_all = "camelCase")]
pub enum IncomingMessage {
    Registration,
    Play(PlayPayload),
    Stop(StopPayload),
}

impl TryFrom<Message> for IncomingMessage {
    type Error = Box<dyn Error>;

    fn try_from(message: Message) -> Result<Self, <Self as TryFrom<Message>>::Error> {
        let message_string = match message.to_str() {
            Ok(message_string) => message_string,
            Err(()) => return Err("Failed to parse message".into()),
        };
        match serde_json::from_str::<IncomingMessage>(&message_string) {
            Ok(input) => Ok(input),
            Err(error) => {
                return Err(error.into());
            }
        }
    }
}
