# syntax=docker/dockerfile:1.4
FROM rust:buster AS base

ENV USER=root

WORKDIR /code
RUN cargo init
COPY Cargo.toml /code/Cargo.toml
RUN cargo fetch
COPY . /code

# FROM base AS developmen
# EXPOSE 8000
# CMD [ "cargo", "run", "--offline" ]

FROM base AS builder
RUN cargo build --release --offline

FROM debian:buster-slim
EXPOSE 8000
COPY --from=builder /code/target/release/remed-lib /remed-lib
CMD [ "/remed-lib" ]