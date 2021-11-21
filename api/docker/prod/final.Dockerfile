ARG BUILDER_IMG=builder

FROM ${BUILDER_IMG} AS builder 

# final image
FROM debian:buster-slim

# copy the build artifact from the build stage
COPY --from=builder /timeline/target/release/timeline .

# run
CMD ["./timeline"]