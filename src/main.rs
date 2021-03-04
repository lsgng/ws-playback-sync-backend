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
mod client_pool;
mod protocol;
use client::Client;
use client_pool::ClientPool;
use protocol::{Input, Output, PlayPayload, RegisteredPayload, StopPayload};

#[tokio::main]
async fn main() {
    env_logger::init();

    let client_pool = ClientPool::new();

    let websocket_route = warp::path("websocket")
        .and(warp::ws())
        .and(with_clients(client_pool.clone()))
        .map(|ws: warp::ws::Ws, client_pool: ClientPool| {
            ws.on_upgrade(|websocket| async {
                tokio::spawn(websocket_handler(websocket, client_pool));
            })
        });

    let routes = websocket_route.with(warp::cors().allow_any_origin());

    warp::serve(routes).run(([127, 0, 0, 1], 1234)).await;
}

fn with_clients(
    client_pool: ClientPool,
) -> impl Filter<Extract = (ClientPool,), Error = Infallible> + Clone {
    warp::any().map(move || client_pool.clone())
}

async fn websocket_handler(websocket: WebSocket, client_pool: ClientPool) {
    let (mut sink, mut stream) = websocket.split();
    let (sender, mut receiver) = mpsc::unbounded_channel();

    let forwarding = tokio::spawn(async move {
        while let Some(message) = receiver.recv().await {
            if let Err(error) = sink.send(message).await {
                error!("Failed to forward message: {}", error);
            };
        }
    });

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
                let client_id = Uuid::new_v4();
                client_pool
                    .register_client(client_id.clone(), sender.clone())
                    .await;

                let output = Output::Registered(RegisteredPayload::new(client_id));
                if let Err(error) = client_pool.clone().send(output, client_id).await {
                    error!("Failed to send output message: {}", error);
                    break;
                }
            }

            Input::Play(payload) => {
                let client_id = match payload.client_id {
                    Some(client_id) => client_id,
                    None => {
                        error!("Failed to read client ID");
                        break;
                    }
                };
                let output = Output::Play(PlayPayload::new(payload.deck, None));
                if let Err(error) = client_pool.clone().send(output, client_id).await {
                    error!("Failed to send output message: {}", error);
                    break;
                }
            }

            Input::Stop(payload) => {
                let client_id = match payload.client_id {
                    Some(client_id) => client_id,
                    None => {
                        error!("Failed to read client ID");
                        break;
                    }
                };
                let output = Output::Stop(StopPayload::new(payload.deck, None));
                if let Err(error) = client_pool.clone().send(output, client_id).await {
                    error!("Failed to send output message: {}", error);
                    break;
                }
            }
        }
    }

    // TODO: Use match instead of unwrapping
    forwarding.await.unwrap();
}
