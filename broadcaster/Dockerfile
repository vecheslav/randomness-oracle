FROM rust:1.56

COPY ./ ./

RUN apt-get update -y
RUN apt-get install -y libudev-dev
RUN apt-get install -y pkg-config

# Build your program for release
RUN cargo build --release

# Run the binary
CMD ["./target/release/randomness-oracle-broadcaster"]