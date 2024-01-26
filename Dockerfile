FROM rust:1.75 as builder
WORKDIR /usr/src/app

COPY src src
COPY Cargo.lock Cargo.lock
COPY Cargo.toml Cargo.toml

RUN cargo build --release

##############################################

FROM debian:bookworm-slim

RUN apt-get update
RUN apt-get install -y cron zstd gnupg
RUN rm -rf /var/lib/apt/lists/*

COPY --from=builder /usr/src/app/target/release/rust-backup-generator /app/rust-backup-generator
COPY generate_backup.sh /app/generate_backup.sh
COPY entrypoint.sh /app/entrypoint.sh

WORKDIR /app
CMD ["./entrypoint.sh"]