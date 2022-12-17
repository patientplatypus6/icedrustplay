
# FROM ubuntu:20.04

# RUN apt update
# RUN apt install
# RUN apt install curl -y
# # RUN apt install rustc -y
# RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs -y | sh
# # RUN rustc -V

# WORKDIR /
# COPY . .

# CMD ["./run.sh"]

# ----

# # FROM rust:latest
# FROM rust:1.57.0-alpine

# RUN apk update
# RUN apk add --no-cache bash
# RUN apk add gtk+3.0

# WORKDIR /
# COPY . .
# RUN ls
# RUN cat run.sh
# RUN chmod 777 run.sh
# # RUN cargo update
# # RUN cargo build

# CMD /run.sh | sh


# -----

# FROM rust:latest
# COPY . .
# RUN apk update
# RUN apk add --no-cache bash
# RUN apk add gtk+3.0
# RUN cargo build --release
# CMD ["./run.sh"]

# FROM ubuntu:latest
# WORKDIR /
# COPY . .
# RUN apt-get update 
# RUN apt-get install protobuf-compiler openssl pkg-config libssl-dev rustc cmake cargo -y
# RUN rustc -V
# CMD ["./run.sh"]

FROM rust:1.66
WORKDIR /
COPY . .
RUN apt update && apt install build-essential libgtk-3-dev libsoup2.4-dev -y
RUN cargo install --path .
# RUN apk add alpine-sdk # This one is necessary for linking cc.
COPY ./src/backend/Cargo.toml ./src/backend/Cargo.lock ./
RUN cargo build --release
CMD ["./run.sh"]


# FROM rust:latest as builder
# WORKDIR /
# COPY . .
# RUN cargo install --path .

# FROM debian:buster-slim
# RUN apt-get update && rm -rf /var/lib/apt/lists/*
# # RUN apt-get install -y
# # RUN apt-get update && apt-get install -y extra-runtime-dependencies && rm -rf /var/lib/apt/lists/*
# COPY --from=builder . .
# CMD ["./run.sh"]