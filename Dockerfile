FROM ekidd/rust-musl-builder:stable as builder

RUN cargo new --bin ws-sync-backend
WORKDIR ./ws-sync-backend
COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml
RUN cargo build --release
RUN rm src/*.rs

ADD . ./

RUN rm ./target/x86_64-unknown-linux-musl/release/deps/ws_sync_backend*
RUN cargo build --release


FROM alpine:latest

ARG APP=/usr/src/app

ENV TZ=Etc/UTC \
    APP_USER=appuser

RUN addgroup -S $APP_USER \
    && adduser -S -g $APP_USER $APP_USER

RUN apk update \
    && apk add --no-cache ca-certificates tzdata \
    && rm -rf /var/cache/apk/*

COPY --from=builder /home/rust/src/ws-sync-backend/target/x86_64-unknown-linux-musl/release/ws-sync-backend ${APP}/ws-sync-backend

RUN chown -R $APP_USER:$APP_USER ${APP}

USER $APP_USER
WORKDIR ${APP}

CMD ["./ws-sync-backend ${PORT}"]