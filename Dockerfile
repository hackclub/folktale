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
COPY templates /app/templates
ENV PORT=2000
EXPOSE 2000
CMD ["./folktale"]