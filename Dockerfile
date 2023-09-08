FROM messense/rust-musl-cross:aarch64-musl as builder
WORKDIR /app
COPY . .
RUN cargo build --release --target aarch64-unknown-linux-musl

FROM scratch
COPY --from=builder /app/target/aarch64-unknown-linux-musl/release/notifier /notifier
EXPOSE 8000
ENTRYPOINT ["/notifier"]