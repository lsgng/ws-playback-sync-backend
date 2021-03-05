use chrono::{serde::ts_milliseconds, DateTime, Utc};
use serde::{Deserialize, Serialize};

use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PlayerID(u32);

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Milliseconds(u32);

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
    pub player: PlayerID,
    #[serde(with = "ts_milliseconds")]
    pub timestamp: DateTime<Utc>,
    pub client_id: Option<Uuid>,
}

impl PlayPayload {
    pub fn new(player: PlayerID, timestamp: DateTime<Utc>, client_id: Option<Uuid>) -> Self {
        PlayPayload {
            player,
            timestamp,
            client_id,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StopPayload {
    pub player: PlayerID,
    pub client_id: Option<Uuid>,
}

impl StopPayload {
    pub fn new(player: PlayerID, client_id: Option<Uuid>) -> Self {
        StopPayload { client_id, player }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FastForwardPayload {
    pub player: PlayerID,
    pub target_position: Milliseconds,
    #[serde(with = "ts_milliseconds")]
    pub timestamp: DateTime<Utc>,
    pub client_id: Option<Uuid>,
}

impl FastForwardPayload {
    pub fn new(
        player: PlayerID,
        target_position: Milliseconds,
        timestamp: DateTime<Utc>,
        client_id: Option<Uuid>,
    ) -> Self {
        FastForwardPayload {
            player,
            target_position,
            timestamp,
            client_id,
        }
    }
}
