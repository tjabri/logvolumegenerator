FROM rust:slim-bookworm
WORKDIR /app
COPY  target/release/logvolumegenerator /app
USER nobody
ENTRYPOINT ["/app/logvolumegenerator"]