use chrono::{DateTime, Utc};
use futures::stream::SplitStream;
use futures::{future, FutureExt, SinkExt, Stream, StreamExt, TryStream, TryStreamExt};
use log::{debug, error, info, log_enabled, Level};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::convert::Infallible;
use std::sync::Arc;
use std::time::Duration;
use std::{error, result};
use tokio::sync::{mpsc, RwLock};
use uuid::Uuid;
use warp::ws::{Message, WebSocket};
use warp::{http::StatusCode, reply::json, Reply};
use warp::{Filter, Rejection};

mod client;
use client::Client;

#[derive(Serialize, Debug)]
pub struct RegisterResponse {
    url: String,
}
type Clients = Arc<RwLock<HashMap<Uuid, Client>>>;

#[tokio::main]
async fn main() {
    env_logger::init();

    let clients: Clients = Arc::new(RwLock::new(HashMap::new()));

    let websocket_route = warp::path("websocket")
        .and(warp::ws())
        .and(with_clients(clients.clone()))
        .map(|ws: warp::ws::Ws, clients: Clients| {
            ws.on_upgrade(|websocket| async {
                info!("Websocket upgrade successful");
                tokio::spawn(websocket_handler(websocket, clients));
            })
        });

    let routes = websocket_route.with(warp::cors().allow_any_origin());

    warp::serve(routes).run(([127, 0, 0, 1], 1234)).await;
}

fn with_clients(clients: Clients) -> impl Filter<Extract = (Clients,), Error = Infallible> + Clone {
    warp::any().map(move || clients.clone())
}

async fn register_client(id: Uuid, clients: Clients) {
    clients.write().await.insert(id, Client { id });
    info!("New client regeistered: {}", id.to_simple().to_string());
}

async fn websocket_handler(websocket: WebSocket, clients: Clients) {
    let (mut sink, mut stream) = websocket.split();

    while let Some(result) = stream.next().await {
        let message = match result {
            Ok(message) => message,
            Err(error) => {
                error!("Failed to receive message: {}", error);
                break;
            }
        };

        match message.to_str() {
            Ok("register") => {
                let uuid = Uuid::new_v4();

                register_client(uuid.clone(), clients.clone()).await;

                let response = Message::text(uuid.to_simple().to_string());

                if let Err(error) = sink.send(response).await {
                    error!("Failed to send message: {}", error);
                    break;
                }
            }
            Ok(message) => {
                error!("Unsupported message: {:?}", message);
                break;
            }
            Err(()) => {
                error!("Failed to parse message");
                break;
            }
        }
    }
}
