FROM --platform=$BUILDPLATFORM rust:1.60 as builder

RUN apt update -y && apt install llvm clang -y

RUN cargo new --bin app
WORKDIR /app

ENV CC_aarch64_unknown_linux_musl=clang
ENV AR_aarch64_unknown_linux_musl=llvm-ar
ENV CARGO_TARGET_AARCH64_UNKNOWN_LINUX_MUSL_RUSTFLAGS="-Clink-self-contained=yes -Clinker=rust-lld"

ENV CC_x86_64_unknown_linux_musl=clang
ENV AR_x86_64_unknown_linux_musl=llvm-ar
ENV CARGO_TARGET_X86_64_UNKNOWN_LINUX_MUSL_RUSTFLAGS="-Clink-self-contained=yes -Clinker=rust-lld"

COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml
ARG TARGETPLATFORM
RUN case "$TARGETPLATFORM" in \
  "linux/arm64") echo aarch64-unknown-linux-musl > /rust_target.txt ;; \
  "linux/amd64") echo x86_64-unknown-linux-musl > /rust_target.txt ;; \
  *) exit 1 ;; \
esac
RUN rustup target add $(cat /rust_target.txt)
RUN cargo build --release --target $(cat /rust_target.txt)
RUN rm src/*.rs

COPY ./src ./src
RUN cargo build --release --target $(cat /rust_target.txt)
RUN cargo install --locked --path . --target $(cat /rust_target.txt)

FROM alpine
COPY --from=builder /usr/local/cargo/bin/minecraft-command-bot .
CMD ["./minecraft-command-bot"]
