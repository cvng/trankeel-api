FROM rust:1.58.1

WORKDIR /api

COPY . .

RUN cargo build \
  --bin=trankeel-api \
  --release \
  --locked \
  --no-default-features \
  --features=release

CMD ROCKET_ADDRESS=0.0.0.0 ROCKET_KEEP_ALIVE=0 ROCKET_PORT=$PORT cargo run \
  --bin=trankeel-api \
  --release \
  --locked \
  --no-default-features \
  --features=release
