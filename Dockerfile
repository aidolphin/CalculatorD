FROM rust:1.87 as builder
WORKDIR /app

COPY Cargo.toml Cargo.lock ./
COPY backend/Cargo.toml backend/Cargo.toml
COPY frontend/Cargo.toml frontend/Cargo.toml
RUN mkdir -p backend/src frontend/src \
    && printf "fn main() {}\n" > backend/src/main.rs \
    && printf "fn main() {}\n" > frontend/src/lib.rs \
    && cargo build --release -p backend \
    && rm -rf backend/src frontend/src

COPY backend backend
COPY frontend frontend
COPY static static
RUN cargo build --release -p backend

FROM debian:bookworm-slim
WORKDIR /app

RUN apt-get update \
    && apt-get install -y --no-install-recommends ca-certificates \
    && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/backend /app/backend
COPY static /app/static

ENV ROCKET_ADDRESS=0.0.0.0
ENV ROCKET_PORT=8000

EXPOSE 8000

CMD ["/app/backend"]
