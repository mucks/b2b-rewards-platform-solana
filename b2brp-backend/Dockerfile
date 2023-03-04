FROM rust:1.65-slim AS builder

RUN apt-get update && apt-get upgrade -y
RUN apt-get install -y curl libssl-dev libpq-dev pkg-config

WORKDIR /app

#to enable cargo build before adding sources
RUN mkdir src
RUN touch src/main.rs
RUN echo "fn main() {}" > src/main.rs

COPY Cargo.toml .
RUN cargo build --release

RUN rm -r src
RUN rm ./target/release/deps/otalp_backend*
COPY src src
RUN cargo build --release



FROM node:lts-slim

WORKDIR /app

RUN apt-get update && apt-get install -y curl libssl-dev pkg-config libpq-dev
RUN rm -rf /var/lib/apt/lists/*

COPY package.json package.json
COPY prisma prisma
RUN npm install

COPY --from=builder /app/target/release/otalp_backend /app/otalp_backend
COPY entrypoint.sh entrypoint.sh
ENTRYPOINT ["./entrypoint.sh"]