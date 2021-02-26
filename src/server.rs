use std::sync::Arc;
use std::{error, result};

use futures::{future, Stream, StreamExt, TryStream, TryStreamExt};

use log::{error, info};
use tokio::sync::mpsc;
use tokio::sync::mpsc::UnboundedSender;
use tokio::time::Duration;
use warp::ws::{Message, WebSocket};
use warp::Filter;

pub struct Server {
    port: u16,
}

impl Server {
    pub fn new(port: u16) -> Self {
        Server { port }
    }

    pub async fn run(&self) {
        info!("Starting server");

        let websocket_route =
            warp::path("websocket")
                .and(warp::ws())
                .map(move |ws: warp::ws::Ws| {
                    ws.on_upgrade(move |websocket| async move {
                        tokio::spawn(Self::process_client(websocket));
                    })
                });

        let shutdown = async {
            tokio::signal::ctrl_c()
                .await
                .expect("failed to install CTRL+C signal handler");
        };

        let (_, serving) = warp::serve(websocket_route)
            .bind_with_graceful_shutdown(([127, 0, 0, 1], self.port), shutdown);

        serving.await;
    }

    pub async fn process_client(ws: WebSocket) {
        let (client_ws_sender, mut client_ws_rcv) = ws.split();
        info!("processing clinet");
        while let Some(result) = client_ws_rcv.next().await {
            info!("WHILE");
            let msg = match result {
                Ok(msg) => print!("received"),
                Err(e) => {
                    eprintln!("error receiving ws message for id");
                    break;
                }
            };
            info!("DONE")
        }
    }

    // async fn process_client(
    //     websocket: WebSocket,
    // ) -> impl Stream<Item = Result<Message, warp::Error>> {
    //     info!("Processing client");
    //     let (_, stream) = websocket.split();
    //     stream
    //         .take_while(|message| {
    //             info!("Taking while...");
    //             future::ready(if let Ok(message) = message {
    //                 message.is_text()
    //             } else {
    //                 false
    //             })
    //         })
    //         .map(move |message| match message {
    //             Err(err) => Err(err),
    //             Ok(message) => Ok(message),
    //         })
    // }
}
