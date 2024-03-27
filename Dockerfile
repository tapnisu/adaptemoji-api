FROM rust:1.77 as builder

WORKDIR /usr/src/adaptemoji-api
COPY . .
RUN cargo build --release

FROM debian:bookworm-slim

# RUN apt-get update && apt-get install -y extra-runtime-dependencies && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/src/adaptemoji-api/target/release/adaptemoji-api /usr/local/bin/adaptemoji-api

CMD ["adaptemoji-api"]

EXPOSE 3000
