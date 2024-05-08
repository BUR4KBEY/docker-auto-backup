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

# from https://docs.docker.com/engine/install/debian/#install-using-the-repository
RUN apt-get update
RUN apt-get install -y ca-certificates curl
RUN install -m 0755 -d /etc/apt/keyrings
RUN curl -fsSL https://download.docker.com/linux/debian/gpg -o /etc/apt/keyrings/docker.asc
RUN chmod a+r /etc/apt/keyrings/docker.asc
RUN echo \
  "deb [arch=$(dpkg --print-architecture) signed-by=/etc/apt/keyrings/docker.asc] https://download.docker.com/linux/debian \
  $(. /etc/os-release && echo "$VERSION_CODENAME") stable" | \
  tee /etc/apt/sources.list.d/docker.list > /dev/null
RUN apt-get update
RUN apt-get install -y docker-ce-cli

RUN rm -rf /var/lib/apt/lists/*

COPY --from=builder /usr/src/app/target/release/docker-auto-backup /app/docker-auto-backup
COPY generate_backup.sh /app/generate_backup.sh
COPY entrypoint.sh /app/entrypoint.sh

WORKDIR /app
CMD ["./entrypoint.sh"]