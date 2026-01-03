FROM rust:latest AS builder

WORKDIR /app

COPY Cargo.toml Cargo.lock ./
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release
RUN rm -rf src
 
COPY . .
RUN cargo build --release
 
FROM debian:bookworm-slim

WORKDIR /app
 
RUN apt-get update && apt-get install -y \
    libx11-6 libxcursor1 libxrandr2 libxi6 \
    libgl1 libasound2 ca-certificates \
    && rm -rf /var/lib/apt/lists/*


COPY --from=builder /app/target/release/adventures-in-rust .

CMD ["./adventures-in-rust"]
