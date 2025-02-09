FROM rust:1.84-slim

COPY . .

# TODO move build to a separate stage
RUN cargo build

RUN cargo run
