# build stage: where we create binary
FROM rust:1.81 AS builder

RUN apt update && apt install -y make clang pkg-config libssl-dev protobuf-compiler build-essential git curl llvm make
WORKDIR /smt
COPY . /smt
RUN cargo build --release

# This is the 2nd stage: a very small image where we copy the smt binary."
FROM docker.io/library/ubuntu:22.04
LABEL description="A sparse Merkle tree backend compatible with the Polkadot ecosystem." \
	io.parity.image.type="builder" \
	io.parity.image.authors="yanoctavian" \
	io.parity.image.vendor="yanoctavian" 
WORKDIR /smt
ENV DB_PATH=/data/db
ENV LOG_PATH=/data/logs
COPY --from=builder /smt/target/release/paritydb-smt-backend ./
COPY --from=builder /smt/target/release/rocksdb-smt-backend ./

EXPOSE 8080
VOLUME ["/data"]
# ENTRYPOINT ["/bin/bash", "-c", "/smt/smt"]