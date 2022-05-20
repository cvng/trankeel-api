FROM rust:1.61.0

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
