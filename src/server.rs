use crate::client_pool::ClientPool;
use crate::outgoing_message::OutgoingMessage;
use crate::payload::{PlayPayload, RegistrationSuccessPayload, StopPayload};
use crate::{incoming_message::IncomingMessage, payload::FastForwardPayload};
use futures::{SinkExt, StreamExt};
use log::error;
use std::convert::Infallible;
use std::convert::TryFrom;
use std::net::SocketAddr;
use tokio::sync::{mpsc, mpsc::UnboundedSender};
use uuid::Uuid;
use warp::ws::{Message, WebSocket};
use warp::Filter;

pub struct Server {
    address: SocketAddr,
}

impl Server {
    pub fn new(address: SocketAddr) -> Self {
        Server { address }
    }

    pub async fn run(&self) {
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

        warp::serve(routes).run(self.address).await;
    }
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

    while let Some(stream_item) = stream.next().await {
        let message = match stream_item {
            Ok(message) => message,
            Err(error) => {
                error!("Failed to read input message: {}", error);
                break;
            }
        };

        if !message.is_text() {
            break;
        }

        let incoming_message = match IncomingMessage::try_from(message) {
            Ok(incoming_message) => incoming_message,
            Err(_) => {
                error!("Failed to parse input message");
                break;
            }
        };
        handle_incoming_message(incoming_message, &client_pool, &sender).await;
    }

    if let Err(error) = forwarding.await {
        error!("Failed to forward messages: {}", error);
    };
}

pub async fn handle_incoming_message(
    incoming_message: IncomingMessage,
    client_pool: &ClientPool,
    sender: &UnboundedSender<Message>,
) {
    match incoming_message {
        IncomingMessage::Registration => {
            let client_id = Uuid::new_v4();
            client_pool
                .register_client(client_id.clone(), sender.clone())
                .await;

            let output =
                OutgoingMessage::RegistrationSuccess(RegistrationSuccessPayload::new(client_id));
            if let Err(error) = client_pool.clone().send_to(output, &client_id).await {
                error!("Failed to send output message: {}", error);
            }
        }

        IncomingMessage::Play(payload) => {
            let client_id = match payload.client_id {
                Some(client_id) => client_id,
                None => {
                    error!("Failed to read client ID");
                    return;
                }
            };
            let outgoing_message =
                OutgoingMessage::Play(PlayPayload::new(payload.player, payload.timestamp, None));
            if let Err(error) = client_pool
                .clone()
                .broadcast_ignore(outgoing_message, &client_id)
                .await
            {
                error!("Failed to send output message: {}", error);
            }
        }

        IncomingMessage::Stop(payload) => {
            let client_id = match payload.client_id {
                Some(client_id) => client_id,
                None => {
                    error!("Failed to read client ID");
                    return;
                }
            };
            let output = OutgoingMessage::Stop(StopPayload::new(payload.player, None));
            if let Err(error) = client_pool
                .clone()
                .broadcast_ignore(output, &client_id)
                .await
            {
                error!("Failed to send output message: {}", error);
            }
        }

        IncomingMessage::FastForward(payload) => {
            let client_id = match payload.client_id {
                Some(client_id) => client_id,
                None => {
                    error!("Failed to read client ID");
                    return;
                }
            };
            let output = OutgoingMessage::FastForward(FastForwardPayload::new(
                payload.player,
                payload.target_position,
                payload.timestamp,
                None,
            ));
            if let Err(error) = client_pool
                .clone()
                .broadcast_ignore(output, &client_id)
                .await
            {
                error!("Failed to send output message: {}", error);
            }
        }
    }
}
