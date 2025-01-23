FROM rust:latest
WORKDIR /app 
COPY Cargo.toml Cargo.toml
COPY src src
RUN cargo build --release

RUN cp /app/target/release/blkcli /usr/local/bin

CMD ["./target/release/blkcli"] 

