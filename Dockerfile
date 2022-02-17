FROM rust:1.58-bullseye AS chef

RUN cargo install cargo-chef; \
    rustup component add rustfmt;
WORKDIR app

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json
COPY . .
RUN cargo build --release --bin chainsaw-demo-grpc

FROM debian:bullseye-slim
WORKDIR app
COPY --from=builder --chown=root:root /app/target/release/chainsaw-demo-grpc /usr/local/bin/

EXPOSE 5001
CMD ["chainsaw-demo-grpc"]
