FROM rust:slim-bookworm
WORKDIR /app
ADD --chmod 755 ./target/release/logvolumegenerator /app/
USER nobody
ENTRYPOINT ["/app/logvolumegenerator"]
