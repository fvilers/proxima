FROM rust:1.56.1 as builder
WORKDIR /usr/src/

# Download the target for static linking.
RUN rustup target add x86_64-unknown-linux-musl

# Create a dummy project and build the app's dependencies.
RUN USER=root cargo new proxima
WORKDIR /usr/src/proxima
COPY Cargo.toml Cargo.lock ./
RUN cargo build --release

# Copy the source and build the application.
COPY src ./src
RUN cargo install --target x86_64-unknown-linux-musl --path .

# Discard symbols from compiled file
RUN strip /usr/local/cargo/bin/proxima

FROM scratch
COPY --from=builder /usr/local/cargo/bin/proxima .
USER 1000
EXPOSE 80
CMD ["./proxima", "-a", "0.0.0.0", "-p", "80"]
