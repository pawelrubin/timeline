FROM rust:latest

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
RUN --mount=type=cache,target=$SCCACHE_DIR cargo build --release && \
    sccache --show-stats && \
    rm src/*.rs && \
    rm ./target/release/deps/timeline*

# copy source code
COPY ./src ./src
COPY ./Rocket.toml ./Rocket.toml

# build for release
RUN --mount=type=cache,target=$SCCACHE_DIR cargo build --release && \
    sccache --show-stats
