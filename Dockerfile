FROM rust:1.65.0
WORKDIR /app
COPY . .
RUN cargo build --release
ENV PORT 8000
CMD ["/app/target/release/rusty_bike"]