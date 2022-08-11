FROM --platform=$BUILDPLATFORM rust:1.63 as builder

RUN apt update -y && apt install python3-pip -y && pip3 install cargo-zigbuild

RUN cargo new --bin app
WORKDIR /app

COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml
ARG TARGETPLATFORM
RUN case "$TARGETPLATFORM" in \
  "linux/arm64") echo aarch64-unknown-linux-musl > /rust_target.txt ;; \
  "linux/amd64") echo x86_64-unknown-linux-musl > /rust_target.txt ;; \
  *) exit 1 ;; \
esac
RUN rustup target add $(cat /rust_target.txt)
RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/app/target \
    cargo zigbuild --release --target $(cat /rust_target.txt)
RUN rm src/*.rs

COPY ./src ./src
RUN touch ./src/main.rs
RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/app/target \
    cargo zigbuild --release --target $(cat /rust_target.txt) && \
    cp target/$(cat /rust_target.txt)/release/minecraft-command-bot /usr/local/bin/minecraft-command-bot

FROM alpine
COPY --from=builder /usr/local/bin/minecraft-command-bot .
CMD ["./minecraft-command-bot"]
