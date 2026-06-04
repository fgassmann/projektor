FROM rust:alpine as chef
WORKDIR /app
RUN apk add --no-cache musl-dev gcc && cargo install cargo-chef

### Step 1 - Plan
# This is just a caching optimization based on
# https://nico.engineer/blog/optimizing-rust-build
FROM chef AS planner
COPY Cargo.toml Cargo.lock .
RUN mkdir -p src && echo "fn main() {}" > src/main.rs
RUN cargo chef prepare --recipe-path recipe.json

### Step 2 - Build
FROM chef AS builder
COPY Cargo.toml Cargo.lock .
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json

COPY src ./src
RUN cargo build --release --target=x86_64-unknown-linux-musl

### Copy the final binary out of the container
FROM scratch as artifact
COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/projektor .
