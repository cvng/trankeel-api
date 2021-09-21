FROM rust:1.55.0

WORKDIR /api

COPY . .

RUN cargo build --release --locked --bin piteo-api

CMD ROCKET_ADDRESS=0.0.0.0 ROCKET_KEEP_ALIVE=0 ROCKET_PORT=$PORT cargo run --release --locked --bin piteo-api
