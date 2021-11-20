FROM rust:latest AS builder

# create a new empty project
RUN USER=root cargo new --bin timeline
WORKDIR /timeline

# install and cache dependencies
COPY ./Cargo.lock ./Cargo.toml ./
RUN cargo build && \ 
    rm src/*.rs && \
    rm ./target/debug/deps/timeline*

# copy source code
COPY ./src ./src
COPY ./Rocket.toml ./Rocket.toml

# build the project
RUN cargo build

# final image
FROM debian:buster-slim

# copy the build artifact from the build stage
COPY --from=builder /timeline/target/debug/timeline .
COPY --from=builder /timeline/Rocket.toml .
# run
CMD ["./timeline"]
