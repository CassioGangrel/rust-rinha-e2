FROM rust:1.76-slim as build

RUN USER=root cargo new --bin rust-rinha-e2
WORKDIR /rust-rinha-e2

COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml
COPY ./src ./src
COPY ./migrations ./migrations

RUN cargo build --release

FROM rust:1.76-slim

COPY --from=build /rust-rinha-e2/target/release/rust-rinha-e2 .

EXPOSE 3000

CMD ["./rust-rinha-e2"]