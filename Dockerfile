FROM rust as builder
WORKDIR /usr/src/rocket-test
COPY . .
RUN cargo install --path .

FROM debian:buster-slim
RUN apt-get update && apt-get install -y curl && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/local/cargo/bin/rocket-test /usr/local/bin/rocket-test
COPY ./Rocket.toml /home
EXPOSE 8000

ENV ROCKET_CONFIG=/home/Rocket.toml
CMD ["rocket-test"]