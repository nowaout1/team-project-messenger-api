# Build stage
FROM rust:slim AS build

WORKDIR /tmp/app

ADD Cargo.toml Cargo.lock ./
ADD src/ ./src
ADD proto/ ./proto
ADD build.rs ./

RUN apt-get update && apt-get install -y \
    protobuf-compiler \
    && rm -rf /var/lib/apt/lists/*

RUN cargo build --release

# Run stage
FROM gcr.io/distroless/cc-debian12

COPY --from=build /tmp/app/target/release/messenger-api ./messenger-api

CMD ["./messenger-api"]
