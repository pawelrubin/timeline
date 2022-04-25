FROM rust:latest AS builder

# Install sccache
RUN cargo install sccache
ENV SCCACHE_CACHE_SIZE="1G"
ENV SCCACHE_DIR=/home/root/.cache/sccache
ENV RUSTC_WRAPPER="/usr/local/cargo/bin/sccache"

# create a new empty project
RUN USER=root cargo new --bin timeline
WORKDIR /timeline

# install and cache dependencies
COPY ./Cargo.lock ./Cargo.toml ./
RUN --mount=type=cache,target=/home/root/.cache/sccache cargo build --release && \
    sccache --show-stats && \
    rm src/*.rs && \
    rm ./target/release/deps/timeline*

# copy source code
COPY ./src ./src
COPY ./Rocket.toml ./Rocket.toml

# build the project witch sccache
RUN --mount=type=cache,target=/home/root/.cache/sccache cargo build --release && sccache --show-stats

# final image
FROM debian:buster-slim

# copy the build artifact from the build stage
COPY --from=builder /timeline/target/release/timeline .
COPY --from=builder /timeline/Rocket.toml .
# run
CMD ["./timeline"]
