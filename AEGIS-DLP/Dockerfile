FROM rust:1.88-bookworm AS rust-build
WORKDIR /src
COPY core/rust ./core/rust
RUN cargo test --manifest-path core/rust/Cargo.toml

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y --no-install-recommends python3 golang g++ cmake ca-certificates && rm -rf /var/lib/apt/lists/*
WORKDIR /aegis
COPY . .
CMD ["/bin/sh"]
