FROM ubuntu:20.04

# Get dependencies
RUN apt-get update && DEBIAN_FRONTEND=noninteractive apt-get install -y \
  cmake \
  build-essential \
  git \
  curl \
  wget ; \
  apt-get autoclean && apt-get clean

# Install rust
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh  -s -- -y
ENV PATH $PATH:/root/.cargo/bin
RUN rustup default nightly

# Prepare env
RUN mkdir /data
ENV DATA_DIR /data
RUN mkdir /res
ENV RES_DIR /res
ENV PHOTOGRAMMETRY_SCRIPT /run.sh
RUN mkdir -p /logs/job

# Build webservice
RUN mkdir /webservice
COPY ./webservice /webservice
RUN cd /webservice && cargo build --release && rm -rf target/debug

COPY ./run.sh /

ENTRYPOINT cd /webservice && cargo run --release