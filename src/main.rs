use chrono::{DateTime, Utc};
use futures::stream::SplitStream;
use futures::{future, FutureExt, SinkExt, Stream, StreamExt, TryStream, TryStreamExt};
use log::{debug, error, info, log_enabled, Level};
use std::time::Duration;
use std::{error, result};
use warp::ws::{Message, WebSocket};
use warp::Filter;

#[tokio::main]
async fn main() {
    env_logger::init();

    let routes = warp::path("websocket")
        .and(warp::ws())
        .map(|ws: warp::ws::Ws| {
            ws.on_upgrade(|websocket| async move {
                info!("Websocket upgrade successful, spawning new task");
                tokio::spawn(process_websocket(websocket));
            })
        });

    warp::serve(routes).run(([127, 0, 0, 1], 1234)).await;
}

async fn process_websocket(websocket: WebSocket) {
    info!("Processing data");
    let (mut sink, mut stream) = websocket.split();

    while let Some(result) = stream.next().await {
        let msg = match result {
            Ok(msg) => msg,
            Err(_e) => {
                eprintln!("error receiving message");
                break;
            }
        };
        info!("sending msg");
        if let Err(_e) = sink.send(msg).await {
            eprintln!("error sending message");
        }
    }
}
