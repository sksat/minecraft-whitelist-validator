FROM rust:latest as builder
WORKDIR build
ADD . .
RUN cargo build --release

FROM gcr.io/distroless/cc
WORKDIR app
COPY --from=builder /build/target/release/minecraft-whitelist-validator /app
COPY entrypoint.sh /entrypoint.sh
ENTRYPOINT ["/entrypoint.sh"]
