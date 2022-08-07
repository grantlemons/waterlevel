FROM rust
WORKDIR /usr/src/rocket-test
COPY . .
RUN cargo install --path .
CMD ["rocket-test"]