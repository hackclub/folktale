# Build stage
FROM rust:1-slim AS builder
WORKDIR /app
COPY . .
RUN cargo build --release

# Runtime stage
FROM debian:trixie-slim
WORKDIR /app
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*
COPY --from=builder /app/target/release/folktale /app/folktale
COPY ui /app/ui
COPY guide /app/guide
ENV PORT=8080
EXPOSE 8080
CMD ["./folktale"]