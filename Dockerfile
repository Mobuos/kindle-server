FROM rust:alpine

WORKDIR /usr/src/app
COPY . .

RUN apk update && apk add --no-cache \
    musl-dev \
    imagemagick \
    openssh

# Create directories and set permissions
RUN mkdir -p /usr/src/app/images/tmp && \
    mkdir -p /usr/src/app/converted && \
    chmod -R 777 /usr/src/app/images && \
    chmod -R 777 /usr/src/app/converted

RUN mkdir -p ~/.ssh
RUN chown -R root:root ~/.ssh
# If your kindle is in a different IP address, change it here, this is the default
RUN printf '%s\n' 'host kindle' '   user root' '    hostname 192.168.15.118' >> ~/.ssh/config
RUN cargo build --release -p kindle_server
CMD ["./target/release/kindle_server"]
EXPOSE 8000