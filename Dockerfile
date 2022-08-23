FROM rust:1.63-bullseye AS chef

RUN cargo install cargo-chef; \
    rustup component add rustfmt;
WORKDIR app

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder

# Under the hood for gRPC dependencies, the Prost package will build `protoc`.
# So `cmake` is required for this.
RUN apt-get update
RUN apt-get install cmake -y

COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json
COPY . .
RUN cargo build --release --bin chainsaw-demo-grpc
RUN cargo build --release --bin chainsaw-demo-http

FROM debian:bullseye-slim AS base-runtime
WORKDIR app
COPY chainsaw.toml chainsaw.toml

FROM base-runtime AS chainsaw-demo-http
COPY --from=builder --chown=root:root /app/target/release/chainsaw-demo-http /usr/local/bin/

EXPOSE 3000
CMD ["chainsaw-demo-http"]

FROM base-runtime AS chainsaw-demo-grpc
COPY --from=builder --chown=root:root /app/target/release/chainsaw-demo-grpc /usr/local/bin/

EXPOSE 5001
CMD ["chainsaw-demo-grpc"]
