FROM rust:latest

WORKDIR /usr/src/app
COPY . .
RUN mkdir -p ~/.ssh
RUN chown -R root:root ~/.ssh
# If your kindle is in a different IP address, change it here, this is the default
# TODO: Use an env variable?
RUN echo "host kindle\n user root\n hostname 192.168.15.118" >> ~/.ssh/config 
RUN cargo build --release
CMD ["./target/release/kindle-server"]
EXPOSE 8000