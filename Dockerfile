FROM rust:latest

RUN apt-get update && apt-get install -y \
    libc6 \
    libssl-dev \
    pkg-config \
    curl

COPY . /app
WORKDIR /app

RUN cargo build --release

ENTRYPOINT ["./target/release/Notes"]