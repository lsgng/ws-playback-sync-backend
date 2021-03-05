use crate::payload::{
    CrossFadePayload, FastForwardPayload, PlayPayload, RegistrationSuccessPayload, StopPayload,
};
use serde::{Deserialize, Serialize};
use serde_json::error::Error as SerdeError;
use std::convert::TryFrom;
use warp::ws::Message;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "payload", rename_all = "camelCase")]
pub enum OutgoingMessage {
    RegistrationSuccess(RegistrationSuccessPayload),
    Play(PlayPayload),
    Stop(StopPayload),
    FastForward(FastForwardPayload),
    CrossFade(CrossFadePayload),
}

impl TryFrom<OutgoingMessage> for Message {
    type Error = SerdeError;

    fn try_from(outgoing_message: OutgoingMessage) -> Result<Self, Self::Error> {
        match serde_json::to_string(&outgoing_message) {
            Ok(output_string) => Ok(Message::text(output_string)),
            Err(error) => {
                return Err(error);
            }
        }
    }
}
