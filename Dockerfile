FROM rust as builder
WORKDIR /usr/src/waterlevel-backend
COPY . .
RUN cargo install --path .

FROM debian:buster-slim
RUN apt-get update && apt-get install -y curl libpq-dev && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/local/cargo/bin/waterlevel-backend /usr/local/bin/waterlevel-backend
COPY ./Rocket.toml /home
EXPOSE 8000

ENV ROCKET_CONFIG=/home/Rocket.toml
CMD ["waterlevel-backend"]
