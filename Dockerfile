ARG DEBIAN_VERSION=buster-slim
ARG RUST_VERSION=1.50

FROM rust:${RUST_VERSION} AS planner
WORKDIR /api
RUN cargo install cargo-chef
COPY . .
# compute a lock-like file for our api
RUN cargo chef prepare  --recipe-path recipe.json

FROM rust:${RUST_VERSION} AS cacher
WORKDIR /api
RUN cargo install cargo-chef
COPY --from=planner /api/recipe.json recipe.json
# build our api dependencies, not our api
RUN cargo chef cook --release --recipe-path recipe.json

FROM rust:${RUST_VERSION} AS builder
WORKDIR /api
# copy over the cached dependencies
COPY --from=cacher /api/target target
COPY --from=cacher /usr/local/cargo /usr/local/cargo
COPY . .
# build our api with the cached dependencies
RUN cargo build --release --bin tide-demo

FROM debian:${DEBIAN_VERSION} AS runtime
WORKDIR /api
RUN apt-get update -y \
    && apt-get install -y --no-install-recommends openssl libcurl4 ca-certificates\
    # clean up
    && apt-get autoremove -y && apt-get clean -y && rm -rf /var/lib/apt/lists/*

RUN update-ca-certificates

COPY --from=builder /api/target/release/tide-demo tide-demo

EXPOSE 8080

ENTRYPOINT ["./tide-demo"]
