FROM rust:1.65.0

WORKDIR /usr/src/app

COPY . .

RUN cargo install --path /usr/src/app

ENV PORT 8000

CMD ["rusty_bike"]