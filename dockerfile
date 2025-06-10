# syntax=docker/dockerfile:1

ARG RUST_VERSION=1.87
FROM rust:${RUST_VERSION} AS build

WORKDIR /app

# Install build dependencies
RUN apt-get update && apt-get install -y sqlite3 && rm -rf /var/lib/apt/lists/*

# Copy each crate and other resources based on your directory tree
COPY Cargo.toml .
COPY Cargo.lock .
COPY src ./src
COPY api ./api
COPY entity ./entity
COPY migration ./migration
COPY service ./service
COPY static ./static

# Create an empty database
RUN touch quote_server.db

# Initialize the DB by running the app with `-i`
RUN cargo run --bin quote-server -- -i

# Build the release binary
RUN cargo build --release

# Final stage for a minimal runtime container
FROM debian:bookworm-slim

ARG UID=10001
RUN adduser --disabled-password --gecos "" --shell "/sbin/nologin" --uid "${UID}" appuser

USER appuser
WORKDIR /home/appuser

# Copy runtime assets and the binary
COPY --from=build /app/target/release/quote-server /bin/quote-server
COPY --from=build /app/quote_server.db ./quote_server.db
COPY --from=build /app/static ./static
COPY --from=build /app/api/templates ./templates

EXPOSE 3000

CMD ["/bin/quote-server", "--release"]
