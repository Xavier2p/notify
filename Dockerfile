# FROM alpine:latest
# RUN apk add --no-cache rust cargo
# WORKDIR /app
# COPY . .
# RUN cargo build --release
# ENTRYPOINT ["./target/release/notifier"]

FROM alpine:latest AS builder
RUN apk add --no-cache rust cargo
WORKDIR /app
COPY . .
RUN cargo build --release

FROM alpine:latest
RUN apk add --no-cache ca-certificates
WORKDIR /app
COPY --from=builder /app/target/release/ /app
CMD ["./notifier"]