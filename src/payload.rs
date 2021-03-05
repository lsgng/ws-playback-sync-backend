use chrono::{serde::ts_milliseconds, DateTime, Utc};
use serde::{Deserialize, Serialize};

use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PlayerID(u32);

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RegistrationSuccessPayload {
    client_id: Uuid,
}

impl RegistrationSuccessPayload {
    pub fn new(client_id: Uuid) -> Self {
        RegistrationSuccessPayload { client_id }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlayPayload {
    pub client_id: Option<Uuid>,
    pub player: PlayerID,
    #[serde(with = "ts_milliseconds")]
    pub timestamp: DateTime<Utc>,
}

impl PlayPayload {
    pub fn new(player: PlayerID, client_id: Option<Uuid>, timestamp: DateTime<Utc>) -> Self {
        PlayPayload {
            client_id,
            player,
            timestamp,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StopPayload {
    pub client_id: Option<Uuid>,
    pub player: PlayerID,
}

impl StopPayload {
    pub fn new(player: PlayerID, client_id: Option<Uuid>) -> Self {
        StopPayload { client_id, player }
    }
}
