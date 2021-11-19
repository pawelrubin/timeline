FROM rust:latest

# create a new empty project
RUN USER=root cargo new --bin timeline
WORKDIR /timeline

# install and cache dependencies
COPY ./Cargo.lock ./Cargo.toml ./
RUN cargo build --release && \ 
    rm src/*.rs && \
    rm ./target/release/deps/timeline*

# copy source code
COPY ./src ./src

# build for release
RUN cargo build --release
