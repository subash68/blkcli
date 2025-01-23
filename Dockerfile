FROM rust:latest



WORKDIR /app



COPY Cargo.toml Cargo.toml

COPY src src



RUN cargo build --release



CMD ["./target/release/blkcli"] 

