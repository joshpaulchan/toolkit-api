FROM rust:1.48-alpine as builder

WORKDIR /opt/app

COPY . .
RUN cargo build --release

ENTRYPOINT ["./target/release/toolkit-api"]

FROM scratch

WORKDIR /opt/app
COPY --from=builder /opt/app/target/release/toolkit-api .
ENTRYPOINT ["./toolkit-api"]