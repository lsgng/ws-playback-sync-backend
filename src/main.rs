use chrono::{DateTime, Utc};
use futures::stream::SplitStream;
use futures::{future, FutureExt, SinkExt, Stream, StreamExt, TryStream, TryStreamExt};
use log::{error, info};
use std::collections::HashMap;
use std::convert::Infallible;
use std::sync::Arc;
use tokio::sync::{mpsc, RwLock};
use uuid::Uuid;
use warp::ws::{Message, WebSocket};
use warp::Filter;

mod client;
use client::Client;
mod protocol;
use protocol::{Input, Output, PlayPayload, RegisteredPayload, StopPayload};

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
    info!("New client registered: {}", id.to_simple().to_string());
}

async fn websocket_handler(websocket: WebSocket, clients: Clients) {
    let (mut sink, mut stream) = websocket.split();

    while let Some(input) = stream.next().await {
        let message = match input {
            Ok(message) => message,
            Err(error) => {
                error!("Failed to read input message: {}", error);
                break;
            }
        };

        let input = match Input::from_message(message) {
            Ok(input) => input,
            Err(_) => {
                error!("Failed to parse input message");
                break;
            }
        };

        match input {
            Input::Register => {
                let uuid = Uuid::new_v4();
                register_client(uuid.clone(), clients.clone()).await;
                let output = Output::Registered(RegisteredPayload::new(uuid));
                if let Err(error) = output.send(&mut sink).await {
                    error!("Failed to send output message: {}", error);
                    break;
                }
            }

            Input::Play(payload) => {
                let output = Output::Play(PlayPayload::new(payload.deck));
                if let Err(error) = output.send(&mut sink).await {
                    error!("Failed to send output message: {}", error);
                    break;
                }
            }

            Input::Stop(payload) => {
                let output = Output::Stop(StopPayload::new(payload.deck));
                if let Err(error) = output.send(&mut sink).await {
                    error!("Failed to send output message: {}", error);
                    break;
                }
            }
        }
    }
}
