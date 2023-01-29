FROM rust:1.67
WORKDIR /app
COPY . .
RUN cargo build --release

FROM ubuntu:22.04
WORKDIR /app
COPY --from=0 /app/target/release/idgen-rs .
CMD /app/idgen-rs
