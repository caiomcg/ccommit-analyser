FROM rust:slim

RUN cargo install cargo-watch
RUN mkdir -p /app
WORKDIR /app

CMD ["cargo", "watch", "-x", "test"]
