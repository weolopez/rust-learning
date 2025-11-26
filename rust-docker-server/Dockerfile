FROM rust:latest as builder
WORKDIR /app
COPY Cargo.toml Cargo.lock ./
COPY src ./src
COPY static ./static
RUN cargo build --release

FROM debian:bookworm-slim
COPY --from=builder /app/target/release/rust-docker-server /usr/local/bin/rust-docker-server
COPY --from=builder /app/static /static
ENV STATIC_ROOT=/static
EXPOSE 8080
CMD ["/usr/local/bin/rust-docker-server"]
