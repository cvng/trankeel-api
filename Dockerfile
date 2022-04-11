FROM rust:1.60.0

WORKDIR /api

COPY . .

RUN cargo build \
  --bin=trankeel-api \
  --release \
  --locked \
  --no-default-features \
  --features=release

CMD cargo run \
  --bin=trankeel-api \
  --release \
  --locked \
  --no-default-features \
  --features=release
