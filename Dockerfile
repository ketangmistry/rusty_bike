FROM rust:1.65.0 AS builder
WORKDIR /build
COPY . .
RUN cargo build --release

FROM alpine:latest  
RUN apk --no-cache add ca-certificates
WORKDIR /app
COPY --from=builder /build/target/release/rusty_bike ./
COPY --from=builder /build/src/feeds/bikes.yaml ./
RUN pwd && ls -al
ENV PORT 8000
ENTRYPOINT ["./rusty_bike"]