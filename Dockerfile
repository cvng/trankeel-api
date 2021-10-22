FROM rust:1.56.0

WORKDIR /api

COPY . .

RUN cargo build --release --locked --bin trankeel-api

CMD ROCKET_ADDRESS=0.0.0.0 ROCKET_KEEP_ALIVE=0 ROCKET_PORT=$PORT cargo run --release --locked --bin trankeel-api
