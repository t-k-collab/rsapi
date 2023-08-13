FROM rust:1.71 as builder
WORKDIR /usr/src/myapp
# install if cargo-watch is necessary.
# RUN cargo install cargo-watch
COPY . .
RUN cargo build --release

FROM debian:bullseye-slim
RUN apt-get update -y && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/src/myapp/target/release/rsapi /usr/local/.
WORKDIR /root
COPY .env .
EXPOSE 8000
CMD ["/usr/local/rsapi"]