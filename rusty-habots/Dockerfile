FROM rust:latest AS builder

RUN rustup override set nightly
RUN rustup target add x86_64-unknown-linux-musl
RUN apt update -y && apt install -y musl-tools musl-dev
RUN update-ca-certificates

ENV USER=habot
ENV UID=10001

RUN adduser \
    --disabled-password \
    --gecos "" \
    --home "/nonexistent" \
    --shell "/sbin/nologin" \
    --no-create-home \
    --uid "${UID}" \
    "${USER}"

WORKDIR /habot

COPY ./ .

RUN cargo build --target x86_64-unknown-linux-musl --release

FROM alpine

COPY --from=builder /etc/passwd /etc/passwd
COPY --from=builder /etc/group /etc/group

WORKDIR /habot

COPY --from=builder /habot/target/x86_64-unknown-linux-musl/release/rusty-habots ./

USER habot:habot

ENTRYPOINT /habot/rusty-habots
