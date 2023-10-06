ARG BASEIMAGE=debian:bookworm-slim

FROM rust:1-bookworm AS chef
RUN cargo install cargo-chef

WORKDIR app

FROM chef as planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS build
COPY --from=planner /app/recipe.json recipe.json
# Build dependencies - this is the caching Docker layer!
RUN cargo chef cook --release --recipe-path recipe.json
# Build application
COPY . .
RUN cargo build --release

# We do not need the Rust toolchain to run the binary!
FROM ${BASEIMAGE} as runtime
WORKDIR app
COPY --from=build /app/target/release/largejson /usr/local/bin/

EXPOSE 8080
