# syntax=docker/dockerfile:1
FROM rust:1.63.0

ENV ROCKET_ADDRESS=0.0.0.0
ENV ROCKET_PORT=8000

WORKDIR /app
COPY Cargo.toml ./Cargo.toml
COPY src ./src

RUN rustup default nightly
RUN cargo build

EXPOSE 8000

CMD ["cargo", "run", "--bin", "server"]
