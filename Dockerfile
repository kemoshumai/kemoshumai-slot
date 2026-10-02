FROM rust:1-bookworm AS build
WORKDIR /app
COPY Cargo.toml Cargo.lock ./
COPY src ./src
RUN cargo build --release --locked
FROM debian:bookworm-slim
COPY --from=build /app/target/release/kemoshumai-slot /usr/local/bin/kemoshumai-slot
USER 10001:10001
ENTRYPOINT ["kemoshumai-slot"]
