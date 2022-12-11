FROM rust as planner
WORKDIR app
COPY . .
RUN cargo install cargo-chef --locked \
    && cargo chef prepare --recipe-path recipe.json

FROM rust as builder
WORKDIR app
COPY --from=planner /app/recipe.json .
RUN cargo install cargo-chef --locked \
    && cargo chef cook --release --recipe-path recipe.json
COPY . .
RUN cargo build --release --bin rusty_bike

FROM debian:bullseye-slim as runtime
WORKDIR app
COPY --from=builder /app/target/release/rusty_bike .
COPY --from=builder /app/src/feeds/bikes.yaml .
ENTRYPOINT ["/app/rusty_bike"]