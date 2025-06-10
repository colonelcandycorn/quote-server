# syntax=docker/dockerfile:1

# This Dockerfile's original author is unknown: maybe Casey
# Bailey or Bastian Gruber. Bart Massey adapted it for this
# project.

ARG RUST_VERSION=1.87

################################################################################
# Create a stage for building the application.

FROM rust:${RUST_VERSION} AS build

# Install host build dependencies.
RUN apt-get install git curl

# Build the application.
# Leverage a cache mount to /usr/local/cargo/registry/
# for downloaded dependencies, a cache mount to /usr/local/cargo/git/db
# for git repository dependencies, and a cache mount to /app/target/ for
# compiled dependencies which will speed up subsequent builds.
# Leverage a bind mount to the src directory to avoid having to copy the
# source code into the container. Once built, copy the executable to an
# output directory before the cache mounted /app/target is unmounted.
RUN --mount=type=bind,source=src,target=src \
    --mount=type=bind,source=Cargo.toml,target=Cargo.toml \
    --mount=type=bind,source=static,target=static \
    --mount=type=bind,source=migration,target=migration \
    --mount=type=bind,source=service,target=service \
    --mount=type=bind,source=api,target=api \
    --mount=type=bind,source=entity,target=entity \
    --mount=type=cache,target=/target/ \
    --mount=type=cache,target=/usr/local/cargo/git/db \
    --mount=type=cache,target=/usr/local/cargo/registry/ \
    cargo build --release && \
    cp target/release/quote-server /quote-server

################################################################################
# run the application with -i once

# Create a non-privileged user that the app will run under.
# See https://docs.docker.com/go/dockerfile-user-best-practices/
ARG UID=10001
RUN adduser \
    --disabled-password \
    --gecos "" \
    --shell "/sbin/nologin" \
    --uid "${UID}" \
    appuser
USER appuser

WORKDIR /home/appuser

COPY --chown=appuser:appuser static ./static
COPY --chown=appuser:appuser migration ./migration
COPY --chown=appuser:appuser service ./service
COPY --chown=appuser:appuser api ./api
COPY --chown=appuser:appuser entity ./entity
COPY --chown=appuser:appuser Cargo.toml ./Cargo.toml
COPY --chown=appuser:appuser Cargo.lock ./Cargo.lock
RUN touch quote_server.db && chown appuser:appuser quote_server.db

# Remember to expose the port that the application listens on
# with -p 3000:300
# This does not do that.
EXPOSE 3000

# What the container should run when it is started.
CMD ["/quote-server", "-i"]
