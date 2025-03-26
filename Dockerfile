ARG RUST_VERSION=1.85.1
FROM rust:${RUST_VERSION}-bookworm AS builder
RUN apt update && apt dist-upgrade -y && apt install -y cmake libclang-dev
COPY . emtk
ARG PROFILE=release
ENV PROFILE=$PROFILE
RUN cd emtk && make

FROM debian:bookworm-slim
ENTRYPOINT ["/usr/local/bin/emtk"]
RUN apt update && apt dist-upgrade -y \
  && apt-get clean \
  && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/local/cargo/bin/emtk /usr/local/bin/emtk
