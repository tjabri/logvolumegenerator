FROM rust:slim-bookworm AS builder
WORKDIR /app
COPY Cargo.toml Cargo.lock ./
COPY src ./src
COPY tests ./
RUN cargo build --release --locked

# Final stage
FROM rust:slim-bookworm
WORKDIR /app
COPY --from=builder /app/target/release/logvolumegenerator /app
USER nobody
ENTRYPOINT ["/app/logvolumegenerator"]