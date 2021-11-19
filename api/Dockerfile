FROM rust:latest as build

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

# final image
FROM debian:buster-slim

# copy the build artifact from the build stage
COPY --from=build /timeline/target/release/timeline .

# run
CMD ["./timeline"]
