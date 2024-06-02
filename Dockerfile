FROM rust:latest

WORKDIR /usr/src/app
COPY . .
RUN cargo build --release
CMD cargo run
EXPOSE 8000