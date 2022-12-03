FROM ghcr.io/evanrichter/cargo-fuzz as builder

ADD . /bit-vec
WORKDIR /bit-vec/fuzz
RUN cargo +nightly fuzz build 

FROM debian:bookworm
COPY --from=builder /bit-vec/fuzz/target/x86_64-unknown-linux-gnu/release/bitvec-fuzz /