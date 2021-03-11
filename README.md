# WebSocket audio playback synchronization - Server

This repository contains the back-end code for a very basic WebSocket based audio playback snychronization prototype. See the [front-end repository](https://github.com/lsgng/ws-playback-sync-frontend) for further information.

A simple WebSocket server built with [Warp](https://github.com/seanmonstar/warp) and [Tokio](https://github.com/tokio-rs/tokio). Broadcasts incoming messages to all connected clients.

## Usage

Build and run:

```
cargo run
```

By default the WebSocket endpoint will be served on port 8000. The optional argument can be used to specify an alternative port:

```
cargo run -- $PORT
```

## Deployment

The repository contains a basic (multistage) Dockerfile that can be used to build and deploy the backend. To make the docker image work with Heroku, the port has to be passed in via the `$PORT` environment variable instead of using the `EXPOSE` instruction. See [this article](https://help.heroku.com/PPBPA231/how-do-i-use-the-port-environment-variable-in-container-based-apps) for further information.
